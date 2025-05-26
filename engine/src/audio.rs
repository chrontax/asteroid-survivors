use std::{
    io::Cursor,
    sync::mpsc::{channel, Receiver, Sender},
};

use espeaker::SpeakerSource;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};

pub struct AudioEngine {
    _os: OutputStream,
    handle: OutputStreamHandle,
    rx: Receiver<Vec<u8>>,
    tx: Sender<Vec<u8>>,
    rxs: Receiver<SpeakerSource>,
    txs: Sender<SpeakerSource>,
}

impl AudioEngine {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (_os, handle) =
            OutputStream::try_default().expect("Failed to initialize audio output stream");
        let (tx, rx) = channel();
        let (txs, rxs) = channel();
        Self {
            _os,
            handle,
            rx,
            tx,
            rxs,
            txs,
        }
    }

    pub fn player(&self) -> AudioPlayer {
        AudioPlayer {
            tx: self.tx.clone(),
            txs: self.txs.clone(),
        }
    }

    pub fn update(&mut self) {
        while let Ok(data) = self.rx.try_recv() {
            match Decoder::new(Cursor::new(data)) {
                Ok(decoder) => {
                    self.handle.play_raw(decoder.convert_samples()).unwrap();
                }
                Err(err) => println!("Error decoding audio: {}", err),
            }
        }
        while let Ok(source) = self.rxs.try_recv() {
            self.handle.play_raw(source.convert_samples()).unwrap();
        }
    }
}

#[derive(Clone, Debug)]
pub struct AudioPlayer {
    tx: Sender<Vec<u8>>,
    txs: Sender<SpeakerSource>,
}

impl AudioPlayer {
    pub fn play(&self, data: Vec<u8>) {
        if let Err(err) = self.tx.send(data) {
            eprintln!("Failed to send audio data: {}", err);
        }
    }

    pub fn play_speaker(&self, source: SpeakerSource) {
        if let Err(err) = self.txs.send(source) {
            eprintln!("Failed to send speaker source: {}", err);
        }
    }
}
