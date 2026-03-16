
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
  <a href="#feat">Features</a> •
  <a href="#install">Install</a> •
  <a href="#usage">Usage</a> •
  <a href="#deps">Dependencies</a> •
  <a href="#license">License</a>
</p>  
  
---
<div id="feat"></div>

## Features
  
- For designing your folder structures Reamake features its own scripting language with support for custom variables, striking a balance between flexibility and ease-of-use. [Neovim plugin for syntax highlighting and such is available here](https://github.com/simon-danielsson/reamake.nvim).
- Subcommands for normalizing, correcting and sorting raw audio stems in record-time (not implemented yet).
  
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
  
Generate initialized reamake template file  
(without target dir, current directory is assumed):  
  
``` terminal
reamake init <target dir>
```
  
Generate folder structure from template file  
(without target dir, current directory is assumed):  
  
``` terminal
reamake -f <reamake template file> <target dir>
```
  
In your reamake template file you can set variables such as client name,  
project name and service name. If you'd like to override one or more of  
these fields you can do so through the cli:  
  
``` terminal
reamake ... -c <client var override>
reamake ... -p <project var override>
reamake ... -s <service var override>
```
  
Display help and version information:  
  
``` terminal
reamake help
```
  
### Template file (.reamake)
  
The core workflow of Reamake consists of feeding the CLI with reamake template files. A reamake file (file with the .reamake extension) is divided up into four sections: variables, sources, settings and hierarchy. This template format accepts comments (prefixed with '#'), but comment lines should be separate from the parameter lines since doing otherwise could lead to undefined behaviour.  
  
#### Full example
  
``` conf
# there are only three different datatypes to keep track of:

# 1: <string> - used for name variables

# 2: <file> - an empty generic file, where the extension in its name will specify the extension of the file once generated
# variables with the datatype <file> will inherit the same extension logic as a generic file, only that the contents of the file variable path supplied will be the contents of the file generated

# 3: <folder> - a generic folder
# variables with the datatype <folder> will inherit the contents of the folder specified in the variables section

variables [
    # built-in: date
    client: string = "client"
    project: string = "project"
    service: string = "mix"
    rpp: file = "/Users/usr/music/templates/mix.RPP"
    stems: folder = "/Users/usr/Downloads/stems_from_client/"
    # you can create any variables you want
    fruit: string = "banana"
]

settings [
    format_names: false
    format_date: "%d-%m-%Y"
]

hierarchy [
    folder "[$service] $client - $project, $date" {
        folder "project" {
            rpp "$project $date.RPP"
            file "$fruit notes.md"
        }

        stems "stems" {
            file "todo.md"
        }

        folder "export" {
            folder "$fruit drafts" {}
        }
    }
]
```
  
---
<div id="deps"></div>
  
## Dependencies
  
+ [chrono](https://github.com/chronotope/chrono)  
  
---
<div id="license"></div>

## License
  
This project is licensed under the [MIT License](https://github.com/simon-danielsson/reamake/blob/main/LICENSE).  
 
