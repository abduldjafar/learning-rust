mod custom_error;

use std::env;
use std::fs::File;
use std::io::Write;

use futures::{stream::StreamExt, TryStreamExt};
use mongodb::{bson::Document, options::ClientOptions, Client, Collection};
use rayon::prelude::*;

const BATCH_SIZE: usize = 1000;
const OUTPUT_FILE: &str = "output"; // Change this to your desired output file

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let client_options = ClientOptions::parse(client_uri).await?;
    let client = Client::with_options(client_options)?;

    let conn: Collection<Document> = client
        .database("sample_airbnb")
        .collection("listingsAndReviews");

    let mut cursor = conn.find(None, None).await?.into_stream();

    // Create a vector to hold documents for each batch
    let mut batch = Vec::new();
    let mut batch_index = 0;

    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(document) => {
                // Add document to the current batch
                batch.push(document);

                // If batch size is reached, process it in parallel
                if batch.len() >= BATCH_SIZE {
                    // Spawn a Tokio task to process the batch
                    let batch_clone = batch.clone();
                    let index_clone = batch_index;

                    tokio::spawn(async move {
                        process_batch(&batch_clone, index_clone).await;
                    });

                    batch.clear();
                    batch_index += 1;
                }
            }
            Err(err) => {
                eprintln!("Error fetching document: {:?}", err);
            }
        }
    }

    if !batch.is_empty() {
        tokio::spawn(async move {
            process_batch(&batch, batch_index).await;
        });
    }

    Ok(())
}

async fn process_batch(batch: &[mongodb::bson::Document], batch_index: i32) {
    // Use Rayon to parallelize document processing within the batch
    batch.par_iter().for_each(|document| {
        process_document(document, batch_index);
    });
}

fn process_document(document: &mongodb::bson::Document, batch_index: i32) {
    // Open the output file in append mode
    if let Ok(mut file) = File::create(format!("{}_{}.json", OUTPUT_FILE, batch_index)) {
        // Write the document content to the file
        if let Ok(document_str) = serde_json::to_string(document) {
            if let Err(err) = writeln!(file, "{}", document_str) {
                eprintln!("Error writing to file: {:?}", err);
            }
        }
    }
}
