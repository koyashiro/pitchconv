mod args;
mod error;
mod pitch;

use crate::{args::Args, pitch::PitchWithFormat};

use clap::Parser;
use pitch::PitchFormat;

fn main() {
    let args = Args::parse();

    let pitch_with_format: PitchWithFormat = args.pitch.parse().unwrap();

    match pitch_with_format.format {
        PitchFormat::ScientificPitchNotation => {
            println!("{}", pitch_with_format.pitch.alternative_pitch_notation());
        }
        PitchFormat::AlternativePitchNotation => {
            println!("{}", pitch_with_format.pitch.scientific_pitch_notation());
        }
    }
}
