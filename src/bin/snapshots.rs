#![deny(rust_2018_idioms)]

use {
    insta::assert_snapshot,
    lazy_static::lazy_static,
    regex::Regex,
    snmp_grammar::parse,
    std::{fmt::Debug, fs, process::exit},
    walkdir::WalkDir,
};

type ProcMacroPat = gll::grammer::proc_macro::Pat<&'static [gll::grammer::proc_macro::FlatTokenPat<&'static str>]>;

fn parse_result_to_str<T: Debug>(
    result: Result<T, gll::grammer::parser::ParseError<proc_macro2::Span, ProcMacroPat>>,
) -> String {
    // FIXME(eddyb) print the location properly in case of error.
    format!("{:#?}", result)
}

macro_rules! snapshot {
    ($production:ident, $src:expr) => {{
        let tts = $src
            .parse::<proc_macro2::TokenStream>()
            .expect("tokenization");
        parse_result_to_str(parse::$production::parse(tts))
    }};
}

macro_rules! dispatch {
    ($src:ident, $s:expr; $($prod:ident)*) => {
        match $s {
            $(stringify!($prod) => snapshot!($prod, $src),)*
            prod => panic!("Unexpected production {} tested", prod),
        }
    };
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"\(\s+_,?\s+\)").unwrap();
}

fn test_snapshot(file: walkdir::DirEntry) {
    let path = file.path();
    let file_name = file.file_name().to_str().unwrap();
    let src = fs::read_to_string(path).unwrap();
    let production = &file_name[..file_name.find('.').unwrap_or(file_name.len())];
    let forest = dispatch! { src, production;
        // expr.lyg
        Expr ExprKind UnaryOp BinaryOp
        // stmt.lyg
        Stmt
    };
    let forest = forest.replace("Span..Span", "_").replace("_ => ", "");
    let forest = RE.replace_all(&forest, "");
    assert_snapshot!(file_name, forest);
}

fn spawn_panicking(name: String, stack_size: usize, f: impl FnOnce() + Send + 'static) -> Result<(), ()> {
    crossbeam::scope(|scope: &crossbeam::thread::Scope<'_>| {
        scope
            .builder()
            .name(name)
            .stack_size(stack_size)
            .spawn(|_| f())
            .unwrap()
            .join()
            .map_err(drop)
    })
    .unwrap()
}

fn main() {
    // Find all the testdata `.input` files.
    let files = WalkDir::new("testdata")
        .contents_first(true)
        .into_iter()
        .map(Result::unwrap)
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "input"));

    // Parse and snapshot each file
    let snapshots = files
        // .par_bridge() // parallel will interleave output, unfortunately
        .map(|f| {
            spawn_panicking(
                f.file_name().to_string_lossy().into_owned(),
                32 * 1024 * 1024, // 32 MiB
                || test_snapshot(f),
            )
        });

    // Collect failures
    if snapshots.filter_map(Result::err).next().is_none() {
        println!("All snapshots passed!");
    } else {
        exit(1);
    }
}
