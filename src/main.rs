mod speech_recogn;

use speech_recogn::{create_recogizer_stream, RecognizerStreamPrefs, RecognizerData, speech_handler::accept_voice};

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
    loop {
        accept_voice(&mut data);
        std::thread::sleep_ms(100);
    }
}