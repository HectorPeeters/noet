use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Location {
    pub line: u32,
    pub col: u32,
}

impl Location {
    pub fn new(line: u32, col: u32) -> Self {
        Self { line, col }
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        match self.line.cmp(&other.line) {
            Ordering::Equal => self.col.cmp(&other.col),
            x => x,
        }
    }
}

pub struct Span {
    pub start: Location,
    pub end: Location,
}

impl Span {
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    pub fn extend(&mut self, other: &Span) {
        self.start = std::cmp::min(self.start, other.start);
        self.end = std::cmp::max(self.end, other.end);
    }
}

pub enum Token {
    Text(String, Span),
    LeftBracket(Span),
    RightBracket(Span),
    FunctionIdentifier(String, Span),
    ArgumentSeparator(Span),
    ParagraphBreak(Span),
}

impl Token {
    pub fn span(&self) -> &Span {
        match self {
            Token::Text(_, s) => s,
            Token::LeftBracket(s) => s,
            Token::RightBracket(s) => s,
            Token::FunctionIdentifier(_, s) => s,
            Token::ArgumentSeparator(s) => s,
            Token::ParagraphBreak(s) => s,
        }
    }
}
