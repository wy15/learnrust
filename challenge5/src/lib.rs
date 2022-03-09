use serde_pickle::{de, value};
pub fn challenge5() {
    let resp = reqwest::blocking::get("http://www.pythonchallenge.com/pc/def/banner.p")
        .unwrap()
        .text()
        .unwrap();
    let pd = resp.as_bytes();
    let rs = de::from_slice(pd, Default::default()).unwrap();
    println!("{:?}", rs);
    if let value::Value::List(l0) = rs {
        for ll in l0 {
            if let value::Value::List(l1) = ll {
                for lll in l1 {
                    if let value::Value::Tuple(t0) = lll {
                        if let value::Value::I64(i0) = t0[1] {
                            for _ in 0..i0 {
                                if let value::Value::Bytes(b0) = &t0[0] {
                                    print!("{}", std::str::from_utf8(&b0).unwrap());
                                }
                            }
                        }
                    }
                }
                print!("\n");
            }
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
