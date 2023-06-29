mod speech_recogn;
mod paths;
mod utils;
mod lang_learning;
mod notify;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use lang_learning::run_learning_cycle;
use paths::Paths;
use speech_recogn::handlers::listening_handler::ListeningHandler;
use speech_recogn::handlers::thanks_handler::ThanksHandler;
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
use tts_rust::languages::Languages;

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
        handling_enabled: Rc::new(RefCell::new(true)),
    };

    let results_clone = recognizer_data.results.clone();
    let prefs_clone = prefs.clone();
    let lang_learn_file_clone = paths.lang_learn_file.clone();


    std::thread::spawn(move || run_learning_cycle(lang_learn_file_clone.clone(), lang_learn_file_clone.clone(), Languages::English));
    std::thread::spawn(move || create_recogizer_stream_repeating(prefs_clone, results_clone, 100));
    main_loop(recognizer_data, prefs, paths);
}

fn main_loop(mut data: RecognizerData, prefs: RecognizerStreamPrefs, paths: Paths) {
    let mut open_phrase_handler = OpenPhraseHandler::new(paths.open_phrases.clone(), prefs.min_phrases_simmilarity);
    open_phrase_handler.append_default_paths(&paths);

    let mut handlers: Vec<Box<dyn PhraseHandler>> = Vec::new();
    handlers.push(Box::new(PhraseLogger{}));
    handlers.push(Box::new(ThanksHandler::new()));
    handlers.push(Box::new(open_phrase_handler));
    handlers.push(Box::new(ListeningHandler::new(data.handling_enabled.clone())));

    loop {
        accept_voice(&mut data, &mut handlers);
        std::thread::sleep(Duration::from_millis(100));
    }
}