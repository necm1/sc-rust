use crate::processor::Processor;
use shared::sc_file::ScFile;

pub struct ScProcessor;

#[async_trait::async_trait]
impl Processor for ScProcessor {
    async fn can_process(&self, sc_file: &ScFile) -> bool {
        if let Some(buffer_reader) = &sc_file.buffer_reader {
            if let Ok(bytes) = buffer_reader.peek_bytes(0, 2) {
                return bytes == b"SC";
            }
        }
        false
    }

    async fn process(&self, sc_file: &ScFile) -> std::io::Result<()> {
        println!("Processing SC file");
        Ok(())
    }

    fn name(&self) -> &'static str {
        "ScProcessor"
    }
}
