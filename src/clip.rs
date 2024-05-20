use copypasta::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(content: &str) -> anyhow::Result<()> {
    let mut ctx = match ClipboardContext::new() {
        Ok(ctx) => ctx,
        Err(e) => return Err(anyhow::anyhow!("ClipboardContext Error: {}", e)),
    };

    match ctx.set_contents(content.to_owned()) {
        Ok(ctx) => ctx,
        Err(e) => return Err(anyhow::anyhow!("Set Contents Error: {}", e)),
    };

    match ctx.get_contents() {
        Ok(ctx) => ctx,
        Err(e) => return Err(anyhow::anyhow!("Get Contents Error: {}", e)),
    };

    Ok(())
}
