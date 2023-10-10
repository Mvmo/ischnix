use ischnix::{Plugin, Editor, EditorEvent, MoveDirection};

pub struct MyP;

trait Event {}

pub struct MoveCursorEvent {
    direction: MoveDirection,
}

impl Plugin for MyP {
    fn init(&self) {
        println!("hallo, welt");
    }

    fn handle_event(&self, editor: &mut Editor, editor_event: EditorEvent) {
        dbg!(editor_event);
    }
}

#[no_mangle]
pub extern "C" fn initialize() -> Box<dyn Plugin> {
    return Box::new(MyP);
}

