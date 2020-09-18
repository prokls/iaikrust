use std::collections::BTreeSet;
use std::error::Error as E;

type Error = Box<dyn E>;

fn tokenize(input: &str) -> Result<Vec<String>, Error> {
    Ok(vec![])
}

fn parse(tokens: &Vec<String>) -> Result<BTreeSet<&str>, Error> {
    Ok(BTreeSet::new())
}

fn compile(src: &str) -> Result<(), Error> {
    let tokens = tokenize(&src)?;
    let ast = parse(&tokens)?;
    // …
    Ok(())
}

fn main() {
    compile("block { exec }");
}
