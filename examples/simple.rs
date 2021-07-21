use anyhow::Result;
use beans::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();

    let res = client.get().await?;
    println!("{:#?}", res);

    let res = client.uuid().await?;
    println!("uuid: {}", res.uuid);

    Ok(())
}
