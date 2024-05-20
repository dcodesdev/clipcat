use tiktoken_rs::o200k_base;

pub fn count_tokens(string: &str) -> anyhow::Result<usize> {
    let bpe = o200k_base()?;
    let tokens = bpe.encode_with_special_tokens(string);

    Ok(tokens.len())
}
