use std::fs::OpenOptions;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

pub fn add(theme_path: &PathBuf, theme: String, entries: Vec<String>) {
    debug!("trying to add: {:?} to {}", entries, theme);

    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .create(true)
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
        Err(e) => error!("{:?}", e),
    }
}

pub fn delete(theme_path: &PathBuf, theme: String, entries: Vec<String>) {
    debug!("trying to delete: {:?} to {}", entries, theme);

    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(theme_path.join(&theme));

    match file {
        Ok(f) => {
            debug!("{} found", theme);
            let buf = BufReader::new(&f);

            let mut lines: Vec<String> = buf
                .lines()
                .map(|l| l.expect("Could not parse line"))
                .collect();

            let deleting: Vec<String> = entries
                .iter()
                .filter(|e| lines.contains(e))
                .cloned()
                .collect();

            lines.retain(|l| !entries.contains(&l));

            if lines.len() > 0 {
                debug!("new entries:\n{}", lines.join("\n"));
                info!("deteting entries: {}", deleting.join(" "));
                // delete old file
                drop(f);

                let mut nf = File::create(theme_path.join(&theme)).expect("oh shit");
                writeln!(nf, "{}", lines.join("\n")).expect("oh shit");
            } else {
                warn!("nothing to remove");
            }
        }
        Err(e) => error!("{}", e.to_string()),
    }
}
