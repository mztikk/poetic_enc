use const_gen::*;
use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("const_gen.rs");
    let mut word_lookup: HashMap<u64, Vec<&str>> = HashMap::new();
    let wordlist = include_str!("wordlist.10000.txt");
    wordlist.lines().for_each(|word| {
        word_lookup
            .entry(word.len() as u64)
            .and_modify(|f| f.push(word))
            .or_insert(vec![word]);
    });

    let const_declarations = vec![const_declaration!(WORD_LOOKUP = word_lookup)].join("\n");
    fs::write(&dest_path, const_declarations).unwrap();
}
