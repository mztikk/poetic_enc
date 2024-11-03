use atty::Stream;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::{
    collections::HashMap,
    io::{self, Read},
    str::Chars,
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

struct Separator {
    x: &'static str,
}

impl Distribution<Separator> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Separator {
        const SEPARATORS: [&str; 31] = [
            " ", "#", "+", "-", "_", ":", ";", ".", ",", "*", "/", "(", ")", "[", "]", "=", "!",
            "?", "$", "%", "&", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ];
        Separator {
            x: SEPARATORS[rng.gen_range(0..SEPARATORS.len())],
        }
    }
}

fn get_separator() -> String {
    rand::thread_rng()
        .sample_iter(Standard)
        .take(rand::thread_rng().gen_range(1..5))
        .map(|f: Separator| f.x)
        .collect::<Vec<&'static str>>()
        .join("")
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
        result.push(format!(
            "{}{}{}",
            get_inc(),
            get_separator(),
            get_word_of_len(9)
        ));
    }

    if rest > 0 {
        result.push(format!(
            "{}{}{}",
            get_inc(),
            get_separator(),
            get_word_of_len(rest as usize)
        ));
    }

    result.join("\n")
}

fn translate_to_dec(n: i32) -> String {
    let times = n / 9;
    let rest = n % 9;

    let mut result = Vec::new();

    for _ in 0..times {
        result.push(format!(
            "{}{}{}",
            get_dec(),
            get_separator(),
            get_word_of_len(9)
        ));
    }

    if rest > 0 {
        result.push(format!(
            "{}{}{}",
            get_dec(),
            get_separator(),
            get_word_of_len(rest as usize)
        ));
    }

    result.join("\n")
}

struct Encoder<'a> {
    input_iter: Chars<'a>,
    last_value: u8,
    is_end: bool,
}

impl<'a> Encoder<'a> {
    pub fn new(input: &'a str) -> Self {
        let iter: Chars<'_> = input.chars().into_iter();
        Encoder {
            input_iter: iter,
            last_value: 0,
            is_end: false,
        }
    }
}

impl<'a> Iterator for Encoder<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end {
            return None;
        }

        match self.input_iter.next() {
            Some(c) => {
                let ascii_value = c as u8;
                let diff_to_last = ascii_value as i32 - self.last_value as i32;
                self.last_value = ascii_value;

                Some(format!(
                    "{}{}{}",
                    match diff_to_last.cmp(&0) {
                        std::cmp::Ordering::Less => translate_to_dec(diff_to_last.abs()),
                        std::cmp::Ordering::Greater => translate_to_inc(diff_to_last.abs()),
                        std::cmp::Ordering::Equal => String::from(""),
                    },
                    get_separator(),
                    get_out()
                ))
            }
            None => {
                self.is_end = true;
                Some(get_end())
            }
        }
    }
}

fn main() {
    if atty::is(Stream::Stdin) {
        return;
    }

    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let encoder = Encoder::new(buf.as_str());
    encoder.for_each(|x| println!("{}", x));
}

#[cfg(test)]
mod tests {
    use super::*;
    use poetic::{interpreter::Interpreter, parser::Parser};
    use rstest::*;
    use std::sync::{Arc, Mutex};

    fn run_and_get_output(input: &str) -> String {
        let encoded = Encoder::new(input).collect::<Vec<String>>().join(" ");
        let intermediate = Parser::parse_intermediate(&encoded);
        let instructions = Parser::parse_instructions(&intermediate).unwrap();
        let result = Arc::new(Mutex::new(String::new()));
        let result_clone = result.clone();

        let mut interpreter = Interpreter::new(instructions).with_output(Box::new(move |s| {
            result_clone.lock().unwrap().push_str(s.as_str());
        }));
        interpreter.run();

        return result.lock().unwrap().to_string();
    }

    #[rstest]
    #[case("test")]
    #[case("täßt")]
    fn encoded_should_decode(#[case] input: &str) {
        let result = run_and_get_output(input);
        assert_eq!(result, input);
    }
}
