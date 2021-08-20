use std::env;

use chrono::Local;
use serde_json::Value;
use tokio_postgres::NoTls;

pub mod errors;
mod model;

const CONRTY_LIST: [&str; 10] = [
    "Australia",
    "Brazil",
    "Canada",
    "China",
    "France",
    "Germany",
    "India",
    "Japan",
    "United Kingdom",
    "US",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = env::var("POSTGRES_HOST")?;
    let user = env::var("POSTGRES_USER")?;
    let password = env::var("POSTGRES_PASSWORD")?;
    let db = env::var("POSTGRES_DB")?;
    let port = env::var("POSTGRES_PORT")?;

    let (client, connection) = tokio_postgres::connect(
        format!("host={} user={} password={} dbname={} port={}", host, user, password, db, port),
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    model::create_table(&client).await?;

    for contry in CONRTY_LIST.iter() {
        let response = reqwest::get(format!("https://covid-api.mmediagroup.fr/v1/cases?country={}", contry))
            .await?
            .json::<Value>()
            .await?;

        model::insert(&client, Local::now(), contry, response).await?;
    }

    let infos = model::select_by_contry(&client, "China").await?;
    println!("{:?}", infos);
    Ok(())
}
