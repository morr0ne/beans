use anyhow::Result;
use beans::Client;

fn main() -> Result<()> {
    let client = Client::new();

    let res = client.get()?;
    println!("{:#?}", res);

    let res = client.uuid()?;
    println!("uuid: {}", res.uuid);

    Ok(())
}
