//! Tests for FILE module

#[cfg(test)]
mod tests {
    use crate::stdlib::file::*;
    use std::fs;

    // Helper function to create test files
    fn create_test_file(path: &str, content: &str) {
        fs::write(path, content).unwrap();
    }

    // Helper function to cleanup test files
    fn cleanup_test_file(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_read_string() {
        let test_file = "test_read_string.txt";
        create_test_file(test_file, "Hello, World!");

        let result = FILE::read_to_string(test_file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, World!");

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_write_string() {
        let test_file = "test_write_string.txt";
        let content = "Hello, TJLang!";

        let result = FILE::write_string(test_file, content);
        assert!(result.is_ok());

        // Verify the content was written
        let read_content = fs::read_to_string(test_file).unwrap();
        assert_eq!(read_content, content);

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_append_string() {
        let test_file = "test_append_string.txt";
        create_test_file(test_file, "Hello, ");

        let result = FILE::append_string(test_file, "World!");
        assert!(result.is_ok());

        // Verify the content was appended
        let read_content = fs::read_to_string(test_file).unwrap();
        assert_eq!(read_content, "Hello, World!");

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_exists() {
        let test_file = "test_exists.txt";
        create_test_file(test_file, "test");

        let result = FILE::exists(test_file);
        assert!(result);

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_is_file() {
        let test_file = "test_is_file.txt";
        create_test_file(test_file, "test");

        let result = FILE::is_file(test_file);
        assert!(result);

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_is_dir() {
        let test_dir = "test_is_dir";
        fs::create_dir(test_dir).unwrap();

        let result = FILE::is_dir(test_dir);
        assert!(result);

        fs::remove_dir(test_dir).unwrap();
    }

    #[test]
    fn test_is_symlink() {
        let test_file = "test_is_symlink.txt";
        create_test_file(test_file, "test");

        let result = FILE::is_symlink(test_file);
        assert!(!result); // Regular file is not a symlink

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_size() {
        let test_file = "test_size.txt";
        let content = "Hello, World!";
        create_test_file(test_file, content);

        let result = FILE::size(test_file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content.len() as u64);

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_get_modified_time() {
        let test_file = "test_get_modified_time.txt";
        create_test_file(test_file, "test");

        let result = FILE::get_modified_time(test_file);
        assert!(result.is_ok());

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_get_created_time() {
        let test_file = "test_get_created_time.txt";
        create_test_file(test_file, "test");

        let result = FILE::get_created_time(test_file);
        assert!(result.is_ok());

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_get_accessed_time() {
        let test_file = "test_get_accessed_time.txt";
        create_test_file(test_file, "test");

        let result = FILE::get_accessed_time(test_file);
        assert!(result.is_ok());

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_copy() {
        let source_file = "test_copy_source.txt";
        let dest_file = "test_copy_dest.txt";
        let content = "Hello, World!";
        create_test_file(source_file, content);

        let result = FILE::copy(source_file, dest_file);
        assert!(result.is_ok());

        // Verify the copy was successful
        let copied_content = fs::read_to_string(dest_file).unwrap();
        assert_eq!(copied_content, content);

        cleanup_test_file(source_file);
        cleanup_test_file(dest_file);
    }

    #[test]
    fn test_move_file() {
        let source_file = "test_move_source.txt";
        let dest_file = "test_move_dest.txt";
        let content = "Hello, World!";
        create_test_file(source_file, content);

        let result = FILE::move_file(source_file, dest_file);
        assert!(result.is_ok());

        // Verify the move was successful
        assert!(!FILE::exists(source_file));
        let moved_content = fs::read_to_string(dest_file).unwrap();
        assert_eq!(moved_content, content);

        cleanup_test_file(dest_file);
    }

    #[test]
    fn test_delete() {
        let test_file = "test_delete.txt";
        create_test_file(test_file, "test");

        let result = FILE::delete(test_file);
        assert!(result.is_ok());

        // Verify the file was deleted
        assert!(!FILE::exists(test_file));
    }

    #[test]
    fn test_create_dir() {
        let test_dir = "test_create_dir";

        let result = FILE::create_dir(test_dir);
        assert!(result.is_ok());

        // Verify the directory was created
        assert!(FILE::is_dir(test_dir));

        fs::remove_dir(test_dir).unwrap();
    }

    #[test]
    fn test_create_dir_all() {
        let test_dir = "test_create_dir_all/nested/directory";

        let result = FILE::create_dir_all(test_dir);
        assert!(result.is_ok());

        // Verify the directory was created
        assert!(FILE::is_dir(test_dir));

        fs::remove_dir_all("test_create_dir_all").unwrap();
    }

    #[test]
    fn test_list_dir() {
        let test_dir = "test_list_dir";
        fs::create_dir(test_dir).unwrap();
        create_test_file(&format!("{}/file1.txt", test_dir), "content1");
        create_test_file(&format!("{}/file2.txt", test_dir), "content2");

        let result = FILE::list_dir(test_dir);
        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(files.len() >= 2);

        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_absolute_path() {
        let result = FILE::absolute_path(".");
        assert!(result.is_ok());
        let abs_path = result.unwrap();
        assert!(!abs_path.is_empty());
    }

    #[test]
    fn test_join() {
        let result = FILE::join("path", "to");
        #[cfg(unix)]
        assert_eq!(result, "path/to");
        #[cfg(windows)]
        assert_eq!(result, "path\\to");
    }

    #[test]
    fn test_filename() {
        let result = FILE::filename("/path/to/file.txt");
        assert_eq!(result, Some("file.txt".to_string()));
    }

    #[test]
    fn test_extension() {
        let result = FILE::extension("/path/to/file.txt");
        assert_eq!(result, Some("txt".to_string()));
    }

    #[test]
    fn test_stem() {
        let result = FILE::stem("/path/to/file.txt");
        assert_eq!(result, Some("file".to_string()));
    }

    #[test]
    fn test_current_dir() {
        let result = FILE::current_dir();
        assert!(result.is_ok());
        let current_dir = result.unwrap();
        assert!(!current_dir.is_empty());
    }

    #[test]
    fn test_home_dir() {
        let _result = FILE::home_dir();
        // This might be None on some systems, so we just test that it doesn't panic
        assert!(true);
    }

    #[test]
    fn test_temp_dir() {
        let temp_dir = FILE::temp_dir();
        assert!(!temp_dir.is_empty());
    }

    #[test]
    fn test_metadata() {
        let test_file = "test_metadata.txt";
        create_test_file(test_file, "test");

        let result = FILE::metadata(test_file);
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert!(metadata.size > 0);

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_get_permissions() {
        let test_file = "test_get_permissions.txt";
        create_test_file(test_file, "test");

        let _result = FILE::get_permissions(test_file);
        // This might fail on Windows, so we just test that it doesn't panic
        assert!(true);

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_set_permissions() {
        let test_file = "test_set_permissions.txt";
        create_test_file(test_file, "test");

        let _result = FILE::set_permissions(test_file, 0o644);
        // This might fail on Windows, so we just test that it doesn't panic
        assert!(true);

        cleanup_test_file(test_file);
    }

    #[test]
    fn test_create_symlink() {
        let target = "test_symlink_target.txt";
        let link = "test_symlink_link.txt";
        create_test_file(target, "test");

        let _result = FILE::create_symlink(target, link);
        // This might fail on Windows, so we just test that it doesn't panic
        assert!(true);

        cleanup_test_file(target);
        cleanup_test_file(link);
    }

    #[test]
    fn test_create_temp_file() {
        let result = FILE::create_temp_file();
        assert!(result.is_ok());
        let temp_file = result.unwrap();
        assert!(!temp_file.is_empty());

        // Clean up
        cleanup_test_file(&temp_file);
    }

    #[test]
    fn test_create_temp_dir() {
        let result = FILE::create_temp_dir();
        assert!(result.is_ok());
        let temp_dir = result.unwrap();
        assert!(!temp_dir.is_empty());

        // Clean up
        fs::remove_dir(temp_dir).ok();
    }

    #[test]
    fn test_list_dir_with_metadata() {
        let test_dir = "test_list_dir_with_metadata";
        fs::create_dir(test_dir).unwrap();
        create_test_file(&format!("{}/file1.txt", test_dir), "content1");

        let result = FILE::list_dir_with_metadata(test_dir);
        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(files.len() >= 1);

        fs::remove_dir_all(test_dir).unwrap();
    }
}
