use std::path::PathBuf;

use dirs::document_dir;

pub struct Paths {
    pub prefs: PathBuf,
    pub open_phrases: PathBuf,
    pub lang_learn_file: String,
}

impl Paths {
    pub fn new() -> Paths {
        let prefs = document_dir().unwrap().join("Froggy");
        let open_phrases = prefs.clone().join("OpenPhrases.xlsx");
        let lang_learn_file = prefs.join("EnglishLearning.txt").to_str().unwrap().to_string();
        Paths {
            prefs,
            open_phrases,
            lang_learn_file,
        }
    }
}