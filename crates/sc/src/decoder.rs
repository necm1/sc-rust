use processor::registry::Registry as ProcessorRegistry;
use shared::sc_file::ScFile;
use std::io::Result;

pub struct Decoder<'a> {
    sc_file: Option<&'a ScFile>,
}

impl<'a> Decoder<'a> {
    pub fn new() -> Self {
        Decoder { sc_file: None }
    }

    pub async fn decode(mut self, sc_file: &ScFile) -> Result<()> {
        print!("> Decoding SC File...\n");

        self.sc_file = Some(sc_file);

        if sc_file.buffer_reader.is_none() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer reader is not initialized.",
            ));
        }

        // let buffer_reader = self
        //     .sc_file
        //     .as_ref()
        //     .unwrap()
        //     .buffer_reader
        //     .as_ref()
        //     .unwrap();

        // let buffer_reader = sc_file.buffer_reader.as_ref().unwrap();
        self.decode_metadata(Some(sc_file)).await?;

        Ok(())
    }

    pub async fn decode_metadata(&self, sc_file: Option<&ScFile>) -> Result<()> {
        print!("> Decoding Metadata...\n");

        let sc_file = match sc_file {
            Some(file) => file,
            None => self.sc_file.expect("No sc_file set in Decoder"),
        };

        let processor_registry = ProcessorRegistry::new();
        let processor = processor_registry
            .get_item(sc_file)
            .await
            .expect("No suitable processor found");
        processor.process(sc_file).await?;

        Ok(())
    }
}
