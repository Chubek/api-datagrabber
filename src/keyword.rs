use rake::*;

pub fn get_keywords(text: &str) -> Vec<&str> {    
    let stop_words_list_path = "resources/stopwords.txt";
    let sw = StopWords::from_file(stop_words_list_path).unwrap();
    let r = Rake::new(sw);
    let keywords = r.run(text);

    let mut v: Vec<&str>;

    for i in 0..5 {
        v.push(&keywords[i].keyword);
    }

    v

}