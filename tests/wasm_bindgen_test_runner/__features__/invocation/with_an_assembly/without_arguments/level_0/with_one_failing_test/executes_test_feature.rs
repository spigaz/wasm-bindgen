use crate::__steps__::assembly::given_there_is_an_assembly_with_one_failing_test;
use crate::__steps__::error_code::then_failure_should_have_been_returned;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_for_runtime;
use crate::__steps__::Context;
use crate::__steps__::Runtime;
use auroka_morpheus_macros_feature::feature;

feature! {
    runtime: Runtime

    given_there_is_an_assembly_with_one_failing_test();
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_for_runtime(runtime);

    "Outputs its running 1 test" {
        then_the_standard_output_should_have("running 1 test");
    }

    "Outputs the failed test summary" {
        then_the_standard_output_should_have("test assembly_with_one_failing_test::fail ... FAIL");
    }

    "Outputs the failed test assertion error" {
        then_the_standard_output_should_have(r#"assertion `left == right` failed
  left: 1
 right: 2"#);
    }

    "Outpus the assembly failure summary" {
        then_the_standard_output_should_have("failures:\n\n    assembly_with_one_failing_test::fail");
    }

    "Outputs the assembly test summary" {
        then_the_standard_output_should_have("test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 filtered out");
    }

    "Returns failure" {
        then_failure_should_have_been_returned();
    }
}
