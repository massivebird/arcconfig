# arcconfig

> short for "archive configuration"

[![Crate](https://img.shields.io/crates/v/arcconfig)](https://crates.io/crates/arcconfig)
[![GitHub License](https://img.shields.io/github/license/massivebird/arcconfig?color=blue)](https://github.com/massivebird/arcconfig/blob/main/LICENSE)

🦀 written in Rust

## What is arcconfig?

Arcconfig is for people who collect catalogues — or "archives" — of games in the form of ROMs, ISOs, etc.

__Arcconfig provides a layer of abstraction over your archive, allowing you to interact with it via Rust!__

Arcconfig represents each game system in your archive as `System` instances, each of which contains data such as a display name and path (see: [Customization](#Customization)). This allows you to write cool Rust projects such as:

+ [massivebird/arcsearch](https://github.com/massivebird/arcsearch): queries archive with regex
+ [massivebird/arcstat](https://github.com/massivebird/arcstat): provides archive statistics

## How do I use it?

To use arcconfig in your own Rust project, you must first add it as a dependency.

One way you can do this according to [crates.io](https://crates.io/crates/arcconfig) is running this command in your project directory:

```bash
cargo add arcconfig
```

<h3 id="Customization">Customization</h3>

`arcconfig::read_config` parses a file called `config.yaml` located in your archive root. This function returns a collection of `System` instances based on that configuration!

> For a quickstart on YAML syntax, click [here](https://docs.ansible.com/ansible/latest/reference_appendices/YAMLSyntax.html).

Here is an example configuration:

```yaml
# config.yaml
systems:
  ds: # system "label" — call it whatever you want!
    display_name: "DS"
    color: [135,215,255]
    path: "ds" # path relative to archive root
    games_are_directories: false # are games stored as directories?
  snes:
    display_name: "SNES"
    color: [95,0,255]
    path: "snes"
    games_are_directories: false
  wii:
    display_name: "WII"
    color: [0,215,255]
    path: "wbfs"
    games_are_directories: true
```

Feel free to use these `System` instances however you'd like!

## Other arcosystem projects

Arcconfig belongs to a family of projects called the arcosystem!

See the projects that arcconfig makes possible:

+ [arcsearch](https://github.com/massivebird/arcsearch): game archive querying
+ [arcstat](https://github.com/massivebird/arcstat): game archive stats
