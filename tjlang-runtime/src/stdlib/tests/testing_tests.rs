//! Tests for TESTING module

#[cfg(test)]
mod tests {
    use crate::stdlib::testing::*;

    #[test]
    fn test_new_suite() {
        let suite = TESTING::new_suite("Test Suite");
        // Just test that it was created successfully
        assert!(true);
    }

    #[test]
    fn test_run_test() {
        let result = TESTING::run_test("test_name", || {
            TESTING::assert_true(true, "This should pass")?;
            Ok(())
        });
        match result {
            TestResult::Passed { name, .. } => assert_eq!(name, "test_name"),
            _ => panic!("Expected passed result"),
        }
    }

    #[test]
    fn test_assert_true() {
        let result = TESTING::assert_true(true, "This should pass");
        assert!(result.is_ok());
    }

    #[test]
    fn test_assert_false() {
        let result = TESTING::assert_false(false, "This should pass");
        assert!(result.is_ok());
    }

    #[test]
    fn test_assert_equal() {
        let result = TESTING::assert_equal(&5, &5, "These should be equal");
        assert!(result.is_ok());
    }

    #[test]
    fn test_assert_not_equal() {
        let result = TESTING::assert_not_equal(&5, &6, "These should not be equal");
        assert!(result.is_ok());
    }

    #[test]
    fn test_assert_in_range() {
        let result = TESTING::assert_in_range(&5, &1, &10, "5 should be in range 1-10");
        assert!(result.is_ok());
    }

    #[test]
    fn test_assert_contains() {
        let result =
            TESTING::assert_contains("Hello World", "World", "String should contain World");
        assert!(result.is_ok());
    }

    #[test]
    fn test_assert_panics() {
        let result = TESTING::assert_panics(
            || {
                panic!("This should panic");
            },
            "Function should panic",
        );
        assert!(result.is_ok());
    }

    // Test TestSuite methods
    #[test]
    fn test_test_suite_add_test() {
        let mut suite = TESTING::new_suite("Test Suite");
        suite.add_test("test1", || {
            TESTING::assert_true(true, "Test 1")?;
            Ok(())
        });
        // Just test that it doesn't panic
        assert!(true);
    }

    #[test]
    fn test_test_suite_run() {
        let mut suite = TESTING::new_suite("Test Suite");
        suite.add_test("test1", || {
            TESTING::assert_true(true, "Test 1")?;
            Ok(())
        });
        let results = suite.run();
        assert_eq!(results.passed, 1);
        assert_eq!(results.failed, 0);
    }
}
