use speech_recogn::{run_recogizer_thread, RecogrizerThreadPrefs};

mod speech_recogn;

fn main() {
    let model_path = "D:/Varia/Projects/RustProjects/test_speech_recogn/vosk-model-small-ru-0.22";
    let prefs = RecogrizerThreadPrefs {
        lang_model_path: model_path.to_string(),
        max_alternatives: 10,
        keep_words: false,
        keep_partial_words: false,
    };
    let stream = run_recogizer_thread(prefs);
    loop {

    }
    drop(stream);
}