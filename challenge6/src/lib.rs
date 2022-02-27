use std::io::Read;
use zip::read::ZipArchive;

pub fn challenge6() {
    let zipfile = std::fs::File::open("/Users/qi/work/2022/learnrust/data/channel.zip").unwrap();
    let mut archive = ZipArchive::new(zipfile).unwrap();
    let mut file = String::new();
    archive
        .by_name("90052.txt")
        .unwrap()
        .read_to_string(&mut file)
        .unwrap();
    print!("{}", archive.by_name("90052.txt").unwrap().comment());
    for _ in 0..archive.len() {
        let filename = file.split(" ").collect::<Vec<&str>>()[3].to_string() + ".txt";
        file.clear();
        archive
            .by_name(&filename)
            .unwrap()
            .read_to_string(&mut file)
            .unwrap();
        print!("{}", archive.by_name(&filename).unwrap().comment());
        // println!("{}", file);
        // println!("{}", file.split(" ").collect::<Vec<&str>>()[3]);
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
