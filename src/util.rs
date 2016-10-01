use std::fmt;
use lalrpop_util::ParseError;

pub struct Range {
    start: Pos,
    end: Pos,
}

pub struct Pos {
    line: usize,
    col: usize,
}

#[derive(Debug)]
pub struct LineInfo {
    line: usize,
    source_start: usize,
    source_end: usize,
}

/**
 * Helper function that calculates the line info based on the source text
 */
pub fn calculate_line_info(source_text: &str) -> Vec<LineInfo> {
    // TODO : cache this somehow, in case we get multiple errors?
    let mut line_info = vec![];
    let mut line_num = 0;
    let mut source_start = 0;
    let mut source_end = -1i64;
    for c in source_text.chars() {
        source_end += 1;
        if c == '\n' {
            line_info.push(LineInfo {
                line: line_num,
                source_start: source_start,
                source_end: source_end as usize,
            });
            source_start = 1 + source_end as usize;
            line_num += 1;
        }
    }
    line_info.push(LineInfo {
        line: line_num,
        source_start: source_start,
        source_end: source_end as usize,
    });
    line_info
}

/**
 * A ParseError wrapper that incorporates a little more information
 */
pub struct RaspParseError<'a, 'source> {
    err: ParseError<usize, (usize, &'a str), ()>,
    line_info: Vec<LineInfo>, // TODO : make this a reference
    source_file: &'source str,
}

/**
 *
 */
impl<'a, 'source> RaspParseError<'a, 'source> {
    pub fn new(err: ParseError<usize, (usize, &'a str), ()>, source_text: &str, source_file: &'source str) -> RaspParseError<'a, 'source> {
        RaspParseError {
            err: err,
            line_info: calculate_line_info(source_text),
            source_file: source_file,
        }
    }

    /**
     * Gets the LineInfo from the array by performing a linear search.
     */
    fn line_from_index(&self, index: usize) -> &LineInfo {
        for line_info in &self.line_info {
            if line_info.source_start <= index && line_info.source_end > index {
                return line_info;
            }
            println!("lineinfo: {:?}", line_info);
        }
        panic!("Invalid source index when dealing with {}: {}", self.source_file, index);
    }
}

impl<'a, 'source> fmt::Display for RaspParseError<'a, 'source> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : ", self.source_file).expect("could not write");
        match self.err {
            /**
             * Invalid token occurs on a character that wasn't recognized in the parser. We probably shouldn't get these
             * once the parser is finished.
             */
            ParseError::InvalidToken { location } => {
                let info = self.line_from_index(location);
                let col = location - info.source_start;
                //                                                      + 1 because these are indices
                write!(f, "{}:{} : start of invalid token encountered", info.line + 1, col+1)
            },
            /**
             * Unrecognized token occurs when we got a token that we didn't expect. This *does* contain a range that
             * covers the token encountered.
             */
            ParseError::UnrecognizedToken { token, ref expected } => {
                if let Some(t) = token {
                    let tok = t.1;
                    let start = t.0;
                    let end = t.2;
                    let info = self.line_from_index(start);
                    write!(f, "{}:{}-{} : unexpected token {} at source index", info.line + 1, start, 
                        end, tok.1)
                }
                else {
                    write!(f, "unexpected EOF")
                }.expect("could not write");
                writeln!(f, ", expected one of the following:").expect("could not write");
                for ex in expected {
                    write!(f, "    {}", ex).expect("could not write");
                }
                write!(f, "")
            },
            ParseError::ExtraToken { token } => {
                let tok = token.1;
                let start = token.0;
                let end = token.2;
                let info = self.line_from_index(start);
                write!(f, "{}:{}-{} : unexpected token {} at source index", info.line + 1, start, end, tok.1)
            }
            _ => write!(f, "")
        }
    }
}
