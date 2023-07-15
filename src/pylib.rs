use std::{fs::File, io::{Write, BufReader, Error, BufRead}, process::Command};

pub fn get_tag(s: &str) -> Result<Vec<(String, String)>, Error>{
    let path = "target/mid.txt";
    let mut file = File::create(path)?;
    write!(file, "{s}")?;

    Command::new("python3")
        .arg("pylib/tagger.py")
        .output()
        .expect("failed");

    let path = "target/mid.csv";
    let file = File::open(path)?;
    let buffered = BufReader::new(file);
    let mut c = String::from("word,tag\n");
    for line in buffered.lines() {
        c += &format!("{}\n", &line?);
    }

    let mut reader = csv::Reader::from_reader(c.as_bytes());
    let mut ret = Vec::new();
    for record in reader.records() {
        let record = record.unwrap();
        ret.push((record[0].to_owned(), record[1].to_owned()))
    }
    return Ok(ret);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let tags = get_tag("You are a good one.").unwrap();
        assert_eq!(6, tags.len());

        assert_eq!(".", tags[5].0);
        assert_eq!(".", tags[5].1);
    }
}