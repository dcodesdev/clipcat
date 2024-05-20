use tiktoken_rs::o200k_base;

// TODO: remove this dead code
#[allow(dead_code)]
pub fn count_tokens(string: &str) -> anyhow::Result<usize> {
    let bpe = o200k_base()?;
    let tokens = bpe.encode_with_special_tokens(string);

    Ok(tokens.len())
}
