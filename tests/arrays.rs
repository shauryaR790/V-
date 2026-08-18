use std::path::PathBuf;

use vpp::check_path;
use vpp::ir::{lower_program, IrType};

fn example(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(name)
}

#[test]
fn lowers_array_literal_and_index() {
    let path = example("arrays.vpp");
    let typed = check_path(&path).unwrap();
    let ir = lower_program(&typed).unwrap();
    assert!(!ir.top_level.is_empty());
    let ir_debug = format!("{ir:?}");
    assert!(ir_debug.contains("Array"), "expected array IR in arrays.vpp");
}

#[test]
fn array_type_maps_to_heap_ir_type() {
    let ty = IrType::Array(Box::new(IrType::Int));
    assert!(ty.is_array());
    assert!(ty.is_heap());
    assert_eq!(ty.elem_type(), &IrType::Int);
}
