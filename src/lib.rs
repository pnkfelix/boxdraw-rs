/// Script for drawing rectangle-based ASCII art.
#[deriving(Clone)]
pub struct Script {
    /// Width of the generated picture.
    pub width: u32,
    /// Height of the generated picture.
    pub height: u32,
    /// Character used to render background of generated picture.
    pub background: char,

    commands: Vec<Command>,
}

/// Draws a rectangle at (`x`,`y`) with width `w` and height `h`,
/// filled with the character `fill` (if there is room for it).
#[deriving(Clone)]
pub struct Command {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub fill: char,
}

/// Simple constructor method for drawing a rectangle at (`x`,`y`)
/// with width `w` and height `h`, filled with the character `fill`
/// (if there is room for it).
pub fn rect(x: u32, y: u32, w: u32, h: u32, fill: char) -> Command {
    assert!(w > 0);
    assert!(h > 0);
    Command { x: x, y: y, w: w, h: h, fill: fill }
}

mod grid;

impl Script {
    /// Creates an empty script for `width` x `height` picture
    /// with default background `'.'`.
    pub fn new(width: u32, height: u32) -> Script {
        Script::new_commands(width, height, [])
    }

    /// Creates a script for `width` x `height` picture with attached
    /// `commands` and default background `'.'`.
    pub fn new_commands(width: u32, height: u32, commands: &[Command]) -> Script {
        Script::new_bg_commands(width, height, '.', commands)
    }

    /// Creates a script for `width` x `height` picture with attached
    /// `commands` and background `bg`.
    pub fn new_bg_commands(w: u32, h: u32, bg: char, cmds: &[Command]) -> Script {
        let mut s = Script {
            width: w,
            height: h,
            background: bg,
            commands: Vec::with_capacity(cmds.len()),
        };
        for c in cmds.iter() {
            s.add_end_command(*c);
        }
        s
    }

    pub fn add_end_command(&mut self, c: Command) {
        Script::check(c.x, c.w, self.width);
        Script::check(c.y, c.h, self.height);
        self.commands.push(c);
    }

    fn check(i: u32, len: u32, max: u32) {
        assert!(i <= std::u32::MAX - len); // check for overflow
        assert!(len > 0);
        assert!(i + len <= max);
    }

    /// The command sequence for the script.
    pub fn commands(&self) -> &[Command] {
        self.commands.as_slice()
    }

    /// Evaluates the script, producing the string for the image.
    pub fn run(&self) -> String {
        let mut grid = grid::Grid::new(self.width, self.height, self.background);
        for cmd in self.commands.iter() {
            grid.exec(cmd)
        }
        grid.to_string()
    }
}

/// Inverse of drawing: given a picture, create a script to draw that picture.
pub trait Undraw {
    /// Given `picture`, a string that holds a rectangular ASCII art
    /// image, return a script that when run produces the same image.
    fn undraw(&self, picture: String) -> Script;
}

#[cfg(test)]
mod tests;