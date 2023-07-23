use std::{rc::Rc, ops::IndexMut};

use nom::{InputTake, Compare, CompareResult, InputLength};

#[derive(Debug)]
pub struct Tagged {
    pub sentence: Rc<Vec<(String, String)>>, // 0: word, 1: tag
    pub start: usize,
    pub end: usize,
}

impl Tagged {
    pub fn get(&self, index: usize) -> (String, String) {
        self.sentence[index].to_owned()
    }
}

impl From<Vec<(&str, &str)>> for Tagged {
    fn from(v: Vec<(&str, &str)>) -> Self {
        let sentence: Vec<(String, String)> = v.iter().map(|&x| (x.0.to_owned(), x.1.to_owned())).collect();
        Tagged { sentence: Rc::new(sentence), start: 0, end: v.len() }
    }
}

impl PartialEq for Tagged {
    fn eq(&self, other: &Self) -> bool {
        if self.end - self.start != other.end - other.start {
            return false;
        } 
        let mut ret = true;
        for i in self.start..self.end {
            if self.sentence[i] != other.sentence[other.start + i - self.start] {
                ret = false
            }
        }
        ret
    }
}
impl Clone for Tagged {
    fn clone(&self) -> Self {
        Self { sentence: Rc::clone(&self.sentence), start: self.start.clone(), end: self.end.clone() }
    }
}
impl InputTake for Tagged {
    fn take(&self, count: usize) -> Self {
        if self.start + count > self.end {
            panic!("error take in InputTake impled by Tagged")
        }
        Tagged { sentence: Rc::clone(&self.sentence), start: self.start, end: self.start + count }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        if self.start + count > self.end {
            panic!("error take in InputTake impled by Tagged")
        }
        (
            Tagged { sentence: Rc::clone(&self.sentence), start: self.start + count, end: self.end },
            Tagged { sentence: Rc::clone(&self.sentence), start: self.start, end: self.start + count },
        )
    }
}

impl Compare<&Tags> for Tagged {
    fn compare(&self, t: &Tags) -> nom::CompareResult {
        let pos = self.sentence.iter().zip(t.tags.iter()).position(|(a, b)| &a.1 != b);

        match pos {
            Some(_) => CompareResult::Error,
            None => {
                if self.sentence.len() >= t.tags.len() {
                    CompareResult::Ok
                } else {
                    CompareResult::Incomplete
                }
            }
        }
    }

    fn compare_no_case(&self, t: &Tags) -> nom::CompareResult {
        self.compare(t)
    }
}

#[derive(Debug)]
pub struct Tags {
    tags: Vec<String>
}

impl From<String> for Tags {
    fn from(s: String) -> Self {
        Tags { tags: vec![s] }
    }
}

impl From<Vec<&str>> for Tags {
    fn from(v: Vec<&str>) -> Self {
        Tags { tags: v.iter().map(|x| x.to_owned().to_owned()).collect() }
    }
}

impl InputLength for &Tags {
    fn input_len(&self) -> usize {
        self.tags.len()
    }
}

impl Clone for Tags {
    fn clone(&self) -> Self {
        Self { tags: self.tags.clone() }
    }
}