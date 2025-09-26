//! Tests for FILE module

#[cfg(test)]
mod tests {
    use super::super::file::*;
    use std::fs;
    use std::path::Path;

    // Helper function to create test files
    fn create_test_file(path: &str, content: &str) {
        fs::write(path, content).unwrap();
    }

    // Helper function to cleanup test files
    fn cleanup_test_file(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_read_to_string() {
        let test_file = "test_read.txt";
        let content = "Hello, World!";
        create_test_file(test_file, content);
        
        let result = FILE::read_to_string(test_file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_write_string() {
        let test_file = "test_write.txt";
        let content = "Hello, World!";
        
        let result = FILE::write_string(test_file, content);
        assert!(result.is_ok());
        
        // Verify the file was written
        let read_content = fs::read_to_string(test_file).unwrap();
        assert_eq!(read_content, content);
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_append_string() {
        let test_file = "test_append.txt";
        let content1 = "Hello, ";
        let content2 = "World!";
        
        // Write initial content
        FILE::write_string(test_file, content1).unwrap();
        
        // Append additional content
        let result = FILE::append_string(test_file, content2);
        assert!(result.is_ok());
        
        // Verify the file contains both contents
        let read_content = fs::read_to_string(test_file).unwrap();
        assert_eq!(read_content, format!("{}{}", content1, content2));
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_read_lines() {
        let test_file = "test_lines.txt";
        let content = "Line 1\nLine 2\nLine 3";
        create_test_file(test_file, content);
        
        let result = FILE::read_lines(test_file);
        assert!(result.is_ok());
        let lines = result.unwrap();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "Line 1");
        assert_eq!(lines[1], "Line 2");
        assert_eq!(lines[2], "Line 3");
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_write_lines() {
        let test_file = "test_write_lines.txt";
        let lines = vec!["Line 1".to_string(), "Line 2".to_string(), "Line 3".to_string()];
        
        let result = FILE::write_lines(test_file, &lines);
        assert!(result.is_ok());
        
        // Verify the file was written
        let read_content = fs::read_to_string(test_file).unwrap();
        assert_eq!(read_content, "Line 1\nLine 2\nLine 3\n");
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_copy() {
        let src_file = "test_copy_src.txt";
        let dst_file = "test_copy_dst.txt";
        let content = "Hello, World!";
        create_test_file(src_file, content);
        
        let result = FILE::copy(src_file, dst_file);
        assert!(result.is_ok());
        
        // Verify the copy was successful
        let copied_content = fs::read_to_string(dst_file).unwrap();
        assert_eq!(copied_content, content);
        
        cleanup_test_file(src_file);
        cleanup_test_file(dst_file);
    }

    #[test]
    fn test_move_file() {
        let src_file = "test_move_src.txt";
        let dst_file = "test_move_dst.txt";
        let content = "Hello, World!";
        create_test_file(src_file, content);
        
        let result = FILE::move_file(src_file, dst_file);
        assert!(result.is_ok());
        
        // Verify the move was successful
        assert!(!Path::new(src_file).exists());
        let moved_content = fs::read_to_string(dst_file).unwrap();
        assert_eq!(moved_content, content);
        
        cleanup_test_file(dst_file);
    }

    #[test]
    fn test_delete() {
        let test_file = "test_delete.txt";
        create_test_file(test_file, "Hello, World!");
        
        let result = FILE::delete(test_file);
        assert!(result.is_ok());
        
        // Verify the file was deleted
        assert!(!Path::new(test_file).exists());
    }

    #[test]
    fn test_exists() {
        let test_file = "test_exists.txt";
        
        // Test non-existent file
        let result = FILE::exists(test_file);
        assert!(result.is_ok());
        assert!(!result.unwrap());
        
        // Test existing file
        create_test_file(test_file, "Hello, World!");
        let result = FILE::exists(test_file);
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_is_file() {
        let test_file = "test_is_file.txt";
        let test_dir = "test_is_file_dir";
        
        // Test non-existent path
        let result = FILE::is_file(test_file);
        assert!(result.is_ok());
        assert!(!result.unwrap());
        
        // Test file
        create_test_file(test_file, "Hello, World!");
        let result = FILE::is_file(test_file);
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        // Test directory
        fs::create_dir(test_dir).unwrap();
        let result = FILE::is_file(test_dir);
        assert!(result.is_ok());
        assert!(!result.unwrap());
        
        cleanup_test_file(test_file);
        fs::remove_dir(test_dir).unwrap();
    }

    #[test]
    fn test_is_dir() {
        let test_file = "test_is_dir.txt";
        let test_dir = "test_is_dir_dir";
        
        // Test non-existent path
        let result = FILE::is_dir(test_file);
        assert!(result.is_ok());
        assert!(!result.unwrap());
        
        // Test file
        create_test_file(test_file, "Hello, World!");
        let result = FILE::is_dir(test_file);
        assert!(result.is_ok());
        assert!(!result.unwrap());
        
        // Test directory
        fs::create_dir(test_dir).unwrap();
        let result = FILE::is_dir(test_dir);
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        cleanup_test_file(test_file);
        fs::remove_dir(test_dir).unwrap();
    }

    #[test]
    fn test_create_dir() {
        let test_dir = "test_create_dir";
        
        let result = FILE::create_dir(test_dir);
        assert!(result.is_ok());
        
        // Verify the directory was created
        assert!(Path::new(test_dir).is_dir());
        
        fs::remove_dir(test_dir).unwrap();
    }

    #[test]
    fn test_create_dir_all() {
        let test_dir = "test_create_dir_all/nested/deep";
        
        let result = FILE::create_dir_all(test_dir);
        assert!(result.is_ok());
        
        // Verify the directory was created
        assert!(Path::new(test_dir).is_dir());
        
        fs::remove_dir_all("test_create_dir_all").unwrap();
    }

    #[test]
    fn test_remove_dir() {
        let test_dir = "test_remove_dir";
        fs::create_dir(test_dir).unwrap();
        
        let result = FILE::remove_dir(test_dir);
        assert!(result.is_ok());
        
        // Verify the directory was removed
        assert!(!Path::new(test_dir).exists());
    }

    #[test]
    fn test_remove_dir_all() {
        let test_dir = "test_remove_dir_all";
        let nested_dir = "test_remove_dir_all/nested";
        fs::create_dir_all(nested_dir).unwrap();
        
        let result = FILE::remove_dir_all(test_dir);
        assert!(result.is_ok());
        
        // Verify the directory was removed
        assert!(!Path::new(test_dir).exists());
    }

    #[test]
    fn test_list_dir() {
        let test_dir = "test_list_dir";
        fs::create_dir(test_dir).unwrap();
        create_test_file("test_list_dir/file1.txt", "content1");
        create_test_file("test_list_dir/file2.txt", "content2");
        
        let result = FILE::list_dir(test_dir);
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 2);
        
        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_list_dir_with_metadata() {
        let test_dir = "test_list_dir_meta";
        fs::create_dir(test_dir).unwrap();
        create_test_file("test_list_dir_meta/file1.txt", "content1");
        
        let result = FILE::list_dir_with_metadata(test_dir);
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 1);
        
        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_find_files() {
        let test_dir = "test_find_files";
        fs::create_dir(test_dir).unwrap();
        create_test_file("test_find_files/file1.txt", "content1");
        create_test_file("test_find_files/file2.js", "content2");
        create_test_file("test_find_files/file3.txt", "content3");
        
        let result = FILE::find_files(test_dir, "*.txt");
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 2);
        
        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_find_files_recursive() {
        let test_dir = "test_find_files_recursive";
        let nested_dir = "test_find_files_recursive/nested";
        fs::create_dir_all(nested_dir).unwrap();
        create_test_file("test_find_files_recursive/file1.txt", "content1");
        create_test_file("test_find_files_recursive/nested/file2.txt", "content2");
        
        let result = FILE::find_files_recursive(test_dir, "*.txt");
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 2);
        
        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_absolute_path() {
        let result = FILE::absolute_path(".");
        assert!(result.is_ok());
        let abs_path = result.unwrap();
        assert!(abs_path.is_absolute());
    }

    #[test]
    fn test_relative_path() {
        let result = FILE::relative_path("/some/absolute/path", "/some");
        assert!(result.is_ok());
        let rel_path = result.unwrap();
        assert_eq!(rel_path, "absolute/path");
    }

    #[test]
    fn test_join() {
        let result = FILE::join("path", "to", "file");
        assert_eq!(result, "path/to/file");
    }

    #[test]
    fn test_normalize() {
        let result = FILE::normalize("path/../to/./file");
        assert_eq!(result, "to/file");
    }

    #[test]
    fn test_metadata() {
        let test_file = "test_metadata.txt";
        create_test_file(test_file, "Hello, World!");
        
        let result = FILE::metadata(test_file);
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert!(metadata.size > 0);
        
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
        let test_file = "test_modified.txt";
        create_test_file(test_file, "Hello, World!");
        
        let result = FILE::get_modified_time(test_file);
        assert!(result.is_ok());
        let timestamp = result.unwrap();
        assert!(timestamp > 0);
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_get_created_time() {
        let test_file = "test_created.txt";
        create_test_file(test_file, "Hello, World!");
        
        let result = FILE::get_created_time(test_file);
        assert!(result.is_ok());
        let timestamp = result.unwrap();
        assert!(timestamp > 0);
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_rename() {
        let old_name = "test_rename_old.txt";
        let new_name = "test_rename_new.txt";
        create_test_file(old_name, "Hello, World!");
        
        let result = FILE::rename(old_name, new_name);
        assert!(result.is_ok());
        
        // Verify the rename was successful
        assert!(!Path::new(old_name).exists());
        assert!(Path::new(new_name).exists());
        
        cleanup_test_file(new_name);
    }

    #[test]
    fn test_touch() {
        let test_file = "test_touch.txt";
        
        let result = FILE::touch(test_file);
        assert!(result.is_ok());
        
        // Verify the file was created
        assert!(Path::new(test_file).exists());
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_chmod() {
        let test_file = "test_chmod.txt";
        create_test_file(test_file, "Hello, World!");
        
        let result = FILE::chmod(test_file, 0o644);
        assert!(result.is_ok());
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_chown() {
        let test_file = "test_chown.txt";
        create_test_file(test_file, "Hello, World!");
        
        let result = FILE::chown(test_file, 1000, 1000);
        assert!(result.is_ok());
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_symlink() {
        let target = "test_symlink_target.txt";
        let link = "test_symlink_link.txt";
        create_test_file(target, "Hello, World!");
        
        let result = FILE::symlink(target, link);
        assert!(result.is_ok());
        
        // Verify the symlink was created
        assert!(Path::new(link).exists());
        
        cleanup_test_file(target);
        cleanup_test_file(link);
    }

    #[test]
    fn test_read_symlink() {
        let target = "test_read_symlink_target.txt";
        let link = "test_read_symlink_link.txt";
        create_test_file(target, "Hello, World!");
        FILE::symlink(target, link).unwrap();
        
        let result = FILE::read_symlink(link);
        assert!(result.is_ok());
        let target_path = result.unwrap();
        assert_eq!(target_path, target);
        
        cleanup_test_file(target);
        cleanup_test_file(link);
    }

    #[test]
    fn test_hardlink() {
        let target = "test_hardlink_target.txt";
        let link = "test_hardlink_link.txt";
        create_test_file(target, "Hello, World!");
        
        let result = FILE::hardlink(target, link);
        assert!(result.is_ok());
        
        // Verify the hardlink was created
        assert!(Path::new(link).exists());
        
        cleanup_test_file(target);
        cleanup_test_file(link);
    }

    #[test]
    fn test_temp_file() {
        let result = FILE::temp_file();
        assert!(result.is_ok());
        let temp_file = result.unwrap();
        assert!(Path::new(&temp_file).exists());
        
        cleanup_test_file(&temp_file);
    }

    #[test]
    fn test_temp_dir() {
        let result = FILE::temp_dir();
        assert!(result.is_ok());
        let temp_dir = result.unwrap();
        assert!(Path::new(&temp_dir).is_dir());
        
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_walk_dir() {
        let test_dir = "test_walk_dir";
        let nested_dir = "test_walk_dir/nested";
        fs::create_dir_all(nested_dir).unwrap();
        create_test_file("test_walk_dir/file1.txt", "content1");
        create_test_file("test_walk_dir/nested/file2.txt", "content2");
        
        let result = FILE::walk_dir(test_dir);
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 3); // 2 files + 1 directory
        
        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_walk_dir_with_metadata() {
        let test_dir = "test_walk_dir_meta";
        let nested_dir = "test_walk_dir_meta/nested";
        fs::create_dir_all(nested_dir).unwrap();
        create_test_file("test_walk_dir_meta/file1.txt", "content1");
        
        let result = FILE::walk_dir_with_metadata(test_dir);
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 2); // 1 file + 1 directory
        
        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_glob() {
        let test_dir = "test_glob";
        fs::create_dir(test_dir).unwrap();
        create_test_file("test_glob/file1.txt", "content1");
        create_test_file("test_glob/file2.js", "content2");
        create_test_file("test_glob/file3.txt", "content3");
        
        let result = FILE::glob("test_glob/*.txt");
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 2);
        
        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_glob_recursive() {
        let test_dir = "test_glob_recursive";
        let nested_dir = "test_glob_recursive/nested";
        fs::create_dir_all(nested_dir).unwrap();
        create_test_file("test_glob_recursive/file1.txt", "content1");
        create_test_file("test_glob_recursive/nested/file2.txt", "content2");
        
        let result = FILE::glob_recursive("test_glob_recursive/**/*.txt");
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 2);
        
        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_file_metadata_from_std_metadata() {
        let test_file = "test_file_metadata.txt";
        create_test_file(test_file, "Hello, World!");
        
        let std_metadata = fs::metadata(test_file).unwrap();
        let file_metadata = FileMetadata::from_std_metadata(std_metadata);
        
        assert!(file_metadata.size > 0);
        assert!(file_metadata.is_file);
        assert!(!file_metadata.is_dir);
        
        cleanup_test_file(test_file);
    }
}
