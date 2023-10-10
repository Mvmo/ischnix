use std::{
    io::{stdout, Stdout},
    sync::mpsc,
    time::Duration,
    thread,
};

use ratatui::{
    prelude::{CrosstermBackend, Rect},
    widgets::{Block, Borders, Paragraph}
};

use crossterm::{
    self,
    event::{self, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, cursor::SetCursorShape
};

pub mod shared;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut editor = Editor { cursor: (0, 0) };
//     unsafe {
//         let lib = libloading::Library::new("/Users/mvmo/development/ischnix/plugin/target/debug/libplugin.dylib")?;
//         let init: Symbol<fn() -> Box<dyn Plugin>> = match lib.get(b"initialize") {
//             Ok(func) => func,
//             Err(err) => {
//                 dbg!(err);
//                 return Ok(())
//             }
//         };
//
//         let plugin_instance = (init)();
//         plugin_instance.init();
//         plugin_instance.handle_event(&mut editor, EditorEvent::Startup());
//         // plugin_instance.handle_event(&mut editor, EditorEvent::MoveCursor(ischnix::MoveDirection::Left));
//
//         Ok(())
//     }
// }


// use std::{io::{stdout, Stdout}, process::Stdio};
//
// use ratatui::{Terminal, prelude::{CrosstermBackend, Rect}, Frame, widgets::Widget};
//
// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
//
// pub trait Drawable {
//     fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, rect: Rect);
// }

// pub struct Window {
//     title: String,
//     window_id: usize,
// }

// fn main() -> Result<()> {
//     let backend = CrosstermBackend::new(stdout());
//     let mut terminal = Terminal::new(backend)?;
//
//     terminal.clear();
//
//     let mut windows: Vec<Window> = vec![];
//     loop {
//         terminal.draw(|frame| {
//             // windows.iter().for_each(|window| window.draw(frame.size()));
//         })?;
//     }
//
//     Ok(())
// }
//

// use tree_sitter::Parser;

// fn main() {
//     let mut parser = Parser::new();
//     parser.set_language(tree_sitter_rust::language())
//         .expect("Error loading Rust grammar");
//
//     let source_code = "fn test() {}";
//     let tree = parser.parse(source_code, None).unwrap();
//     let root_node = tree.root_node();
//
//     dbg!(root_node);
// }
//


type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Backend = CrosstermBackend<Stdout>;
type Terminal = ratatui::Terminal<Backend>;
type Frame<'a> = ratatui::Frame<'a, Backend>;

struct Buffer {
    text: String
}

impl Buffer {
    fn new(text: String) -> Self {
        return Self {
            text
        }
    }

    fn draw(&self, frame: &mut Frame, area: Rect) {
        let p = Paragraph::new(self.text.as_str());
        frame.render_widget(p, area);
    }
}

pub struct WindowManager {
    windows: Vec<Window>,
    active_window_idx: usize,
}

impl WindowManager {
    fn push_window(&mut self, window: Window) {
        self.windows.push(window)
    }

    fn active_window(&self) -> &Window {
        // TODO: unwrap
        self.windows.get(self.active_window_idx).unwrap()
    }

    fn active_window_mut(&mut self) -> &mut Window {
        // TODO: unwrap
        self.windows.get_mut(self.active_window_idx).unwrap()
    }

    fn layout_windows(&mut self, frame_rect: Rect) {
        for i in 1..self.windows.len() {
            let (last_window, window) = self.windows.split_at_mut(i);
            let last_window = &mut last_window[i - 1];
            let window = &mut window[0];

            if i == 1 {
                last_window.area = frame_rect;
            }

            let (new_x, new_y) = if i % 2 != 0 {
                let new_width = last_window.area.width / 2;
                let pixels_left = last_window.area.width - (new_width * 2);

                last_window.area.width = new_width + pixels_left;

                (new_width + last_window.area.x, last_window.area.y)
            } else {
                let new_height = last_window.area.height / 2;
                let pixels_left = last_window.area.height - (new_height * 2);

                last_window.area.height = new_height + pixels_left;

                (last_window.area.x, new_height + last_window.area.y)
            };

            let new_w = last_window.area.width;
            let new_h = last_window.area.height;

            let rect = Rect {
                x: new_x,
                y: new_y,
                width: new_w,
                height: new_h,
            };

            window.area = rect;
        }
    }

    fn new() -> Self {
        Self {
            windows: Vec::new(),
            active_window_idx: 0
        }
    }
}

struct Window {
    area: Rect,
    buf_idx: usize,
    cursor: (u16, u16),
}

impl Window {
    fn new(buf_idx: usize, area: Rect) -> Self {
        return Self {
            buf_idx,
            area,
            cursor: (0, 0),
        }
    }

    fn set_active(&self, terminal: &mut Terminal) {
        // TODO: unwrap
        terminal.set_cursor(self.area.x + self.cursor.0, self.area.y + self.cursor.1).unwrap();
    }
}

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    enable_raw_mode()?;
    crossterm::execute!(stdout(), EnterAlternateScreen, SetCursorShape(crossterm::cursor::CursorShape::Block))?;
    terminal.show_cursor()?;
    terminal.set_cursor(5, 8)?;

    let mut buffers: Vec<Buffer> = Vec::new();
    let mut window_manager= WindowManager::new();

    buffers.push(Buffer::new(String::from("hallo, welt")));
    window_manager.push_window(Window::new(0, Rect::default()));
    window_manager.push_window(Window::new(0, Rect::default()));

    window_manager.active_window_mut().set_active(&mut terminal);

    window_manager.layout_windows(terminal.get_frame().size());

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            if event::poll(Duration::from_millis(50)).unwrap() {
                if let event::Event::Key(key_event) = event::read().unwrap() {
                    tx.send(key_event).unwrap();
                }
            }
        }
    });

    loop {
        if let Ok(key_event) = rx.try_recv() {
            match key_event.code {
                KeyCode::Esc => {
                    terminal.clear()?;
                    break;
                }
                KeyCode::Char('h') => {
                    window_manager.push_window(Window::new(0, Rect::default()));
                    window_manager.active_window_idx = window_manager.windows.len() - 1;
                }
                KeyCode::Char('c') => {
                    terminal.set_cursor(5, 5);
                    // window_manager.active_window_mut().cursor = (3, 8);
                }
                KeyCode::Char(any) => {
                    buffers.get_mut(0).unwrap().text.push(any);
                }
                _ => {}
            }
        }

        window_manager.layout_windows(terminal.get_frame().size());
        terminal.draw(|frame| {
            window_manager.windows.iter()
                .enumerate()
                .for_each(|(idx, window)| {
                    let block = Block::default()
                        .borders(Borders::all())
                        .title(format!("Window {idx}"));

                    let buffer = buffers.get(window.buf_idx).unwrap();

                    let window_area = window.area;
                    let buffer_area = block.inner(window_area);

                    frame.render_widget(block, window_area);
                    buffer.draw(frame, buffer_area);
                });
        })?;
    }

    disable_raw_mode()?;
    crossterm::execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}
