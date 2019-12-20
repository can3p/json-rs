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
//    Whitespace(String),
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
        self.to_source()
    }
}

impl From<HashMap<String, Json>> for Json
{
    fn from(map: HashMap<String, Json>) -> Json
    {
        let mut source_string = String::new();
        source_string.push('{');
        let mut iterator = map.iter();
        match iterator.next() {
            Some((key, val)) => {
                let part = format!("\"{}\":{}", key, &val.to_source());
                source_string.push_str(&part);
            },
            None => {
                source_string.push('}');
                return Json::Object(map, source_string);
            }
        };

        while let Some((key, val)) = iterator.next() {
            let part = format!(",\"{}\":{}", key, &val.to_source());
            source_string.push_str(&part);
        }

        source_string.push('}');
        Json::Object(map, source_string)
    }
}

impl From<Vec<Json>> for Json
{
    fn from(vector: Vec<Json>) -> Json
    {
        let mut source_string = String::new();
        source_string.push('[');
        let mut iterator = vector.iter();
        match iterator.next() {
            Some(val) => {
                source_string.push_str(&val.to_source());
            },
            None => {
                source_string.push(']');
                return Json::Array(vector, source_string);
            }
        };

        while let Some(val) = iterator.next() {
            source_string.push(',');
            source_string.push_str(&val.to_source());
        }

        source_string.push(']');
        return Json::Array(vector, source_string);
    }
}

impl From<String> for Json
{
    fn from(string: String) -> Json
    {
        let mut source_string = String::new();
        source_string.push('\"');
        source_string.push_str(&string);
        source_string.push('\"');
        Json::String(string, source_string)
    }
}

impl<'a> From<&'a str> for Json
{
    fn from(string: &'a str) -> Json
    {
        let mut source_string = String::new();
        source_string.push('\"');
        source_string.push_str(&string);
        source_string.push('\"');
        Json::String(String::from(string), source_string)
    }
}

impl From<u64> for Json
{
    fn from(number: u64) -> Json
    {
        Json::Number(Number::Unsigned(number), number.to_string())
    }
}

impl From<i32> for Json
{
    fn from(number: i32) -> Json
    {
        Json::Number(Number::Integer(i64::from(number)), number.to_string())
    }
}

impl From<i64> for Json
{
    fn from(number: i64) -> Json
    {
        Json::Number(Number::Integer(number), number.to_string())
    }
}

impl From<f64> for Json
{
    fn from(number: f64) -> Json
    {
        Json::Number(Number::Float(number), number.to_string())
    }
}

impl From<bool> for Json
{
    fn from(value: bool) -> Json
    {
        let source = match value {
            true => "true",
            false => "false",
        };

        Json::Boolean(value, source.to_string())
    }
}

impl From<()> for Json
{
    fn from(_: ()) -> Json
    {
        Json::Null("null".to_string())
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

