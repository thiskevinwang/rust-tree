use std::collections::HashMap;
use std::path::PathBuf;
use walkdir::WalkDir;

static SPACE: &str = "    ";
static BRANCH: &str = "│   ";
static STEM: &str = "├── ";
static CORNER: &str = "└── ";

pub fn dfs(root: PathBuf, cache: &mut HashMap<String, String>) {
    for entry in WalkDir::new(root.clone())
        .into_iter()
        .filter_entry(|e| !e.file_name().to_str().unwrap_or("").starts_with("."))
        .filter_map(Result::ok)
        .filter(|e| e.path() != root)
    {
        let absolute_path = entry.path();
        let relative_path = absolute_path.strip_prefix(root.as_path()).unwrap();

        let display_string = relative_path.display().to_string();
        let parts = display_string.split("/").collect::<Vec<&str>>();

        let mut abs = root.clone();
        let mut res = String::from("");

        // draw the parts
        let parts_len = parts.iter().count();
        for (i, part) in parts.iter().enumerate() {
            abs = abs.join(part);
            let is_base = i == parts_len - 1;

            let dir = abs.clone().parent().unwrap().read_dir().unwrap();
            let iter = dir.enumerate();
            let len = abs.clone().parent().unwrap().read_dir().unwrap().count();

            if is_base {
                for (i, f) in iter {
                    if *part == f.unwrap().file_name().to_str().unwrap() {
                        if i == len - 1 {
                            res += &CORNER;
                        } else {
                            res += &STEM;
                        }
                    }
                }
            } else {
                if cache.get(abs.to_str().unwrap()).is_some() {
                    res = cache.get(abs.to_str().unwrap()).unwrap().clone();
                } else {
                    for (i, f) in iter {
                        if *part == f.unwrap().file_name().to_str().unwrap() {
                            if i == len - 1 {
                                res += &SPACE;
                            } else {
                                res += &BRANCH;
                            }
                        }
                    }
                    cache.insert(abs.to_str().unwrap().to_string(), res.clone());
                }
            }
        }

        println!("{}{}", res, entry.file_name().to_str().unwrap());
    }
}
