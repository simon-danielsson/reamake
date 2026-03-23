use std::{
    env,
    fs::{self},
    io,
    path::PathBuf,
};

const SORT_TERMS: &str = include_str!("../static/sort-terms.txt");

use crate::utils::{args::Arguments, messages};

pub fn run(args: &Arguments) -> io::Result<()> {
    // init
    let mut s = Sorter::new()?;

    if args.opt_target.exists() {
        s.dir = args.opt_target.clone();
    }

    // check for audio files
    s.scan()?;

    if s.audio_files.is_empty() {
        println!("{}", messages::NO_AUDIO_FOUND);
        return Ok(());
    }

    s.collect_sort_terms()?;

    s.match_files()?;

    s.build()?;

    Ok(())
}

#[derive(Debug, Clone)]
enum SortName {
    Percussion,
    Guitars,
    Vocals,
    Strings,
    Winds,
    Keys,
    Bass,
    Fx,
    Other,
}

impl SortName {
    fn as_str(&self) -> &'static str {
        match self {
            SortName::Percussion => "Percussion",
            SortName::Guitars => "Guitars",
            SortName::Vocals => "Vocals",
            SortName::Strings => "Strings",
            SortName::Winds => "Winds",
            SortName::Keys => "Keys",
            SortName::Bass => "Bass",
            SortName::Fx => "Fx",
            SortName::Other => "Other",
        }
    }
}

#[derive(Debug, Clone)]
struct SortGroup {
    name: SortName,
    terms: Vec<String>,
    audio_files: Vec<PathBuf>,
}

impl SortGroup {
    fn new(name: SortName) -> Self {
        Self {
            name,
            terms: Vec::new(),
            audio_files: Vec::new(),
        }
    }
}

struct Sorter {
    dir: PathBuf,
    audio_files: Vec<PathBuf>,
    sort_groups: Vec<SortGroup>,
}

impl Sorter {
    fn new() -> io::Result<Self> {
        Ok(Self {
            dir: env::current_dir()?,
            audio_files: Vec::new(),
            sort_groups: Vec::new(),
        })
    }

    /// match scanned audio files to terms in the sort groups
    fn match_files(&mut self) -> io::Result<()> {
        for file in &self.audio_files {
            let path_str = file.display().to_string().trim().to_lowercase();
            for sg in &mut self.sort_groups {
                if sg.terms.iter().any(|t| path_str.contains(t)) {
                    sg.audio_files.push(file.to_owned());
                }
            }
        }

        // filter out sort groups that didn't get any matches
        self.sort_groups = self
            .sort_groups
            .iter()
            .filter(|t| !t.audio_files.is_empty())
            .cloned()
            .collect();

        Ok(())
    }

    fn build(&mut self) -> io::Result<()> {
        for sg in &self.sort_groups {
            let folder = self.dir.join(sg.name.as_str());
            fs::create_dir(&folder)?;
            for audio_file in &sg.audio_files {
                let name = audio_file.file_name().unwrap();
                let from = PathBuf::from(audio_file);
                let to = folder.join(name);
                fs::rename(&from, &to)?;
            }
        }
        Ok(())
    }

    fn collect_sort_terms(&mut self) -> io::Result<()> {
        let mut lines = SORT_TERMS.lines().map(str::trim).peekable();

        while let Some(line) = lines.next() {
            let sort_name = match line {
                "_percussion" => SortName::Percussion,
                "_keys" => SortName::Keys,
                "_guitars" => SortName::Guitars,
                "_vocals" => SortName::Vocals,
                "_strings" => SortName::Strings,
                "_winds" => SortName::Winds,
                "_bass" => SortName::Bass,
                "_fx" => SortName::Fx,
                "_other" => SortName::Other,
                "" => continue,
                _ => continue,
            };

            let mut sort_grp = SortGroup::new(sort_name);

            while let Some(next_line) = lines.peek() {
                if next_line.is_empty() || next_line.starts_with('_') {
                    break;
                }
                sort_grp.terms.push(lines.next().unwrap().to_string());
            }

            self.sort_groups.push(sort_grp);
        }

        Ok(())
    }

    fn scan(&mut self) -> io::Result<()> {
        let paths = fs::read_dir(&self.dir)?;
        for entry in paths {
            let entry = entry?;
            let path = entry.path();
            match path.extension().and_then(|ext| ext.to_str()) {
                Some("wav") => self.audio_files.push(path),
                Some("mp3") => self.audio_files.push(path),
                Some("flac") => self.audio_files.push(path),
                Some("aif") => self.audio_files.push(path),
                Some("aiff") => self.audio_files.push(path),
                Some("m4v") => self.audio_files.push(path),
                _ => {}
            }
        }
        Ok(())
    }
}
