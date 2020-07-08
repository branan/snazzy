use snazzy::{codegen, parser};

#[test]
fn empty() {
    let ast = parser::program("");
    assert!(ast.is_ok());
    assert!(codegen::assemble(&ast.unwrap()).is_ok());
}

#[test]
fn simple() {
    let expected = vec![169, 48, 141, 0, 16, 96];
    let ast = parser::program(include_str!("input/simple.snz"));
    assert!(ast.is_ok());
    let bytes = codegen::assemble(ast.as_ref().unwrap());
    assert!(bytes.is_ok());
    assert_eq!(expected, &bytes.as_ref().unwrap()[0..expected.len()])
}

#[test]
fn snes() {
    let expected_code = vec![
        64, 64, 64, 64, 64, 64, 64, 120, 24, 251, 194, 32, 169, 255, 1, 27, 169, 0, 0, 91, 226, 48,
        169, 143, 141, 0, 33, 156, 1, 33, 156, 2, 33, 156, 3, 33, 156, 5, 33, 156, 6, 33, 156, 7,
        33, 156, 8, 33, 156, 9, 33, 156, 10, 33, 156, 11, 33, 156, 12, 33, 156, 13, 33, 156, 13,
        33, 169, 255, 141, 14, 33, 141, 16, 33, 141, 18, 33, 141, 20, 33, 169, 7, 141, 14, 33, 141,
        16, 33, 141, 18, 33, 141, 20, 33, 156, 15, 33, 156, 15, 33, 156, 17, 33, 156, 17, 33, 156,
        19, 33, 156, 19, 33, 169, 128, 141, 21, 33, 156, 22, 33, 156, 23, 33, 156, 26, 33, 156, 27,
        33, 169, 1, 141, 27, 33, 156, 28, 33, 156, 28, 33, 156, 29, 33, 156, 29, 33, 156, 30, 33,
        141, 30, 33, 156, 31, 33, 156, 31, 33, 156, 32, 33, 156, 32, 33, 156, 33, 33, 156, 35, 33,
        156, 36, 33, 156, 37, 33, 156, 38, 33, 156, 39, 33, 156, 40, 33, 156, 41, 33, 156, 42, 33,
        156, 43, 33, 141, 44, 33, 156, 45, 33, 156, 46, 33, 156, 47, 33, 169, 48, 141, 48, 33, 156,
        49, 33, 169, 224, 141, 50, 33, 156, 51, 33, 169, 255, 156, 0, 66, 141, 1, 66, 156, 2, 66,
        156, 3, 66, 156, 4, 66, 156, 5, 66, 156, 6, 66, 156, 7, 66, 156, 8, 66, 156, 9, 66, 156,
        10, 66, 156, 11, 66, 156, 12, 66, 156, 13, 66, 88, 32, 17, 129, 56, 251, 64, 169, 28, 156,
        34, 33, 141, 34, 33, 169, 15, 141, 0, 33, 128,
    ];
    let expected_header = vec![
        90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 32, 0,
        5, 0, 1, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 1, 128, 0, 0, 2, 128, 0, 0, 3, 128, 0, 0,
        0, 0, 4, 128, 0, 0, 0, 0, 5, 128, 7, 128, 6, 128,
    ];
    let ast = parser::program(include_str!("input/snes.snz"));
    assert!(ast.is_ok());
    use std::io::Write;
    let bytes = codegen::assemble(ast.as_ref().unwrap());
    assert!(bytes.is_ok());
    std::fs::File::create("/Users/branan/test.bin")
        .unwrap()
        .write_all(bytes.as_ref().unwrap())
        .unwrap();
    let bytes = bytes.unwrap();
    assert_eq!(expected_code, &bytes[0..expected_code.len()]);
    assert_eq!(expected_header, &bytes[0x7FC0..0x8000]);
}
