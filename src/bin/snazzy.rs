use snazzy::{codegen, parser};
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let in_path = std::path::Path::new(&args[1]);
    let out_path = in_path.with_extension("bin");

    let mut code = String::new();
    let mut input = std::fs::File::open(in_path).expect("Could not open input file");
    input
        .read_to_string(&mut code)
        .expect("Could not read code from file");
    let ast = parser::program(&code).expect("Parse Error loading program");
    let image = codegen::assemble(&ast).expect("Codegen error with program");

    let mut output = std::fs::File::create(&out_path).expect("Could not open output file");
    output
        .write_all(&image)
        .expect("Could not write image to file");
}
