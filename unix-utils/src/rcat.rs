use std::{fs::File, io};

pub fn cat(filenames: &[String]) -> io::Result<()>{
    let mut handle = io::stdout().lock();
    for filename in filenames {
        let mut file = File::open(filename)?;
        io::copy(&mut file, &mut handle)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::rcat;

    fn get_test_files() -> Vec<String> {
        let test_data_path = PathBuf::from("cat_test");
        let mut filenames = Vec::new();
        if let Ok(entries) = test_data_path.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(path_str) = path.to_str() {
                            filenames.push(path_str.to_string());
                        }
                    }
                }
            }
        }
        filenames
    }

    #[test]
    fn test_cat_success() {
        let test_files = get_test_files();
        assert!(!test_files.is_empty(), "Test files directory is empty");

        let result = rcat::cat(&test_files);
        assert!(result.is_ok(), "cat function failed: {:?}", result.err());
    }
}
