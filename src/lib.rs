use colored::Colorize;
use self::system::System;
use std::{fs, path::Path};
use yaml_rust::YamlLoader;

mod system;

pub fn read_config(archive_root: &str) -> Vec<System> {
    if !Path::new(archive_root).exists() {
        panic!("Archive path does not exist: {archive_root}");
    }

    let yaml_path = String::from(archive_root) + "/config.yaml";
    let yaml_contents = fs::read_to_string(yaml_path).expect(
        "`config.yaml` not found in archive root."
    );

    let data = &YamlLoader::load_from_str(&yaml_contents).expect(
        "`config.yaml` could not be parsed."
    )[0]["systems"];

    if data.is_badvalue() {
        panic!("`config.yaml` does not contain a `systems` key.");
    }

    let mut systems: Vec<System> = Vec::new();

    let declared_systems_iter = || {
        data
            .as_hash()
            .expect("something is seriously wrong with this yaml")
            .iter()
    };

    for (label, system) in declared_systems_iter() {
        let label = label
            .as_str()
            // if the label cannot be parsed, then I'm not sure how to provide
            // precise feedback about it
            .expect("archive error: bad system label somewhere :3 idk");

        let error_msg = |msg: &str| -> String {
            format!("archive error: system labeled `{label}`: {msg}")
        };

        let display_name = system["display_name"]
            .as_str()
            .expect(&error_msg("missing `display_name` property"));

        let color = system["color"]
            .as_vec()
            .expect(&error_msg("missing `color` property"));

        let path = system["path"]
            .as_str()
            .expect(&error_msg("missing `path` property"));

        let games_are_directories = system["games_are_directories"]
            .as_bool()
            .expect(&error_msg("missing `games_are_directories` property"));

        let color_error_msg: &str = "unexpected `color` value. Expected: `[u8, u8, u8]`";

        let nth_color = |n: usize| -> u8 {
            color
                .get(n)
                .expect(&error_msg(color_error_msg))
                .as_i64()
                .expect(&error_msg(color_error_msg))
            as u8
        };

        let display_name = display_name.truecolor(
            nth_color(0),
            nth_color(1),
            nth_color(2)
        );

        systems.push(System::new(
            label,
            display_name,
            path,
            games_are_directories,
        ));
    }

    systems
}

#[cfg(test)]
mod tests {
    use yaml_rust::{YamlLoader, Yaml};

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
    fn parse_path() {
        let data = &YamlLoader::load_from_str(DEMO).unwrap()[0]["systems"];
        assert_eq!(data["gamecube"]["path"], Yaml::String("games".to_string()));
    }

    #[test]
    fn parse_games_are_directories() {
        let data = &YamlLoader::load_from_str(DEMO).unwrap()[0]["systems"];
        assert_eq!(data["ds"]["games_are_directories"], Yaml::Boolean(false));
    }

}
