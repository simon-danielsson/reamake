
![logo](media/logo.png)

ReaMake is a CLI application for quickly creating folder and file structures for mixing, mastering, and production projects in **Reaper**.  
  
Built with **Rust**, using:  
+ [clap](https://github.com/BurntSushi/clap-rs?tab=readme-ov-file) for command-line parsing  
+ [yaml-rust2](https://github.com/ethiraric/yaml-rust2) for YAML parsing  
  
Features:  
+ Batch creation of multiple projects at once  
+ Customizable folder/file structures and templates  
  
> ⚠️ Currently, ReaMake only supports **macOS**.  
  
## Usage  
  
### reamake  
`reamake [OPTIONS] <destin>`  
  
**-c** : Client name    
Optional; defaults to a generic name if omitted.    
All words are normalized to lowercase with underscores (e.g., cool_project).    
`-c 'Cool Client'`    
  
**-p** : Project name      
Optional; defaults to a generic name if omitted.    
All words are normalized to lowercase with underscores (e.g., cool_project).    
`-c 'Cool Project'`    
    
**-b** : BPM    
Optional; defaults to 120 BPM if omitted.    
`-b 120`    
    
**-t** : Project template    
Absolute path to a Reaper template file (.RPP).    
`-p 'Users/user/Desktop/music/mixing-projects/templates/mixing.RPP'`    
    
**-s** : Structure    
 Absolute path to a folder/file structure template (.yaml).    
Optional; defaults to a standard structure if omitted.    
`-s 'Users/user/Desktop/music/mixing-projects/templates/structure.yaml'`    
  
### reamake batch  
`reamake batch <batchfile> <destin>`  
+ Provide all flags through a .csv file.  
+ Create several structures at once using a single command. Add path to .csv, and then the destination path.  
  
### reamake init  
`reamake batch <destin>`  
+ Create initialized .csv and .yaml files in chosen directory for further customization.

