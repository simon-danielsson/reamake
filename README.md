<p align="center">
    <img src="media/logo.png" alt="reamake" width="200"/>
</p>
  
<p align="center">
  <em>A CLI application built for quickly creating folder structures and preparing<br>
        stems for mixing, mastering and production projects in <a href="https://www.reaper.fm/">Cockos Reaper</a></em>
</p>
  
<p align="center">
    <img src="https://img.shields.io/github/v/release/simon-danielsson/reamake?color=blueviolet&style=flat-square" alt="Latest Release" />
    <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT License" />
    <img src="https://img.shields.io/github/last-commit/simon-danielsson/reamake/main?style=flat-square&color=blue" alt="Last commit" />
</p>
  
<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#built-with">Built With</a> •
  <a href="#license">License</a>
</p>  
  
---
<div id="features"></div>

## ✨ Features
- 🎨 Customizable folder/file structures and templates  
- 📦 Batch creation of multiple projects at once  
- 🎚️ Normalize to -1dBFS and fold 'fake stereo' audio tracks to mono in bulk  
- 📂 Sort stems into subdirectories automatically using keywords  
- ⏱️ Progress bars & time tracking (via [indicatif](https://github.com/console-rs/indicatif))  

---
<div id="installation"></div>

## 💻 Installation
  
``` bash
cargo install reamake
```
  
---
<div id="quick-start"></div>

## 🚀 Quick Start

**Generate initialized template files**  
``` bash
reamake init <destination>
```
---
<div id="built-with"></div>

## 🛠️ Built With
+ [clap](https://github.com/BurntSushi/clap-rs)  
+ [yaml-rust2](https://github.com/ethiraric/yaml-rust2)  
+ [indicatif](https://github.com/console-rs/indicatif)  
+ [chrono](https://github.com/chronotope/chrono)  
+ [hound](https://github.com/ruuda/hound)  
+ [symphonia](https://github.com/pdeljanov/Symphonia)  

---
<div id="license"></div>

## 📜 License
This project is licensed under the [MIT License](https://github.com/simon-danielsson/reamake/blob/main/LICENSE).  

This project uses third-party crates with other licenses:
+ [hound](https://github.com/ruuda/hound) is licensed under Apache-2.0.  
+ [symphonia](https://github.com/pdeljanov/Symphonia) is licensed under MPL-2.0.  
  
All original files under MPL-2.0 remain under MPL-2.0. Any modifications to these files also remain under MPL-2.0.  
Original files under MPL-2.0 are not relicensed; modifications are also MPL-2.0.
