use serde_pickle::{de, value};
pub fn challenge5() {
    let resp = reqwest::blocking::get("http://www.pythonchallenge.com/pc/def/banner.p")
        .unwrap()
        .text()
        .unwrap();
    let pd = resp.as_bytes();
    let rs = de::value_from_slice(pd, Default::default()).unwrap();
    println!("{:?}", value::from_value::<Vec<value::Value>(rs));
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
