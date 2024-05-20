use anyhow::Ok;
use copypasta::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(content: &str) -> anyhow::Result<()> {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(content.to_owned()).unwrap();
    ctx.get_contents().unwrap();

    Ok(())
}
