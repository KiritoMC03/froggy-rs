use crate::utils::str_simmilarity;

use super::RecognizerData;

pub trait PhraseHandler {
    fn handle_phrase(&mut self, phrase: &String);

    fn handle_empty_tick(&mut self) {}

    fn min_phrases_simmilarity(&self) -> f64 {
        0.0
    }

    fn accept_phrases_on_disabled(&self) -> bool {
        false
    }

    fn match_phrase(&self, phrase: &String, patterns: &[&str]) -> bool {
        for pat in patterns {
            if phrase.contains(pat) {
                return true;
            }

            for word in phrase.split(' ') {
                if str_simmilarity(word, pat) > self.min_phrases_simmilarity() {
                    return true;
                }
            }
        }

        false
    }
}

pub fn accept_voice(recognizer: &mut RecognizerData, handlers: &mut Vec<Box<dyn PhraseHandler>>) {
    let results = &mut recognizer.results.lock().unwrap().results;
    if results.len() == 0 {
        handle_empty_tick(handlers);
        return;
    }
    for result in results.drain(..) {
        for alt in result.alternatives.iter() {
            if !alt.is_empty() {
                let handling_enabled = recognizer.handling_enabled.borrow().clone();
                handle_phrase(alt, handlers, handling_enabled);
            }
        }
    }
}

pub fn handle_phrase(phrase: &String, handlers: &mut Vec<Box<dyn PhraseHandler>>, handling_enabled: bool) {
    if phrase.is_empty() { return }
    for h in handlers {
        if handling_enabled || h.accept_phrases_on_disabled() {
            h.as_mut().handle_phrase(phrase);
        }
    }
}

fn handle_empty_tick(handlers: &mut Vec<Box<dyn PhraseHandler>>) {
    for h in handlers {
        h.as_mut().handle_empty_tick();
    }
}