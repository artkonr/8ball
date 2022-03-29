use std::collections::HashMap;
use std::{fs, io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use rand::Rng;


fn main() -> io::Result<()> {

    let files: HashMap<usize, PathBuf> = fs::read_dir("src/txt")?
        .enumerate()
        .map(|tpl| (
            tpl.0,
            tpl.1.expect("Failed to traverse file").path()
        ))
        .collect();

    let mut rng = rand::thread_rng();
    let rnd = rng.gen_range(0..files.len()) as usize;

    let path = files.get(&rnd)
        .expect(format!("No file at number {}", rnd).as_str());
    let file = File::open(path.as_os_str())?;
    let reader = BufReader::new(file);

    let lns: Vec<String> = reader.lines()
        .into_iter()
        .map(|ln| ln.expect("Failed to read line"))
        .collect();
    let toln = rng.gen_range(0..lns.len());

    let ln = lns.get(toln)
        .expect(format!("Illegal line {}", toln).as_str());
    println!("{}", ln);

    Ok(())
}

