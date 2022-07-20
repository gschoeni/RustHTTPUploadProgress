
use clap::{arg, Command};

// Error Handling
#[derive(Debug)]
pub enum UploadError {
    IO(std::io::Error),
    HTTP(reqwest::Error),
}

impl From<std::io::Error> for UploadError {
    fn from(error: std::io::Error) -> Self {
        UploadError::IO(error)
    }
}

impl From<reqwest::Error> for UploadError {
    fn from(error: reqwest::Error) -> Self {
        UploadError::HTTP(error)
    }
}

// Progress Bar Updating
pub struct ReadProgress<R> {
    pub inner: R,
    pub progress_bar: indicatif::ProgressBar
}

impl<R: std::io::Read> std::io::Read for ReadProgress<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf).map(|n| {
            self.progress_bar.inc(n as u64);
            n
        })
    }
}

fn upload<P: AsRef<std::path::Path>>(path: P) -> Result<(), UploadError> {
    let path = path.as_ref();
    let buffer: Vec<u8> = std::fs::read(path)?;

    println!("Uploading {} bytes from {:?}", buffer.len(), path);

    let upload_progress = indicatif::ProgressBar::new(buffer.len() as u64);
    let cursor = std::io::Cursor::new(Vec::from(buffer));
    let upload_source = ReadProgress {
        progress_bar: upload_progress,
        inner: cursor,
    };

    let url = "http://0.0.0.0:3030/upload";
    let client = reqwest::blocking::Client::builder()
        .build()?;

    match client
        .post(url)
        .body(reqwest::blocking::Body::new(upload_source))
        .send()
    {
        Ok(res) => {
            let status = res.status();
            let body = res.text()?;
            println!("Upload got response[{}]\n{}", status, body);
        },
        Err(err) => {
            eprintln!("Error uploading file: {}", err);
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let matches = Command::new("sample-tool")
        .version("0.0.1")
        .about("Showing how to create CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("upload")
                .about("Upload a file")
                .arg(arg!(<FILENAME> "Name of the file you want to upload"))
                .arg_required_else_help(true),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("upload", sub_matches)) => {
            let filename = sub_matches.value_of("FILENAME").expect("required");
            upload(filename).expect("Upload failed.");
            Ok(())
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
