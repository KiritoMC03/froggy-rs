use std::path::PathBuf;

use dirs::document_dir;

pub struct Paths {
    pub prefs: PathBuf,
    pub open_phrases: PathBuf,
}

impl Paths {
    pub fn new() -> Paths {
        let prefs = document_dir().unwrap().join("Froggy");
        let open_phrases = prefs.clone().join("OpenPhrases.xlsx");
        Paths {
            prefs,
            open_phrases,
        }
    }
}