use std::{path::Path, fs::File, io::{BufReader, BufRead, Write}, time::Duration};

use tts_rust::{ tts::GTTSClient, languages::Languages };

use rand::thread_rng;
use rand::seq::SliceRandom;

const STD_WORDS_DELAY : u64 = 1;
const STD_ITEMS_DELAY : u64 = 3;
const STD_REPEATS_DELAY : u64 = 10;
const STD_INITIAL_DELAY : u64 = 10;

const WORDS_DELAY_KEY : &str = "words_delay";
const ITEMS_DELAY_KEY : &str = "items_delay";
const REPEATS_DELAY_KEY : &str = "repeats_delay";
const INITIAL_DELAY_KEY : &str = "initial_delay";

const SETTINGS_KEY : &str = "`set`";

#[derive(Debug)]
pub struct Translate {
    rus: String,
    learn: String,
}

#[derive(Debug)]
pub struct Settings {
    words_delay : u64,
    items_delay: u64,
    repeats_delay : u64,
    initial_delay : u64,
}

pub fn run_learning_cycle<T: Into<String> + Clone>(settings_path: T, translates_path: T, target_lang: Languages) {
    let rus_speaker: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::Russian,
        tld: "com",
    };

    let lang_speaker: GTTSClient = GTTSClient {
        volume: 1.0,
        language: target_lang,
        tld: "com",
    };

    let settings = load_settings(settings_path.clone());
    std::thread::sleep(Duration::from_secs(settings.initial_delay));
    drop(settings);
    loop {
        let settings = load_settings(settings_path.clone());
        let mut tanslates = load_translates(translates_path.clone());
        tanslates.shuffle(&mut thread_rng());
        let _ = notificate();
        std::thread::sleep(Duration::from_secs(settings.items_delay * 3));
        for item in &tanslates {
            println!("{} - {}", item.rus, item.learn);
            say_trananslate(item, &rus_speaker, &lang_speaker, settings.words_delay);
            std::thread::sleep(Duration::from_secs(settings.items_delay));
        }
        drop(tanslates);
        std::thread::sleep(Duration::from_secs(settings.repeats_delay));
    }
}

pub fn load_translates<T: Into<String>>(path: T) -> Vec<Translate> {
    let mut result = Vec::new();
    let file = open_translates(path.into());
    for (i, line) in BufReader::new(file).lines().enumerate() {
        match line {
            Ok(str) => {
                if str.contains(SETTINGS_KEY) {
                    continue;
                }
                if let Some(trsl) = Translate::from_split(str.split("-")) {
                    result.push(trsl);
                }
            },
            Err(e) => eprintln!("Can`t read line number {} with error {e}", i + 1),
        }
    }

    result
}

pub fn load_settings<T: Into<String>>(path: T) -> Settings {
    let mut settings = Settings {
        words_delay: STD_WORDS_DELAY,
        items_delay: STD_ITEMS_DELAY,
        repeats_delay: STD_REPEATS_DELAY,
        initial_delay: STD_INITIAL_DELAY,
    };

    let mut map = [
        (WORDS_DELAY_KEY, &mut settings.words_delay),
        (ITEMS_DELAY_KEY, &mut settings.items_delay),
        (REPEATS_DELAY_KEY, &mut settings.repeats_delay),
        (INITIAL_DELAY_KEY, &mut settings.initial_delay),
        ];

    {
        let file = open_translates(path.into());
        let reader = BufReader::new(file);
        for str in reader.lines() {
            if let Ok(data) = str {
                for (key, prop) in map.iter_mut() {
                    if data.contains(*key) {
                        let parsed = data.clone()
                                    .chars()
                                    .filter(is_settings_valid_char)
                                    .collect::<String>()
                                    .parse::<u64>();
                        if let Ok(t) = parsed {
                            **prop = t;
                        }
                    }
                }
            }
        }
    }

    settings
}

pub fn open_translates(path: String) -> File {
    if !Path::new(path.as_str()).exists(){
        match File::create(path.clone()) {
            Ok(mut f) => {
                let _ = f.write(format!("{} {} - {}\n", SETTINGS_KEY, WORDS_DELAY_KEY, STD_WORDS_DELAY).as_bytes());
                let _ = f.write(format!("{} {} - {}\n", SETTINGS_KEY, ITEMS_DELAY_KEY, STD_ITEMS_DELAY).as_bytes());
                let _ = f.write(format!("{} {} - {}\n", SETTINGS_KEY, REPEATS_DELAY_KEY, STD_REPEATS_DELAY).as_bytes());
                let _ = f.write(format!("{} {} - {}", SETTINGS_KEY, INITIAL_DELAY_KEY, STD_INITIAL_DELAY).as_bytes());
                return f;
            },
            Err(e) => eprintln!("Can`t create default translates file with error: {}", e),
        };
    }

    File::open(path).unwrap()
}

pub fn say_trananslate(target: &Translate, rus_sp: &GTTSClient, speaker: &GTTSClient, delay: u64) {
    match rus_sp.speak(target.rus.as_str()) {
        Ok(_) => {
            std::thread::sleep(Duration::from_secs(delay));
            let _ = speaker.speak(target.learn.as_str());
        },
        Err(_) => { },
    }
}

impl Translate {
    pub fn from_split(mut split: std::str::Split<&str>) -> Option<Translate> {
        if let Some(rus) = split.next() {
            if let Some(eng) = split.next() {
                return Some(Translate {
                    rus: rus.to_string(),
                    learn: eng.to_string(),
                });
            }
        }

        None
    }
}

fn is_settings_valid_char(c: &char) -> bool {
    c.is_numeric()
}

pub fn notificate() -> Result<(), notify_rust::error::Error> {
    custom_notificate("English speaker", "LEARN FUCKING ENGLISH!")
}

pub fn custom_notificate(appname: &str, bodyname: &str) -> Result<(), notify_rust::error::Error> {
    notify_rust::Notification::new()
        .appname(appname)
        .body(bodyname)
        .show()
}