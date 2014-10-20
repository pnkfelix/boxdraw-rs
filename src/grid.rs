use Command;

pub struct Grid {
    width: u32,
    height: u32,
    chars: Vec<char>,
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
