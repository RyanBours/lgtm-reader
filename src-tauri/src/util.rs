use std::fs;

pub fn list_all_files(path: &str) -> Vec<String> {
    let mut files: Vec<String> = vec![];

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap().to_string();

        if path.is_dir() {
            files.append(&mut list_all_files(&path_str));
        } else {
            files.push(path_str);
        }
    }

    files
}
