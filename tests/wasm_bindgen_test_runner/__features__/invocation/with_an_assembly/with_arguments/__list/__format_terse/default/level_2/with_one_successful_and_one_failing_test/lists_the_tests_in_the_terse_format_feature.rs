use crate::__steps__::assembly::given_there_is_an_assembly_with;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments;
use crate::__steps__::Context;
use auroka_morpheus_macros_feature::feature;

feature! {
    given_there_is_an_assembly_with(r#"
mod level_1 {
    mod level_2 {
        use wasm_bindgen_test::*;

        #[cfg(test)]
        #[wasm_bindgen_test]
        fn pass() {
            assert_eq!(1, 1);
        }

        #[cfg(test)]
        #[wasm_bindgen_test]
        fn fail() {
            assert_eq!(1, 2);
        }
    }
}
"#);
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments("--list --format terse");

    "Ouputs the test in the terse format" {
        then_the_standard_output_should_have(r#"level_1::level_2::pass: test
level_1::level_2::fail: test"#);
    }

    "Returns success" {
        then_success_should_have_been_returned();
    }
}
