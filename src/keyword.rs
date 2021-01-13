use rake::*;

pub fn get_keywords(text: &str) -> Vec<String> {    
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