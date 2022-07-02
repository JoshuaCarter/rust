pub use std::fmt::Display;

use serde_json::Value;
use anyhow::*;

type Pairs = Vec<(String, String)>;

#[derive(Debug)]
pub struct Params {
    pairs: Pairs,
}

impl From<Value> for Params {
    fn from(from: Value) -> Self {
        return Params { pairs: json_to_vec(&from).unwrap() };
    }
}
impl From<&Value> for Params {
    fn from(from: &Value) -> Self {
        return Params { pairs: json_to_vec(from).unwrap() };
    }
}
impl From<Pairs> for Params {
    fn from(from: Pairs) -> Self {
        return Params { pairs: from };
    }
}
impl From<&str> for Params {
    fn from(from: &str) -> Self {
        return Params { pairs: str_to_vec(from).unwrap() };
    }
}

impl Display for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(vec_to_str(&self.pairs).unwrap().as_str());
    }
}

impl Params {
    pub fn for_query(&self) -> &Pairs {
        return &self.pairs;
    }

    pub fn insert(&mut self, name: &str, value: &str) -> &Self {
        self.pairs.push((String::from(name), String::from(value)));
        return self;
    }

    pub fn remove(&mut self, name: &str) -> &Self {
        let find = self.pairs.iter().position(|v| { name == v.0 });
        match find {
            Some(i) => { self.pairs.swap_remove(i); },
            None => (),
        }
        return self;
    }
}

fn json_to_vec(json: &Value) -> Result<Pairs, Error> {
    let mut params: Pairs = Vec::new();

    match json {
        Value::Object(ref map) => {
            for (key, val) in map {
                match val {
                    Value::Null => params.push((key.to_string(), String::from("Null"))),
                    Value::Bool(x) => params.push((key.to_string(), x.to_string())),
                    Value::Number(ref x) => params.push((key.to_string(), x.to_string())),
                    Value::String(ref x) => params.push((key.to_string(), x.to_string())),
                    _ => return Err(anyhow!("Value type not supported")),
                }
            }
        },
        _ => return Err(anyhow!("Expected Map<String,Value> at JSON root")),
    }

    return Ok(params);
}

fn vec_to_str(params: &Pairs) -> Result<String, Error> {
    let mut string: String = String::new();

    for (key, val) in params {
        let amp = if string.is_empty() { "" } else { "&" };
        string.push_str(format!("{}{}={}", amp, key, val).as_str());
    }

    return Ok(string);
}

fn str_to_vec(string: &str) -> Result<Pairs, Error> {
    let mut params: Pairs = Vec::new();

    for pair in string.split('&') {
        let (key, val) = pair.split_once('=').unwrap();
        params.push((String::from(key), String::from(val)));
    }

    return Ok(params);
}
