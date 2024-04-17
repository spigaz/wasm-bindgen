use crate::__steps__::assembly::given_there_is_an_assembly_with_two_successful_tests;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments;
use crate::__steps__::Context;

#[test]
fn outputs_its_running_2_test_feature() {
    let mut context = Context::new();
    given_there_is_an_assembly_with_two_successful_tests(&mut context);
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments(
        &mut context,
        "--skip=fail --skip=pass_1 --skip=pass_2",
    );
    then_the_standard_output_should_have(context, "running 2 tests");
}
