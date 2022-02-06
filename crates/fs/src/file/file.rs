use super::range::Range;
use crate::error;
use once_cell::sync::Lazy;
use regex::Regex;
use tokio::fs::metadata;
use tokio::fs::File as TokioFile;
use tokio::io::{AsyncReadExt, AsyncSeekExt, Result, SeekFrom};

pub type Bytes = Vec<u8>;

#[derive(Debug, PartialEq)]
pub struct File {
    path: String,
    size: u64,
}

static REGEX_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"([^/]+$)").unwrap());
static REGEX_EXTENSION: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\.[^.]+$)").unwrap());

impl File {
    /// size is only used to return f.size() not asynchroniously
    pub fn new(path: String, size: u64) -> Self {
        let path = path.replace("\\", "/");
        Self { path, size }
    }

    pub async fn new_from_path(path: &str) -> Result<Self> {
        Ok(Self {
            path: path.to_owned(),
            size: metadata(path).await?.len(),
        })
    }

    pub fn path(&self) -> String {
        self.path.to_owned()
    }

    /// example: for path '/my/dir/my-file.txt' => '/my/dir/'
    pub fn path_of_dir(&self) -> String {
        let name_with_extension = self.name_with_extension();
        let mut chars = self.path.chars();
        for _ in 0..name_with_extension.len() {
            chars.next_back();
        }
        return chars.as_str().to_owned();
    }

    /// returns name with extension 'textfile.txt' (captures everything after last '/' in path)
    /// if no '/' exists -> returns ""
    pub fn name_with_extension(&self) -> String {
        let regex_name_match = match REGEX_NAME.captures(&self.path) {
            Some(s) => s.get(0),
            None => return String::from(""),
        };

        let name_with_extension = match regex_name_match {
            Some(s) => s.as_str(),
            None => return String::from(""),
        };

        return name_with_extension.to_string();
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    /// returns everything between last "/" and last "." => "/path/name.adsf.extension" -> "name.asdf"
    pub fn name(&self) -> String {
        let name_with_extension = self.name_with_extension();

        let regex_extension_match = match REGEX_EXTENSION.captures(&self.path) {
            Some(s) => s.get(0),
            None => return name_with_extension.to_owned(),
        };

        return match regex_extension_match {
            Some(s) => name_with_extension.replace(s.as_str(), ""),
            None => return name_with_extension,
        };
    }

    /// guesses by evaluating the file extension
    pub fn mime(&self) -> Result<String> {
        if self.path.ends_with(".m3u8") {
            return Ok("application/x-mpegURL".to_string());
        }

        let guess = mime_guess::from_path(&self.path).first();
        return match guess {
            Some(s) => Ok(s.to_string()),
            None => error::other(format!(
                "could not guess MimeType for path: '{}'",
                self.path
            )),
        };
    }

    /// "chunk-size == range.offset" if file is large enough
    pub async fn chunk(&self, range: &Range) -> Result<Bytes> {
        let mut buffer = Bytes::new();
        let mut file = TokioFile::open(self.path.to_owned()).await?;

        file.seek(SeekFrom::Start(range.start())).await?;
        file.take(range.offset()).read_to_end(&mut buffer).await?;

        return Ok(buffer);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn path() {
        assert_eq!(
            File::new("/path/name.extension".to_owned(), 0).path(),
            String::from("/path/name.extension")
        );
    }

    #[tokio::test]
    async fn size() {
        assert_eq!(
            File::new("/path/name.extension".to_owned(), 1000).size(),
            1000
        );
    }

    mod mime {
        use super::*;

        #[tokio::test]
        async fn jpg() {
            assert_eq!(
                File::new("/path/name.jpg".to_owned(), 0).mime().unwrap(),
                String::from("image/jpeg")
            );
        }

        #[tokio::test]
        async fn mp4() {
            assert_eq!(
                File::new("/path/name.mp4".to_owned(), 0).mime().unwrap(),
                String::from("video/mp4")
            );
        }

        #[tokio::test]
        async fn invalid() {
            assert!(File::new("/path/name.invalid".to_owned(), 0)
                .mime()
                .is_err());
        }

        #[tokio::test]
        async fn m3u8() {
            assert_eq!(
                File::new("/path/name.m3u8".to_owned(), 0).mime().unwrap(),
                String::from("application/x-mpegURL")
            );
        }
    }

    mod path_of_dir {
        use super::*;

        #[tokio::test]
        async fn normal_path() {
            assert_eq!(
                File::new("/path/name.extension".to_owned(), 0).path_of_dir(),
                String::from("/path/")
            );
        }

        #[tokio::test]
        async fn only_name() {
            assert_eq!(
                File::new("name.extension".to_owned(), 0).path_of_dir(),
                String::from("")
            );
        }

        #[tokio::test]
        async fn empty_path() {
            assert_eq!(File::new("".to_owned(), 0).path_of_dir(), String::from(""));
        }
    }

    mod name_with_extension {
        use super::*;

        #[tokio::test]
        async fn normal_path() {
            assert_eq!(
                File::new("/path/name.extension".to_owned(), 0).name_with_extension(),
                String::from("name.extension")
            );
        }

        #[tokio::test]
        async fn no_extension() {
            assert_eq!(
                File::new("/path/name".to_owned(), 0).name_with_extension(),
                String::from("name")
            );
        }

        #[tokio::test]
        async fn no_name_but_extension() {
            assert_eq!(
                File::new("/path/.extension".to_owned(), 0).name_with_extension(),
                String::from(".extension")
            );
        }

        #[tokio::test]
        async fn no_name_and_no_extension() {
            assert_eq!(
                File::new("/path/".to_owned(), 0).name_with_extension(),
                String::from("")
            );
        }

        #[tokio::test]
        async fn no_prepath() {
            assert_eq!(
                File::new("name.extension".to_owned(), 0).name_with_extension(),
                String::from("name.extension")
            );
        }

        #[tokio::test]
        async fn name_with_dot() {
            assert_eq!(
                File::new("/path/name.asdf.extension".to_owned(), 0).name_with_extension(),
                String::from("name.asdf.extension")
            );
        }

        #[tokio::test]
        async fn empty_path() {
            assert_eq!(
                File::new("".to_owned(), 0).name_with_extension(),
                String::from("")
            );
        }
    }

    mod name {
        use super::*;

        #[tokio::test]
        async fn normal_path() {
            assert_eq!(
                File::new("/path/name.extension".to_owned(), 0).name(),
                String::from("name")
            );
        }

        #[tokio::test]
        async fn no_extension() {
            assert_eq!(
                File::new("/path/name".to_owned(), 0).name(),
                String::from("name")
            );
        }

        #[tokio::test]
        async fn no_name_but_extension() {
            assert_eq!(
                File::new("/path/.extension".to_owned(), 0).name(),
                String::from("")
            );
        }

        #[tokio::test]
        async fn no_prepath() {
            assert_eq!(
                File::new("name.extension".to_owned(), 0).name(),
                String::from("name")
            );
        }

        #[tokio::test]
        async fn no_name_and_no_extension() {
            assert_eq!(File::new("/path/".to_owned(), 0).name(), String::from(""));
        }

        #[tokio::test]
        async fn name_with_dot() {
            assert_eq!(
                File::new("/path/name.asdf.extension".to_owned(), 0).name(),
                String::from("name.asdf")
            );
        }

        #[tokio::test]
        async fn empty_path() {
            assert_eq!(File::new("".to_owned(), 0).name(), String::from(""));
        }
    }
}
