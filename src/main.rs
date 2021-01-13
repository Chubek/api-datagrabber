use tide::Request;
use tide::prelude::*;
use mongodb::bson::doc;
mod util;
mod keyword;
mod db;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_article() {
        let article = "Hello. What? Me!";
        let split = util::split_article(article).unwrap();
        assert_eq!(split, vec!["Hello", "What", "Me"]);
    }

    #[test]
    fn test_split_sentence() {
        let sentences = vec!["Hello world!", "Assume the worst about this"];
        let sentences_split = vec![vec!["Hello", "world!"], vec!["Assume", "the", "worst", "about", "this"]];
        let split = util::split_sentence(sentences).unwrap();
        assert_eq!(split, sentences_split);
    }
}



#[async_std::main]
async fn main() -> mongodb::error::Result<()> {
    
    let docs = vec![
    doc! { "title": "1984", "author": "George Orwell" },
    doc! { "title": "Animal Farm", "author": "George Orwell" },
    doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
];

    db::insert_into_db(docs).await?;

    Ok(())

}
