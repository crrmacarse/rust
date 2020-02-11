use ferris_say::say;
use std::io::{stdout, BufWriter};

fn main(){
    let stdout = stdout();
    let out = b"Hello fellow Rusteceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}
