use rand::seq::SliceRandom;
use regex::Regex;
use serde::Serialize;
use std::error::Error as stdError;
use tera::{Context, Tera};

use std::collections::BTreeMap;
use std::io::{prelude::*, BufReader};
// use std::path::Path;
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
        adjs: Vec<&str>,
        themes_path: &PathBuf,
        // themes: Option<Vec<&str>>,
        themes: Vec<&str>,
    ) -> Result<Self, Error> {
        // if let Some(ref adjs) = adjs {
        //     let adjs_path = adjs_path.to_str().expect("shit, that's my luck...");
        //     let r: Vec<String> = adjs
        //         .into_iter()
        //         .filter(|a| !a.starts_with('!'))
        //         .map(|a| {
        //             if !Path::new(&format!("{}/{}", adjs_path, &a)).exists() {
        //                 Some(a.to_owned())
        //             } else {
        //                 None
        //             }
        //         })
        //         .flatten()
        //         .collect();

        //     if r.len() > 0 {
        //         bail!("couldn't find file for adjective(s) {} in path {}, aborting", r.join(", "), adjs_path);
        //     }
        // };

        // if let Some(ref themes) = themes {
        //     let themes_path = themes_path.to_str().expect("shit, that's my luck...");
        //     let r: Vec<String> = themes
        //         .into_iter()
        //         .filter(|t| !t.starts_with('!'))
        //         .map(|t| {
        //             if !Path::new(&format!("{}/{}", themes_path, &t)).exists() {
        //                 Some(t.to_owned())
        //             } else {
        //                 None
        //             }
        //         })
        //         .flatten()
        //         .collect();

        //     if r.len() > 0 {
        //         bail!("couldn't find file for theme(s) {} in path {}, aborting", r.join(", "), themes_path);
        //     }
        // };

        let (excluded_adjs, adjs_path) = (
            adjs.iter()
                .filter(|t| t.starts_with('!'))
                .map(|x| x.clone())
                .collect::<Vec<&str>>(),
            adjs_path,
        );
        debug!("excluded adjectives {:?}", excluded_adjs);

        let adjs: Vec<Type> = WalkDir::new(adjs_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|metadata| metadata.file_type().is_file())
            .filter(|file| {
                debug!("adjective file: {:#?}", &file);
                if adjs.is_empty() {
                    return true;
                }
                let adj_file_name = match file.file_name().to_str() {
                    Some(file_name) => file_name,
                    None => {
                        warn!("couldn't get name for adjective file");
                        return false;
                    }
                };

                let adj_name: &str = &format!("!{}", &adj_file_name);

                if !excluded_adjs.is_empty() {
                    !&excluded_adjs.contains(&adj_name)
                } else {
                    adjs.contains(&adj_file_name.as_ref())
                }
            })
            .map(|a| Type::new(&a))
            .filter_map(Result::ok)
            .collect();

        let (excluded_themes, themes_path) = (
            themes
                .iter()
                .filter(|t| t.starts_with('!'))
                .map(|t| t.clone())
                .collect::<Vec<&str>>(),
            themes_path,
        );

        debug!("excluded themes {:?}", excluded_adjs);
        debug!("themes path {:?}", themes_path);

        let themes: Vec<Type> = WalkDir::new(themes_path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|metadata| metadata.file_type().is_file())
            .filter(|file| {
                debug!("theme file: {:#?}", file);
                if themes.is_empty() {
                    return true;
                }
                let theme_file_name = match file.file_name().to_str() {
                    Some(file_name) => file_name,
                    None => {
                        warn!("couldn't get name for theme file");
                        return false;
                    }
                };

                let theme_name: &str = &format!("!{}", theme_file_name);

                if !excluded_themes.is_empty() {
                    !excluded_themes.contains(&&theme_name)
                } else {
                    themes.contains(&theme_file_name)
                }
            })
            .map(|t| Type::new(&t))
            .filter_map(Result::ok)
            .collect();

        Ok(Self {
            adjs,
            themes,
        })
    }

    pub fn randomize(
        &self,
        pattern: Option<&str>,
        number: usize,
        template: Option<&str>,
        with_details: bool,
    ) -> Result<Vec<String>, Error> {
        let re_adjs = Regex::new(r"adjs").expect("this shouldn't fail");
        let re_themes = Regex::new(r"themes").expect("this shouldn't fail");
        let re_adj = Regex::new(r"adj[^s]").expect("this shouldn't fail");
        let re_theme = Regex::new(r"theme[^s]").expect("this shouldn't fail");

        match template {
            Some(template) => {
                let mut context = Context::new();

                let results: Vec<_> = (0..number)
                    .map(|_| {
                        let adjs: BTreeMap<_, Vec<_>> = self
                            .adjs
                            .iter()
                            .map(|a| {
                                let subset = a.random_entries(pattern, number);
                                debug!("adjs subset: {:?}", subset);
                                (a.name.clone(), subset)
                            })
                            .collect();
                        let themes: BTreeMap<_, Vec<_>> = self
                            .themes
                            .iter()
                            .map(|t| {
                                let subset = t.random_entries(pattern, number);
                                debug!("themes subset: {:?}", subset);
                                (t.name.clone(), subset)
                            })
                            .collect();

                        let available_adjs: Vec<_> = adjs.keys().collect();
                        let available_themes: Vec<_> = themes.keys().collect();

                        if re_adj.is_match(template) {
                            let rand_adj:Vec<_> = (0..15).map(|_| {
                                match available_adjs.choose(&mut rand::thread_rng()) {
                                    Some(adj) => {
                                        match adjs.get(&adj.to_string()) {
                                            Some(adj) => match adj.is_empty() {
                                                false => {
                                                    adj.choose(&mut rand::thread_rng())
                                                },
                                                true => {
                                                    None }
                                            },
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
                                }
                            })
                                // .skip_while(|a| a.is_some())
                                .skip_while(|a| a.is_none())
                                // .take_while(|a| a.is_some())
                                .flatten()
                                .collect();


                            debug!("rand_adj len: {}", rand_adj.len());
                            debug!("rand_adj: {:#?}", rand_adj);


                            // let rand_adj = match available_adjs.choose(&mut rand::thread_rng()) {
                            //     Some(adj) => {
                            //         match adjs.get(&adj.to_string()) {
                            //             Some(adj) => adj,
                            //             None => {
                            //                 warn!("unable to get random adjective, skipping");
                            //                 panic!("unable to get random adjective, aborting (this is a bug)")
                            //             }
                            //         }
                            //     },
                            //     None => {
                            //         warn!("unable to get random adjective, skipping");
                            //         panic!("unable to get random adjective, aborting (this is a bug)")
                            //     }
                            // };

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
                                warn!("{}, skipping", e.source().expect("shouldn't fail... I think"));
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
                        let (adj_name, adj) = match adjs.choose(&mut rand::thread_rng()) {
                            Some(ref a) => (&a.name, a.random_entry(pattern)?),
                            None => {
                                warn!("couldn\'t get random adjectives entries");
                                bail!("\'chier")
                            }
                        };
                        let (theme_name, theme) = match themes.choose(&mut rand::thread_rng()) {
                            Some(ref t) => (&t.name, t.random_entry(pattern)?),
                            None => {
                                warn!("couldn't get random themes entries");
                                bail!("'chier")
                            }
                        };
                        if with_details {
                            Ok(format!(
                                "[ {:^12} | {:^12} ]\t\t{} {}",
                                adj_name, theme_name, adj, theme
                            ))
                        } else {
                            Ok(format!("{} {}", adj, theme))
                        }
                    })
                    .filter_map(Result::ok)
                    .collect())
            }
        }
    }
}

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
            .map(|l| l)
            .filter_map(Result::ok)
            .collect::<Vec<String>>();
        let name = match file.file_name().to_str() {
            Some(name) => name.to_owned(),
            None => bail!("fuck, couldn't get file_name"),
        };
        let path = match file.path().to_str() {
            Some(path) => path.to_owned(),
            None => bail!("fuck, couldn't get path"),
        };

        Ok(Self {
            name,
            path,
            entries,
        })
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
        match self.entries(pattern).choose(&mut rand::thread_rng()) {
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
