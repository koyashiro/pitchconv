mod args;
mod error;
mod pitch;

use std::io::{stdin, Read};

use clap::Parser;

use crate::{
    args::Args,
    pitch::{PitchFormat, PitchWithFormat},
};

fn main() {
    let args = Args::parse();

    let pitch = args.pitch.unwrap_or_else(|| {
        let mut buf = String::new();
        stdin().lock().read_to_string(&mut buf).unwrap();
        buf.truncate(buf.trim_end().len());
        buf
    });

    let pitch_with_format: PitchWithFormat = pitch.parse().unwrap();

    match pitch_with_format.format {
        PitchFormat::ScientificPitchNotation => {
            println!("{}", pitch_with_format.pitch.alternative_pitch_notation());
        }
        PitchFormat::AlternativePitchNotation => {
            println!("{}", pitch_with_format.pitch.scientific_pitch_notation());
        }
    }
}
