use cpal::{
    traits::{EventLoopTrait, HostTrait},
    Format, SampleFormat, SampleRate, StreamData, UnknownTypeOutputBuffer,
};
use sfxr::{Generator, Sample, WaveType};
use std::{
    sync::{Arc, Mutex},
    thread,
};

#[const_tweaker::tweak(min = 0.0, max = 1.0, step = 0.001)]
const BOOST_ENV_ATTACK: f32 = 0.1;
#[const_tweaker::tweak(min = 0.0, max = 1.0, step = 0.001)]
const BOOST_ENV_SUSTAIN: f32 = 0.1;
#[const_tweaker::tweak(min = 0.0, max = 1.0, step = 0.001)]
const BOOST_ENV_DECAY: f32 = 1.0;
#[const_tweaker::tweak(min = 0.0, max = 1.0, step = 0.001)]
const BOOST_BASE_FREQ: f64 = 0.05;

const BOOST_INTERVAL: usize = 8;

/// Manages the audio.
pub struct Audio {
    boost_interval: usize,
    generator: Arc<Mutex<Option<Generator>>>,
}

impl Audio {
    /// Instantiate a new audio object without a generator.
    pub fn new() -> Self {
        Self {
            boost_interval: 0,
            generator: Arc::new(Mutex::new(None)),
        }
    }

    /// Play a boost sound.
    pub fn play_boost(&mut self, speed: f64) {
        // Only play the sample at a set interval
        if self.boost_interval >= BOOST_INTERVAL {
            let mut sample = Sample::new();

            sample.wave_type = WaveType::Noise;
            sample.base_freq = *BOOST_BASE_FREQ;
            sample.env_attack = *BOOST_ENV_ATTACK;
            sample.env_sustain = *BOOST_ENV_SUSTAIN;
            sample.env_decay = *BOOST_ENV_DECAY;

            self.reset();
            self.play(sample, speed as f32 / 200.0);

            self.boost_interval = 0;
        }

        self.boost_interval += 1;
    }

    /// Play a laser sound.
    pub fn play_laser(&mut self) {
        self.play(Sample::laser(None), 1.0);
    }

    /// Play a sample.
    pub fn play(&mut self, sample: Sample, volume: f32) {
        let mut new_generator = Generator::new(sample);
        new_generator.volume = volume;

        let mut generator = self.generator.lock().unwrap();
        *generator = Some(new_generator);
    }

    /// Reset the sound.
    pub fn reset(&mut self) {
        let mut generator = self.generator.lock().unwrap();
        if let Some(ref mut generator) = *generator {
            generator.reset();
        }
    }

    /// Start a thread which will emit the audio.
    pub fn run(&mut self) {
        let generator = self.generator.clone();

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
                sample_rate: SampleRate(44_100),
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
                    } => match *generator.lock().unwrap() {
                        Some(ref mut generator) => generator.generate(&mut buffer),
                        None => {
                            for elem in buffer.iter_mut() {
                                *elem = 0.0;
                            }
                        }
                    },
                    _ => panic!("output type buffer can not be used"),
                }
            });
        });
    }
}
