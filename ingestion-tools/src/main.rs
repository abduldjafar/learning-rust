mod mongo_operation;
mod writer;

use clap::Parser;
use mongo_operation::{get_split_keys, get_mongo_datas};
use mongodb::{
    bson::Document, Collection,
};
use std::env;


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
