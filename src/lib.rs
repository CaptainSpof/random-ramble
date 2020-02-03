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
            .map(|a| Type::new(adjs_path, &a))
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
            .map(|a| Type::new(themes_path, &a))
            .collect();

        Self { adjs, themes }
    }
}

enum EntryType {
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
    pub fn new(_path: &PathBuf, file: &DirEntry) -> Self {
        Self {
            // provenance: "foo".to_owned(),
            name: file.file_name().to_str().expect("FIX ME").to_owned(),
            path: file.path().to_str().expect("FIX ME").to_owned(),
            entries: vec![],
        }
    }
}

fn get_adjs(adjs_path: &PathBuf, adjs: Option<Vec<String>>, pattern: Option<&str>) -> Vec<String> {
    WalkDir::new(adjs_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|metadata| metadata.file_type().is_file())
        .filter(|file| {
            debug!("adjective file: {:#?}", file);
            match &adjs {
                Some(sel_adjs) if sel_adjs.iter().any(|t| t.starts_with('!')) => {
                    let excluded_adjs: Vec<_> = sel_adjs
                        .iter()
                        .filter(|t| t.starts_with('!'))
                        .cloned()
                        .collect();
                    debug!("excluded adjectives {:?}", excluded_adjs);
                    let a = format!("!{}", file.file_name().to_str().unwrap());
                    !excluded_adjs.contains(&a)
                }
                Some(sel_themes) => {
                    debug!("selected themes {:?}", sel_themes);
                    sel_themes.contains(&file.file_name().to_str().unwrap().to_string())
                }
                None => true,
            }
        })
        .flat_map(|f| {
            let file = std::fs::File::open(f.path()).expect("Unable to open file");
            let buf = BufReader::new(file);
            buf.lines()
                .map(|l| l.expect("Could not parse line"))
                .filter(|l| match pattern {
                    Some(ref p) => l.to_lowercase().starts_with(&p.to_lowercase()),
                    None => true,
                })
                .collect::<Vec<String>>()
        })
        .collect()
}

fn get_themes(
    themes_path: &PathBuf,
    themes: Option<Vec<String>>,
    pattern: Option<&str>,
) -> Vec<(String, String)> {
    WalkDir::new(&themes_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|metadata| metadata.file_type().is_file())
        .filter(|file| {
            debug!("theme file: {:#?}", file);
            match &themes {
                Some(sel_themes) if sel_themes.iter().any(|t| t.starts_with('!')) => {
                    let excluded_themes: Vec<_> = sel_themes
                        .iter()
                        .filter(|t| t.starts_with('!'))
                        .cloned()
                        .collect();
                    debug!("excluded themes {:?}", excluded_themes);
                    let t = format!("!{}", file.file_name().to_str().unwrap());
                    !excluded_themes.contains(&t)
                }
                Some(sel_themes) => {
                    debug!("selected themes {:?}", sel_themes);
                    sel_themes.contains(&file.file_name().to_str().unwrap().to_string())
                }
                None => true,
            }
        })
        .flat_map(|f| {
            let file = std::fs::File::open(f.path()).expect("Unable to open file");
            let file_name = f.file_name().to_str().unwrap();
            let buf = BufReader::new(file);
            buf.lines()
                .map(|l| (l.expect("Could not parse line"), file_name.to_string()))
                .filter(|(l, _)| match pattern {
                    Some(ref p) => l.to_lowercase().starts_with(&p.to_lowercase()),
                    None => true,
                })
                .collect::<Vec<(String, String)>>()
        })
        .collect()
}

pub fn get_random_ramble(
    adjs_path: &PathBuf,
    adjs: Option<Vec<String>>,
    themes_path: &PathBuf,
    themes: Option<Vec<String>>,
    pattern: Option<&str>,
    number: usize,
) -> Vec<String> {
    let adjs: Vec<_> = get_adjs(adjs_path, adjs, pattern);
    let themes: Vec<_> = get_themes(themes_path, themes, pattern);

    let adj_random_sel: Vec<_> = adjs
        .choose_multiple(&mut rand::thread_rng(), number)
        .collect();

    let themes_random_sel: Vec<_> = themes
        .choose_multiple(&mut rand::thread_rng(), number)
        .collect();

    adj_random_sel
        .iter()
        .zip(themes_random_sel.iter())
        .map(|(a, (t, _))| format!("{} {}", a, t))
        .collect()
}

pub fn get_random_ramble_with_provenance(
    adjs_path: &PathBuf,
    adjs: Option<Vec<String>>,
    themes_path: &PathBuf,
    themes: Option<Vec<String>>,
    starts_with: Option<&str>,
    number: usize,
) -> Vec<String> {
    let adjs: Vec<_> = get_adjs(adjs_path, adjs, starts_with);
    let themes: Vec<_> = get_themes(themes_path, themes, starts_with);

    let adj_random_sel: Vec<_> = adjs
        .choose_multiple(&mut rand::thread_rng(), number)
        .collect();

    let themes_random_sel: Vec<_> = themes
        .choose_multiple(&mut rand::thread_rng(), number)
        .collect();

    adj_random_sel
        .iter()
        .zip(themes_random_sel.iter())
        .map(|(a, (t, p))| format!("[{:^15}]\t{} {}", p, a, t))
        .collect()
}
