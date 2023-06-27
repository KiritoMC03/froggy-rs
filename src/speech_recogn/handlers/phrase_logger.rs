use crate::speech_recogn::speech_handler::PhraseHandler;

pub struct PhraseLogger;

impl PhraseHandler for PhraseLogger {
    fn handle_phrase(&mut self, phrase: &String) where Self: Sized {
        println!("Phrase variant: {}", phrase);
    }
}