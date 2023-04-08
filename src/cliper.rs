use clipboard::{ClipboardContext, ClipboardProvider};

pub fn clip(text: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(text.to_owned()).unwrap();
}