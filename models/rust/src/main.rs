mod helpers;
mod model;

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Error;
use kdam::{BarExt, tqdm};
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
    let src = fs::read_to_string(path)?;
    Ok(sxml().from_str(&src)?)
}

fn main() {
    env_logger::init();
    let paths: Vec<_> = fs::read_dir("../source").unwrap().collect();
    let target_dir = "../data";

    let mut pb = tqdm!(total = paths.len());
    for path in paths {
        let p = path.unwrap().path();
        match parse(&p) {
            Ok(project) => {
                let rcn = project.rcn.to_string();
                let prefix = &rcn[..rcn.len() - 3];
                let project_dir = format!("{target_dir}/{}", prefix);
                let project_path = format!("{project_dir}/{}.xml", project.rcn);
                fs::create_dir_all(&project_dir).unwrap();
                let mut f = File::create(project_path).expect("Should be able to create file");
                f.write_all(&sxml().to_string(&project).unwrap().as_bytes())
                    .unwrap();
            }
            Err(e) => pb.write(format!("{}: {e}", p.display())).unwrap(),
        }

        pb.update(1).unwrap();
    }
}
