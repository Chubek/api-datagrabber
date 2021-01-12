mod util;
mod keyword;

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


fn main() {
    
    let text = "However, just to be safe, let's enable Geolocation API as well";

    let kws = keyword::get_keywords(text);

    for kw in kws {
        println!("{}", kw);
    }

}
