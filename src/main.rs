use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let dir_or_file = if args.len() == 1 { "." } else { &args[1] };

    if is_file(dir_or_file) {
        let path = Path::new(dir_or_file);
        if let Some(file_name) = path.file_name() {
            println!("{}", file_name.to_str().unwrap());
            return;
        }
    }

    let dir = dir_or_file;
    let ls_result = ls(dir);
    match ls_result {
        Ok(files) => {
            let ls_formatted = format(&files);
            println!("{}", ls_formatted);
        }
        Err(e) => {
            println!("rust-ls: {}: {}", dir, e)
        }
    }
}

fn ls(dir: &str) -> Result<Vec<String>, io::Error> {
    let dir_entry: fs::ReadDir = fs::read_dir(dir)?;
    let mut files: Vec<String> = dir_entry
        .filter_map(|res| {
            res.ok()
                .and_then(|entry| entry.file_name().into_string().ok())
                .and_then(|file_name| {
                    if file_name.starts_with(".") {
                        None
                    } else {
                        Some(file_name)
                    }
                })
        })
        .collect();

    files.sort();

    Ok(files)
}

fn is_file(path: &str) -> bool {
    if let Some(file) = fs::metadata(path).ok() {
        file.is_file()
    } else {
        false
    }
}

fn format(ls_result: &Vec<String>) -> String {
    ls_result.join("\t")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ls_gets_file_names() {
        assert_eq!(ls("./testdata/test01").unwrap(), ["bar", "foo"]);
    }
}
