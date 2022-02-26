pub fn challenge4() {
    let mut n: i32;
    let mut resp: i32;

    n = 249;
    resp = 66831;
    while n < 400 {
        n += 1;
        println!("{} {}", n, resp);
        resp = reqwest::blocking::get(format!(
            "http://www.pythonchallenge.com/pc/def/linkedlist.php?nothing={}",
            resp
        ))
        .unwrap()
        .text()
        .unwrap()
        .to_string()
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    }
    println!("{}", resp);
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 44827;
        let resp = reqwest::blocking::get(format!(
            "http://www.pythonchallenge.com/pc/def/linkedlist.php?nothing={}",
            12345
        ))
        .unwrap()
        .text()
        .unwrap()
        .to_string()
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()[5]
            .parse::<i32>()
            .unwrap();
        assert_eq!(result, resp);
    }
}
