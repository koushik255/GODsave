use dioxus::prelude::*;
use serde::Serialize;

#[cfg(feature = "server")]
use rusqlite::Connection;

#[derive(serde::Deserialize, Serialize, Debug)]
pub struct FruitCall {
    pub id: i64,
    pub name: String,
    pub roman_name: String,
    pub r#type: String,
    pub description: String,
    pub filename: String,
    pub technicalFile: String,
}

#[derive(serde::Deserialize, Serialize, Debug, Clone)]
pub struct Save {
    pub name: String,
    pub link: String,
    pub id: i64,
}

#[server(Echo)]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[server(Name)]
pub async fn name() -> Result<String, ServerFnError> {
    let blud = "koushik (This is from server)".to_string();

    Ok(blud)
}

#[server(Df)]
pub async fn df() -> Result<FruitCall, ServerFnError> {
    let fruit = reqwest::get("https://api.api-onepiece.com/v2/fruits/en/39")
        .await
        .unwrap()
        .json::<FruitCall>()
        .await
        .unwrap();

    Ok(fruit)
}

#[server(Input)]
pub async fn input(y: String) -> Result<String, ServerFnError> {
    println!("From frontend NAME: {}", y);
    let x = format!("sup dude from input {}", y);

    Ok(x)
}

#[server(Link)]
pub async fn link(y: String) -> Result<String, ServerFnError> {
    println!("From frontend LINK : {}", y);
    let x = format!("sup dude from input {}", y);

    Ok(x)
}

#[server(Whole)]
pub async fn whole(name: String, link: String) -> Result<Save, ServerFnError> {
    let id = 1;
    let new_save = Save { name, link, id };
    println!("{:?}", new_save);

    Ok(new_save)
}

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("saver.db").expect("db err blud");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS saves (
                id   INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                link TEXT NOT NULL
            );"
        ).unwrap();

        conn
    };
}

#[server(SaveDb)]
pub async fn save(sda: Save) -> Result<(), ServerFnError> {
    DB.with(|db| {
        db.execute(
            "INSERT INTO saves (name, link) VALUES (?1, ?2)",
            (&sda.name, &sda.link),
        )
    })?;
    Ok(())
}
