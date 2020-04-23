use ferris_says::say;
use std::io::{stdout, BufWriter};

mod guess_number;
use guess_number::guess_number;

fn main() {
    let stdout = stdout();
    let out = b"Program runs Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();

    guess_number();
}