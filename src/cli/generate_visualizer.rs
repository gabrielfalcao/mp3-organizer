use std::fs;

use audio_visualizer::WaveformVisualizer;
use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use iocore::Path;

use crate::{
    Error,
    Exit,
    Result,
    UNKNOWN_ARTIST,
    UNKNOWN_TITLE,
    dispatch::ParserDispatcher,
    map_albums_and_artists,
    setup_logger,
};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "mp3-visualizer command-line")]
pub struct VisualizerCli {
    #[arg()]
    mp3_files: Vec<Path>,
}
impl VisualizerCli {
    pub fn mp3_files(&self) -> Vec<Path> {
        self.mp3_files.clone()
    }
}
impl ParserDispatcher<Error> for VisualizerCli {
    fn dispatch(&mut self) -> Result<()> {
        for track in self.mp3_files() {
            WaveformVisualizer::new(&samples)
                .sample_rate(44100.0)
                .title("sample_1.mp3 (left channel)")
                .write_png("waveform.png")?;
        }
        Ok(())
    }
}
