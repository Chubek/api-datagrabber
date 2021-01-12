pub fn split_article(article: &'static str) -> Option<Vec<&'static str>> {
    
    if article.len() < 1 {
        None
    } else {

        let mut v: Vec<&str> = article
                    .trim()
                    .split(|c| c == '.' || c == '!' || c == '?')
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();

        v.retain(|&s| !s.is_empty());

        Some(v)
    }
}

pub fn split_sentence(sentence: Vec<&'static str>) -> Option<Vec<Vec<&'static str>>> {
    
    if sentence.len() < 1 {
        None
    } else {

        let words = sentence
                    .iter()
                    .map(|x| x.split(" ").collect())
                    .collect();


        Some(words)
    }
}