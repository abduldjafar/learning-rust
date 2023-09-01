use bson::{doc, Bson};
use futures::StreamExt;
use google_cloud_storage::client::{Client, ClientConfig};
use mongodb::{
    bson::{self, oid::ObjectId, Document},
    Collection,
};
use rayon::prelude::*;

use crate::writer::{DataWriter, GcsStorage};

pub async fn get_split_keys(
    db: mongodb::Database,
    database: String,
    collection: String,
    batch_size_in_mb: i32,
) -> Result<Vec<(ObjectId, ObjectId)>, Box<dyn std::error::Error>> {
    env_logger::init();

    let split_vector_command = doc! {
        "splitVector": format!("{}.{}", database, collection),
        "keyPattern": doc! { "_id": Bson::Int32(1) },
        "force": Bson::Boolean(false),
        "maxChunkSize": batch_size_in_mb
    };

    log::info!("getting partition keys...");
    
    let result = db.run_command(split_vector_command, None).await?;
    let option_bson = result.get("splitKeys");

    let unwrapped_keys: Option<Vec<ObjectId>> = option_bson.and_then(|bson| {
        bson.as_array().map(|array| {
            array
                .par_iter() // Use par_iter() to iterate in parallel
                .filter_map(|item| {
                    if let Bson::Document(doc) = item {
                        doc.get("_id").and_then(Bson::as_object_id)
                    } else {
                        None
                    }
                })
                .collect()
        })
    });

    let unwrapped_keys = unwrapped_keys.unwrap_or_default();

    let tuple_vector: Vec<(ObjectId, ObjectId)> = unwrapped_keys
        .par_iter()
        .zip(unwrapped_keys.par_iter().skip(1))
        .map(|(&current_key, &next_key)| (current_key.clone(), next_key.clone()))
        .collect();

    log::info!("getting partition keys Done");

    Ok(tuple_vector)
}

pub async fn get_mongo_datas(
    tuple_object_id: (ObjectId, ObjectId),
    conn: Collection<Document>,
    output: String,
    index: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let query = doc! {
        "_id": {
            "$gte": tuple_object_id.0,
            "$lt": tuple_object_id.1
        }
    };

    let mut datas: String = Default::default();
    let config = ClientConfig::default().with_auth().await?;

    let gcs_storage = GcsStorage {
        bucket: String::from("quipper-fact-dev"),
        client: Client::new(config),
    };

    let writer = DataWriter::new(gcs_storage);

    let mut cursor = conn.find(query, None).await?;

    log::info!("processing batch {:?}...", index);

    while let Some(doc) = cursor.next().await {
        let real_doc = doc?;
        let json_str = serde_json::to_string(&real_doc)?;

        datas.push_str(&format!("{}\n", json_str));
    }

    writer
        .write_data(
            &format!("data/from_rust/{}_{}.json", &output, index),
            &datas,
        )
        .await?;

    log::info!("===================================================================");
    log::info!(
        "write to gs://{}data/from_rust/{}_{}.json Done",
        "quipper-fact-dev", &output, index
    );

    Ok(())
}
