use rand::seq::SliceRandom;

use crate::{speech_recogn::speech_handler::PhraseHandler, notify::simply_notify};

pub struct ThanksHandler {
    phrases: Vec<PhraseAndAnswer>,
    last_pair_idx: usize,
}

pub struct PhraseAndAnswer {
    p: Vec<String>,
    a: Vec<String>,
}

impl ThanksHandler {
    pub fn new() -> ThanksHandler {
        let phrases = vec![
        PhraseAndAnswer {
            p: vec!["спасибо".into(), "благодарю".into()],
            a: vec!["Не за что =)".into(), "Пожауйста =)".into()],
        },
        ];
        
        ThanksHandler {
            phrases,
            last_pair_idx: usize::MAX,
        }
    }
}

impl PhraseHandler for ThanksHandler {
    fn handle_phrase(&mut self, phrase: &String) {
        for (idx, pair) in self.phrases.iter().enumerate() {
            if self.last_pair_idx == idx {
                continue;
            }
            for key in pair.p.iter() {
                if self.match_phrase(phrase, &[key.as_str()]) {
                    match pair.a.choose(&mut rand::thread_rng()) {
                        Some(answer) => {
                            let _ = simply_notify(&answer);
                            self.last_pair_idx = idx;
                        },
                        None => break,
                    };
                    return;
                }
            }
        }

        self.last_pair_idx = usize::MAX;
    }
}