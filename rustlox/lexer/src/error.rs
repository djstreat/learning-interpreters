pub enum LexError {
    UnterminatedString(usize)
    UnexpectedCharacter(usize, char),
    UnexpectedToken(usize, String),
    UnexpectedEOF(usize),
}

pub enum ParseError {
    InvalidOperands(String),
    UnsupportedOperator(String),
    InvalidOperand(String),
}

// #[derive(PartialEq, Debug, Clone)]
// pub struct LexError {
//     pub message: String,
//     pub line: usize,
// }

// impl LexError {
//     pub fn new(error_msg: &str, line: usize) -> Self {
//         LexError {
//             message: format!("[Line {}]Unexpected character: {}", line, error_msg),
//             line,
//         }
//     }
// }

// impl fmt::Display for LexError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Error at line {}: {}", self.line, self.message)
//     }
// }
