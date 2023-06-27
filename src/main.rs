mod speech_recogn;

use std::thread;

use speech_recogn::{create_recogizer_stream, RecognizerStreamPrefs, RecognizerData};

const RUS_MODEL_PATH: &str = "D:/Varia/Projects/RustProjects/test_speech_recogn/vosk-model-small-ru-0.22";

fn main() {
    let prefs = RecognizerStreamPrefs {
        lang_model_path: RUS_MODEL_PATH.to_string(),
        max_alternatives: 10,
        keep_words: false,
        keep_partial_words: false,
    };
    let (_stream, recognizer_data) = create_recogizer_stream(prefs);
    thread::spawn(|| main_loop(recognizer_data));
    loop { }
}

fn main_loop(data: RecognizerData) {
    loop {
        {
            let mut results_lock = data.results.lock().unwrap();
            let results = &mut results_lock.results;
            for (ir, r) in results.drain(..).enumerate() {
                println!("result ({})", ir);
                for (i, alt) in r.alternatives.iter().enumerate() {
                    println!("\t{} ({})", alt, i);
                }
            }
        }

        std::thread::sleep_ms(100);
    }
}