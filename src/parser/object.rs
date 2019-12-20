use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use crate::error::Error;
use crate::json::Json;
use crate::parser::{ node, string };

pub fn object(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error>
{
    let mut object = HashMap::new();
    let mut index  = String::new();

    #[derive(Debug, PartialEq)]
    enum Stages {
        Start,
        Index,
        Colon,
        Value,
        Comma,
        End,
    }

    let mut stage = Stages::Start;
    let mut source      = String::new();

    'tokenizer: loop {
        let current = match slice.peek() {
            Some(chr) => *chr,
            None      => { break 'tokenizer },
        };

        match stage {
            Stages::Start => match current {
                ' ' | '\r' | '\n' | '\t' => { slice.next(); },
                '{' => { stage = Stages::Index; slice.next(); },

                // Waiting for quotation mark.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
            Stages::Index => match current {
                ' ' | '\r' | '\n' | '\t' => { slice.next(); },
                '}' => {
                    stage = Stages::End;
                    continue;
                },
                _   => {
                    stage = Stages::Colon;
                    index = match string(slice) {
                        Ok(Json::String(index, index_source)) => {
                            source.push_str(&index_source);
                            index
                        },
                        Err(e) => { return Err(e); },
                        _      => { return Err(Error::InvalidCharacter(current.to_string())); }
                    };
                    continue;
                },
            },
            Stages::Colon => match current {
                ' ' | '\r' | '\n' | '\t' => { slice.next(); },
                ':' => { stage = Stages::Value; slice.next(); },

                // Waiting for valid escape code.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
            Stages::Value => {
                stage = Stages::Comma;

                let node = match node(slice) {
                    Ok(node) => node,
                    Err(e)   => { return Err(e) },
                };


                source.push_str(&node.to_source());
                object.insert(index.clone(), node);
                continue;
            },
            Stages::Comma => match current {
                ' ' | '\r' | '\n' | '\t' => { slice.next(); },
                ',' => { stage = Stages::Index; slice.next(); },
                '}' => { stage = Stages::End; continue; },

                // Waiting for valid escape code.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
            Stages::End => match current {
                ' ' | '\r' | '\n' | '\t' => { slice.next(); },
                '}' => {
                    slice.next();
                    source.push(current);
                    break 'tokenizer;
                },
                // Waiting for valid escape code.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
        }

        source.push(current);
    }

    Ok(Json::Object(object, source))
}

