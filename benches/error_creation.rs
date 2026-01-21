fn main() {
    divan::main();
}

#[divan::bench]
fn create_error_from_string() {
    let _ = divan::black_box(anyhow::anyhow!("test error"));
}

#[divan::bench]
fn create_error_with_context() {
    let result: Result<(), std::io::Error> = Err(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"));
    let _ = divan::black_box(result.context("Failed to read file"));
}

#[divan::bench]
fn create_and_downcast_error() {
    use anyhow::Context;
    let err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let anyhow_err = anyhow::Error::new(err);
    let _ = divan::black_box(anyhow_err.downcast_ref::<std::io::Error>());
}

#[divan::bench]
fn format_error_display() {
    let err = anyhow::anyhow!("test error with formatting: {}", 42);
    let _ = divan::black_box(format!("{}", err));
}

#[divan::bench]
fn error_chain_iteration() {
    use anyhow::Context;
    let base_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let err = anyhow::Error::new(base_err)
        .context("Failed to read config")
        .context("Failed to initialize app");
    
    let count = divan::black_box(err.chain().count());
    let _ = count;
}
