use buffer_reader::BufferReader;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};

#[derive(Default)]
pub struct ScFile {
    file_path: PathBuf,
    file_type: Option<String>,
    file_hash: Option<String>,
    version: Option<i32>,
    has_texture: bool,
    is_texture_file: bool,
    reader: Option<BufReader<File>>,
    pub buffer_reader: Option<BufferReader>,
    pub buffer: Vec<u8>,
}

impl ScFile {
    pub fn new(file_path: String) -> ScFile {
        ScFile {
            file_path: Path::new(&file_path).to_path_buf(),
            ..Default::default()
        }
    }

    pub async fn load(&mut self) -> Result<()> {
        let file: &Path = &self.file_path;

        if !file.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", self.file_path.display()),
            ));
        }

        self.is_texture_file = file
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(|stem| stem.ends_with("_tex"))
            .unwrap_or(false);

        self.buffer = self.read_stream().await?;
        self.buffer_reader = Some(BufferReader::new(self.buffer.clone()));

        print!("> Loaded file: {}\n", self.file_path.display());
        Ok(())
    }

    async fn read_stream(&mut self) -> Result<Vec<u8>> {
        let file = File::open(&self.file_path).await?;
        let mut buffer = Vec::new();

        self.reader = BufReader::new(file).into();

        if let Some(reader) = &mut self.reader {
            reader.read_to_end(&mut buffer).await?;
        }

        let pattern = b"START";

        if let Some(start_index) = buffer.windows(pattern.len()).position(|w| w == pattern) {
            buffer = buffer[..start_index].to_vec();
        }

        println!(
            "> Read {} bytes from file: {}",
            buffer.len(),
            self.file_path.display(),
        );

        Ok(buffer)
    }
}
