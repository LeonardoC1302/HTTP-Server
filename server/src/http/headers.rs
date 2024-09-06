use std::collections::HashMap;
use std::convert::{From, TryFrom};
use std::iter::Iterator;

use std::fmt;
use std::str::Split;

type HeadersDataType = HashMap<String, String>;

pub struct Headers {
    data: HeadersDataType
}

impl Headers {
    pub fn user_agent(&self) -> Option<&String> {
        self.get("User-Agent")
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.data.iter()
    }
}

impl TryFrom<&mut Split<'_, char>> for Headers {
    type Error = &'static str;

    fn try_from(str_iter: &mut Split<'_, char>) -> Result<Self, Self::Error> {
        let mut data = HeadersDataType::new();
        for line in str_iter {
            if line.trim().is_empty() {
                break;
            }
            let mut line_splitted = line.splitn(2, ':');
            let k = match line_splitted.next() {
                Some(s) => s.trim(),
                None => return Err("Invalid header"),
            };
            if k.is_empty() {
                continue;
            }
            let v = match line_splitted.next() {
                Some(s) => s.trim(),
                None => return Err("Invalid header, no ': ' found"),
            };
            data.insert(String::from(k), String::from(v));
        }
        Ok(Self { data })
    }
}

impl From<&Vec<(&str, &str)>> for Headers {
    fn from(vec: &Vec<(&str, &str)>) -> Self {
        Self {
            data: vec
                .iter()
                .map(|(k, v)| (String::from(*k), String::from(*v)))
                .collect(),
        }
    }
}

impl fmt::Debug for Headers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.data.iter()).finish()
    }
}