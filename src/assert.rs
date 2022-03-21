use std::panic;

use proptest::prelude::TestCaseError;

fn panic_message(err: Box<dyn std::any::Any + Send>) -> Option<String> {
    err.downcast_ref::<String>()
        .map(|s| s.to_string())
        .or_else(|| err.downcast_ref::<&str>().map(|s| s.to_string()))
}

// TODO: Remove when proptest gets built-in `should_panic` behavior
pub fn expect_panic<F: FnOnce() -> R + std::panic::UnwindSafe, R>(
    expected_message: &str,
    f: F,
) -> Result<(), TestCaseError> {
    let initial_panic = panic::catch_unwind(f).err().map(panic_message);

    match initial_panic {
        Some(Some(actual_message)) if actual_message == expected_message => Ok(()),
        Some(Some(actual_message)) => Err(TestCaseError::fail(actual_message)),
        Some(None) => Err(TestCaseError::fail("no message or panic not string")),
        None => Err(TestCaseError::fail("Did not panic")),
    }
}
