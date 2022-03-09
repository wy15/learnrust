use serde_pickle::{de, value};
pub fn challenge8() {
    let un = r#"BZh91AY&SYA\xaf\x82\r\x00\x00\x01\x01\x80\x02\xc0\x02\x00 \x00!\x9ah3M\x07<]\xc9\x14\xe1BA\x06\xbe\x084"#;
    let pw = r#"BZh91AY&SY\x94$|\x0e\x00\x00\x00\x81\x00\x03$ \x00!\x9ah3M\x13<]\xc9\x14\xe1BBP\x91\xf08"#;
    // af82\r000001018002c00200
    println!("{} {}", un, pw);
    println!(
        "{:#?}",
        de::from_slice::<value::Value>(un.as_bytes(), Default::default())
    );
    println!(
        "{:#?}",
        de::from_slice::<value::Value>(pw.as_bytes(), Default::default())
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
