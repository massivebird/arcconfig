use self::error::{BadSystemLabelSomewhere, MissingProperty, SystemDirNotFound};
use crate::system::System;
use std::{io, path::Path};
use yaml_rust::Yaml;

pub mod error;

#[derive(Debug)]
pub struct ConfigFile<'a> {
    archive_root: &'a Path,
    contents: Yaml,
}

impl<'a> ConfigFile<'a> {
    /// Parses the `config.yaml` located at the specified archive.
    ///
    /// # Errors
    ///
    /// Returns some `std::io::Error` if:
    ///
    /// + The archive root does not exist, or
    /// + The archive root is not a directory, or
    /// + Failed to parse `config.yaml`.
    pub fn in_archive(archive_root: &'a Path) -> io::Result<Self> {
        if !archive_root.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("archive root {archive_root:?} does not exist. (expected a directory)"),
            ));
        }

        if !archive_root.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                format!("archive root {archive_root:?} is not a valid directory."),
            ));
        }

        Ok(Self {
            archive_root,
            contents: {
                let yaml_path = archive_root.join("config.yaml");

                let raw_contents = std::fs::read_to_string(yaml_path)?;

                match yaml_rust::YamlLoader::load_from_str(&raw_contents) {
                    Ok(y) => y[0].clone(),
                    Err(scan_err) => {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, scan_err))
                    }
                }
            },
        })
    }

    #[must_use]
    pub const fn contents(&self) -> &Yaml {
        &self.contents
    }

    /// Compiles a collection of systems based on the configuration.
    ///
    /// # Panics
    ///
    /// Will panic if the config contains no `systems` key. Among other things.
    ///
    /// # Errors
    ///
    /// Returns a variety of possible errors. Hopefully no one else uses this crate so I don't have
    /// to document them.
    pub fn systems(&self) -> Result<Vec<System>, Box<dyn std::error::Error>> {
        macro_rules! return_err {
            ( $err: expr ) => {
                return Err(Box::new($err))
            };
        }

        let mut systems: Vec<System> = Vec::new();

        for (sys_label, properties) in self.contents["systems"]
            .as_hash()
            .expect("`systems` contains a single value, expected a collection of labels")
        {
            let Some(label) = sys_label.as_str() else {
                // If the label cannot be parsed, then I'm not sure how to provide
                // precise feedback about it. Sorry >w<
                return_err!(BadSystemLabelSomewhere {})
            };

            let sys_error_msg = |msg: &str| -> String {
                format!("archive config error: system labeled `{label}`: {msg}")
            };

            macro_rules! extract_property {
                ( $property_name: expr, $converter: ident ) => {{
                    let Some(x) = properties[$property_name].$converter() else {
                        return_err!(MissingProperty::new(
                            label.to_owned(),
                            $property_name.to_owned()
                        ))
                    };

                    x
                }};
            }

            let display_name = extract_property!("display_name", as_str);
            let color = extract_property!("color", as_vec);
            let path = extract_property!("path", as_str);
            let games_are_dirs = extract_property!("games_are_directories", as_bool);

            let system_path = self.archive_root.join(path);

            if !system_path.exists() || !system_path.is_dir() {
                return_err!(SystemDirNotFound {
                    sys_label: label.to_owned(),
                    dir: system_path.to_string_lossy().into_owned(),
                });
            }

            let color_sys_error_msg: &str =
                &sys_error_msg("unexpected `color` value. Expected: `[u8, u8, u8]`");

            let nth_color = |n: usize| -> u8 {
                u8::try_from(
                    color
                        .get(n)
                        // Panics if there are less than 3 values.
                        .unwrap_or_else(|| panic!("{color_sys_error_msg}"))
                        .as_i64()
                        // Panics if a value could not be read as an i32.
                        // (Can't directly convert from `Yaml` to `u32`)
                        .unwrap_or_else(|| panic!("{color_sys_error_msg}")),
                )
                // Panics if `i32` cannot be converted to a `u32`.
                .unwrap_or_else(|_| panic!("{color_sys_error_msg}"))
            };

            let rgb = [nth_color(0), nth_color(1), nth_color(2)];

            systems.push(System::new(label, display_name, rgb, path, games_are_dirs));
        }

        Ok(systems)
    }
}

impl std::ops::Index<&str> for ConfigFile<'_> {
    type Output = Yaml;

    fn index(&self, index: &str) -> &Self::Output {
        &self.contents[0][index]
    }
}
