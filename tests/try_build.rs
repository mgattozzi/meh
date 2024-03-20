#[test]
fn compile_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile/retryable.rs");
    t.pass("tests/compile/retryable_pass.rs");
}
