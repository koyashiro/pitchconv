use regex::Regex;

use crate::error::{ParsePitchClassError, ParsePitchError};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PitchClass {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl PitchClass {
    pub fn as_str(&self) -> &str {
        match self {
            PitchClass::C => "C",
            PitchClass::CSharp => "C#",
            PitchClass::D => "D",
            PitchClass::DSharp => "D#",
            PitchClass::E => "E",
            PitchClass::F => "F",
            PitchClass::FSharp => "F#",
            PitchClass::G => "G",
            PitchClass::GSharp => "G#",
            PitchClass::A => "A",
            PitchClass::ASharp => "A#",
            PitchClass::B => "B",
        }
    }
}

impl std::fmt::Display for PitchClass {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for PitchClass {
    type Err = ParsePitchClassError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_pitch_class(s)
    }
}

fn parse_pitch_class(s: &str) -> Result<PitchClass, ParsePitchClassError> {
    let pitch_class = match s {
        "C" => PitchClass::C,
        "C#" => PitchClass::CSharp,
        "D" => PitchClass::D,
        "D#" => PitchClass::DSharp,
        "E" => PitchClass::E,
        "F" => PitchClass::F,
        "F#" => PitchClass::FSharp,
        "G" => PitchClass::G,
        "G#" => PitchClass::GSharp,
        "A" => PitchClass::A,
        "A#" => PitchClass::ASharp,
        "B" => PitchClass::B,
        _ => return Err(ParsePitchClassError),
    };

    Ok(pitch_class)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pitch {
    pub octave: u8,
    pub pitch_class: PitchClass,
}

impl Pitch {
    pub fn scientific_pitch_notation(&self) -> ScientificPitchNotation {
        ScientificPitchNotation(self)
    }

    pub fn alternative_pitch_notation(&self) -> AlternativePitchNotation {
        AlternativePitchNotation(self)
    }
}

impl From<PitchWithFormat> for Pitch {
    fn from(value: PitchWithFormat) -> Self {
        value.pitch
    }
}

impl std::fmt::Display for Pitch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.scientific_pitch_notation().fmt(f)
    }
}

impl std::str::FromStr for Pitch {
    type Err = ParsePitchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PitchWithFormat::from_str(s).map(Into::into)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PitchFormat {
    ScientificPitchNotation,
    AlternativePitchNotation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PitchWithFormat {
    pub pitch: Pitch,
    pub format: PitchFormat,
}

impl std::str::FromStr for PitchWithFormat {
    type Err = ParsePitchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(pitch) = parse_scientific_pitch_notation(s) {
            return Ok(PitchWithFormat {
                pitch,
                format: PitchFormat::ScientificPitchNotation,
            });
        }

        if let Ok(pitch) = parse_alternative_pitch_notation(s) {
            return Ok(PitchWithFormat {
                pitch,
                format: PitchFormat::AlternativePitchNotation,
            });
        }

        Err(ParsePitchError)
    }
}

fn parse_scientific_pitch_notation(s: &str) -> Result<Pitch, ParsePitchError> {
    let Some(caps) = Regex::new(r"^(?<pitch_class>[A-G][#]?)(?<octave>0|([1-9]\d*))$")
        .unwrap()
        .captures(s)
    else {
        return Err(ParsePitchError);
    };

    let octave = caps.name("octave").unwrap().as_str().parse()?;
    let pitch_class = caps.name("pitch_class").unwrap().as_str().parse()?;

    Ok(Pitch {
        octave,
        pitch_class,
    })
}

fn parse_alternative_pitch_notation(s: &str) -> Result<Pitch, ParsePitchError> {
    let Some(caps) =
        Regex::new(r"^(?<octave>low|lowlow|lowlowlow|mid[12]|(hi)+)(?<pitch_class>[A-G][#]?)$")
            .unwrap()
            .captures(s)
    else {
        return Err(ParsePitchError);
    };

    let pitch_class = caps.name("pitch_class").unwrap().as_str().parse()?;

    let octave = {
        let octave_str = caps.name("octave").unwrap().as_str();

        let base_octave = match octave_str {
            "lowlowlow" => 0,
            "lowlow" => 1,
            "low" => 2,
            "mid1" => 3,
            "mid2" => 4,
            s => {
                let count = s.matches("hi").count();

                if count == 0 {
                    return Err(ParsePitchError);
                }

                count + 4
            }
        };

        match pitch_class {
            PitchClass::A | PitchClass::ASharp | PitchClass::B => base_octave - 1,
            _ => base_octave,
        }
    };

    if octave > u8::MAX as _ {
        return Err(ParsePitchError);
    }

    Ok(Pitch {
        octave: octave as _,
        pitch_class,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScientificPitchNotation<'a>(&'a Pitch);

impl std::fmt::Display for ScientificPitchNotation<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.0.pitch_class, self.0.octave)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativePitchNotation<'a>(&'a Pitch);

impl std::fmt::Display for AlternativePitchNotation<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let o = match self.0.pitch_class {
            PitchClass::A | PitchClass::ASharp | PitchClass::B => self.0.octave as u16 + 1,
            _ => self.0.octave as u16,
        };

        match o {
            0 => write!(f, "lowlowlow")?,
            1 => write!(f, "lowlow")?,
            2 => write!(f, "low")?,
            3 => write!(f, "mid1")?,
            4 => write!(f, "mid2")?,
            n => {
                for _ in 0..n - 4 {
                    write!(f, "hi")?;
                }
            }
        }

        write!(f, "{}", self.0.pitch_class)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct PitchClassCase {
        pitch_class: PitchClass,
        s: &'static str,
    }

    struct PitchCase {
        pitch: Pitch,
        scientific_pitch_notation: &'static str,
        alternative_pitch_notation: &'static str,
    }

    const PITCH_CLASS_CASES: [PitchClassCase; 12] = [
        PitchClassCase {
            pitch_class: PitchClass::C,
            s: "C",
        },
        PitchClassCase {
            pitch_class: PitchClass::CSharp,
            s: "C#",
        },
        PitchClassCase {
            pitch_class: PitchClass::D,
            s: "D",
        },
        PitchClassCase {
            pitch_class: PitchClass::DSharp,
            s: "D#",
        },
        PitchClassCase {
            pitch_class: PitchClass::E,
            s: "E",
        },
        PitchClassCase {
            pitch_class: PitchClass::F,
            s: "F",
        },
        PitchClassCase {
            pitch_class: PitchClass::FSharp,
            s: "F#",
        },
        PitchClassCase {
            pitch_class: PitchClass::G,
            s: "G",
        },
        PitchClassCase {
            pitch_class: PitchClass::GSharp,
            s: "G#",
        },
        PitchClassCase {
            pitch_class: PitchClass::A,
            s: "A",
        },
        PitchClassCase {
            pitch_class: PitchClass::ASharp,
            s: "A#",
        },
        PitchClassCase {
            pitch_class: PitchClass::B,
            s: "B",
        },
    ];

    const PITCH_CASES: [PitchCase; 12 * 10] = [
        PitchCase {
            pitch: Pitch {
                octave: 0,
                pitch_class: PitchClass::C,
            },
            scientific_pitch_notation: "C0",
            alternative_pitch_notation: "lowlowlowC",
        },
        PitchCase {
            pitch: Pitch {
                octave: 0,
                pitch_class: PitchClass::CSharp,
            },
            scientific_pitch_notation: "C#0",
            alternative_pitch_notation: "lowlowlowC#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 0,
                pitch_class: PitchClass::D,
            },
            scientific_pitch_notation: "D0",
            alternative_pitch_notation: "lowlowlowD",
        },
        PitchCase {
            pitch: Pitch {
                octave: 0,
                pitch_class: PitchClass::DSharp,
            },
            scientific_pitch_notation: "D#0",
            alternative_pitch_notation: "lowlowlowD#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 0,
                pitch_class: PitchClass::E,
            },
            scientific_pitch_notation: "E0",
            alternative_pitch_notation: "lowlowlowE",
        },
        PitchCase {
            pitch: Pitch {
                octave: 0,
                pitch_class: PitchClass::F,
            },
            scientific_pitch_notation: "F0",
            alternative_pitch_notation: "lowlowlowF",
        },
        PitchCase {
            pitch: Pitch {
                octave: 0,
                pitch_class: PitchClass::FSharp,
            },
            scientific_pitch_notation: "F#0",
            alternative_pitch_notation: "lowlowlowF#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 0,
                pitch_class: PitchClass::G,
            },
            scientific_pitch_notation: "G0",
            alternative_pitch_notation: "lowlowlowG",
        },
        PitchCase {
            pitch: Pitch {
                octave: 0,
                pitch_class: PitchClass::GSharp,
            },
            scientific_pitch_notation: "G#0",
            alternative_pitch_notation: "lowlowlowG#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 0,
                pitch_class: PitchClass::A,
            },
            scientific_pitch_notation: "A0",
            alternative_pitch_notation: "lowlowA",
        },
        PitchCase {
            pitch: Pitch {
                octave: 0,
                pitch_class: PitchClass::ASharp,
            },
            scientific_pitch_notation: "A#0",
            alternative_pitch_notation: "lowlowA#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 0,
                pitch_class: PitchClass::B,
            },
            scientific_pitch_notation: "B0",
            alternative_pitch_notation: "lowlowB",
        },
        PitchCase {
            pitch: Pitch {
                octave: 1,
                pitch_class: PitchClass::C,
            },
            scientific_pitch_notation: "C1",
            alternative_pitch_notation: "lowlowC",
        },
        PitchCase {
            pitch: Pitch {
                octave: 1,
                pitch_class: PitchClass::CSharp,
            },
            scientific_pitch_notation: "C#1",
            alternative_pitch_notation: "lowlowC#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 1,
                pitch_class: PitchClass::D,
            },
            scientific_pitch_notation: "D1",
            alternative_pitch_notation: "lowlowD",
        },
        PitchCase {
            pitch: Pitch {
                octave: 1,
                pitch_class: PitchClass::DSharp,
            },
            scientific_pitch_notation: "D#1",
            alternative_pitch_notation: "lowlowD#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 1,
                pitch_class: PitchClass::E,
            },
            scientific_pitch_notation: "E1",
            alternative_pitch_notation: "lowlowE",
        },
        PitchCase {
            pitch: Pitch {
                octave: 1,
                pitch_class: PitchClass::F,
            },
            scientific_pitch_notation: "F1",
            alternative_pitch_notation: "lowlowF",
        },
        PitchCase {
            pitch: Pitch {
                octave: 1,
                pitch_class: PitchClass::FSharp,
            },
            scientific_pitch_notation: "F#1",
            alternative_pitch_notation: "lowlowF#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 1,
                pitch_class: PitchClass::G,
            },
            scientific_pitch_notation: "G1",
            alternative_pitch_notation: "lowlowG",
        },
        PitchCase {
            pitch: Pitch {
                octave: 1,
                pitch_class: PitchClass::GSharp,
            },
            scientific_pitch_notation: "G#1",
            alternative_pitch_notation: "lowlowG#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 1,
                pitch_class: PitchClass::A,
            },
            scientific_pitch_notation: "A1",
            alternative_pitch_notation: "lowA",
        },
        PitchCase {
            pitch: Pitch {
                octave: 1,
                pitch_class: PitchClass::ASharp,
            },
            scientific_pitch_notation: "A#1",
            alternative_pitch_notation: "lowA#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 1,
                pitch_class: PitchClass::B,
            },
            scientific_pitch_notation: "B1",
            alternative_pitch_notation: "lowB",
        },
        PitchCase {
            pitch: Pitch {
                octave: 2,
                pitch_class: PitchClass::C,
            },
            scientific_pitch_notation: "C2",
            alternative_pitch_notation: "lowC",
        },
        PitchCase {
            pitch: Pitch {
                octave: 2,
                pitch_class: PitchClass::CSharp,
            },
            scientific_pitch_notation: "C#2",
            alternative_pitch_notation: "lowC#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 2,
                pitch_class: PitchClass::D,
            },
            scientific_pitch_notation: "D2",
            alternative_pitch_notation: "lowD",
        },
        PitchCase {
            pitch: Pitch {
                octave: 2,
                pitch_class: PitchClass::DSharp,
            },
            scientific_pitch_notation: "D#2",
            alternative_pitch_notation: "lowD#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 2,
                pitch_class: PitchClass::E,
            },
            scientific_pitch_notation: "E2",
            alternative_pitch_notation: "lowE",
        },
        PitchCase {
            pitch: Pitch {
                octave: 2,
                pitch_class: PitchClass::F,
            },
            scientific_pitch_notation: "F2",
            alternative_pitch_notation: "lowF",
        },
        PitchCase {
            pitch: Pitch {
                octave: 2,
                pitch_class: PitchClass::FSharp,
            },
            scientific_pitch_notation: "F#2",
            alternative_pitch_notation: "lowF#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 2,
                pitch_class: PitchClass::G,
            },
            scientific_pitch_notation: "G2",
            alternative_pitch_notation: "lowG",
        },
        PitchCase {
            pitch: Pitch {
                octave: 2,
                pitch_class: PitchClass::GSharp,
            },
            scientific_pitch_notation: "G#2",
            alternative_pitch_notation: "lowG#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 2,
                pitch_class: PitchClass::A,
            },
            scientific_pitch_notation: "A2",
            alternative_pitch_notation: "mid1A",
        },
        PitchCase {
            pitch: Pitch {
                octave: 2,
                pitch_class: PitchClass::ASharp,
            },
            scientific_pitch_notation: "A#2",
            alternative_pitch_notation: "mid1A#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 2,
                pitch_class: PitchClass::B,
            },
            scientific_pitch_notation: "B2",
            alternative_pitch_notation: "mid1B",
        },
        PitchCase {
            pitch: Pitch {
                octave: 3,
                pitch_class: PitchClass::C,
            },
            scientific_pitch_notation: "C3",
            alternative_pitch_notation: "mid1C",
        },
        PitchCase {
            pitch: Pitch {
                octave: 3,
                pitch_class: PitchClass::CSharp,
            },
            scientific_pitch_notation: "C#3",
            alternative_pitch_notation: "mid1C#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 3,
                pitch_class: PitchClass::D,
            },
            scientific_pitch_notation: "D3",
            alternative_pitch_notation: "mid1D",
        },
        PitchCase {
            pitch: Pitch {
                octave: 3,
                pitch_class: PitchClass::DSharp,
            },
            scientific_pitch_notation: "D#3",
            alternative_pitch_notation: "mid1D#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 3,
                pitch_class: PitchClass::E,
            },
            scientific_pitch_notation: "E3",
            alternative_pitch_notation: "mid1E",
        },
        PitchCase {
            pitch: Pitch {
                octave: 3,
                pitch_class: PitchClass::F,
            },
            scientific_pitch_notation: "F3",
            alternative_pitch_notation: "mid1F",
        },
        PitchCase {
            pitch: Pitch {
                octave: 3,
                pitch_class: PitchClass::FSharp,
            },
            scientific_pitch_notation: "F#3",
            alternative_pitch_notation: "mid1F#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 3,
                pitch_class: PitchClass::G,
            },
            scientific_pitch_notation: "G3",
            alternative_pitch_notation: "mid1G",
        },
        PitchCase {
            pitch: Pitch {
                octave: 3,
                pitch_class: PitchClass::GSharp,
            },
            scientific_pitch_notation: "G#3",
            alternative_pitch_notation: "mid1G#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 3,
                pitch_class: PitchClass::A,
            },
            scientific_pitch_notation: "A3",
            alternative_pitch_notation: "mid2A",
        },
        PitchCase {
            pitch: Pitch {
                octave: 3,
                pitch_class: PitchClass::ASharp,
            },
            scientific_pitch_notation: "A#3",
            alternative_pitch_notation: "mid2A#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 3,
                pitch_class: PitchClass::B,
            },
            scientific_pitch_notation: "B3",
            alternative_pitch_notation: "mid2B",
        },
        PitchCase {
            pitch: Pitch {
                octave: 4,
                pitch_class: PitchClass::C,
            },
            scientific_pitch_notation: "C4",
            alternative_pitch_notation: "mid2C",
        },
        PitchCase {
            pitch: Pitch {
                octave: 4,
                pitch_class: PitchClass::CSharp,
            },
            scientific_pitch_notation: "C#4",
            alternative_pitch_notation: "mid2C#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 4,
                pitch_class: PitchClass::D,
            },
            scientific_pitch_notation: "D4",
            alternative_pitch_notation: "mid2D",
        },
        PitchCase {
            pitch: Pitch {
                octave: 4,
                pitch_class: PitchClass::DSharp,
            },
            scientific_pitch_notation: "D#4",
            alternative_pitch_notation: "mid2D#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 4,
                pitch_class: PitchClass::E,
            },
            scientific_pitch_notation: "E4",
            alternative_pitch_notation: "mid2E",
        },
        PitchCase {
            pitch: Pitch {
                octave: 4,
                pitch_class: PitchClass::F,
            },
            scientific_pitch_notation: "F4",
            alternative_pitch_notation: "mid2F",
        },
        PitchCase {
            pitch: Pitch {
                octave: 4,
                pitch_class: PitchClass::FSharp,
            },
            scientific_pitch_notation: "F#4",
            alternative_pitch_notation: "mid2F#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 4,
                pitch_class: PitchClass::G,
            },
            scientific_pitch_notation: "G4",
            alternative_pitch_notation: "mid2G",
        },
        PitchCase {
            pitch: Pitch {
                octave: 4,
                pitch_class: PitchClass::GSharp,
            },
            scientific_pitch_notation: "G#4",
            alternative_pitch_notation: "mid2G#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 4,
                pitch_class: PitchClass::A,
            },
            scientific_pitch_notation: "A4",
            alternative_pitch_notation: "hiA",
        },
        PitchCase {
            pitch: Pitch {
                octave: 4,
                pitch_class: PitchClass::ASharp,
            },
            scientific_pitch_notation: "A#4",
            alternative_pitch_notation: "hiA#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 4,
                pitch_class: PitchClass::B,
            },
            scientific_pitch_notation: "B4",
            alternative_pitch_notation: "hiB",
        },
        PitchCase {
            pitch: Pitch {
                octave: 5,
                pitch_class: PitchClass::C,
            },
            scientific_pitch_notation: "C5",
            alternative_pitch_notation: "hiC",
        },
        PitchCase {
            pitch: Pitch {
                octave: 5,
                pitch_class: PitchClass::CSharp,
            },
            scientific_pitch_notation: "C#5",
            alternative_pitch_notation: "hiC#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 5,
                pitch_class: PitchClass::D,
            },
            scientific_pitch_notation: "D5",
            alternative_pitch_notation: "hiD",
        },
        PitchCase {
            pitch: Pitch {
                octave: 5,
                pitch_class: PitchClass::DSharp,
            },
            scientific_pitch_notation: "D#5",
            alternative_pitch_notation: "hiD#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 5,
                pitch_class: PitchClass::E,
            },
            scientific_pitch_notation: "E5",
            alternative_pitch_notation: "hiE",
        },
        PitchCase {
            pitch: Pitch {
                octave: 5,
                pitch_class: PitchClass::F,
            },
            scientific_pitch_notation: "F5",
            alternative_pitch_notation: "hiF",
        },
        PitchCase {
            pitch: Pitch {
                octave: 5,
                pitch_class: PitchClass::FSharp,
            },
            scientific_pitch_notation: "F#5",
            alternative_pitch_notation: "hiF#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 5,
                pitch_class: PitchClass::G,
            },
            scientific_pitch_notation: "G5",
            alternative_pitch_notation: "hiG",
        },
        PitchCase {
            pitch: Pitch {
                octave: 5,
                pitch_class: PitchClass::GSharp,
            },
            scientific_pitch_notation: "G#5",
            alternative_pitch_notation: "hiG#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 5,
                pitch_class: PitchClass::A,
            },
            scientific_pitch_notation: "A5",
            alternative_pitch_notation: "hihiA",
        },
        PitchCase {
            pitch: Pitch {
                octave: 5,
                pitch_class: PitchClass::ASharp,
            },
            scientific_pitch_notation: "A#5",
            alternative_pitch_notation: "hihiA#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 5,
                pitch_class: PitchClass::B,
            },
            scientific_pitch_notation: "B5",
            alternative_pitch_notation: "hihiB",
        },
        PitchCase {
            pitch: Pitch {
                octave: 6,
                pitch_class: PitchClass::C,
            },
            scientific_pitch_notation: "C6",
            alternative_pitch_notation: "hihiC",
        },
        PitchCase {
            pitch: Pitch {
                octave: 6,
                pitch_class: PitchClass::CSharp,
            },
            scientific_pitch_notation: "C#6",
            alternative_pitch_notation: "hihiC#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 6,
                pitch_class: PitchClass::D,
            },
            scientific_pitch_notation: "D6",
            alternative_pitch_notation: "hihiD",
        },
        PitchCase {
            pitch: Pitch {
                octave: 6,
                pitch_class: PitchClass::DSharp,
            },
            scientific_pitch_notation: "D#6",
            alternative_pitch_notation: "hihiD#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 6,
                pitch_class: PitchClass::E,
            },
            scientific_pitch_notation: "E6",
            alternative_pitch_notation: "hihiE",
        },
        PitchCase {
            pitch: Pitch {
                octave: 6,
                pitch_class: PitchClass::F,
            },
            scientific_pitch_notation: "F6",
            alternative_pitch_notation: "hihiF",
        },
        PitchCase {
            pitch: Pitch {
                octave: 6,
                pitch_class: PitchClass::FSharp,
            },
            scientific_pitch_notation: "F#6",
            alternative_pitch_notation: "hihiF#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 6,
                pitch_class: PitchClass::G,
            },
            scientific_pitch_notation: "G6",
            alternative_pitch_notation: "hihiG",
        },
        PitchCase {
            pitch: Pitch {
                octave: 6,
                pitch_class: PitchClass::GSharp,
            },
            scientific_pitch_notation: "G#6",
            alternative_pitch_notation: "hihiG#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 6,
                pitch_class: PitchClass::A,
            },
            scientific_pitch_notation: "A6",
            alternative_pitch_notation: "hihihiA",
        },
        PitchCase {
            pitch: Pitch {
                octave: 6,
                pitch_class: PitchClass::ASharp,
            },
            scientific_pitch_notation: "A#6",
            alternative_pitch_notation: "hihihiA#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 6,
                pitch_class: PitchClass::B,
            },
            scientific_pitch_notation: "B6",
            alternative_pitch_notation: "hihihiB",
        },
        PitchCase {
            pitch: Pitch {
                octave: 6,
                pitch_class: PitchClass::C,
            },
            scientific_pitch_notation: "C6",
            alternative_pitch_notation: "hihiC",
        },
        PitchCase {
            pitch: Pitch {
                octave: 7,
                pitch_class: PitchClass::CSharp,
            },
            scientific_pitch_notation: "C#7",
            alternative_pitch_notation: "hihihiC#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 7,
                pitch_class: PitchClass::D,
            },
            scientific_pitch_notation: "D7",
            alternative_pitch_notation: "hihihiD",
        },
        PitchCase {
            pitch: Pitch {
                octave: 7,
                pitch_class: PitchClass::DSharp,
            },
            scientific_pitch_notation: "D#7",
            alternative_pitch_notation: "hihihiD#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 7,
                pitch_class: PitchClass::E,
            },
            scientific_pitch_notation: "E7",
            alternative_pitch_notation: "hihihiE",
        },
        PitchCase {
            pitch: Pitch {
                octave: 7,
                pitch_class: PitchClass::F,
            },
            scientific_pitch_notation: "F7",
            alternative_pitch_notation: "hihihiF",
        },
        PitchCase {
            pitch: Pitch {
                octave: 7,
                pitch_class: PitchClass::FSharp,
            },
            scientific_pitch_notation: "F#7",
            alternative_pitch_notation: "hihihiF#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 7,
                pitch_class: PitchClass::G,
            },
            scientific_pitch_notation: "G7",
            alternative_pitch_notation: "hihihiG",
        },
        PitchCase {
            pitch: Pitch {
                octave: 7,
                pitch_class: PitchClass::GSharp,
            },
            scientific_pitch_notation: "G#7",
            alternative_pitch_notation: "hihihiG#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 7,
                pitch_class: PitchClass::A,
            },
            scientific_pitch_notation: "A7",
            alternative_pitch_notation: "hihihihiA",
        },
        PitchCase {
            pitch: Pitch {
                octave: 7,
                pitch_class: PitchClass::ASharp,
            },
            scientific_pitch_notation: "A#7",
            alternative_pitch_notation: "hihihihiA#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 7,
                pitch_class: PitchClass::B,
            },
            scientific_pitch_notation: "B7",
            alternative_pitch_notation: "hihihihiB",
        },
        PitchCase {
            pitch: Pitch {
                octave: 8,
                pitch_class: PitchClass::C,
            },
            scientific_pitch_notation: "C8",
            alternative_pitch_notation: "hihihihiC",
        },
        PitchCase {
            pitch: Pitch {
                octave: 8,
                pitch_class: PitchClass::CSharp,
            },
            scientific_pitch_notation: "C#8",
            alternative_pitch_notation: "hihihihiC#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 8,
                pitch_class: PitchClass::D,
            },
            scientific_pitch_notation: "D8",
            alternative_pitch_notation: "hihihihiD",
        },
        PitchCase {
            pitch: Pitch {
                octave: 8,
                pitch_class: PitchClass::DSharp,
            },
            scientific_pitch_notation: "D#8",
            alternative_pitch_notation: "hihihihiD#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 8,
                pitch_class: PitchClass::E,
            },
            scientific_pitch_notation: "E8",
            alternative_pitch_notation: "hihihihiE",
        },
        PitchCase {
            pitch: Pitch {
                octave: 8,
                pitch_class: PitchClass::F,
            },
            scientific_pitch_notation: "F8",
            alternative_pitch_notation: "hihihihiF",
        },
        PitchCase {
            pitch: Pitch {
                octave: 8,
                pitch_class: PitchClass::FSharp,
            },
            scientific_pitch_notation: "F#8",
            alternative_pitch_notation: "hihihihiF#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 8,
                pitch_class: PitchClass::G,
            },
            scientific_pitch_notation: "G8",
            alternative_pitch_notation: "hihihihiG",
        },
        PitchCase {
            pitch: Pitch {
                octave: 8,
                pitch_class: PitchClass::GSharp,
            },
            scientific_pitch_notation: "G#8",
            alternative_pitch_notation: "hihihihiG#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 8,
                pitch_class: PitchClass::A,
            },
            scientific_pitch_notation: "A8",
            alternative_pitch_notation: "hihihihihiA",
        },
        PitchCase {
            pitch: Pitch {
                octave: 8,
                pitch_class: PitchClass::ASharp,
            },
            scientific_pitch_notation: "A#8",
            alternative_pitch_notation: "hihihihihiA#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 8,
                pitch_class: PitchClass::B,
            },
            scientific_pitch_notation: "B8",
            alternative_pitch_notation: "hihihihihiB",
        },
        PitchCase {
            pitch: Pitch {
                octave: 255,
                pitch_class: PitchClass::C,
            },
            scientific_pitch_notation: "C255",
            alternative_pitch_notation: "hihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihiC",
        },
        PitchCase {
            pitch: Pitch {
                octave: 255,
                pitch_class: PitchClass::CSharp,
            },
            scientific_pitch_notation: "C#255",
            alternative_pitch_notation: "hihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihiC#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 255,
                pitch_class: PitchClass::D,
            },
            scientific_pitch_notation: "D255",
            alternative_pitch_notation: "hihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihiD",
        },
        PitchCase {
            pitch: Pitch {
                octave: 255,
                pitch_class: PitchClass::DSharp,
            },
            scientific_pitch_notation: "D#255",
            alternative_pitch_notation: "hihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihiD#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 255,
                pitch_class: PitchClass::E,
            },
            scientific_pitch_notation: "E255",
            alternative_pitch_notation: "hihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihiE",
        },
        PitchCase {
            pitch: Pitch {
                octave: 255,
                pitch_class: PitchClass::F,
            },
            scientific_pitch_notation: "F255",
            alternative_pitch_notation: "hihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihiF",
        },
        PitchCase {
            pitch: Pitch {
                octave: 255,
                pitch_class: PitchClass::FSharp,
            },
            scientific_pitch_notation: "F#255",
            alternative_pitch_notation: "hihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihiF#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 255,
                pitch_class: PitchClass::G,
            },
            scientific_pitch_notation: "G255",
            alternative_pitch_notation: "hihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihiG",
        },
        PitchCase {
            pitch: Pitch {
                octave: 255,
                pitch_class: PitchClass::GSharp,
            },
            scientific_pitch_notation: "G#255",
            alternative_pitch_notation: "hihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihiG#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 255,
                pitch_class: PitchClass::A,
            },
            scientific_pitch_notation: "A255",
            alternative_pitch_notation: "hihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihiA",
        },
        PitchCase {
            pitch: Pitch {
                octave: 255,
                pitch_class: PitchClass::ASharp,
            },
            scientific_pitch_notation: "A#255",
            alternative_pitch_notation: "hihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihiA#",
        },
        PitchCase {
            pitch: Pitch {
                octave: 255,
                pitch_class: PitchClass::B,
            },
            scientific_pitch_notation: "B255",
            alternative_pitch_notation: "hihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihiB",
        },
    ];

    #[test]
    fn test_parse_pitch_class() {
        for case in PITCH_CLASS_CASES {
            assert_eq!(Ok(case.pitch_class), parse_pitch_class(case.s));
        }

        assert_eq!(Err(ParsePitchClassError), parse_pitch_class("invalid"));
        for case in PITCH_CLASS_CASES {
            assert_eq!(
                Err(ParsePitchClassError),
                parse_pitch_class(&case.s.to_lowercase()),
            );
        }
    }

    #[test]
    fn test_pitch_class_to_string() {
        for case in PITCH_CLASS_CASES {
            assert_eq!(case.s, case.pitch_class.to_string());
        }
    }

    #[test]
    fn test_parse_scientific_pitch_notation() {
        for case in PITCH_CASES {
            assert_eq!(
                Ok(case.pitch),
                parse_scientific_pitch_notation(case.scientific_pitch_notation),
            );
        }

        assert_eq!(
            Err(ParsePitchError),
            parse_scientific_pitch_notation("invalid"),
        );
        assert_eq!(Err(ParsePitchError), parse_scientific_pitch_notation("B-1"));
        assert_eq!(
            Err(ParsePitchError),
            parse_scientific_pitch_notation("C256"),
        );
        for case in PITCH_CASES {
            assert_eq!(
                Err(ParsePitchError),
                parse_scientific_pitch_notation(&case.scientific_pitch_notation.to_lowercase()),
            );
        }
    }

    #[test]
    fn test_scientific_pitch_notation_to_string() {
        for case in PITCH_CASES {
            assert_eq!(
                case.scientific_pitch_notation,
                case.pitch.scientific_pitch_notation().to_string(),
            );
        }
    }

    #[test]
    fn test_parse_alternative_pitch_notation() {
        for case in PITCH_CASES {
            assert_eq!(
                Ok(case.pitch),
                parse_alternative_pitch_notation(case.alternative_pitch_notation),
            );
        }

        assert_eq!(
            Err(ParsePitchError),
            parse_alternative_pitch_notation("invalid"),
        );
        assert_eq!(
            Err(ParsePitchError),
            parse_alternative_pitch_notation("hihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihiC"),
        );
        for case in PITCH_CASES {
            assert_eq!(
                Err(ParsePitchError),
                parse_alternative_pitch_notation(&case.alternative_pitch_notation.to_lowercase()),
            );
        }
    }

    #[test]
    fn test_alternative_pitch_notation_to_string() {
        for case in PITCH_CASES {
            assert_eq!(
                case.alternative_pitch_notation,
                case.pitch.alternative_pitch_notation().to_string(),
            );
        }
    }
}
