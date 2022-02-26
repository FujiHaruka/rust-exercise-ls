use std::env;
use std::fs;
use std::io;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let dir = if args.len() == 1 { "." } else { &args[1] };

    let out = ls(dir);
    println!("{}", out.unwrap());
}

fn ls(dir: &str) -> Result<String, io::Error> {
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

    Ok(files.join("\t"))
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         assert_eq!(echo(vec!["", "foo", "bar"]), "foo bar\n");
//     }
// }
