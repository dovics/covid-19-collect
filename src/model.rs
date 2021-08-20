use std::convert::Into;
use std::result::Result;

use num_enum::IntoPrimitive;
use tokio_postgres::Client;
use chrono::{DateTime, Local};
use serde_json::Value;

use crate::errors::{PostgresError, PostgresErrorKind};

#[derive(Debug, Eq, PartialEq, IntoPrimitive)]
#[repr(usize)]
enum PostgresSqlIndex {
    CreateTable = 0,
    Insert = 1,
    SelectByContry = 2,
}

const SQL_STRING: [&'static str; 3] = [
    r#"CREATE TABLE IF NOT EXISTS basic.test_json (
        create_time TIMESTAMPTZ,
        contry VARCHAR(20),
        json_data JSON
    )"#,
    r#"INSERT INTO basic.test_json(create_time, contry, json_data) VALUES ($1, $2, $3)"#,
    r#"SELECT create_time, contry, json_data FROM basic.test_json WHERE contry = $1"#,
];

pub async fn create_table(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let index: usize = PostgresSqlIndex::CreateTable.into();
    let _affect = client.execute(SQL_STRING[index], &[]).await?;

    Ok(())
}

#[derive(Debug)]
pub struct CovidInfo {
    time: DateTime<Local>,
    contry: String,
    data: Value,
}

pub async fn insert(client: &Client, time: DateTime<Local>, contry: &str, data: Value) -> Result<(), Box<dyn std::error::Error>> {
    let index: usize = PostgresSqlIndex::Insert.into();
    let affect = client.execute(SQL_STRING[index], &[&time, &contry, &data]).await?;
    if affect == 0 {
        return Err(Box::new(PostgresError{kind: PostgresErrorKind::InsertData, message: "insert affect 0 rows".to_string()}));
    }

    Ok(())
}

pub async fn select_by_contry(client: &Client, contry: &str) -> Result<Vec<CovidInfo>, Box<dyn std::error::Error>> {
    let index: usize = PostgresSqlIndex::SelectByContry.into();
    let rows = client.query(SQL_STRING[index], &[&contry]).await?;
    let mut result: Vec<CovidInfo> = Vec::new();
    for row in rows {
        let time: DateTime<Local> = row.get(0);
        let contry: String = row.get(1);
        let data: Value = row.get(2);

        result.push(CovidInfo{time, contry, data})
    }
    Ok(result)
}