use anyhow::Context;
use std::io;

fn main() {
    divan::main();
}

// Benchmark error creation from different sources
#[divan::bench]
fn create_error_from_message() {
    let _ = divan::black_box(anyhow::anyhow!("error message"));
}

#[divan::bench]
fn create_error_from_io_error() {
    let io_err = io::Error::new(io::ErrorKind::Other, "io error");
    let _ = divan::black_box(anyhow::Error::new(io_err));
}

// Benchmark context operations
#[divan::bench]
fn add_context() {
    let io_err = io::Error::new(io::ErrorKind::Other, "io error");
    let result: Result<(), io::Error> = Err(io_err);
    let _ = divan::black_box(result.context("additional context"));
}

#[divan::bench]
fn add_context_with_closure() {
    let io_err = io::Error::new(io::ErrorKind::Other, "io error");
    let result: Result<(), io::Error> = Err(io_err);
    let _ = divan::black_box(result.with_context(|| format!("context with {}", "formatting")));
}

// Benchmark downcasting operations
#[divan::bench]
fn downcast_ref() {
    let io_err = io::Error::new(io::ErrorKind::Other, "io error");
    let err = anyhow::Error::new(io_err);
    let _ = divan::black_box(err.downcast_ref::<io::Error>());
}

#[divan::bench]
fn downcast() {
    let io_err = io::Error::new(io::ErrorKind::Other, "io error");
    let err = anyhow::Error::new(io_err);
    let _ = divan::black_box(err.downcast::<io::Error>());
}

// Benchmark error chain iteration
#[divan::bench]
fn iterate_error_chain() {
    let io_err = io::Error::new(io::ErrorKind::Other, "io error");
    let err = anyhow::Error::new(io_err).context("first context").context("second context");
    let count = divan::black_box(err.chain().count());
    divan::black_box(count);
}

// Benchmark error display formatting
#[divan::bench]
fn format_error() {
    let io_err = io::Error::new(io::ErrorKind::Other, "io error");
    let err = anyhow::Error::new(io_err).context("context message");
    let _ = divan::black_box(format!("{}", err));
}

#[divan::bench]
fn format_error_debug() {
    let io_err = io::Error::new(io::ErrorKind::Other, "io error");
    let err = anyhow::Error::new(io_err).context("context message");
    let _ = divan::black_box(format!("{:?}", err));
}

// Benchmark error propagation pattern
#[divan::bench]
fn error_propagation() -> anyhow::Result<()> {
    fn inner() -> anyhow::Result<()> {
        let io_err = io::Error::new(io::ErrorKind::Other, "io error");
        Err(anyhow::Error::new(io_err))
    }
    
    divan::black_box(inner())?;
    Ok(())
}
