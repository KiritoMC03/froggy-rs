use std::{path::{Path, PathBuf}, io::BufReader, fs::File};

use calamine::{open_workbook, Xlsx, Reader, Error};

use crate::{speech_recogn::speech_handler::PhraseHandler, paths::Paths};

const OPEN_KEYS : &[&str] = &["открой", "запусти", "включи"];
const PREFS_FOLDER_KEY: &str = "папка настроек";

type ComandKeys = Vec<String>;

pub struct OpenPhraseHandler {
    commands: Vec<OpenCommand>,
    last_command_key: ComandKeys,
    phrases_xlsx: PathBuf,
    min_phrases_simmilarity: f64,
}

#[derive(Debug, Clone)]
struct OpenCommand {
    keys: ComandKeys,
    path: String,
}

struct OpenCommandsXlsxIterator {
    workbook: Xlsx<BufReader<File>>,
    line: usize,
}

impl OpenPhraseHandler {
    pub fn new(phrases_xlsx: PathBuf, min_phrases_simmilarity: f64) -> OpenPhraseHandler {
        let mut result = OpenPhraseHandler {
            commands: Vec::new(),
            last_command_key: vec![],
            phrases_xlsx,
            min_phrases_simmilarity,
        };
        
        if let Ok(iter) = OpenCommandsXlsxIterator::new(result.phrases_xlsx.clone()) {
            for command in iter {
                result.commands.push(command.clone());
            }
        }
        else {
            eprintln!("Can`t iterate rows of phrases_xlsx");
        }

        result
    }

    pub fn append_default_paths(&mut self, paths: &Paths) {
        self.commands.push(OpenCommand {
            keys: vec![PREFS_FOLDER_KEY.to_string()],
            path: paths.prefs.to_str().unwrap().to_string(),
        });
    }

    fn open(&self, path: &String) {
        let path = Path::new(path);
        if path.exists() {
            match opener::open(path) {
                Ok(_) => { },
                Err(e) => {eprintln!("Can`t open with error: {}", e)},
            }
        }
    }
}

impl OpenCommand {
    pub fn new<T: Into::<String>>(keys: ComandKeys, path: T) -> OpenCommand {
        OpenCommand {
            keys,
            path: path.into(),
        }
    }
}

impl PhraseHandler for OpenPhraseHandler {
    fn handle_phrase(&mut self, phrase: &String) {
        if self.match_phrase(phrase, &OPEN_KEYS) {
            for command in self.commands.iter() {
                if command.keys == self.last_command_key {
                    continue;
                }
                for key in command.keys.iter() {
                    if self.match_phrase(phrase, &[key.as_str()]) {
                        self.last_command_key = command.keys.clone();
                        self.open(&command.path);
                        return;
                    }
                }
            }
            return;
        }
    }

    fn handle_empty_tick(&mut self) {
        self.last_command_key = vec![];
    }

    fn min_phrases_simmilarity(&self) -> f64 {
        self.min_phrases_simmilarity
    }
}

impl OpenCommandsXlsxIterator {
    pub fn new(path: PathBuf) -> Result<OpenCommandsXlsxIterator, Error> {
        let workbook = open_workbook(path)?;
        Ok(OpenCommandsXlsxIterator {
            workbook,
            line: 0
        })
    }

    fn item_ok(key: &ComandKeys, path: &String) -> bool {
        !key.is_empty() && !path.is_empty()
    }

    fn parse_keys_str(str: String) -> ComandKeys {
        let str : String = str.as_str().chars().filter(|c| c.is_alphabetic() || *c == ' ').collect();
        let mut keys = Vec::new();
        for part in str.split(' ') {
            if part.is_empty() || part == " " {
                continue;
            }
            keys.push(part.to_string());
        }

        keys
    }
}

impl Iterator for OpenCommandsXlsxIterator {
    type Item = OpenCommand;

    fn next(&mut self) -> Option<Self::Item> {
        let worksheets = self.workbook.worksheets();
        let worksheet = worksheets.first().expect("Cannot find first worksheet");
        for row in worksheet.1.rows().skip(self.line) {
            if row.len() < 2 {
                continue;
            }

            let items = row.to_vec();
            let keys = Self::parse_keys_str(items[0].to_string());
            let path = items[1].to_string();
            self.line += 1;
            if Self::item_ok(&keys, &path) {
                return Some(OpenCommand::new(keys, path));
            }            
        }
        
        None
    }
}