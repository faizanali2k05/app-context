//! ui verification for app-context.
#[test]
fn invalid_thread_safety_and_macro_inputs_are_rejected() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/core/*.rs");
    #[cfg(feature = "derive")]
    {
        tests.compile_fail("tests/ui/derive/*.rs");
        tests.pass("tests/ui/pass/*.rs");
    }
}
