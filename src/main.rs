use atty::Stream;
use rand::Rng;
use std::{
    collections::HashMap,
    io::{self, Read},
    sync::LazyLock,
};

static WORD_LOOKUP: LazyLock<HashMap<usize, Vec<&str>>> = LazyLock::new(|| {
    let mut word_lookup = HashMap::<usize, Vec<&str>>::with_capacity(10000);
    let wordlist = include_str!("wordlist.10000.txt");
    wordlist.lines().for_each(|word| {
        word_lookup
            .entry(word.len())
            .and_modify(|f| f.push(word))
            .or_insert(vec![word]);
    });

    word_lookup
});

fn get_word_of_len(len: usize) -> String {
    match WORD_LOOKUP.get(&len) {
        Some(words) => words[rand::thread_rng().gen_range(0..words.len())].to_string(),
        None => str::repeat("a", len),
    }
}

fn get_inc() -> String {
    // len of 3
    // String::from("inc")
    get_word_of_len(3)
}

fn get_dec() -> String {
    // len of 4
    // String::from("decc")
    get_word_of_len(4)
}

fn get_out() -> String {
    // len of 7
    // String::from("outputa")
    get_word_of_len(7)
}

fn get_end() -> String {
    // len of 10 / 0
    // String::from("endprogram")
    get_word_of_len(10)
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
    format!(
        "{} {} {} {}",
        get_if(),
        get_dec(),
        get_word_of_len(1),
        get_eif()
    )
}

fn translate_to_inc(n: i32) -> String {
    let times = n / 9;
    let rest = n % 9;

    let mut result = Vec::new();

    for _ in 0..times {
        result.push(format!("{} {}", get_inc(), get_word_of_len(9)));
    }

    if rest > 0 {
        result.push(format!("{} {}", get_inc(), get_word_of_len(rest as usize)));
    }

    result.join("\n")
}

fn translate_to_dec(n: i32) -> String {
    let times = n / 9;
    let rest = n % 9;

    let mut result = Vec::new();

    for _ in 0..times {
        result.push(format!("{} {}", get_dec(), get_word_of_len(9)));
    }

    if rest > 0 {
        result.push(format!("{} {}", get_dec(), get_word_of_len(rest as usize)));
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

    let mut last_val: u8 = 0;
    for c in chars {
        let ascii_value = c as u8;
        let diff_to_last = ascii_value as i32 - last_val as i32;

        match diff_to_last.cmp(&0) {
            std::cmp::Ordering::Less => {
                println!("{}", translate_to_dec(diff_to_last.abs()));
            }
            std::cmp::Ordering::Greater => {
                println!("{}", translate_to_inc(diff_to_last.abs()));
            }
            std::cmp::Ordering::Equal => {}
        }

        println!("{}", get_out());

        last_val = ascii_value;
    }

    println!("{}", get_end());
}
