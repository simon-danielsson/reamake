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
  
The core workflow of Reamake is feeding the CLI with reamake template files. A reamake file (file with the .reamake extension) is divided up into four sections: variables, sources, settings and hierarchy. This template format accepts comments, but comment lines should be separate from the parameter lines since doing otherwise could lead to undefined behaviour.  
  
#### Section - variables
  
In this section you can set variables for the names of the folder hierarchy. These can be set in the template or the CLI - my advice is that you keep fallback values here and override them explicitly in the CLI!
  
#### Section - sources
  
Here you will set which RPP file you want to use as your project template (as an absolute path). If no RPP path is assigned here, a new empty RPP project will be generated for you.
  
#### Section - settings
  
`format_names` : < true | false > format all variable, file and folder names to kebab-case (if omitted, defaults to false).  
`format_date` : < US | EU | ISO > date formatting (if omitted, defaults to EU).  
  
#### Section - hierarchy
  
Here is where you set the folder hierarchy using a custom hierarchy "language" developed especially for Reamake! The language itself is self-explanatory, the only keywords to keep track of are "folder"(a folder), "file"(a file) and "rpp"(the template project you've got set in the section "sources"). Every keyword takes a name within double-qoutes directly after it. When naming files you can add an extension as part of the name, and the file will be generated with that extension.  
  
You can (and should) be making use of variables, since these make a template more general and flexible. But it's not obligatory to use variables - for example, if you don't want to use the "service" or "date" variable you can simply omit them when naming your hierarchy.  
  
#### Full example
  
``` conf
[variables]

client: Cool Artist Name
project: Funk Song
service: Mix

[sources]

rpp: /Users/simondanielsson/dev/rust/reamake/test/default.RPP

[settings]

format_names: false
format_date: EU

[hierarchy]

folder "$date, [$service] $client - $project" {
    folder "project" {
        rpp "$project $date"
        file "notes.md"
    }
    folder "stems" {
        folder "processed" {}
        folder "raw" {
            file "todo.md"
        }
    }
    folder "export" {
        folder "drafts" {
            folder "v1" {}
        }
    }
    file "deadlines $project.md"
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
 
