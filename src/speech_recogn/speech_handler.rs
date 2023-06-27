use super::RecognizerData;

pub fn accept_voice(recognizer: &mut RecognizerData) {
    let results = &mut recognizer.results.lock().unwrap().results;
    for (ir, result) in results.drain(..).enumerate() {
        println!("result ({})", ir);
        for (i, alt) in result.alternatives.iter().enumerate() {
            println!("\t{} ({})", alt, i);
        }
    }
}