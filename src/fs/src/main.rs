use serde::{Deserialize, Serialize};
use serde_json::{json, Result};
use std::process::Command;
use tokio::fs;
// use uuid::Uuid;

#[tokio::main]
async fn main() {
  // write_to_file().await;
  // read_and_parse_json().await;
  // rust_cmd();
  read_and_manipulate_articles().await;
}

#[derive(Serialize, Deserialize, Debug)]
struct Article {
  id: String,
  articleName: String,
  articleDescription: String,
  articleData: String,
  articleState: String,
  articleCreationDate: String,
  articleLastUpdateDate: String,
  articleCategories: Vec<String>,
}

async fn read_and_manipulate_articles() {
  let path = "articles.json";

  //* read the content of the file */
  let content = fs::read_to_string(path).await.unwrap();

  //* Convert the content to array of structs */
  let mut p: Vec<Article> = serde_json::from_str(&content).unwrap();

  //* Modify the vector */
  // get the index of the wanted object
  let index = p.iter().position(|r| r.articleName == "article 4").unwrap();
  p[index].articleCategories.push("cat1".to_string());

  //* Reconvert the content to json again */
  let json = serde_json::to_string(&p).unwrap();

  //* Write the new json to the file */
  fs::write(path, json).await.unwrap();
}

fn rust_cmd() {
  let mut cmd = Command::new("sh");
  cmd.arg("./run.sh");
  match cmd.output() {
    Ok(_) => println!("Successfully Build"),
    Err(e) => println!("Error Occurred {:?}", e),
  }
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
  name: String,
  age: u8,
  phones: Vec<String>,
}

async fn read_and_parse_json() {
  let path = "articles.json";
  let content = fs::read_to_string(path).await.unwrap();
  let p: Person = serde_json::from_str(&content).unwrap();
  println!("{:?}", p);
}

async fn write_to_file() {
  let path = "articles.json";
  let value = r#"{
      "name": "John Doe",
      "age": 43,
      "phones": [
          "+44 1234567",
          "+44 2345678"
      ]
  }"#;

  fs::write(path, value).await.unwrap();
}
