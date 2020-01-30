use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

pub fn add(theme_path: &PathBuf, theme: String, entries: Vec<String>) {
    debug!("trying to add: {:?} to {}", entries, theme);

    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .open(theme_path.join(&theme));

    match file {
        Ok(mut f) => {
            debug!("{} found", theme);
            let buf = BufReader::new(&f);

            let lines: Vec<String> = buf
                .lines()
                .map(|l| l.expect("Could not parse line"))
                .collect();
            let missing: Vec<String> = entries
                .iter()
                .filter(|e| !lines.contains(e))
                .cloned()
                .collect();

            if missing.len() > 0 {
                info!("adding entries: {}", missing.join(" "));
                writeln!(f, "{}", missing.join("\n")).expect("oh shit");
            } else {
                warn!("nothing to add");
            }
        }
        Err(_e) => unimplemented!("{} not found, creating", &theme),
    }
}

pub fn delete() {
    println!("delete");
}
