use image::imageops::crop_imm;
use image::{GenericImageView, Rgba};

pub fn challenge7() {
    let img = image::open("/Users/qima/work/2022/learnrust/data/oxygen.png").unwrap();
    let (width, height) = img.dimensions();
    // get center line
    let centerimg = crop_imm(&img, 0, height / 2, width, 1);
    let rs = centerimg
        .pixels()
        .step_by(7)
        .filter_map(
            |(_, _, Rgba([x, y, z, _]))| {
                if x == y && y == z {
                    Some(x)
                } else {
                    None
                }
            },
        )
        .collect::<Vec<u8>>();
    println!("{}", String::from_utf8(rs).unwrap());
    // smart guy, you made it. the next level is [105, 110, 116, 101, 103, 114, 105, 116, 121]
    let nextlevel = vec![105, 110, 116, 101, 103, 114, 105, 116, 121];
    println!("{}", String::from_utf8(nextlevel).unwrap());
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
