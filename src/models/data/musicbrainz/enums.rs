use core::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

pub enum MusicBrainzEntity {
    Artist,
    Recording,
    Work,
}

impl Display for MusicBrainzEntity {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Artist => write!(f, "artist"),
            Self::Recording => write!(f, "recording"),
            Self::Work => write!(f, "work")
        }
    }
}