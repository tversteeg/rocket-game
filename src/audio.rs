use cpal::{
    traits::{EventLoopTrait, HostTrait},
    Format, SampleFormat, SampleRate, StreamData, UnknownTypeOutputBuffer,
};
use std::{
    sync::{Arc, Mutex},
    thread,
};
use usfx::{Mixer, OscillatorType, Sample};

const SAMPLE_RATE: usize = 22_050;

#[const_tweaker::tweak(min = 0.0, max = 1.0, step = 0.001)]
const BOOST_ENV_ATTACK: f32 = 0.45;
#[const_tweaker::tweak(min = 0.0, max = 1.0, step = 0.001)]
const BOOST_ENV_DECAY: f32 = 1.0;
#[const_tweaker::tweak(min = 0.0, max = 1.0, step = 0.001)]
const BOOST_ENV_SUSTAIN: f32 = 0.5;
#[const_tweaker::tweak(min = 0.0, max = 1.0, step = 0.001)]
const BOOST_ENV_RELEASE: f32 = 0.9;
#[const_tweaker::tweak(min = 0.0, max = 5.0, step = 0.001)]
const BOOST_DIS_CRUNCH: f32 = 0.3;
#[const_tweaker::tweak(min = 0.0, max = 5.0, step = 0.001)]
const BOOST_DIS_DRIVE: f32 = 1.0;
#[const_tweaker::tweak(min = 1, max = 2000, step = 1)]
const BOOST_FREQUENCY: usize = 200;

const BOOST_INTERVAL: usize = 8;

/// Manages the audio.
pub struct Audio {
    boost_interval: usize,
    mixer: Arc<Mutex<Mixer>>,
}

impl Audio {
    /// Instantiate a new audio object without a mixer.
    pub fn new() -> Self {
        Self {
            boost_interval: 0,
            mixer: Arc::new(Mutex::new(Mixer::new(SAMPLE_RATE))),
        }
    }

    /// Play a boost sound.
    pub fn play_boost(&mut self, speed: f64) {
        // Only play the sample at a set interval
        if self.boost_interval >= BOOST_INTERVAL {
            let mut sample = Sample::default();
            sample.osc_type(OscillatorType::Saw);
            sample.osc_frequency(*BOOST_FREQUENCY);
            sample.env_attack(*BOOST_ENV_ATTACK);
            sample.env_decay(*BOOST_ENV_DECAY);
            sample.env_sustain(*BOOST_ENV_SUSTAIN);
            sample.env_release(*BOOST_ENV_RELEASE);
            sample.dis_crunch(*BOOST_DIS_CRUNCH);
            sample.dis_drive(*BOOST_DIS_DRIVE);

            self.play(sample, speed as f32 / 200.0);

            self.boost_interval = 0;
        }

        self.boost_interval += 1;
    }

    /// Play a laser sound.
    pub fn play_laser(&mut self) {
        let mut sample = Sample::default();
        sample.osc_type(OscillatorType::Triangle);
        sample.osc_frequency(1000);
        sample.env_attack(0.2);
        sample.env_decay(0.1);
        sample.env_sustain(0.5);
        sample.env_release(0.2);

        self.play(sample, 1.0);
    }

    /// Play a sample.
    pub fn play(&mut self, sample: Sample, _volume: f32) {
        self.mixer.lock().unwrap().play(sample);
    }

    /// Start a thread which will emit the audio.
    pub fn run(&mut self) {
        let mixer = self.mixer.clone();

        thread::spawn(|| {
            // Setup the audio system
            let host = cpal::default_host();
            let event_loop = host.event_loop();

            let device = host
                .default_output_device()
                .expect("no output device available");

            // This is the only format sfxr supports
            let format = Format {
                channels: 1,
                sample_rate: SampleRate(SAMPLE_RATE as u32),
                data_type: SampleFormat::F32,
            };

            let stream_id = event_loop
                .build_output_stream(&device, &format)
                .expect("could not build output stream");

            event_loop
                .play_stream(stream_id)
                .expect("could not play stream");

            event_loop.run(move |stream_id, stream_result| {
                let stream_data = match stream_result {
                    Ok(data) => data,
                    Err(err) => {
                        eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                        return;
                    }
                };

                match stream_data {
                    StreamData::Output {
                        buffer: UnknownTypeOutputBuffer::F32(mut buffer),
                    } => mixer.lock().unwrap().generate(&mut buffer),
                    _ => panic!("output type buffer can not be used"),
                }
            });
        });
    }
}
