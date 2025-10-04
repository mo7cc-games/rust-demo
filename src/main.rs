use ferris_says::say;
use std::io::{BufWriter, stdout};

fn main() {
    let out = "Hello fellow 墨七!";
    let width = 24;

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}
