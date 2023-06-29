use std::{cell::RefCell, rc::Rc};

use crate::{speech_recogn::speech_handler::PhraseHandler, notify::simply_notify};

const ENABLE_LINTENING : &[&str] = &["открой уши"];
const DISABLE_LINTENING : &[&str] = &["закрой уши"];

pub struct ListeningHandler {
    pub enabled_listening: Rc<RefCell<bool>>,
}

impl ListeningHandler {
    pub fn new(enabled_listening: Rc<RefCell<bool>>) -> ListeningHandler {
        ListeningHandler {
            enabled_listening,
        }
    }
}

impl PhraseHandler for ListeningHandler {
    fn handle_phrase(&mut self, phrase: &String) {
        let mut enabled = self.enabled_listening.borrow_mut();
        if !*enabled && self.match_phrase(phrase, ENABLE_LINTENING) {
            *enabled = true;
            let _ = simply_notify("Уши открыл!");
        }
        else if *enabled && self.match_phrase(phrase, DISABLE_LINTENING) {
            *enabled = false;
            let _ = simply_notify("Уши закрыл!");
        }
    }
    
    fn min_phrases_simmilarity(&self) -> f64 {
        1.0
    }
    
    fn accept_phrases_on_disabled(&self) -> bool {
        true
    }
}