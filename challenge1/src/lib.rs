pub fn challenge1() {
    let odd = "g fmnc wms bgblr rpylqjyrc gr zw fylb. rfyrq ufyr amknsrcpq ypc dmp. bmgle gr gl zw fylb gq glcddgagclr ylb rfyr'q ufw rfgq rcvr gq qm jmle. sqgle qrpgle.kyicrpylq() gq pcamkkclbcb. lmu ynnjw ml rfc spj.".as_bytes();
    let newurl = "map".as_bytes();
    println!(
        "{}",
        odd.iter()
            .map(|&b| {
                if b >= 97 && b <= 122 {
                    (b - 97 + 2) % 26 + 97
                } else {
                    b
                }
            })
            .collect::<Vec<u8>>()
            .iter()
            .map(|&b| b as char)
            .collect::<String>()
    );
    println!(
        "{}",
        newurl
            .iter()
            .map(|&b| {
                if b >= 97 && b <= 122 {
                    (b - 97 + 2) % 26 + 97
                } else {
                    b
                }
            })
            .collect::<Vec<u8>>()
            .iter()
            .map(|&b| b as char)
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
