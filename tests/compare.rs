use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

use walkdir::WalkDir;

fn join(
    first_file_list: HashMap<PathBuf, PathBuf>,
    second_file_list: HashMap<PathBuf, PathBuf>,
) -> Vec<(PathBuf, PathBuf)> {
    let all_suffixes: HashSet<_> = first_file_list
        .keys()
        .chain(second_file_list.keys())
        .collect();

    let mut pairs: Vec<(PathBuf, PathBuf)> = Vec::new();

    for suffix in all_suffixes {
        let first_path = first_file_list
            .get(suffix)
            .expect(format!("No file {suffix:?} in first folder").as_str());
        let second_path = second_file_list
            .get(suffix)
            .expect(format!("No file {suffix:?} in second folder").as_str());

        pairs.push((first_path.to_owned(), second_path.to_owned()));
    }

    pairs
}

// Return a HashMap of Paths indexed by path relative to parent folder.
fn browse_folder(path: &str) -> HashMap<PathBuf, PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|f| f.file_type().is_file())
        .map(|f| {
            (
                f.path().strip_prefix(path).unwrap().to_path_buf(),
                f.path().to_path_buf(),
            )
        })
        .collect()
}

fn compare_files_content(first_file_path: PathBuf, second_file_path: PathBuf) {
    let first_file = fs::read(first_file_path.to_owned())
        .expect(format!("impossible to read {:?}", first_file_path).as_str());
    let second_file = fs::read(second_file_path.to_owned())
        .expect(format!("impossible to read {:?}", second_file_path).as_str());

    for content in first_file.into_iter().zip(second_file) {
        if content.0 != content.1 {
            panic!(
                "{:?} and {:?} are different",
                first_file_path, second_file_path
            );
        }
    }
}

pub fn compare_folders_content(first_path: &str, second_path: &str) {
    let first_file_list = browse_folder(&first_path);
    let second_file_list = browse_folder(&second_path);

    let pairs = join(first_file_list, second_file_list);

    for pair in pairs {
        compare_files_content(pair.0, pair.1);
    }
}
