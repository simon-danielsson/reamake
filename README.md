
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
  
- Sort stems into sub-directories automatically!
- Normalize to -1dBFS and fold 'fake stereo' audio tracks to mono in bulk.
- For designing your folder structures Reamake features its own scripting language with support for custom variables, striking a balance between flexibility and ease-of-use. [Neovim plugin for syntax highlighting and such is available here](https://github.com/simon-danielsson/reamake.nvim).
  
> [!IMPORTANT]  
> Reamake only supports **Unix** systems.  
  
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
  
**Subcommand: init**  
Generate initialized reamake template file (without target dir, current directory is assumed):  
  
``` terminal
reamake init <target dir>
```
  
**Subcommand: sort**  
Sort stems automatically into sub-directories in target dir (without target dir, current directory is assumed). Reamake will not sort files that do not have the extensions: wav, mp3, flac, aif, aiff and m4v. Proceed with caution and be sure to keep backups of all your stems!  
  
``` terminal
reamake sort <target dir>
```
  
**Subcommand: gen | generate | build**  
Generate folder structure from template file (without target dir, current directory is assumed):  
  
``` terminal
reamake gen <reamake template file> <target dir>
reamake generate <reamake template file> <target dir>
reamake build <reamake template file> <target dir>
```
  
**Subcommand: norm**  
Normalize stems, as well as check for "fake stereo" (i.e. the file is stereo but the source is mono) and fold stems into mono files accordingly (without target dir, current directory is assumed). Reamake will not process files that do not have the extensions: wav, mp3, flac, aif, aiff and m4v. Note that this command will recursively scan for all compatible audio files in the target directory and its children. Proceed with caution and be sure to keep backups of all your stems!  
  
``` terminal
reamake norm <target dir>
```
  
**Subcommand: help**  
Display help and version information:  
  
``` terminal
reamake help
```
  
**Flag: -var | -v | --var**  
In your reamake template file you can set variables, and they can all be overridden through the CLI at your leisure. Just make sure that the variable you're overriding exists in you reamake file and that you're supplying the right kind of value! (e.g a folder variable override would need a filepath, a string variable would need a string and so on)  
  
``` terminal
reamake ... -var variable="value"
reamake ... -v variable="value"
reamake ... --var variable="value"

Example:
reamake build template.reamake --var stems="/Users/usr/Downloads/stems/" --var client="New Client" --var rpp="/Users/usr/music/templates/mix.rpp"
```
  
### Template file (.reamake)
  
The core workflow of Reamake consists of feeding the CLI with reamake template files. A reamake file (file with the .reamake extension) is divided up into three sections: variables, settings and hierarchy. This format accepts comments (prefixed with '#'), but comment lines should be separate from the parameter lines since doing otherwise could lead to undefined behaviour. For more details, read the comments in the example below!
  
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

        # it's legal to add extra files/folders to a folder variable
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
+ [hound](https://github.com/ruuda/hound)  
+ [symphonia](https://github.com/pdeljanov/Symphonia)  
  
---
<div id="license"></div>

## License
  
This project is licensed under the [MIT License](https://github.com/simon-danielsson/reamake/blob/main/LICENSE).  
 
