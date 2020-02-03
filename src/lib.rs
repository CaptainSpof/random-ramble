#[macro_use]
extern crate log;

use rand::seq::SliceRandom;

use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
pub struct RandomRamble {
    adjs: Vec<Type>,
    themes: Vec<Type>,
}

impl RandomRamble {
    pub fn new(
        adjs_path: &PathBuf,
        adjs: Option<Vec<String>>,
        themes_path: &PathBuf,
        themes: Option<Vec<String>>,
    ) -> Self {
        let adjs: Vec<Type> = WalkDir::new(adjs_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|metadata| metadata.file_type().is_file())
            .filter(|file| {
                debug!("adjective file: {:#?}", file);
                match &adjs {
                    Some(sel_adjs) => {
                        let excluded_adjs: Vec<_> = sel_adjs
                            .iter()
                            .filter(|t| t.starts_with('!'))
                            .cloned()
                            .collect();
                        debug!("excluded adjectives {:?}", excluded_adjs);
                        let adj_name = format!("!{}", file.file_name().to_str().unwrap());
                        if excluded_adjs.len() > 0 {
                            !excluded_adjs.contains(&adj_name)
                        } else {
                            sel_adjs.contains(&file.file_name().to_str().unwrap().to_string())
                        }
                    }
                    None => true,
                }
            })
            .map(|a| Type::new(&a))
            .collect();
        let themes: Vec<Type> = WalkDir::new(themes_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|metadata| metadata.file_type().is_file())
            .filter(|file| {
                debug!("theme file: {:#?}", file);
                match &themes {
                    Some(sel_themes) => {
                        let excluded_themes: Vec<_> = sel_themes
                            .iter()
                            .filter(|t| t.starts_with('!'))
                            .cloned()
                            .collect();
                        debug!("excluded themes {:?}", excluded_themes);
                        let theme_name = format!("!{}", file.file_name().to_str().unwrap());
                        if excluded_themes.len() > 0 {
                            !excluded_themes.contains(&theme_name)
                        } else {
                            sel_themes.contains(&file.file_name().to_str().unwrap().to_string())
                        }
                    }
                    None => true,
                }
            })
            .map(|a| Type::new(&a))
            .collect();

        Self { adjs, themes }
    }

    pub fn randomize(&self, pattern: Option<&str>, number: usize) -> Vec<String> {
        let adjs: Vec<_> = self.adjs.iter().flat_map(|a| a.entries(pattern)).collect();

        let themes: Vec<_> = self
            .themes
            .iter()
            .flat_map(|a| a.entries(pattern))
            .collect();

        let adj_random_sel: Vec<_> = adjs
            .choose_multiple(&mut rand::thread_rng(), number)
            .collect();

        let themes_random_sel: Vec<_> = themes
            .choose_multiple(&mut rand::thread_rng(), number)
            .collect();

        adj_random_sel
            .iter()
            .zip(themes_random_sel.iter())
            .map(|(a, t)| format!("{} {}", a, t))
            .collect()
    }

    pub fn randomize_with_details(&self, pattern: Option<&str>, number: usize) -> Vec<String> {
        let adjs: Vec<(_, _)> = self
            .adjs
            .iter()
            .flat_map(|a| a.entries(pattern).into_iter().map(move |e| (e, &a.name)))
            .collect();

        let themes: Vec<_> = self
            .themes
            .iter()
            .flat_map(|t| t.entries(pattern).into_iter().map(move |e| (e, &t.name)))
            .collect();

        let adj_random_sel: Vec<_> = adjs
            .choose_multiple(&mut rand::thread_rng(), number)
            .collect();

        let themes_random_sel: Vec<_> = themes
            .choose_multiple(&mut rand::thread_rng(), number)
            .collect();

        adj_random_sel
            .iter()
            .zip(themes_random_sel.iter())
            .map(|((a, ap), (t, tp))| format!("[{} | {:^12}]\t{} {}", ap, tp, a, t))
            .collect()
    }
}

enum _EntryType {
    Adjective(Type),
    Theme(Type),
}

#[derive(Debug)]
struct Type {
    // The provenance of the entry
    // provenance: String,
    // Name
    name: String,
    path: String,

    entries: Vec<String>,
}

impl Type {
    pub fn new(file: &DirEntry) -> Self {
        let f = std::fs::File::open(file.path()).expect("Unable to open file");
        let buf = BufReader::new(f);
        let entries = buf
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect::<Vec<String>>();

        Self {
            // provenance: "foo".to_owned(),
            name: file.file_name().to_str().expect("FIX ME").to_owned(),
            path: file.path().to_str().expect("FIX ME").to_owned(),
            entries,
        }
    }

    fn _populate_entries(&mut self) {
        let file = std::fs::File::open(&self.path).expect("Unable to open file");
        let buf = BufReader::new(file);
        self.entries = buf
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect::<Vec<String>>()
    }

    pub fn entries(&self, pattern: Option<&str>) -> Vec<String> {
        self.entries
            .iter()
            .filter(|e| match pattern {
                Some(ref p) => e.to_lowercase().starts_with(&p.to_lowercase()),
                None => true,
            })
            .cloned()
            .collect::<Vec<String>>()
    }
}
