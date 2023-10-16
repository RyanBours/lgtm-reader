use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::BufReader;
use std::io::Read;
use zip::ZipArchive;

use crate::util::is_valid_image_extension;

pub struct CBZ {
    archive: ZipArchive<BufReader<fs::File>>,
    pub info: InfoYAML,
    pub pages: Vec<String>, // maybe unnecessary
}

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoYAML {
    Artist: Vec<String>,
    Description: String,
    Magazine: Vec<String>,
    Pages: i32,
    Parody: Vec<String>,
    Publisher: Vec<String>,
    Released: i32,
    Tags: Vec<String>,
    Thumbnail: i32,
    Title: String,
    URL: String,
}

impl CBZ {
    pub fn new(file_location: String) -> Self {
        let mut archive = Self::get_archive(file_location);
        let info = Self::get_info(&mut archive);
        let pages = Self::get_pages(&mut archive);

        Self {
            archive,
            info,
            pages,
        }
    }

    fn get_archive(file_location: String) -> ZipArchive<BufReader<fs::File>> {
        let file = fs::File::open(file_location).unwrap();
        let reader = BufReader::new(file);

        zip::ZipArchive::new(reader).unwrap()
    }

    fn get_info(archive: &mut ZipArchive<BufReader<fs::File>>) -> InfoYAML {
        let info_yaml = archive.by_name("info.yaml");

        let mut file = match info_yaml {
            Ok(file) => file,
            Err(_) => panic!("No info.yaml found in archive"),
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let info: InfoYAML = serde_yaml::from_str(&contents).unwrap();

        info
    }

    fn get_pages(archive: &mut ZipArchive<BufReader<fs::File>>) -> Vec<String> {
        let mut pages: Vec<String> = vec![];

        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            let outpath = match file.enclosed_name() {
                Some(path) => path,
                None => {
                    println!("Entry {} has a suspicious path", file.name());
                    continue;
                }
            };

            let page_name = outpath.display().to_string();

            if is_valid_image_extension(&page_name) {
                pages.push(page_name);
            }
        }

        pages
    }

    pub fn get_image_by_page(&mut self, page: usize) -> String {
        let page_name = &self.pages[page];
        let mut file = self.archive.by_name(page_name).unwrap();
        let mut contents = vec![];
        file.read_to_end(&mut contents).unwrap();

        general_purpose::STANDARD.encode(&contents)
    }

    pub fn get_thumbnail(&mut self) -> String {
        let page_name = &self.pages[(self.info.Thumbnail - 1) as usize];
        let mut file = self.archive.by_name(page_name).unwrap();
        let mut contents = vec![];
        file.read_to_end(&mut contents).unwrap();

        general_purpose::STANDARD.encode(&contents)
    }
}
