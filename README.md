<p align="center">
    <img src="media/logo.png" alt="reamake" width="200"/>
</p>
  
<p align="center">
  <em>A CLI application built for quickly creating folder structures and preparing<br>
        stems for mixing, mastering and production projects in <a href="https://www.reaper.fm/">Cockos Reaper</a></em>
</p>
  
<p align="center">
        <img src="https://img.shields.io/crates/v/reamake?style=flat-square&color=blueviolet&link=https%3A%2F%2Fcrates.io%2Fcrates%reamake" alt="Crates.io version" />
    <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT License" />
    <img src="https://img.shields.io/github/last-commit/simon-danielsson/reamake/main?style=flat-square&color=blue" alt="Last commit" />
</p>
  
<p align="center">
  <a href="#info">Information</a> •
  <a href="#install">Install</a> •
  <a href="#usage">Usage</a> •
  <a href="#deps">Dependencies</a> •
  <a href="#license">License</a>
</p>  
  
---
<div id="info"></div>

## Information
  
Reamake makes it easier and faster for mixing- and mastering-engineers to organize and prepare their projects for production.
  
### Features
  
- For building folder structures Reamake features its own custom scripting language with variable support - built to strike a balance between being powerful and beginner-friendly.  
- Subcommands for normalizing, correcting and sorting raw audio stems in record-time.
  
---
<div id="install"></div>

## Install
  
``` bash
cargo install reamake
```
  
---
<div id="usage"></div>

## Usage
  
### CLI
  
Generate initialized reamake template file:  
  
``` terminal
reamake init <optional target dir>
```
  
In your reamake template file you can set variables such as client name,  
project name and service name. If you'd like to override one or more of  
these fields you can do so through the cli:  
  
``` terminal
$ reamake ... -c <client name override>
$ reamake ... -p <project name override>
$ reamake ... -s <service name override>
```
  
Display help and version information:  
  
``` terminal
reamake help
```
  
### Template file (.reamake)
  
``` conf
# values you can customize and re-use
[variables]

# These variables can be set right here and/or they can be overridden at in the CLI.
# My advice is that you keep fallback values here and override them explicitly in the CLI!

# These can (and should) be used as variables (with prefix "$") in the hierarchy tree down below!

client: CoolBoi420
project: My cool song
service: Mix

# external files used for generation
[sources]

rpp: /Users/simondanielsson/dev/rust/reamake/test/default.RPP

# changes in behavior
[settings]

# normalize variable names and other folder/file names to "kebab-case" (true | false)
# setting to false will preserve original capitalization
format_names: false

# US: MM-DD-YYYY, EU: DD-MM-YYYY, ISO: YYYY-MM-DD
format_date: EU

# folder tree structure
# this is custom hierarchy syntax developed especially for Reamake!
# the language is self-explanatory, the only keywords to keep track of are "folder", "file" and "rpp"
[hierarchy]

folder "[$service] $client - $project ($date)" {
    folder "project" {
        rpp "$project $date"
            file "notes.md"
    }

    folder "stems" {
        folder "processed" {}
        folder "raw" {}
    }

    folder "export" {
        folder "drafts" {}
    }
}
```
  
---
<div id="deps"></div>
  
## Dependencies
  
+ [chrono](https://github.com/chronotope/chrono)  
  
---
<div id="license"></div>

## License
  
This project is licensed under the [MIT License](https://github.com/simon-danielsson/reamake/blob/main/LICENSE).  
 
