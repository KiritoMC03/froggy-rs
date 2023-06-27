use super::RecognizerData;

pub trait PhraseHandler {
    fn handle_phrase(&mut self, phrase: &String);
}

pub fn accept_voice(recognizer: &mut RecognizerData, handlers: &mut Vec<Box<dyn PhraseHandler>>) {
    let results = &mut recognizer.results.lock().unwrap().results;
    for result in results.drain(..) {
        for alt in result.alternatives.iter() {
            if !alt.is_empty() {
                handle_phrase(alt, handlers);
            }
        }
    }
}

pub fn handle_phrase(phrase: &String, handlers: &mut Vec<Box<dyn PhraseHandler>>) {
    if phrase.is_empty() { return }
    for h in handlers {
        h.as_mut().handle_phrase(phrase);
    }
}