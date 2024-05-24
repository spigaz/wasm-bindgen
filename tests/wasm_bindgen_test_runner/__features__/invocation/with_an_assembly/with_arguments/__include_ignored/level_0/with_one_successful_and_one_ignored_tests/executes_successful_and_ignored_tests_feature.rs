use crate::__steps__::assembly::given_there_is_an_assembly_with_one_successful_and_one_ignored_tests;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments;
use crate::__steps__::Context;
use auroka_morpheus_macros_feature::feature;

feature! {
    given_there_is_an_assembly_with_one_successful_and_one_ignored_tests();
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments("--include-ignored");

    "Outputs its running 2 tests" {
        then_the_standard_output_should_have("running 2 tests");
    }

    "Outputs the successful test summary" {
        then_the_standard_output_should_have("test assembly_with_one_successful_and_one_ignored_tests::pass ... ok");
    }

    "Outputs the ignored test summary" {
        then_the_standard_output_should_have("test assembly_with_one_successful_and_one_ignored_tests::ignored ... ok");
    }

    "Outputs the assembly test summary" {
        then_the_standard_output_should_have("test result: ok. 2 passed; 0 failed; 0 ignored; 0 filtered out");
    }

    "Returns success" {
        then_success_should_have_been_returned();
    }
}