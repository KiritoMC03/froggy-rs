pub mod speech_handler;

use std::sync::{Arc, Mutex};

use cpal::{Sample, ChannelCount, traits::{HostTrait, DeviceTrait, StreamTrait}, SampleFormat, Device, SupportedStreamConfig, Stream};
use dasp::sample::ToSample;
use vosk::{Recognizer, DecodingState, Model, CompleteResultMultiple};

#[derive(Clone)]
pub struct RecognizerStreamPrefs {
    pub lang_model_path: String,
    pub max_alternatives: u16,
    pub keep_words: bool,
    pub keep_partial_words: bool,
}

pub struct RecognizerData {
    pub results: Arc<Mutex<RecognitionResults>>,
}

pub struct RecognitionResults {
    pub results: Vec<RecognitionResult>
}

pub struct RecognitionResult {
    pub alternatives: Vec<String>,
}

impl RecognitionResults {
    fn new() -> RecognitionResults {
        RecognitionResults {
            results: Vec::new(),
        }
    }
}

impl From<CompleteResultMultiple<'_>> for RecognitionResult {
    fn from(value: CompleteResultMultiple<'_>) -> Self {
        let mut result = RecognitionResult {
            alternatives: Vec::with_capacity(value.alternatives.len()),
        };

        for alt in value.alternatives {
            result.alternatives.push(alt.text.to_string());
        }

        result
    }
}

pub fn create_recogizer_stream(prefs: RecognizerStreamPrefs) -> (Stream, RecognizerData) {
    let audio_input_device = cpal::default_host()
        .default_input_device()
        .expect("No input device connected");

    let config = audio_input_device
        .default_input_config()
        .expect("Failed to load default input config");
    let channels = config.channels();

    let model = Model::new(prefs.lang_model_path).expect("Could not create the model");
    let mut recognizer = Recognizer::new(&model, config.sample_rate().0 as f32).expect("Could not create the Recognizer");

    recognizer.set_max_alternatives(prefs.max_alternatives);
    recognizer.set_words(prefs.keep_words);
    recognizer.set_partial_words(prefs.keep_partial_words);

    let recognizer = Arc::new(Mutex::new(recognizer));
    let results = Arc::new(Mutex::new(RecognitionResults::new()));
    let stream = create_stream(config, audio_input_device, recognizer.clone(), results.clone(), channels);

    stream.play().expect("Could not play stream");
    (
        stream,
        RecognizerData {
            results,
        }
    )
}

pub fn create_stream(
    config: SupportedStreamConfig,
    audio_input_device: Device,
    recognizer: Arc<Mutex<Recognizer>>,
    results: Arc<Mutex<RecognitionResults>>,
    channels: u16
    ) -> Stream {
    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    match config.sample_format() {
        SampleFormat::F32 => audio_input_device.build_input_stream(
            &config.into(),
            move |data: &[f32], _| {
//                println!("");
//                println!("lock");
                recognize(&mut recognizer.lock().unwrap(), &mut results.lock().unwrap(), data, channels);
//                println!("unlock");
            },
            err_fn,
            None,
        ),
        SampleFormat::U16 => audio_input_device.build_input_stream(
            &config.into(),
            move |data: &[u16], _| recognize(&mut recognizer.lock().unwrap(), &mut results.lock().unwrap(), data, channels),
            err_fn,
            None,
        ),
        SampleFormat::I16 => audio_input_device.build_input_stream(
            &config.into(),
            move |data: &[i16], _| recognize(&mut recognizer.lock().unwrap(), &mut results.lock().unwrap(), data, channels),
            err_fn,
            None,
        ),
        _ => unreachable!(),
    }
    .expect("Could not build stream")
}

pub fn recognize<T: Sample + ToSample<i16>> (
    recognizer: &mut Recognizer,
    results: &mut RecognitionResults,
    data: &[T],
    channels: ChannelCount,
    ) {
    let data: Vec<i16> = data.iter().map(|v| v.to_sample()).collect();
    let data = if channels != 1 {
        stereo_to_mono(&data)
    } else {
        data
    };

    let state = recognizer.accept_waveform(&data);
    match state {
        DecodingState::Running => {
            //            println!("partial: {:#?}", recognizer.partial_result());
        }
        DecodingState::Finalized => {
            // Result will always be multiple because we called set_max_alternatives
            println!("res write");
            results.results.push(recognizer.result().multiple().unwrap().into());
//            println!("result: {:#?}", recognizer.result().multiple().unwrap());
        }
        DecodingState::Failed => eprintln!("error"),
    }
}

pub fn stereo_to_mono(input_data: &[i16]) -> Vec<i16> {
    let mut result = Vec::with_capacity(input_data.len() / 2);
    result.extend(
        input_data
            .chunks_exact(2)
            .map(|chunk| chunk[0] / 2 + chunk[1] / 2),
    );

    result
}