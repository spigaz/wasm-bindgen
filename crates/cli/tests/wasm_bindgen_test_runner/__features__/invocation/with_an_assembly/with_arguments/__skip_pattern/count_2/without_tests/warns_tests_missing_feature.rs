use crate::__steps__::assembly::given_there_is_an_assembly_without_anything;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments;
use crate::__steps__::Context;
use auroka_morpheus_macros_feature::feature;

feature! {
    given_there_is_an_assembly_without_anything();
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments("--skip pattern1 --skip pattern2");

    "Outputs no tests to run warning" {
        then_the_standard_output_should_have("no tests to run!");
    }

    "Returns true" {
        then_success_should_have_been_returned();
    }
}
