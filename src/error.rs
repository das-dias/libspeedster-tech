/** ***********************************
* *[author] Diogo André (git-hub : das-dias)
* *[date] 21-03-2022
* *[filename] errors.rs
* *[summary] Handle and Generate Errors
* ***********************************
*/
//std lib

// crates.io
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

/**
* *[name] TLefParserContext
* *[description] Provides Parser error contexts
* [variables]
* @par Library  : Library error context
* @par Macro    : Macro / Rules error context (unrecognized macro/rule)
* @par Geometry : Geometry error context (unrecognized geometry params)
* @par Site     : Site error context 
* @par Units    : Units error context (unrecognized parsed units)
* @par Unknown  : Unknown error context (unknown token, symbol, or string was parsed)
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TLefParserContext {
    Library,
    Macro, // Rules
    Geometry,
    Site,
    Units,
    Unknown,
}

/**
* *[name] TLefParserErrorType
* *[description] Provides parser error types
* [variables]
* @par Unsupported  : Unsupported (But Spec-Valid) Features
* @par InvalidKey   : Invalid Key
* @par InvalidValue : Invalid Value
* @par InvalidToken : Invalid Token
* @par RequireWord  : Syntax Error: missing keyword or identifier at a required location
* @par Other        : All other errors
*/
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TLefParserErrorType {
    Unsupported,
    InvalidKey,
    InvalidValue,
    InvalidToken { expected: super::read::TokenType },
    RequiredWord { expected: String },
    Other,
}

/**
* *[name] TLefError
* *[description] Error type for the tech parsing library
* [variables]
* @par Lexer    (struct)                        : Lexer errors - check lexical integrity of LEF spec
* @par Parser   (struct)                        : Parser errors - check errors existing on the TLEF file when reading it
* @par Boxed    (Box<dyn std::error::Error>)    : Standard environmnet error wrapper for crate-specific errors
* @par Str      (String)                        : Environment error wrapper for string (error) messages
*/
#[derive(Debug)]
pub enum TLefError {
    // Lexer Errors
    Lexer {
        next_char: Option<char>,
        line: usize,
        pos: usize,
    },
    // Parser Errors
    Parser {
        tp: TLefParserErrorType,
        ctx: Vec<TLefParserContext>,
        token: String,
        line_content: String,
        line_num: usize,
        pos: usize,
    },
    /// Wrapped errors, generally from other crates
    Boxed(Box<dyn std::error::Error>),
    /// String message-valued errors
    Str(String),
}
impl From<crate::utils::errors::Error> for TLefError {
    fn from(err: crate::utils::ser::Error) -> Self {
        Self::Boxed(Box::new(err))
    }
}
impl From<std::io::Error> for TLefError {
    fn from(err: std::io::Error) -> Self {
        Self::Boxed(Box::new(err))
    }
}
impl From<rust_decimal::Error> for TLefError {
    fn from(err: rust_decimal::Error) -> Self {
        Self::Boxed(Box::new(err))
    }
}
impl From<String> for TLefError {
    /// Convert string-based errors by wrapping them
    fn from(err: String) -> Self {
        Self::Str(err)
    }
}
impl From<&str> for TLefError {
    /// Convert string-based errors by wrapping them
    fn from(err: &str) -> Self {
        Self::Str(err.into())
    }
}

/**
* *[name] TLefResult
* *[description] Techn Parser lib Result
*/
pub type TLefResult<T> = Result<T, TLefError>;