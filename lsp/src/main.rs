//! `rescind-lsp`: the language server, over stdio.
//!
//! An editor starts this and speaks LSP to it; there are no arguments and
//! no configuration. What it answers with is in `lib.rs`.

fn main() {
    if let Err(e) = rescind_lsp::server::run() {
        eprintln!("rescind-lsp: {e}");
        std::process::exit(1);
    }
}
