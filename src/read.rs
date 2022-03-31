use serde::{Serialize, Deserialize};
/// Token Types Enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    Name,
    Number,
    SemiColon,
    StringLiteral,
    NewLine,
    WhiteSpace,
    Comment,
    End,
}