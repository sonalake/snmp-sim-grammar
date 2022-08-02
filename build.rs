#![deny(rust_2018_idioms)]

use std::{env, fs, path::PathBuf};

use walkdir::WalkDir;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // FIXME(eddyb) streamline this process in `gll`.

    // Find all the `.lyg` grammar fragments in `grammar/`.
    let fragments = WalkDir::new("grammar")
        .contents_first(true)
        .into_iter()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "lyg"));

    // Start with the builtin rules for proc-macro grammars.
    let mut cx = gll::grammer::proc_macro::Context::new();
    let cx = &mut cx;
    let mut grammar = gll::grammer::proc_macro::builtin(cx);

    // Add in each grammar fragment to the grammar.
    for fragment in fragments {
        let path = fragment.into_path();

        // Inform Cargo about our dependency on the fragment files.
        println!("cargo:rerun-if-changed={}", path.display());

        let src = fs::read_to_string(&path).unwrap();
        let fragment = gll::parse_grammar(cx, src.parse().unwrap()).unwrap();
        grammar.extend(fragment);
    }

    // Generate a Rust parser from the combined grammar and write it out.
    fs::write(
        &out_dir.join("parse.rs"),
        gll::generate::rust::generate(cx, &grammar).to_rustfmt_or_pretty_string(),
    )
    .unwrap();
}
