pub mod refactor {

    use crate::error::Result;
    use derivative::Derivative;
    use serde::ser::{SerializeMap, Serializer};
    use serde::{Deserialize, Serialize};
    use walkdir::{DirEntry, WalkDir};

    use std::io::{BufRead, BufReader};
    use std::{
        collections::HashMap,
        fmt::{self, Display},
        path::Path,
    };

    use rand::Rng;
    use rayon::prelude::*;
    use tera::{Context, Tera, Value};

    #[derive(Deserialize, Debug, Default, PartialEq)]
    // pub struct RambleMap<'a>(#[serde(borrow)] pub HashMap<RambleKind<'a>, Vec<Ramble<'a>>>);
    pub struct RambleMap<'a>(#[serde(borrow)] pub HashMap<RambleKind<'a>, Vec<Ramble>>);
    impl<'a> Serialize for &'a RambleMap<'_> {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut seq = serializer.serialize_map(Some(self.0.len()))?;
            for (k, v) in &self.0 {
                seq.serialize_entry(&k.to_string(), &v)?;
            }
            seq.end()
        }
    }

    /// A `tera` filter to get random value from an array
    pub fn random_filter(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
        debug!("value: {:#?}", value);

        let values = value
            .as_array()
            .expect("must provide values alongside random");
        if values.is_empty() {
            debug!("empty values");
            return Ok(Value::default());
        }

        debug!("values: {:#?}", values);

        let category = if let Some(category) = args.get("c") {
            // FIXME handle errors more politely
            values
                .iter()
                .find(|&x| {
                    let a = x
                        .as_object()
                        .expect("requires a valid object")
                        .get("category")
                        .expect("requires a valid category");

                    a == category
                })
                .ok_or("nope, no match")?
                .as_object()
        } else {
            // get random category
            debug!("values len: {}", values.len());
            let rng = rand::thread_rng().gen_range(0..values.len());
            values[rng].as_object()
        };

        debug!("category: {:#?}", category);

        let category = category
            .ok_or("oups, no object")?
            .get("values")
            .ok_or("shit, no values")?
            .as_array()
            .ok_or("fuck, no array")?;

        debug!("category: {:#?}", category);

        if category.is_empty() {
            debug!("empty category");
            return Ok(Value::default());
        }

        let rng = rand::thread_rng().gen_range(0..category.len());
        let val = category[rng].to_owned();

        debug!("val {}", val);

        Ok(val)
    }

    /// The struct that holds the collection of `rambles` and its template.
    // TODO fields shouldn't be pub
    #[derive(Derivative)]
    #[derivative(Debug, PartialEq)]
    pub struct RandomRamble<'a> {
        // FIXME: how to use `T` for key ?
        pub rambles: RambleMap<'a>,
        pub templates: Vec<&'a str>,
        pub context: Option<Context>,
        #[derivative(PartialEq = "ignore")]
        pub tera: Option<Tera>,
    }

    impl<'a> RandomRamble<'a> {
        pub fn new() -> Self {
            Self::default()
        }

        fn load_from_file(file: &DirEntry) -> Result<Ramble> {
            let file_name = file.file_name().to_string_lossy();

            let f = std::fs::File::open(file.path())?;
            let buf = BufReader::new(f);
            let entries = buf
                .lines()
                .filter_map(std::result::Result::ok)
                .collect::<Vec<String>>();

            Ok(Ramble {
                category: Some(file_name.into()),
                values: entries,
            })
        }

        pub fn with_ramble(mut self, kind: &'a str, value: &'a str) -> Self {
            self.rambles.0.insert(kind.into(), vec![value.into()]);
            self
        }

        // pub fn with_rambles(mut self, kind: &'a str, values: Vec<&'a str>) -> Self {
        pub fn with_rambles(mut self, kind: &'a str, values: Vec<&str>) -> Self {
            let others = Ramble {
                category: None,
                // values,
                values: values.into_iter().map(|v| v.into()).collect(),
            };
            self.rambles.0.insert(kind.into(), vec![others]);
            self
        }

        pub fn with_rambles_path(mut self, kind: &'a str, path: &Path) -> Result<Self> {
            // HACK: let std::io handle error for us
            let _ = Path::new(path).metadata()?;

            let rambles = WalkDir::new(path)
                .into_iter()
                .filter_map(std::result::Result::ok)
                .filter(|metadata| metadata.file_type().is_file())
                .map(|t| RandomRamble::load_from_file(&t))
                .filter_map(std::result::Result::ok)
                .collect();

            // debug!("others({}): {:#?}", kind, &others);

            self.rambles.0.insert(kind.into(), rambles);
            Ok(self)
        }

        pub fn with_template(mut self, template: &'a str) -> Self {
            self.templates.push(template);
            self
        }

        pub fn with_templates(mut self, templates: Vec<&'a str>) -> Self {
            self.templates = templates;
            self
        }

        pub fn with_context(mut self, context: Context) -> Self {
            self.context = Some(context);
            self
        }

        /// Generates a String from a template.
        /// use `to_string()` if you don't care about the Error.
        pub fn render(&self) -> Result<String> {
            let template_name = match self.templates.len() {
                0 => "rr".into(),
                _ => format!(
                    "rr{}",
                    rand::thread_rng().gen_range(0..self.templates.len())
                ),
            };

            self.tera
                .as_ref()
                .ok_or_else(|| fail!("invalid tera object"))?
                .render(
                    &template_name,
                    self.context
                        .as_ref()
                        .ok_or_else(|| fail!("invalid context"))?,
                )
                .map_err(|e| e.into())
        }

        pub fn take(&self, n: usize) -> Vec<String> {
            (0..n).into_par_iter().map(|_| self.to_string()).collect()
        }

        fn get_tera(&self) -> Result<Tera> {
            debug!("getting tera");
            let mut tera = Tera::default();
            tera.register_filter("rr", random_filter);

            // for (name, addon) in jen::helper::builtin() {
            //     tera.register_function(name, addon);
            // }

            match self.templates.len() {
                0 => {
                    warn!("No template found, using the default one…");
                    // TODO make filter implicit
                    tera.add_raw_template("rr", "{{ adj | rr }} {{ theme | rr }}")?;
                }
                _ => {
                    tera.add_raw_templates(
                        self.templates
                            .iter()
                            .enumerate()
                            .map(|(i, t)| (format!("rr{}", i), t)),
                    )?;
                }
            }

            Ok(tera)
        }

        fn get_context(&self) -> Result<Context> {
            debug!("getting context");
            debug!("rambles: {:#?}", &self.rambles);
            Context::from_serialize(&self.rambles).map_err(|e| e.into())
        }

        pub fn build(mut self) -> Result<Self> {
            self.tera = Some(self.get_tera()?);
            self.context = Some(self.get_context()?);

            Ok(self)
        }
    }

    impl Default for RandomRamble<'_> {
        fn default() -> Self {
            Self {
                rambles: RambleMap(HashMap::new()),
                templates: Vec::new(),
                context: None,
                tera: None,
            }
        }
    }

    impl Display for RandomRamble<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // TODO: handle error
            // let s = self.replace().unwrap_or_else(|_| "???".into());

            let s = match self.render() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Ima let you finish, but…\n{:#?}", e);
                    "???".into()
                }
            };

            write!(f, "{}", s)
        }
    }

    /// Struct that old the category and the values.
    ///
    /// *Note* I initially tried to have a ref to the values and category.
    /// But I am not sure that it can be done. It might work if the user passes in the values.
    /// But if we try to load the values from a file, the initial references will be droped
    /// when exiting the function responsible to load the file.
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    // pub struct Ramble<'a> {
    pub struct Ramble {
        pub category: Option<String>,
        // pub category: Option<&'a str>,
        // pub values: Vec<&'a str>,
        pub values: Vec<String>,
    }

    // impl<'a> From<&'a str> for Ramble<'a> {
    impl From<&str> for Ramble {
        fn from(source: &str) -> Self {
            Self {
                category: None,
                values: vec![source.into()],
            }
        }
    }

    #[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone)]
    pub struct RambleKind<'a>(pub &'a str);

    impl<'a> Display for RambleKind<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl<'a> From<&'a str> for RambleKind<'a> {
        fn from(source: &'a str) -> Self {
            Self(source)
        }
    }
}

use rand::seq::SliceRandom;
use regex::Regex;
use serde::Serialize;
use std::error::Error as stdError;
use tera::{Context, Tera};

use std::collections::BTreeMap;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use crate::{bail, error::RambleError};

#[derive(Debug)]
pub struct RandomRamble {
    adjs: Vec<Type>,
    themes: Vec<Type>,
}

impl RandomRamble {
    pub fn new(
        adjs_path: &Path,
        adjs: Vec<&str>,
        themes_path: &Path,
        themes: Vec<&str>,
    ) -> Result<Self, RambleError> {
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
                    !excluded_themes.contains(&theme_name)
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
    ) -> Result<Vec<String>, RambleError> {
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
                            Some(a) => (&a.name, a.random_entry(pattern)?),
                            None => {
                                warn!("couldn\'t get random adjectives entries");
                                bail!("\'chier")
                            }
                        };
                        let (theme_name, theme) = match themes.choose(&mut rand::thread_rng()) {
                            Some(t) => (&t.name, t.random_entry(pattern)?),
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
    fn new(file: &DirEntry) -> Result<Self, RambleError> {
        let f = std::fs::File::open(file.path())?;
        let buf = BufReader::new(f);
        let entries = buf.lines().filter_map(Result::ok).collect::<Vec<String>>();
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
                Some(p) => e.to_lowercase().starts_with(&p.to_lowercase()),
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

    fn random_entry(&self, pattern: Option<&str>) -> Result<String, RambleError> {
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
