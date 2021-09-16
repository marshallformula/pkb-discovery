pub mod files {
    use glob;
    use std::path::PathBuf;

    /// Recursively search the given glob path for matching files.
    pub fn discover(glob_path: &str) -> Result<Vec<PathBuf>, String> {
        let paths = glob::glob(glob_path).map_err(|e| format!("Error Discovering Glob: {}", e))?;

        return paths.map(|p| p.map_err(|e| format!("{}", e))).collect();
    }
}

#[cfg(test)]
mod files_tests {

    const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

    use super::*;

    #[test]
    fn empty_vec_when_no_files_found() {
        let target = format!("{}/test/resources/**/*.pdf", MANIFEST_DIR);

        let found = files::discover(target.as_str());

        match found {
            Err(e) => panic!("{}", e),
            Ok(paths) => {
                assert_eq!(paths.len(), 0);
            }
        }
    }

    #[test]
    fn finds_markdown_files() {
        let target = format!("{}/test/resources/**/*.md", MANIFEST_DIR);

        let found = files::discover(target.as_str()).unwrap();

        assert_eq!(found.len(), 2);
        assert!(found[0].exists());
        assert!(found[1].exists());

        let paths = found
            .iter()
            .map(|f| f.to_str())
            .collect::<Option<Vec<&str>>>()
            .unwrap();

        assert!(paths.contains(&format!("{}/test/resources/test1.md", MANIFEST_DIR).as_str()));
        assert!(paths.contains(&format!("{}/test/resources/test2.md", MANIFEST_DIR).as_str()));
    }
}

pub mod hashy {
    use blake3;
    use std::fs;
    use std::path::PathBuf;

    /// Calculates the blake3 hash of the file at the given PathBuf
    pub fn file_hash(file: &PathBuf) -> Result<String, String> {
        if !file.exists() {
            return Err(format!("File does not exist: {}", &file.to_string_lossy()));
        }

        fs::read_to_string(file.as_path())
            .map(|d| blake3::hash(d.as_bytes()).to_hex().to_string())
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod hashy_tests {

    use super::*;
    use std::path::PathBuf;

    const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

    #[test]
    fn calculates_the_hash() {
        let path = PathBuf::from(format!("{}/test/resources/test1.md", MANIFEST_DIR));

        assert_eq!(
            hashy::file_hash(&path).unwrap(),
            "4920940fa45f2e237a453573df58575c1905d2c5c31249a264a17d9dc16db8e3"
        );
    }

    #[test]
    fn fails_if_file_not_exists() {
        let path = PathBuf::from("/over/there/not/exists");

        assert_eq!(
            hashy::file_hash(&path).expect_err("Should fail"),
            "File does not exist: /over/there/not/exists"
        );
    }
}
