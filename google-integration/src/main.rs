use std::fs::File;
use std::io::Read;
use reqwest;
use google_authenticator::GoogleAuthenticator::Credentials;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials from JSON key file
    let json_key_path = "path/to/your/key.json";
    let credentials = Credentials::from_file(json_key_path)?;

    // Replace with your own values
    let project_id = "your-project-id";
    let dataset_id = "your-dataset-id";
    let table_id = "your-table-id";
    let source_uri = "gs://your-bucket/file.json";
    let schema_file_path = "path/to/your/schema.json";

    let url = format!(
        "https://www.googleapis.com/upload/bigquery/v2/projects/{}/jobs",
        project_id
    );

    let client = reqwest::blocking::Client::new();

    // Read schema from the schema file
    let mut schema_content = String::new();
    File::open(schema_file_path)?.read_to_string(&mut schema_content)?;

    let job_config = json!({
        "load": {
            "sourceUris": [source_uri],
            "destinationTable": {
                "projectId": project_id,
                "datasetId": dataset_id,
                "tableId": table_id,
            },
            "schema": {
                "fields": serde_json::from_str(&schema_content)?
            },
            // Other job configuration options can be added here
        }
    });

    let request_body = json!({
        "configuration": {
            "load": job_config,
        }
    });

    let response = client
        .post(&url)
        .bearer_auth(credentials.access_token())
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()?;

    let response_json: serde_json::Value = response.json()?;

    println!("{:#?}", response_json);

    Ok(())
}
