mod speech_recogn;
mod paths;
mod utils;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use paths::Paths;
use speech_recogn::{
    RecognizerStreamPrefs,
    RecognizerData, RecognitionResults,
    create_recogizer_stream_repeating,
};

use speech_recogn::speech_handler::{
    accept_voice,
    PhraseHandler
};

use speech_recogn::handlers::{
    phrase_logger::PhraseLogger,
    open_phrase_handler::OpenPhraseHandler,
};

const RUS_MODEL_PATH: &str = "D:/Varia/Projects/RustProjects/test_speech_recogn/vosk-model-small-ru-0.22";

fn main() {
    let paths = Paths::new();
    let prefs = RecognizerStreamPrefs {
        lang_model_path: RUS_MODEL_PATH.to_string(),
        max_alternatives: 10,
        keep_words: false,
        keep_partial_words: false,
        min_phrases_simmilarity: 0.7,
    };

    let results = Arc::new(Mutex::new(RecognitionResults::new()));
    let recognizer_data = RecognizerData {
        results,
    };

    let results_clone = recognizer_data.results.clone();
    let prefs_clone = prefs.clone();
    std::thread::spawn(move || {
        create_recogizer_stream_repeating(prefs_clone, results_clone, 100);
    });
    main_loop(recognizer_data, prefs, paths);
}

fn main_loop(mut data: RecognizerData, prefs: RecognizerStreamPrefs, paths: Paths) {
    let mut handlers: Vec<Box<dyn PhraseHandler>> = Vec::new();
    handlers.push(Box::new(PhraseLogger{}));
    handlers.push(Box::new(OpenPhraseHandler::new(paths.open_phrases, prefs.min_phrases_simmilarity)));

    loop {
        accept_voice(&mut data, &mut handlers);
        std::thread::sleep(Duration::from_millis(100));
    }
}