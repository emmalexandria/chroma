use ansi16::Ansi16;
use ansi256::Ansi256;
use rgb::Rgb;

mod ansi16;
mod ansi256;
mod rgb;

enum Color {
    Ansi(Ansi16),
    Ansi256(Ansi256),
    Rgb(Rgb),
}
