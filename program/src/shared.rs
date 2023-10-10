pub trait Plugin {
    fn init(&self);
    fn handle_event(&self, editor: &mut Editor, editor_event: EditorEvent);
}

pub struct EventQueue {
}

pub struct Editor {
    pub cursor: (isize, isize),
}

#[derive(Debug)]
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down
}

#[derive(Debug)]
pub enum EditorEvent {
    MoveCursor(MoveDirection),
    Startup()
}

impl Editor {
    pub fn move_cursor_left(&mut self) {
        println!("move cursor left");
        let c = self.cursor;
        self.cursor = (c.0 - 1, c.1);
    }

    pub fn move_cursor_right(&mut self) {
        println!("move cursor right");
        let c = self.cursor;
        self.cursor = (c.0 + 1, c.1);
    }

    pub fn print_cursor(&self) {
        println!("{}", self.cursor.0)
    }
}
