use crate::speech_recogn::speech_handler::PhraseHandler;

const SEARCH_KEYS : &[&str] = &["найди в яндексе", "поищи в яндексе"];

pub struct BrowserSearchHandler;

impl BrowserSearchHandler {
    fn trim_request(&self, phrase: &String, key: &str) -> String {
        if phrase.len() <= key.len() {
            return String::with_capacity(0);
        }
        let request = &phrase[key.len()..];
        match request.find(' ') {
            Some(idx) => request[idx+1..].to_string(),
            None => request.to_string(),
        }
    }
}

impl PhraseHandler for BrowserSearchHandler {
    fn handle_phrase(&mut self, phrase: &String) {
        for key in SEARCH_KEYS {
            if self.match_phrase(phrase, &[key]) {
                let search_query = self.trim_request(phrase, key);
                let search_url = format!("https://yandex.ru/search/?text={}", search_query);
                match webbrowser::open(&search_url) {
                    Ok(_) => {},
                    Err(e) => eprintln!("Can`t open browser request with error: {e}"),
                }
            }
        }
    }
    
    fn min_phrases_simmilarity(&self) -> f64 {
        0.9
    }
}