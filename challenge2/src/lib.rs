use std::collections::HashMap;
use std::fs;

pub fn findrarechar() {
    let contents = fs::read_to_string("/Users/qima/work/2022/learnrust/data/ocr")
        .expect("Something went wrong reading the file");
    let mut rare_char = ' ';
    let mut _rare_char_count = u32::MAX;
    let mut kv = HashMap::new();
    for c in contents.chars() {
        if c == '\n' || c == '\r' {
            continue;
        }
        let count = kv.entry(c).or_insert(0);
        *count += 1;
    }
    println!("{:?}", kv);
    for (k, v) in kv {
        if v < _rare_char_count {
            rare_char = k;
            _rare_char_count = v;
        }
    }
    println!("{}", rare_char);
    println!(
        "{}",
        contents
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect::<String>()
    );
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
