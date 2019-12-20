use std::iter::Peekable;
use std::str::Chars;

use crate::error::Error;
use crate::json::Json;
use crate::parser::node;

pub fn array(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error>
{
    let mut array = vec![];

    #[derive(Debug, PartialEq)]
    enum Stages {
        Start,
        FirstValue,
        Value,
        Comma,
        End,
    }

    let mut source      = String::new();
    let mut stage = Stages::Start;

    'tokenizer: loop {
        let current = match slice.peek() {
            Some(chr) => *chr,
            None      => { break 'tokenizer },
        };

        // absorb all whitespace into the array source
        match current {
            ' ' | '\r' | '\n' | '\t' => {
                source.push(current);
                slice.next();
                continue;
            },

            _ => {}
        };

        match stage {
            Stages::Start => match current {
                '[' => {
                    stage = Stages::FirstValue;
                    slice.next();
                    source.push(current);
                },

                // Waiting for quotation mark.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
            Stages::FirstValue => match current {
                ']' => { stage = Stages::End; },
                _   => {
                    stage = Stages::Comma;

                    let node = match node(slice) {
                        Ok(node) => node,
                        Err(e)   => { return Err(e) },
                    };

                    source.push_str(&node.to_source());

                    array.push(node);
                },
            },
            Stages::Comma => match current {
                ',' => {
                    stage = Stages::Value;
                    slice.next();
                    source.push(current);
                },
                ']' => { stage = Stages::End; },

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

                array.push(node);
            },
            Stages::End => match current {
                ']' => {
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
    }

    Ok(Json::Array(array, source))
}

