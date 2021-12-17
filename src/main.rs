use atty::Stream;
use std::io::{self, Read};

fn get_word_of_len(len: usize) -> String {
    str::repeat("a", len)
}

fn get_inc() -> String {
    // len of 3
    String::from("inc")
}

fn get_dec() -> String {
    // len of 4
    String::from("decc")
}

fn get_out() -> String {
    // len of 7
    String::from("outputa")
}

fn get_end() -> String {
    // len of 10 / 0
    String::from("endprogram")
}

fn get_fwd() -> String {
    // len of 5
    String::from("movef")
}

fn get_bak() -> String {
    // len of 6
    String::from("moveba")
}

fn get_if() -> String {
    // len of 1
    String::from("i")
}

fn get_eif() -> String {
    // len of 2
    String::from("ei")
}

fn set_current_mem_to_zero() -> String {
    format!("{} {} {} {}", get_if(), get_dec(), get_word_of_len(1), get_eif())
}

fn translate_to_inc(c: &char) -> String {
    let ascii_value = *c as u8;
    let times = ascii_value / 9;
    let rest = ascii_value % 9;

    let mut result = Vec::new();

    for _ in 0..times {
        result.push(format!("{} {}", get_inc(), get_word_of_len(9)));
    }

    if rest > 0 {
        result.push(format!("{} {}", get_inc(), get_word_of_len(rest as usize)));
    }

    result.join("\n")
}

fn main() {
    if atty::is(Stream::Stdin) {
        return;
    }

    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let chars = buf.chars().collect::<Vec<char>>();
    // let len = chars.len();

    for c in chars {
        println!("{}", translate_to_inc(&c));

        println!("{}", get_out());

        println!("{}", set_current_mem_to_zero());
    }

    println!("{}", get_end());
}
