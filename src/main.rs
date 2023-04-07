use clap::Parser;
use std::error::Error;

use serde::Deserialize;
use std::fs::File;
use std::io::{Error as IoError, Write};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(
        long,
        help = "問題の情報が書かれているymlファイル(e.g. challenge.yml, task.yml)",
        default_value = "challenge.yml"
    )]
    chall_yml: Option<String>,

    #[arg(
        long,
        help = "作問チェックの情報が書かれているymlファイル(i.e. tested.yml)",
        default_value = "tested.yml"
    )]
    tested_yml: Option<String>,

    #[arg(long, help = "ディレクトリパス", default_value = "./")]
    dir_path: Option<String>,

    #[arg(long, help = "出力先のパス", default_value = "./")]
    output_path: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Challenge {
    name: String,
    author: String,
    category: String,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Tested {
    tested: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
struct ChallengeTested {
    challenge: Challenge,
    tested: Tested,
}

impl ChallengeTested {
    fn from_dir_entry(dir_entry: &DirEntry) -> Result<Self, Box<dyn Error>> {
        let challenge_file_path = dir_entry.path().join("challenge.yml");
        let tested_file_path = dir_entry.path().join("tested.yml");

        let challenge_file = File::open(challenge_file_path)?;
        let challenge: Challenge = serde_yaml::from_reader(challenge_file)?;

        let tested_file = File::open(tested_file_path)?;
        let tested: Tested = serde_yaml::from_reader(tested_file)?;

        Ok(Self {
            challenge: challenge,
            tested: tested,
        })
    }

    fn to_markdown_row(&self) -> String {
        let tags = self.challenge.tags.join(", ");
        format!(
            "| {} | {} | {} | {} | {} |",
            self.tested.tested,
            self.challenge.name,
            self.challenge.author,
            self.challenge.category,
            tags,
        )
    }
}

fn write_challenge_to_readme<P: AsRef<Path>>(path: P) -> Result<(), IoError> {
    let mut readme_file = File::create(path.as_ref().join("README.md"))?;

    writeln!(readme_file, "| tested | name | author | category | tags |",)?;
    writeln!(readme_file, "|--------|------|--------|----------|------|",)?;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        match ChallengeTested::from_dir_entry(&entry) {
            Ok(challenge_tested) => {
                writeln!(readme_file, "{}", challenge_tested.to_markdown_row())?;
            }
            _ => {}
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = write_challenge_to_readme(".") {
        eprintln!("Error writing to README.md: {}", err);
    }
}
