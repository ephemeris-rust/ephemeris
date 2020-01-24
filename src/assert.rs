use std::panic;

use proptest::prelude::TestCaseError;

// TODO: Remove when proptest gets built-in `should_panic` behavior
pub fn expect_panic<F: FnOnce() -> R + std::panic::UnwindSafe, R>(
    expected_message: &str,
    f: F,
) -> Result<(), TestCaseError> {
    let initial_panic = panic::catch_unwind(f)
        .err()
        .map(|a| a.downcast_ref::<String>().map(|u| u.to_owned()));

    match initial_panic {
        Some(Some(actual_message)) if actual_message == expected_message => Ok(()),
        Some(Some(actual_message)) => Err(TestCaseError::fail(actual_message)),
        Some(None) => Err(TestCaseError::fail("no message or panic not string")),
        None => Err(TestCaseError::fail("Did not panic")),
    }
}
