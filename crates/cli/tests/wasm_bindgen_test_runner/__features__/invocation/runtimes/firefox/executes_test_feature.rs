use crate::__steps__::assembly::given_there_is_an_assembly_with;
use crate::__steps__::standard_error::then_the_standard_error_should_be_empty;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_for_test_mode;
use crate::__steps__::Context;
use crate::__steps__::TestMode;
use auroka_morpheus_macros_feature::feature;

feature! {
    given_there_is_an_assembly_with(r#"
#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1, 1);
}
"#);
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_for_test_mode(TestMode::BrowserFirefox);

    "Outputs its running 1 test" {
        then_the_standard_output_should_have("running 1 test");
    }

    "Outputs the successful test summary" {
        then_the_standard_output_should_have("test assembly::pass ... ok");
    }

    "Outputs the assembly test summary" {
        then_the_standard_output_should_have("test result: ok. 1 passed; 0 failed; 0 ignored; 0 filtered out");
    }

    "Outputs no error" {
        then_the_standard_error_should_be_empty();
    }

    "Returns success" {
        then_success_should_have_been_returned();
    }
}
