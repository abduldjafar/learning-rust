use std::env;
use std::fs::File;
use std::io::Write;

use clap::Parser;
use futures::{stream::StreamExt, TryStreamExt};
use mongodb::{bson::Document, options::ClientOptions, Client, Collection};
use tokio::task::JoinHandle;

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
    batch_size: usize,
    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let client_options = ClientOptions::parse(client_uri).await?;
    let client = Client::with_options(client_options)?;

    let conn: Collection<Document> = client
        .database(args.database.as_str())
        .collection(args.collection.as_str());

    let mut cursor = conn.find(None, None).await?.into_stream();

    // Create a vector to hold documents for each batch
    let mut batch = Vec::new();
    let mut batch_index = 0;

    let mut join_handles: Vec<JoinHandle<()>> = Vec::new();

    while let Some(doc) = cursor.next().await {
        let document = doc?;

        batch.push(document);

        if batch.len() >= args.batch_size {
            let batch_clone = batch.clone();
            let index_clone = batch_index;
            let prefix_output_file_clone = args.prefix_output_file.clone();


            let join_handle = tokio::spawn(async move {
                let process = process_batch(batch_clone, index_clone,prefix_output_file_clone).await;
                match process {
                    Ok(process) => process,
                    Err(err) =>println!("{}",err)
                }
            });

            join_handles.push(join_handle);

            batch.clear();
            batch_index += 1;
        }
    }

    if !batch.is_empty() {
        let join_handle = tokio::spawn(async move {
            let process = process_batch(batch, batch_index,args.prefix_output_file).await;
            match process {
                Ok(process) => process,
                Err(err) =>println!("{}",err)
            }
        });
        join_handles.push(join_handle);
    }

    for join_handle in join_handles {
        join_handle.await?;
    }

    Ok(())
}


async fn process_batch(batch: Vec<mongodb::bson::Document>, batch_index: i32,output_prefix:String) ->  Result<(),Box<dyn std::error::Error>> {
    let mut join_handles: Vec<JoinHandle<()>> = Vec::new();

    /** 
    batch.iter().for_each(|document| {
        let document_clone = document.clone();
        let prefix_output_file = output_prefix.clone();

        let result = process_document(&document_clone, batch_index,prefix_output_file.as_str()).await ;
            match result {
                Ok(result) => result,
                Err(err) =>  println!("{}", err),
            }

        
    });
    */

    println!("{}",batch.len());



    Ok(())
}

async fn process_document(document: &mongodb::bson::Document, batch_index: i32,output_prefix:&str) -> Result<(),Box<dyn std::error::Error>>{
    //let mut file = File::create(format!("{}_{}.json", output_prefix, batch_index))?;
    let document_str  = serde_json::to_string(document)?;
    //writeln!(file, "{}", document_str)?;

    Ok(())
}
