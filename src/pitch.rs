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
        Regex::new(r"^(?<octave>low|lowlow|lowlowlow|mid[12]|(high)+)(?<pitch_class>[A-G][#]?)$")
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
                let count = s.matches("high").count();

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
            PitchClass::A | PitchClass::ASharp | PitchClass::B => self.0.octave + 1,
            _ => self.0.octave,
        };

        match o {
            0 => write!(f, "lowlowlow")?,
            1 => write!(f, "lowlow")?,
            2 => write!(f, "low")?,
            3 => write!(f, "mid1")?,
            4 => write!(f, "mid2")?,
            n => {
                for _ in 0..n - 4 {
                    write!(f, "high")?;
                }
            }
        }

        write!(f, "{}", self.0.pitch_class)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pitch_class() {
        assert_eq!(Ok(PitchClass::C), parse_pitch_class("C"));
        assert_eq!(Ok(PitchClass::CSharp), parse_pitch_class("C#"));
        assert_eq!(Ok(PitchClass::D), parse_pitch_class("D"));
        assert_eq!(Ok(PitchClass::DSharp), parse_pitch_class("D#"));
        assert_eq!(Ok(PitchClass::E), parse_pitch_class("E"));
        assert_eq!(Ok(PitchClass::F), parse_pitch_class("F"));
        assert_eq!(Ok(PitchClass::FSharp), parse_pitch_class("F#"));
        assert_eq!(Ok(PitchClass::G), parse_pitch_class("G"));
        assert_eq!(Ok(PitchClass::GSharp), parse_pitch_class("G#"));
        assert_eq!(Ok(PitchClass::A), parse_pitch_class("A"));
        assert_eq!(Ok(PitchClass::ASharp), parse_pitch_class("A#"));
        assert_eq!(Ok(PitchClass::B), parse_pitch_class("B"));

        assert_eq!(Err(ParsePitchClassError), parse_pitch_class("invalid"));
    }

    #[test]
    fn test_parse_scientific_pitch_notation() {
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::C
            }),
            parse_scientific_pitch_notation("C0"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::CSharp
            }),
            parse_scientific_pitch_notation("C#0"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::D
            }),
            parse_scientific_pitch_notation("D0"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::DSharp
            }),
            parse_scientific_pitch_notation("D#0"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::E
            }),
            parse_scientific_pitch_notation("E0"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::F
            }),
            parse_scientific_pitch_notation("F0"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::FSharp
            }),
            parse_scientific_pitch_notation("F#0"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::G
            }),
            parse_scientific_pitch_notation("G0"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::GSharp
            }),
            parse_scientific_pitch_notation("G#0"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::A
            }),
            parse_scientific_pitch_notation("A0"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::ASharp
            }),
            parse_scientific_pitch_notation("A#0"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::B
            }),
            parse_scientific_pitch_notation("B0"),
        );
        assert_eq!(
            Err(ParsePitchError),
            parse_scientific_pitch_notation("invalid"),
        );
    }

    #[test]
    fn test_parse_alternative_pitch_notation() {
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::C
            }),
            parse_alternative_pitch_notation("lowlowlowC"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::CSharp
            }),
            parse_alternative_pitch_notation("lowlowlowC#"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::D
            }),
            parse_alternative_pitch_notation("lowlowlowD"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::DSharp
            }),
            parse_alternative_pitch_notation("lowlowlowD#"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::E
            }),
            parse_alternative_pitch_notation("lowlowlowE"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::F
            }),
            parse_alternative_pitch_notation("lowlowlowF"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::FSharp
            }),
            parse_alternative_pitch_notation("lowlowlowF#"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::G
            }),
            parse_alternative_pitch_notation("lowlowlowG"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::GSharp
            }),
            parse_alternative_pitch_notation("lowlowlowG#"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::A
            }),
            parse_alternative_pitch_notation("lowlowA"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::ASharp
            }),
            parse_alternative_pitch_notation("lowlowA#"),
        );
        assert_eq!(
            Ok(Pitch {
                octave: 0,
                pitch_class: PitchClass::B
            }),
            parse_alternative_pitch_notation("lowlowB"),
        );
        assert_eq!(
            Err(ParsePitchError),
            parse_alternative_pitch_notation("invalid"),
        );
    }
}
