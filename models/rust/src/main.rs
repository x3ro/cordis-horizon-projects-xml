mod helpers;
mod model;

use std::{
    fs::{self, File},
    io::{Cursor, Write},
    path::PathBuf,
};

use anyhow::Error;
use kdam::{BarExt, tqdm};
use loe::{Config, process};
use serde_xml_rs::SerdeXml;
use xml::{EmitterConfig, ParserConfig};

use crate::model::Project;

fn sxml() -> SerdeXml {
    SerdeXml::new()
        .emitter(
            EmitterConfig::new()
                .perform_indent(true)
                .normalize_empty_elements(true),
        )
        .parser(
            ParserConfig::new()
                .trim_whitespace(true)
                .whitespace_to_characters(true)
                .cdata_to_characters(true)
                .ignore_comments(true)
                .coalesce_characters(true)
                .ignore_end_of_stream(true),
        )
}

fn parse(path: &PathBuf) -> Result<Project, Error> {
    let src = convert(fs::read_to_string(path)?);
    Ok(sxml().from_str(&src)?)
}

fn convert(input: String) -> String {
    let mut input = Cursor::new(input);
    let mut output = Cursor::new(Vec::new());

    process(&mut input, &mut output, Config::default()).unwrap();
    String::from_utf8(output.into_inner()).unwrap()
}

const DATA_DIR: &'static str = "../../data";

fn data_path(rcn: u64) -> PathBuf {
    let mut path = PathBuf::from(DATA_DIR);
    let rcn = rcn.to_string();
    let prefix = &rcn[..rcn.len() - 3];

    path.push(prefix);
    path.push(format!("{rcn}.xml"));
    path
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let paths: Vec<_> = fs::read_dir("../../source")?.collect();

    let mut pb = tqdm!(total = paths.len());
    for path in paths {
        pb.update(1)?;

        let p = path?.path();
        if !p.to_str().unwrap_or("").ends_with(".xml") {
            continue;
        }

        match parse(&p) {
            Ok(project) => {
                let target = data_path(project.rcn);

                if let Ok(mut existing) = parse(&target) {
                    existing.last_update_date = project.last_update_date.clone();
                    if existing == project {
                        // Skipping update if the only difference between the old
                        // and the new version is the lastUpdateDate.
                        continue;
                    }
                }

                fs::create_dir_all(target.parent().unwrap())?;
                let mut f = File::create(target).expect("Should be able to create file");
                f.write_all(&sxml().to_string(&project).unwrap().as_bytes())?;
            }
            Err(e) => pb.write(format!("{}: {e}", p.display())).unwrap(),
        }
    }

    Ok(())
}
