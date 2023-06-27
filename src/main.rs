mod speech_recogn;

use std::time::Duration;

use speech_recogn::{
    create_recogizer_stream,
    RecognizerStreamPrefs,
    RecognizerData,
    handlers::phrase_logger::PhraseLogger
};

use speech_recogn::speech_handler::{
    accept_voice,
    PhraseHandler
};

const RUS_MODEL_PATH: &str = "D:/Varia/Projects/RustProjects/test_speech_recogn/vosk-model-small-ru-0.22";

fn main() {
    let prefs = RecognizerStreamPrefs {
        lang_model_path: RUS_MODEL_PATH.to_string(),
        max_alternatives: 10,
        keep_words: false,
        keep_partial_words: false,
    };
    let (_stream, recognizer_data) = create_recogizer_stream(prefs);
    main_loop(recognizer_data)
}

fn main_loop(mut data: RecognizerData) {
    let mut handlers: Vec<Box<dyn PhraseHandler>> = Vec::new();
    handlers.push(Box::new(PhraseLogger{}));
    loop {
        accept_voice(&mut data, &mut handlers);
        std::thread::sleep(Duration::from_millis(100));
    }
}