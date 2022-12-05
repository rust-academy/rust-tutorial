// motivation: imperative search function.
fn _search_word_in_text_imperative<'a>(pat: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&pat.to_lowercase()) {
            results.push(line);
        }
    }
    results
}

// functional equivalent of the above code block
// FP for working on collections has some upsides:
// 1) concise
// 2) Easier (less complex)
// 3) Less error prune
fn _search_word_in_text_functional<'a>(pat: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|line| line.to_lowercase().contains(&pat.to_lowercase())).collect()
}
