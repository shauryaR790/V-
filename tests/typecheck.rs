use vpp::check;
use vpp::VppError;

#[test]
fn infers_local_types() {
    let source = "let x = 10\nlet name = \"Shaurya\"\nlet active = true\n";
    check(source).unwrap();
}

#[test]
fn accepts_fib_program() {
    let source = include_str!("../examples/fib.vpp");
    check(source).unwrap();
}

#[test]
fn accepts_arrays_program() {
    let source = include_str!("../examples/arrays.vpp");
    check(source).unwrap();
}

#[test]
fn rejects_type_mismatch_with_error_code() {
    let source = "let age = 20\nlet name = \"Alex\"\nlet result = age + name\n";
    let err = check(source).unwrap_err();
    assert!(matches!(err, VppError::TypeMismatch { .. }));
}

#[test]
fn rejects_undefined_variable() {
    let err = check("print(x)\n").unwrap_err();
    assert!(matches!(err, VppError::UndefinedVariable { .. }));
}

#[test]
fn rejects_empty_array() {
    let err = check("let xs = []\n").unwrap_err();
    assert!(matches!(err, VppError::EmptyArrayNoType { .. }));
}
