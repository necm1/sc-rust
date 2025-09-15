use shared::sc_file::ScFile;
use std::io::Result;

#[async_trait::async_trait]
pub trait Processor {
    fn name(&self) -> &'static str;
    async fn can_process(&self, sc_file: &ScFile) -> bool;
    async fn process(&self, sc_file: &ScFile) -> Result<()>;
}
