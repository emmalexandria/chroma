use std::str::FromStr;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ansi16 {
    Default = 0,
    Black = 1,
    Red = 2,
    Green = 3,
    Yellow = 4,
    Blue = 5,
    Magenta = 6,
    Cyan = 7,
    White = 8,
    BrightBlack = 9,
    BrightRed = 10,
    BrightGreen = 11,
    BrightYellow = 12,
    BrightBlue = 13,
    BrightMagenta = 14,
    BrightCyan = 15,
    BrightWhite = 16,
}

#[derive(Debug, PartialEq)]
pub enum Ansi16Error {
    InvalidName,
    U8TooLarge,
}

impl Into<u8> for Ansi16 {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for Ansi16 {
    type Error = Ansi16Error;
    fn try_from(value: u8) -> Result<Ansi16, Self::Error> {
        match value {
            _ => Err(Ansi16Error::U8TooLarge),
        }
    }
}

impl FromStr for Ansi16 {
    type Err = Ansi16Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variants = [
            Self::Black,
            Self::Red,
            Self::Green,
            Self::Yellow,
            Self::Blue,
            Self::Magenta,
            Self::Cyan,
            Self::White,
            Self::BrightBlack,
            Self::BrightRed,
            Self::BrightGreen,
            Self::BrightYellow,
            Self::BrightBlue,
            Self::BrightMagenta,
            Self::BrightCyan,
            Self::BrightWhite,
        ];
        for v in variants {
            if generate_name_variants(v).contains(&s.to_owned()) {
                return Ok(v);
            }
        }

        Err(Ansi16Error::InvalidName)
    }
}

fn generate_name_variants<S: ToString>(name: S) -> [String; 2] {
    let base = name.to_string();
    let lowercase = base.to_lowercase();

    [base, lowercase]
}

impl std::fmt::Display for Ansi16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //Hacky display implementation using the auto derived debug
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod ansi_tests {
    use super::*;

    #[test]
    fn test_ansi16_from_str() {
        assert_eq!(Ansi16::from_str("brightblack"), Ok(Ansi16::BrightBlack));
        assert_eq!(
            Ansi16::from_str("bright black"),
            Err(Ansi16Error::InvalidName)
        );
    }
}
