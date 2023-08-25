use bson::{doc, Bson};
use clap::Parser;
use futures::StreamExt;
use mongodb::{
    bson::{self, oid::ObjectId, Document}, Collection,
};
use std::{env, fs::File};
use std::io::Write;

/// ... (Args struct and other imports)
/// CLI arguments structure
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Page number
    #[arg(short, long, help = "mongodb database")]
    database: String,

    #[arg(short, long, help = "mongodb collection")]
    collection: String,

    #[arg(short, long, help = "prefix file name for output")]
    prefix_output_file: String,

    #[arg(short, long, help = "batch size data for processing")]
    batch_size_in_mb: i32,
}


async fn get_split_keys(
    db: mongodb::Database,
    database: String,
    collection: String,
    batch_size_in_mb: i32,
) -> Result<Vec<(ObjectId, ObjectId)>, Box<dyn std::error::Error>> {
    // Construct the splitVector command document
    let split_vector_command = doc! {
        "splitVector": format!("{}.{}", database, collection),
        "keyPattern": doc! { "_id": Bson::Int32(1) },
        "force": Bson::Boolean(false),
        "maxChunkSize": batch_size_in_mb
    };

    let result = db.run_command(split_vector_command, None).await?;
    let option_bson = result.get("splitKeys");

    let unwrapped_keys: Option<Vec<ObjectId>> = option_bson.and_then(|bson| {
        bson.as_array().map(|array| {
            array
                .iter()
                .filter_map(|item| {
                    if let Bson::Document(doc) = item {
                        let object_id = doc.get("_id").unwrap().as_object_id();
                        object_id
                    } else {
                        None
                    }
                })
                .collect()
        })
    });

    let unwrapped_keys = unwrapped_keys.unwrap_or_default();
    let mut tuple_vector = Vec::new();

    for (index, _) in unwrapped_keys.iter().enumerate() {
        match unwrapped_keys.get(index + 1) {
            Some(next_key) => {
                tuple_vector.push((unwrapped_keys[index].clone(), next_key.clone()));
            }
            None => break,
        }
    }

    Ok(tuple_vector)
}

async fn get_mongo_datas(
    tuple_object_id: (ObjectId, ObjectId),
    conn: Collection<Document>,
    output: String,
    index: i32
) -> Result<(), Box<dyn std::error::Error>> {
    let query = doc! {
        "_id": {
            "$gte": tuple_object_id.0,
            "$lt": tuple_object_id.1
        }
    };

    let mut cursor = conn.find(query, None).await?;

    println!("processing batch {:?}...", index);
    let mut file = File::create(format!("{}_{}.json",&output,index))?;

    while let Some(doc) = cursor.next().await {
        let real_doc = doc?;
        let json_str = serde_json::to_string(&real_doc)?;
        writeln!(file, "{}", json_str)?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let client = mongodb::Client::with_uri_str(client_uri).await?;

    let conn: Collection<Document> = client.database(&args.database).collection(&args.collection);

    let db: mongodb::Database = client.database(&args.database);
    let db_clone = args.database.clone();
    let collection_clone = args.collection.clone();

    let splitted_keys = get_split_keys(db, db_clone, collection_clone,args.batch_size_in_mb).await?;

    let mut tasks = Vec::new();
    
    let mut index = 0;

    for key in splitted_keys {
        let conn_clone = conn.clone();
        let output_clone = args.prefix_output_file.clone();

        let join_handle = tokio::spawn(async move {
            let result = get_mongo_datas(key, conn_clone,output_clone,index).await;
            match result {
                Ok(result) => result,
                Err(err) => println!("error:{}", err),
            }
        });
        tasks.push(join_handle);
        index+=1;
    }

    for task in tasks {
        task.await?;
    }

    Ok(())
}
