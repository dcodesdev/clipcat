use copypasta::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(content: &str) -> anyhow::Result<()> {
    let mut ctx = match ClipboardContext::new() {
        Ok(ctx) => ctx,
        Err(e) => return Err(anyhow::anyhow!("ClipboardContext Error: {}", e)),
    };

    if let Err(e) = ctx.set_contents(content.to_owned()) {
        return Err(anyhow::anyhow!("Set Contents Error: {}", e));
    }

    if let Err(e) = ctx.get_contents() {
        return Err(anyhow::anyhow!("Get Contents Error: {}", e));
    }

    Ok(())
}
