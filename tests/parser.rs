use vpp::parse;

#[test]
fn parses_let_binding() {
    let program = parse("let x = 10").unwrap();
    assert_eq!(program.items.len(), 1);
}

#[test]
fn parses_function_declaration() {
    let program = parse("fn add(a: int, b: int) -> int { return a + b }").unwrap();
    assert_eq!(program.items.len(), 1);
}

#[test]
fn parses_if_else() {
    let program = parse("if x > 0 { print(x) } else { print(0) }").unwrap();
    assert_eq!(program.items.len(), 1);
}

#[test]
fn parses_for_in_array() {
    let program = parse("for user in users { print(user) }").unwrap();
    assert_eq!(program.items.len(), 1);
}

#[test]
fn parses_array_literal() {
    let program = parse("let nums = [1, 2, 3]").unwrap();
    assert_eq!(program.items.len(), 1);
}
