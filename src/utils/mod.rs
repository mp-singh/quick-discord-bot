use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

pub mod interactions;
pub mod optl;
pub mod syllables;

pub fn read_dir(dir: String) -> HashMap<String, Vec<String>> {
    let paths = fs::read_dir(dir).unwrap();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for path in paths {
        let mut cur: Vec<String> = Vec::new();
        let curpath = path.unwrap().path();
        let filename = curpath.file_name().unwrap().to_str().unwrap().to_string();
        let filetype = filename[..(filename.len() - 4)].to_string();
        if let Ok(lines) = read_lines(curpath) {
            for line in lines.flatten() {
                if !line.is_empty() {
                    cur.push(line);
                }
            }
            map.insert(filetype, cur);
        }
    }
    map
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

mod test {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use std::io::Write;

    #[test]
    fn test_read_dir() {
        let dir = "test_dir";
        fs::create_dir_all(dir).unwrap();
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        map.insert("test1".to_string(), vec!["test1".to_string()]);
        map.insert("test2".to_string(), vec!["test2".to_string()]);
        map.insert("test3".to_string(), vec!["test3".to_string()]);
        for (key, value) in map.iter() {
            let mut file = File::create(format!("{}/{}.txt", dir, key)).unwrap();
            for line in value {
                writeln!(file, "{}", line).unwrap();
            }
        }
        let result = read_dir(dir.to_string());
        assert_eq!(result, map);
        fs::remove_dir_all(dir).unwrap();
    }
}
