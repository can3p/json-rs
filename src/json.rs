use std::collections::HashMap;


use crate::error::Error;
use crate::parser::node;
use crate::number::Number;

#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Object(HashMap<String, Json>, String),
    Array(Vec<Json>, String),
    String(String, String),
    Number(Number, String),
    Boolean(bool, String),
    Null(String),
}

impl Json {
    pub fn parse(text: &str) -> Result<Json, Error>
    {
        let mut slice    = text.chars();
        let mut peekable = (&mut slice).peekable();

        node(&mut peekable)
    }

    pub fn to_source(&self) -> String
    {
        match self {
            Json::Null(source) => {
                source.clone()
            },
            Json::Boolean(_value, source) => {
                source.clone()
            },
            Json::Number(_value, source) => {
                source.clone()
            },
            Json::String(_value, source) => {
                source.clone()
            },
            Json::Array(_value, source) => {
                source.clone()
            },
            Json::Object(_value, source) => {
                source.clone()
            },
        }
    }

    pub fn to_string(&self) -> String
    {
        let mut string: String = String::new();

        match self {
            Json::Null(_) => {
                string.push_str("null");
            },
            Json::Boolean(value, _) => {
                string.push_str(
                    if *value { "true"  }
                    else      { "false" }
                );
            },
            Json::Number(ref value, _) => {
                string.push_str(value.to_string().as_str());
            },
            Json::String(ref value, _) => {
                string.push('"');
                for chr in value.chars() {
                    if chr == '"' {
                        string.push('\\');
                    }
                    string.push(chr);
                }
                string.push('"');
            },
            Json::Array(ref value, _) => {
                let mut first = true;

                string.push('[');
                for elem in value {
                    if !first {
                        string.push(',');
                    }
                    string.push_str(elem.to_string().as_str());
                    first = false;
                }
                string.push(']');
            },
            Json::Object(ref value, _) => {
                let mut first = true;

                string.push('{');
                for (k, v) in value {
                    if !first {
                        string.push(',');
                    }
                    string.push('"');
                    for chr in k.chars() {
                        if chr == '"' {
                            string.push('\\');
                        }
                        string.push(chr);
                    }
                    string.push('"');
                    string.push(':');
                    string.push_str(v.to_string().as_str());
                    first = false;
                }
                string.push('}');
            },
        }

        string
    }
}

impl From<HashMap<String, Json>> for Json
{
    fn from(map: HashMap<String, Json>) -> Json
    {
        Json::Object(map, "".to_string())
    }
}

impl From<Vec<Json>> for Json
{
    fn from(vector: Vec<Json>) -> Json
    {
        Json::Array(vector, "".to_string())
    }
}

impl From<String> for Json
{
    fn from(string: String) -> Json
    {
        Json::String(string, "".to_string())
    }
}

impl<'a> From<&'a str> for Json
{
    fn from(string: &'a str) -> Json
    {
        Json::String(String::from(string), "".to_string())
    }
}

impl From<u64> for Json
{
    fn from(number: u64) -> Json
    {
        Json::Number(Number::Unsigned(number), "".to_string())
    }
}

impl From<i32> for Json
{
    fn from(number: i32) -> Json
    {
        Json::Number(Number::Integer(i64::from(number)), "".to_string())
    }
}

impl From<i64> for Json
{
    fn from(number: i64) -> Json
    {
        Json::Number(Number::Integer(number), "".to_string())
    }
}

impl From<f64> for Json
{
    fn from(number: f64) -> Json
    {
        Json::Number(Number::Float(number), "".to_string())
    }
}

impl From<bool> for Json
{
    fn from(value: bool) -> Json
    {
        Json::Boolean(value, "".to_string())
    }
}

impl From<()> for Json
{
    fn from(_: ()) -> Json
    {
        Json::Null("".to_string())
    }
}

impl From<Json> for HashMap<String, Json>
{
    fn from(json: Json) -> HashMap<String, Json>
    {
        if let Json::Object(ref value, _) = json {
            return value.clone();

        } else {
            panic!("Expecting Json::Boolean, got {:?}", json);
        }
    }
}

impl From<Json> for Vec<Json>
{
    fn from(json: Json) -> Vec<Json>
    {
        if let Json::Array(ref value, _) = json {
            return value.clone();

        } else {
            panic!("Expecting Json::Boolean, got {:?}", json);
        }
    }
}

impl From<Json> for String
{
    fn from(json: Json) -> String
    {
        if let Json::String(ref value, _) = json {
            return value.clone();

        } else {
            panic!("Expecting Json::String, got {:?}", json);
        }
    }
}

impl From<Json> for u64
{
    fn from(json: Json) -> u64
    { 
        match json {
            Json::Number(value, _) => value.into(),
            _ => {
                panic!("Expecting Json::Number, got {:?}", json);
            }
        }
    }
}

impl From<Json> for i64
{
    fn from(json: Json) -> i64
    {
        if let Json::Number(ref value, _) = json {
            value.clone().into()

        } else {
            panic!("Expecting Json::Number, got {:?}", json);
        }
    }
}

impl From<Json> for f64
{
    fn from(json: Json) -> f64
    {
        if let Json::Number(ref value, _) = json {
            value.clone().into()

        } else {
            panic!("Expecting Json::Number, got {:?}", json);
        }
    }
}

impl From<Json> for bool
{
    fn from(json: Json) -> bool
    {
        if let Json::Boolean(value, _) = json {
            return value;

        } else {
            panic!("Expecting Json::Boolean, got {:?}", json);
        }
    }
}

impl From<Json> for ()
{
    fn from(json: Json)
    {
        if let Json::Null(_) = json {
        } else {
            panic!("Expecting Json::Null, got {:?}", json);
        }
    }
}

