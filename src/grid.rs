use std::fmt;
use Command;

#[deriving(PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid {
    width: u32,
    height: u32,
    chars: Vec<char>,
}

impl fmt::Show for Grid {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        let s: String = String::from_chars(self.chars.as_slice());
        write!(w, "Grid {{ width: {}, height: {}, chars: {} }}",
               self.width, self.height, s)
    }
}

#[deriving(PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseErr {
    PrematureLineEnd(u32, String, u32),
    BadTerminationChar(u32, String, char),
}

impl fmt::Show for ParseErr {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PrematureLineEnd(h, ref s, width) =>
                write!(w, "line {} content {} ended prematurely; \
                           expected {} characters", h, s, width),
            BadTerminationChar(h, ref s, c) =>
                write!(w, "line {} expected end of input or line \
                           after {}, got {}", h, s, c),
        }
    }
}

impl Grid {
    pub fn new(width: u32, height: u32, background: char) -> Grid {
        let len = width * height;
        let chars = Vec::from_elem(len.to_uint().unwrap(), background);
        Grid { width: width, height: height, chars: chars }
    }

    pub fn to_string(&self) -> String {
        let len = self.height * (self.width + 1);
        let mut s = String::with_capacity(len.to_uint().unwrap());
        for y in range(0, self.height) {
            for x in range(0, self.width) {
                s.push(self.get(x, y));
            }
            s.push('\n');
        }
        s
    }

    pub fn from_str(s: &str) -> Result<Grid, ParseErr> {
        let mut w = 0u32;
        for c in s.chars() {
            if c == '\n' {
                break;
            } else {
                w += 1;
            }
        }

        let mut grid = vec![];
        let mut chars = s.chars();
        let mut h = 0;

        loop {
            let mut started_line = false;
            let row_start = grid.len();
            for i in range(0, w) {
                match (i, chars.next()) {
                    (0, None) => break,
                    (_, None) => {
                        let line = String::from_chars(grid.slice_from(row_start));
                        return Err(PrematureLineEnd(h, line, w))
                    }
                    (_, Some(c)) => {
                        if !started_line {
                            h += 1;
                            started_line = true;
                        }
                        grid.push(c)
                    }
                }
            }
            match chars.next() {
                None => break,
                Some('\n') => continue,
                Some(c) => {
                    let line = String::from_chars(grid.slice_from(row_start));
                    return Err(BadTerminationChar(h, line, c))
                }
            }
        }

        Ok(Grid { width: w, height: h, chars: grid })
    }

    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }

    pub fn get(&self, x: u32, y: u32) -> char {
        assert!(x < self.width);
        assert!(y < self.height);
        let idx = y * self.width + x;
        self.chars[idx.to_uint().unwrap()]
    }

    pub fn set(&mut self, x: u32, y: u32, c: char) {
        assert!(x < self.width);
        assert!(y < self.height);
        let idx = y * self.width + x;
        // *self.chars.get_mut(idx.to_uint().unwrap()) = c;
        self.chars[idx.to_uint().unwrap()] = c;
    }

    pub fn exec(&mut self, command: &Command) {
        let Command { x, y, w, h, fill } = *command;

        if w == 1 || h == 1 {
            return self.draw_line(command);
        }

        self.set(x, y, '+');
        self.set(x, y+h-1, '+');
        self.set(x+w-1, y, '+');
        self.set(x+w-1, y+h-1, '+');

        for i in range(x + 1, x + w - 1) {
            self.set(i, y, '-');
            self.set(i, y + h - 1, '-');
        }

        for j in range(y + 1, y + h - 1) {
            self.set(x, j, '|');
            self.set(x + w - 1, j, '|');
        }

        for i in range(x+1, x + w - 1) {
            for j in range(y + 1, y + h - 1) {
                self.set(i, j, fill);
            }
        }
    }

    fn draw_line(&mut self, command: &Command) {
        assert!(command.w == 1 || command.h == 1);
        let Command { x, y, w, h, fill } = *command;
        for i in range(x, x+w) {
            for j in range(y, y+h) {
                self.set(i, j, fill);
            }
        }
    }
}

#[test]
fn simple_parse() {
    assert_eq!(Grid::from_str("...").unwrap(),
               Grid {width: 3, height: 1, chars: "...".chars().collect() });
    assert_eq!(Grid::from_str("...\n").unwrap(),
               Grid {width: 3, height: 1, chars: "...".chars().collect() });
    assert_eq!(Grid::from_str("...\n...\n").unwrap(),
               Grid {width: 3, height: 2, chars: "......".chars().collect() });
    assert_eq!(Grid::from_str("...\n...").unwrap(),
               Grid {width: 3, height: 2, chars: "......".chars().collect() });
    assert_eq!(Grid::from_str("abc\ndef\n").unwrap(),
               Grid {width: 3, height: 2, chars: "abcdef".chars().collect() });
}

#[test]
fn premature_line_end() {
    assert_eq!(Grid::from_str("abc\nd"),
               Err(PrematureLineEnd(2, "d".to_string(), 3)));
}

#[test]
fn unexpected_terminator() {
    assert_eq!(Grid::from_str("abc\ndefg"),
               Err(BadTerminationChar(2, "def".to_string(), 'g')))
}
