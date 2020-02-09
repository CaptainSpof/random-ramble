#[macro_use]
extern crate log;

use rand::seq::SliceRandom;
use serde::Serialize;
use tera::{Context, Tera};

use std::collections::{BTreeMap, HashMap};
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
                        if !excluded_adjs.is_empty() {
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
                        if !excluded_themes.is_empty() {
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

    pub fn randomize(
        &self,
        pattern: Option<&str>,
        number: usize,
        template: Option<&str>,
    ) -> Vec<String> {
        match template {
            Some(template) => {
                let mut context = Context::new();

                (0..number)
                    .map(|_| {
                        let aa: BTreeMap<_, Vec<_>> = self
                            .adjs
                            .iter()
                            .map(|a| {
                                let subset = a.random_entries(pattern, number);
                                (a.name.clone(), subset)
                            })
                            .collect();
                        let tt: BTreeMap<_, Vec<_>> = self
                            .themes
                            .iter()
                            .map(|t| {
                                let subset = t.random_entries(pattern, number);
                                (t.name.clone(), subset)
                            })
                            .collect();

                        let available_adjs: Vec<_> = aa.keys().map(|k| k.to_owned()).collect();
                        let available_themes: Vec<_> = tt.keys().map(|k| k.to_owned()).collect();
                        let rd_adj: String = available_adjs
                            .choose(&mut rand::thread_rng())
                            .unwrap()
                            .to_owned();
                        let rd_theme: String = available_themes
                            .choose(&mut rand::thread_rng())
                            .unwrap()
                            .to_owned();

                        let a = aa.get(&rd_adj).unwrap();
                        let t = tt.get(&rd_theme).unwrap();

                        context.insert("adj", &a.choose(&mut rand::thread_rng()));
                        context.insert("adjs", &aa);
                        context.insert("theme", &t.choose(&mut rand::thread_rng()));
                        context.insert("themes", &tt);
                        Tera::one_off(template, &context, true).unwrap()
                    })
                    .collect()
            }
            None => {
                let (adjs, themes): (Vec<_>, Vec<_>) = match pattern {
                    Some(pattern) => (
                        self.adjs
                            .iter()
                            .filter(|a| {
                                a.entries
                                    .iter()
                                    .any(|e| e.to_lowercase().starts_with(&pattern.to_lowercase()))
                            })
                            .collect(),
                        self.themes
                            .iter()
                            .filter(|a| {
                                a.entries
                                    .iter()
                                    .any(|e| e.to_lowercase().starts_with(&pattern.to_lowercase()))
                            })
                            .collect(),
                    ),
                    None => (self.adjs.iter().collect(), self.themes.iter().collect()),
                };

                (0..number)
                    .map(|_| {
                        format!(
                            "{} {}",
                            adjs.choose(&mut rand::thread_rng())
                                .expect("Fuck me")
                                .random_entry(pattern),
                            themes
                                .choose(&mut rand::thread_rng())
                                .expect("Fuck me")
                                .random_entry(pattern),
                        )
                    })
                    .collect()
            }
        }
    }

    pub fn randomize_with_details(
        &self,
        pattern: Option<&str>,
        number: usize,
        template: Option<&str>,
    ) -> Vec<String> {
        let adjs: Vec<(_, _)> = self
            .adjs
            .iter()
            .flat_map(|a| a.entries(pattern).into_iter().map(move |e| (&a.name, e)))
            .collect();

        let themes: Vec<(_, _)> = self
            .themes
            .iter()
            .flat_map(|t| t.entries(pattern).into_iter().map(move |e| (&t.name, e)))
            .collect();

        match template {
            Some(template) => {
                let mut themes_m = HashMap::new();
                for (k, v) in &themes {
                    themes_m.entry(k).or_insert_with(Vec::new).push(v)
                }

                let mut adjs_m = HashMap::new();
                for (k, v) in &adjs {
                    adjs_m.entry(k).or_insert_with(Vec::new).push(v)
                }

                let mut context = Context::new();
                (0..number)
                    .map(|_| {
                        let aa: HashMap<_, Vec<_>> = adjs_m
                            .iter()
                            .map(|(k, v)| {
                                (
                                    k,
                                    v.choose_multiple(&mut rand::thread_rng(), number)
                                        .map(|e| e.to_owned())
                                        .collect(),
                                )
                            })
                            .collect();
                        let tt: HashMap<_, Vec<_>> = themes_m
                            .iter()
                            .map(|(k, v)| {
                                (
                                    k,
                                    v.choose_multiple(&mut rand::thread_rng(), number)
                                        .map(|e| e.to_owned())
                                        .collect(),
                                )
                            })
                            .collect();

                        context.insert("adjs", &aa);
                        context.insert("themes", &tt);
                        Tera::one_off(template, &context, true).unwrap()
                    })
                    .collect()
            }
            None => {
                let adj_random_sel: Vec<_> = adjs
                    .choose_multiple(&mut rand::thread_rng(), number)
                    .map(|a| a.to_owned())
                    .collect();

                let themes_random_sel: Vec<_> = themes
                    .choose_multiple(&mut rand::thread_rng(), number)
                    .map(|e| e.to_owned())
                    .collect();
                adj_random_sel
                    .iter()
                    .zip(themes_random_sel.iter())
                    .map(|((ap, a), (tp, t))| format!("[{} | {:^12}]\t{} {}", ap, tp, a, t))
                    .collect()
            }
        }
    }
}

enum _EntryType {
    Adjective(Type),
    Theme(Type),
}

#[derive(Debug, Serialize)]
struct TypeT(String, Vec<String>);

#[derive(Debug, Serialize)]
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

    pub fn random_entries(&self, pattern: Option<&str>, number: usize) -> Vec<String> {
        self.entries(pattern)
            .choose_multiple(&mut rand::thread_rng(), number)
            .map(|e| e.to_owned())
            .collect()
    }

    pub fn random_entry(&self, pattern: Option<&str>) -> String {
        let r = self.entries(pattern);
        let r = r.choose(&mut rand::thread_rng());
        match r {
            Some(r) => r.to_string(),
            None => format!(
                "<< nothing found with pattern {} for {} >>",
                pattern.unwrap_or("''"),
                self.name
            ),
        }
    }
}
