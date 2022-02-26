use regex::Regex;
use std::fs;
pub fn challenge3() {
    let contents = fs::read_to_string("/Users/qima/work/2022/learnrust/data/3")
        .expect("Something went wrong reading the file");
    let re = Regex::new(r"([a-z]{1})([A-Z]{3})([a-z]{1})([A-Z]{3})([a-z]{1})").unwrap();
    contents.lines().for_each(|line| {
        for cap in re.captures_iter(line) {
            print!("{}", cap.get(3).unwrap().as_str());
        }
    });
    print!("\n");
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
