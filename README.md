
<p align="center">
    <img src="media/logo.png" alt="reamake" width="300"/>
</p>

**reamake** is a CLI application built for quickly creating folder/file structures for mixing, mastering, and production projects in [**Cockos Reaper**](https://www.reaper.fm/).  
  
Built with **Rust** using:  
+ [clap](https://github.com/BurntSushi/clap-rs)  
+ [yaml-rust2](https://github.com/ethiraric/yaml-rust2)  
+ [progress_bar](https://github.com/Mubelotix/progress-bar)  
+ [chrono](https://github.com/chronotope/chrono)  
  
Features:  
+ Customizable folder/file structures and templates.  
+ Batch creation of multiple projects at once.  
  
> ⚠️ Currently, reamake only has support for Unix systems.  

## Installation (MacOS)
**Download the latest release**  
``` bash
curl -L https://github.com/simon-danielsson/reamake/releases/download/v1.0.0/reamake -o /usr/.local/bin/reamake
```
  
**Make it executable**  
``` bash
chmod +x /usr/.local/bin/reamake
```
  
**Verify installation**  
``` bash
reamake --help
```
  
**Get started by generating initialized template files**  
``` bash
reamake init <destination>
```
