use std::process;
use std::fmt::Write as FmtWrite; // to make writing to String work.
use std::path::Path;
use std::env;
use std::io::{self, Read};
use std::fs;
use std::collections::HashMap;
use sha1::{Sha1, Digest};

fn main() {
    // script to find similar photos by using md5
    // this script accepts one parameter which is the directory to find files in.
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() <= 1 {
        println!("Missing directory argument");
        process::exit(1);
    }
    
    let directory = &args[1];
    let directory_path = Path::new(&directory);
    
    // ensure this directory exists
    if !directory_path.exists() {
        println!("Directory {} does not exist", directory);
        process::exit(1);
    }
    
    // ok, directory exists. time to hash all files in it.
    let mut hash_to_filenames: HashMap<String, String> = HashMap::new();
    visit_dirs(&directory_path, &mut|entry| {
        let path = entry.path();
        let path_as_string = String::from(path.to_str().unwrap());
        let md5_hash = md5_hash_file(&path).unwrap();
        
        if hash_to_filenames.contains_key(&md5_hash) {
            println!("Duplicate file: {}", path_as_string);
        } else {
            hash_to_filenames.insert(md5_hash, path_as_string);
        }
    }).expect("Unable to visit directory");
}

// Hash a single path and return as utf8 string.
fn md5_hash_file(path: &Path) -> Option<String> {
    let mut file = fs::File::open(&path).unwrap();
    
    let mut hasher = Sha1::new();
    let mut buffer = [0u8; 1024];
    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        hasher.input(&buffer[..bytes_read]);
        if bytes_read == 0 {
            break;
        }
    }
    
    let hash_result = hasher.result();
    
    // struggeld with what the heck "hasher.result()" was.
    // turns out its an slice/array of hex bytes.
    // cant use str::from_utf8 to convert it.
    
    let mut s = String::new();
    write!(&mut s, "{:x}", hash_result).expect("Unable to write");
    
    Some(s)
}

// Source at the documentation for DirEntry.
fn visit_dirs(dir: &Path, cb: &mut FnMut(&fs::DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}