use anyhow::Result;
use mongodb::Client;

pub async fn establish_connection(mongodb_url: &str) -> Result<Client> {
    let client = Client::with_uri_str(mongodb_url).await?;

    // Optionally ping to verify connection
    client
        .database("admin")
        .run_command(mongodb::bson::doc! {"ping": 1})
        .await?;

    Ok(client)
}
