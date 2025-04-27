use self::system::System;
use colored::Colorize;
use std::{fs, path::Path};
use yaml_rust::YamlLoader;

pub mod system;

/// Returns a collection of `System` instances based on the archive's configuration file.
///
/// # Valid file system structure example
///
/// Follow these guidelines to reduce unexpected behavior when using this crate.
///
/// _Side note: A "system directory" is a directory that contains games for a single system._
///
/// Your archive's file structure is valid if:
///
/// + The configuration file is located in the archive root
/// + System directories are never nested
/// + For any system directory, games are represented as either normal files or directories (never both)
///
/// ```bash
/// /game/archive/root
/// ├── ds
/// │   ├── game-1.nds
/// │   ├── game-2.nds
/// │   └── game-3.nds
/// ├── wii
/// │   ├── game-1-dir
/// │   │   └── game-1.wbfs
/// │   └── game-2-dir
/// │       └── game-2.wbfs
/// └── config.yaml
/// ```
///
/// # Valid configuration file example
///
/// ```yaml
/// # config.yaml in archive root
/// systems:
///   ds: # system "label" — call it whatever you want!
///     display_name: "DS"
///     color: [135,215,255]
///     path: "ds" # path to system dir relative to archive root
///     games_are_directories: false # are games stored as directories?
///   snes:
///     display_name: "SNES"
///     color: [95,0,255]
///     path: "snes"
///     games_are_directories: false
///   wii:
///     display_name: "WII"
///     color: [0,215,255]
///     path: "wbfs"
///     games_are_directories: true
/// ```
///
/// # Panics
///
/// Will panic if any of the following are true:
///
/// + The provided `archive_root` path does not exist.
/// + The configuration file
///   + Cannot be found.
///   + Does not contain the expected fields.
///   + Contains a system with a nonexistent `path`.
#[must_use]
pub fn read_config(archive_root: &Path) -> Vec<System> {
    let error_msg = |msg: &str| -> String { format!("archive config error: {msg}") };

    let yaml_contents = {
        assert!(
            archive_root.exists(),
            "{}",
            &error_msg(&format!("path does not exist: {}", archive_root.display()))
        );

        let yaml_path = archive_root.join("config.yaml");

        fs::read_to_string(yaml_path).expect(&error_msg(&format!(
            "`config.yaml` not found in archive root."
        )))
    };

    let systems_key = &YamlLoader::load_from_str(&yaml_contents)
        .expect(&error_msg(&format!("`config.yaml` could not be parsed.")))[0]["systems"];

    assert!(
        !systems_key.is_badvalue(),
        "{}",
        &error_msg(&format!("`config.yaml` does not contain a `systems` key."))
    );

    let mut systems: Vec<System> = Vec::new();

    for (label, properties) in systems_key
        .as_hash()
        .expect("`systems` contains a single value, expected a collection of labels")
        .iter()
    {
        let label = label
            .as_str()
            // If the label cannot be parsed, then I'm not sure how to provide
            // precise feedback about it.
            .expect("archive config error: bad system label somewhere");

        let sys_error_msg = |msg: &str| -> String {
            format!("archive config error: system labeled `{label}`: {msg}")
        };

        macro_rules! extract_property {
            ( $property_name: expr, $converter: ident ) => {
                properties[$property_name]
                    .$converter() // type conversion is declared at macro invocation
                    .expect(&sys_error_msg(&format!(
                        "missing `{}` property",
                        $property_name
                    )))
            };
        }

        let display_name = extract_property!("display_name", as_str);
        let color = extract_property!("color", as_vec);
        let path = extract_property!("path", as_str);
        let games_are_dirs = extract_property!("games_are_directories", as_bool);

        let system_path = archive_root.join(path);
        let path_error_msg = format!("system path `{}` does not exist", system_path.display());

        assert!(
            Path::new(&system_path).exists(),
            "{}",
            sys_error_msg(&path_error_msg)
        );

        let color_sys_error_msg: &str =
            &sys_error_msg("unexpected `color` value. Expected: `[u8, u8, u8]`");

        let nth_color = |n: usize| -> u8 {
            u8::try_from(
                color
                    .get(n)
                    .unwrap_or_else(|| panic!("{color_sys_error_msg}"))
                    .as_i64()
                    .unwrap_or_else(|| panic!("{color_sys_error_msg}")),
            )
            .unwrap_or_else(|_| panic!("{color_sys_error_msg}"))
        };

        let display_name = display_name.truecolor(nth_color(0), nth_color(1), nth_color(2));

        systems.push(System::new(label, display_name, path, games_are_dirs));
    }

    systems
}

#[cfg(test)]
mod tests {
    use yaml_rust::{Yaml, YamlLoader};

    const DEMO: &str = "
systems:
    wii:
        display_name: WII
        color: [0,255,0]
        path: wbfs
        games_are_directories: true
    gamecube:
        display_name: GCN
        color: [62,255,0]
        path: games
        games_are_directories: true
    ds:
        display_name: DS
        color: [0,255,55]
        path: ds
        games_are_directories: false
";

    #[test]
    fn parse_display_name() {
        let data = &YamlLoader::load_from_str(DEMO).unwrap()[0]["systems"];
        assert_eq!(data["wii"]["display_name"], Yaml::String("WII".to_string()));
    }

    #[test]
    fn parse_color() {
        let data = &YamlLoader::load_from_str(DEMO).unwrap()[0]["systems"];
        let color_vec = data["wii"]["color"].as_vec().unwrap();
        assert_eq!(color_vec.get(1).unwrap().as_i64().unwrap(), 255);
    }

    #[test]
    fn parse_root() {
        let data = &YamlLoader::load_from_str(DEMO).unwrap()[0]["systems"];
        assert_eq!(data["gamecube"]["path"], Yaml::String("games".to_string()));
    }

    #[test]
    fn parse_games_are_directories() {
        let data = &YamlLoader::load_from_str(DEMO).unwrap()[0]["systems"];
        assert_eq!(data["ds"]["games_are_directories"], Yaml::Boolean(false));
    }

    // #[test]
    // fn read_real() {
    //     super::read_config("/home/penguino/game-archive");
    // }
}
