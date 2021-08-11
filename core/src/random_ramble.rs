use rand::seq::SliceRandom;
use regex::Regex;
use serde::Serialize;
use std::error::Error as stdError;
use tera::{Context, Tera};

use std::collections::BTreeMap;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

use crate::{bail, error::Error};

pub mod refactor {
    use std::{collections::HashMap, fmt::{self, Display}, path::PathBuf};

    use tera::{Context, Error, Tera};

    #[derive(Debug, PartialEq)]
    pub struct RandomRamble<'a> {
        pub _rambles: HashMap<RambleKind<'a>, RambleValue<'a>>,
        pub rambles: Vec<Ramble<'a>>,
        pub template: Option<&'a str>,
    }

    impl<'a> RandomRamble<'a> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_adj(mut self, adj: Ramble<'a>) -> Self {
            // REVIEW: Maybe we want to ensure variant before calling the function?
            let adj = match adj.kind {
                RambleKind::Adjective => adj,
                _ => adj.with_kind(RambleKind::Adjective),
            };

            self.rambles.push(adj);
            self
        }

        pub fn with_adjs(mut self, adjs: Vec<Ramble<'a>>) -> Self {
            // REVIEW: Maybe we want to ensure variant before calling the function?
            let adjs: Vec<Ramble> = adjs
                .into_iter()
                .map(|adj| match adj.kind {
                    RambleKind::Adjective => adj,
                    _ => adj.with_kind(RambleKind::Adjective),
                })
                .collect();

            self.rambles.extend(adjs);
            self
        }

        pub fn with_theme(mut self, theme: Ramble<'a>) -> Self {
            // REVIEW: Maybe we want to ensure variant before calling the function?
            let theme = match theme.kind {
                RambleKind::Theme => theme,
                _ => theme.with_kind(RambleKind::Theme),
            };

            self.rambles.push(theme);
            self
        }

        pub fn with_themes(mut self, themes: Vec<Ramble<'a>>) -> Self {
            // REVIEW: Maybe we want to ensure variant before calling the function?
            let themes: Vec<Ramble> = themes
                .into_iter()
                .map(|theme| match theme.kind {
                    RambleKind::Theme => theme,
                    _ => theme.with_kind(RambleKind::Theme),
                })
                .collect();

            self.rambles.extend(themes);
            self
        }

        pub fn with_template(mut self, template: &'a str) -> Self {
            self.template = Some(template);
            self
        }

        pub fn replace(&self) -> Result<String, Error> {
            let context = self.set_context();

            match self.template {
                Some(template) => Tera::one_off(template, &context, true),
                None => {
                    warn!("No template, using default");
                    Tera::one_off("{{ adj }} {{ theme }}", &context, true)
                }
            }
        }

        fn set_context(&self) -> Context {
            let mut context = Context::new();
            self.rambles.iter().for_each(|r| {
                context.insert(r.kind.to_string(), r.value);
            });
            context
        }
    }

    impl Default for RandomRamble<'_> {
        fn default() -> Self {
            Self {
                rambles: vec![],
                _rambles: HashMap::new(),
                template: None,
            }
        }
    }

    impl Display for RandomRamble<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // TODO: handle error
            let s = self.replace().unwrap_or("???".into());
            write!(f, "{}", s)
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct RambleValue<'a>(&'a str);

    #[derive(Debug, PartialEq)]
    pub struct Ramble<'a> {
        pub kind: RambleKind<'a>,
        pub value: &'a str,
        pub file: Option<PathBuf>,
    }

    impl<'a> Ramble<'a> {
        pub fn new(value: &'a str) -> Self {
            Self {
                value,
                kind: RambleKind::Other("other"),
                file: None,
            }
        }

        pub fn with_kind(mut self, kind: RambleKind<'a>) -> Self {
            self.kind = kind;
            self
        }
    }

    impl<'a> From<&'a str> for Ramble<'a> {
        fn from(value: &'a str) -> Self {
            Self::new(value)
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum RambleKind<'a> {
        Adjective,
        Theme,
        Other(&'a str),
    }

    impl Display for RambleKind<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let s = match self {
                &RambleKind::Adjective => "adj",
                &RambleKind::Theme => "theme",
                &RambleKind::Other(o) => o,
            };
            write!(f, "{}", s)
        }
    }
}

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
        themes: Vec<&str>,
    ) -> Result<Self, Error> {
        let (excluded_adjs, adjs_path) = (
            adjs.iter()
                .filter(|t| t.starts_with('!'))
                .copied()
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
                    adjs.contains(&adj_file_name)
                }
            })
            .map(|a| Type::new(&a))
            .filter_map(Result::ok)
            .collect();

        let (excluded_themes, themes_path) = (
            themes
                .iter()
                .filter(|t| t.starts_with('!'))
                .copied()
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

        Ok(Self { adjs, themes })
    }

    pub fn randomize(
        &self,
        pattern: Option<&str>,
        number: usize,
        template: Option<&str>,
        with_details: bool,
    ) -> Result<Vec<String>, Error> {
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
                                        match adjs.get(&(*adj).to_string()) {
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
                                    match themes.get(&(*theme).to_string()) {
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

                        if template.contains("adjs") {
                            context.insert("adjs", &adjs);
                        }
                        if template.contains("themes") {
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
    fn new(file: &DirEntry) -> Result<Self, Error> {
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

    fn entries(&self, pattern: Option<&str>) -> Vec<String> {
        self.entries
            .iter()
            .filter(|e| match pattern {
                Some(ref p) => e.to_lowercase().starts_with(&p.to_lowercase()),
                None => true,
            })
            .cloned()
            .collect::<Vec<String>>()
    }

    fn random_entries(&self, pattern: Option<&str>, number: usize) -> Vec<String> {
        self.entries(pattern)
            .choose_multiple(&mut rand::thread_rng(), number)
            .map(|e| e.to_owned())
            .collect()
    }

    fn random_entry(&self, pattern: Option<&str>) -> Result<String, Error> {
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
