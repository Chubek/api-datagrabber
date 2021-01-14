//! Application endpoints.

use rake::*;
use async_std::prelude::*;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use tide::{Request, Response};
use serde::{Serialize, Deserialize};
use http_types::{Body};

use super::state::State;

#[derive(Deserialize, Serialize)]
struct Record {
    title: String,
    text: String,
    text_label: String,
    sentences: Vec<String>,
    sentence_labels: Vec<String>
}

#[derive(Deserialize, Serialize)]
struct Records {
    records: Vec<mongodb::bson::Document>
}

#[derive(Deserialize, Serialize)]
struct Title {
    title: String    
}


fn get_keywords(text: &str) -> Vec<String> {    
    let stop_words_list_path = "src/resources/stopwords.txt";
    let sw = StopWords::from_file(stop_words_list_path).unwrap();
    let r = Rake::new(sw);
    let keywords = r.run(text);

    let mut v: Vec<String> = Vec::new();

    keywords.iter().for_each(
        |&KeywordScore {
            ref keyword,
            ref score,
        }| v.push(keyword.to_string()),
    );

    v

}

/// List all databases
pub(crate) async fn list_dbs(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let names = req.state().mongo().list_database_names(None, None).await?;
    Ok(names.join("\n"))
}

/// Get a single database
pub(crate) async fn list_colls(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let name: String = req.param("db")?.to_string();
    log::info!("accessing database {}", name);
    let db = req.state().mongo().database(&name);
    let collections = db.list_collection_names(None).await?;
    Ok(collections.join("\n"))
}

/// Insert a document into a collection
pub(crate) async fn insert_doc(mut req: Request<State>) -> tide::Result<impl Into<Response>> {
    let name: String = req.param("db")?.to_string();
    log::debug!("accessing database {}", name);
    let db = req.state().mongo().database(&name);

    let name: String = req.param("collection")?.to_string();
    log::debug!("accessing collection {}", name);
    let coll = db.collection(&name);

    let record: Record = req.body_json().await?;


    let keywords = get_keywords(&record.text);

    let sent = record.sentences
            .iter()
            .zip(record.sentence_labels.iter())
            .map(|(x, y)| doc!{ "sentence": x, "label": y })
            .collect::<Vec<mongodb::bson::Document>>();

    let doc = doc! { "title": record.title, "text": record.text, "text_label": record.text_label, "keywords": keywords, "sentences": sent };
    
    let _res = coll.insert_one(doc, None).await?;
    Ok("Insert successful!")
}

/// Insert a document into a collection
pub(crate) async fn find_doc(mut req: Request<State>) -> tide::Result<impl Into<Response>> {
    let name: String = req.param("db")?.to_string();
    log::debug!("accessing database {}", name);
    let db = req.state().mongo().database(&name);

    let name: String = req.param("collection")?.to_string();
    log::debug!("accessing collection {}", name);
    let coll = db.collection(&name);

    let record: Title = req.body_json().await?;


    // Query the documents in the collection with a filter and an option.
    let filter = doc! { "title": &record.title };
    let find_options = FindOptions::builder().sort(doc! { "title": &record.title }).build();
    let mut cursor = coll.find(filter, find_options).await?;

    let mut ret: Vec<mongodb::bson::Document> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                ret.push(document);
            }
            Err(e) => return Err(e.into()),
        }
    }

    let res = Records { records: ret };

    let mut response = Response::new(200);
    response.set_body(Body::from_json(&res)?);

    Ok(response)
}

/// Update a document in the collection.
pub(crate) async fn update_doc(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let name: String = req.param("db")?.to_string();
    log::debug!("accessing database {}", name);
    let db = req.state().mongo().database(&name);

    let name: String = req.param("collection")?.to_string();
    log::debug!("accessing collection {}", name);
    let coll = db.collection(&name);

    // Query the documents in the collection with a filter and an option.
    let filter = doc! { "author": "George Orwell" };

    let other = doc! { "$set": { "title": "[censored]" } };
    coll.find_one_and_update(filter, other, None).await?;
    Ok("update successful!")
}