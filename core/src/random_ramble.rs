use rand::seq::SliceRandom;
use regex::Regex;
use serde::Serialize;
use std::error::Error as stdError;
use tera::{Context, Tera};

use std::collections::{BTreeMap, HashMap};
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

use crate::{bail, error::Error};

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
    ) -> Result<Self, Error> {
        let adjs: Vec<Type> = WalkDir::new(adjs_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|metadata| metadata.file_type().is_file())
            .filter(|file| {
                debug!("adjective file: {:#?}", file);
                match &adjs {
                    Some(sel_adjs) => {
                        // exclude adjectives that starts with '!'
                        let excluded_adjs: Vec<_> = sel_adjs
                            .into_iter()
                            .filter(|t| t.starts_with('!'))
                            .collect();
                        debug!("excluded adjectives {:?}", excluded_adjs);

                        let adj_file_name = match file.file_name().to_str() {
                            Some(file_name) => file_name,
                            None => {
                                warn!("couldn't get name for adjective file");
                                return false;
                            }
                        };

                        let adj_name = &format!("!{}", adj_file_name);

                        if !excluded_adjs.is_empty() {
                            !excluded_adjs.contains(&adj_name)
                        } else {
                            sel_adjs.contains(&adj_file_name.to_string())
                        }
                    }
                    None => true,
                }
            })
            .map(|a| Type::new(&a))
            .filter_map(Result::ok)
            .collect();

        let themes: Vec<Type> = WalkDir::new(themes_path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|metadata| metadata.file_type().is_file())
            .filter(|file| {
                debug!("theme file: {:#?}", file);
                match &themes {
                    Some(sel_themes) => {
                        // exclude themes that starts with '!'
                        let excluded_themes: Vec<_> = sel_themes
                            .into_iter()
                            .filter(|t| t.starts_with('!'))
                            .collect();
                        debug!("excluded themes {:?}", excluded_themes);

                        let theme_file_name = match file.file_name().to_str() {
                            Some(file_name) => file_name,
                            None => {
                                warn!("couldn't get name for theme file");
                                return false;
                            }
                        };

                        let theme_name = &format!("!{}", theme_file_name);

                        if !excluded_themes.is_empty() {
                            !excluded_themes.contains(&theme_name)
                        } else {
                            sel_themes.contains(&theme_file_name.to_string())
                        }
                    }
                    None => true,
                }
            })
            .map(|a| Type::new(&a))
            .filter_map(Result::ok)
            .collect();

        Ok(Self { adjs, themes })
    }

    pub fn randomize(
        &self,
        pattern: Option<&str>,
        number: usize,
        template: Option<&str>,
    ) -> Result<Vec<String>, Error> {
        let re_adjs = Regex::new(r"adjs").expect("this shouldn't fail");
        let re_themes = Regex::new(r"themes").expect("this shouldn't fail");
        let re_adj = Regex::new(r"adj[^s]").expect("this shouldn't fail");
        let re_theme = Regex::new(r"theme[^s]").expect("this shouldn't fail");

        match template {
            Some(template) => {
                let mut context = Context::new();

                let results: Vec<String> = (0..number)
                    .map(|_| {
                        let adjs: BTreeMap<_, Vec<_>> = self
                            .adjs
                            .iter()
                            .map(|a| {
                                let subset = a.random_entries(pattern, number);
                                (a.name.clone(), subset)
                            })
                            .collect();
                        let themes: BTreeMap<_, Vec<_>> = self
                            .themes
                            .iter()
                            .map(|t| {
                                let subset = t.random_entries(pattern, number);
                                (t.name.clone(), subset)
                            })
                            .collect();

                        let available_adjs: Vec<_> = adjs.keys().collect();
                        let available_themes: Vec<_> = themes.keys().collect();

                        if re_adj.is_match(template) {
                            let rand_adj = match available_adjs.choose(&mut rand::thread_rng()) {
                                Some(adj) => {
                                    match adjs.get(&adj.to_string()) {
                                        Some(adj) => adj,
                                        None => {
                                            warn!("unable to get random adjective, skipping");
                                            panic!("unable to get random adjective, aborting (this is a bug)")
                                        }
                                    }
                                },
                                None => {
                                    warn!("unable to get random adjective, skipping");
                                    panic!("unable to get random adjective, aborting (this is a bug)")
                                }
                            };

                            context.insert("adj", &rand_adj.choose(&mut rand::thread_rng()));
                        }

                        if re_theme.is_match(template) {
                            let rand_theme = match available_themes.choose(&mut rand::thread_rng()) {
                                Some(theme) => {
                                    match themes.get(&theme.to_string()) {
                                        Some(theme) => theme,
                                        None => {
                                            warn!("unable to get random themeective, skipping");
                                            panic!("unable to get random themeective, aborting (this is a bug)")
                                        }
                                    }
                                },
                                None => {
                                    warn!("unable to get random theme, skipping");
                                    panic!("unable to get random theme, aborting (this is a bug)")
                                }
                            };

                            context.insert("theme", &rand_theme.choose(&mut rand::thread_rng()));
                        }

                        if re_adjs.is_match(template) {
                            context.insert("adjs", &adjs);
                        }
                        if re_themes.is_match(template) {
                            context.insert("themes", &themes);
                        }
                        match Tera::one_off(template, &context, true) {
                            Ok(r) => Ok(r),
                            Err(e) => {
                                warn!("{:#?}, skipping", e.source().unwrap().to_string());
                                Err(e)
                            }
                        }
                    })
                    .filter_map(Result::ok)
                    .collect();

                Ok(results)
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

                Ok((0..number)
                    .map(|_| {
                        let adj = match adjs.choose(&mut rand::thread_rng()) {
                            Some(a) => a.random_entry(pattern)?,
                            None => {
                                warn!(r#"couldn't get random adjectives entries"#);
                                bail!(r#"'chier"#)
                            }
                        };
                        let theme = match themes.choose(&mut rand::thread_rng()) {
                            Some(t) => t.random_entry(pattern)?,
                            None => {
                                warn!("couldn't get random themes entries");
                                bail!("'chier")
                            }
                        };
                        Ok(format!("{} {}", adj, theme))
                    })
                    .filter_map(Result::ok)
                    .collect())
            }
        }
    }

    pub fn randomize_with_details(
        &self,
        pattern: Option<&str>,
        number: usize,
        template: Option<&str>,
    ) -> Result<Vec<String>, Error> {
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
                Ok((0..number)
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
                    .collect())
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

                Ok(adj_random_sel
                    .iter()
                    .zip(themes_random_sel.iter())
                    .map(|((ap, a), (tp, t))| format!("[{} | {:^12}]\t{} {}", ap, tp, a, t))
                    .collect())
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct _TypeT(String, Vec<String>);

#[derive(Debug, Serialize)]
struct Type {
    name: String,
    path: String,

    entries: Vec<String>,
}

impl Type {
    pub fn new(file: &DirEntry) -> Result<Self, Error> {
        let f = std::fs::File::open(file.path())?;
        let buf = BufReader::new(f);
        let entries = buf
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect::<Vec<String>>();

        Ok(Self {
            name: file.file_name().to_str().expect("FIX ME").to_owned(),
            path: file.path().to_str().expect("FIX ME").to_owned(),
            entries,
        })
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

    pub fn random_entry(&self, pattern: Option<&str>) -> Result<String, Error> {
        let r = self.entries(pattern);
        let r = r.choose(&mut rand::thread_rng());
        match r {
            Some(r) => Ok(r.to_string()),
            None => match pattern {
                Some(p) => bail!(
                    "could not find any entry in {} with pattern '{}'",
                    self.name,
                    p
                ),
                None => bail!("could not find any entry in {}", self.name),
            },
        }
    }
}
