use crate::processor::Processor;
use crate::processors::sc::ScProcessor;
use std::sync::Arc;

pub struct Registry {
    items: Vec<Arc<dyn Processor + Send + Sync>>,
}

impl Registry {
    pub fn new() -> Self {
        let mut items: Vec<Arc<dyn Processor + Send + Sync>> = Vec::new();
        items.push(Arc::new(ScProcessor));

        Self { items }
    }

    pub fn register(&mut self, processor: Arc<dyn Processor + Send + Sync>) {
        self.items.push(processor);
    }

    pub fn all(&self) -> &Vec<Arc<dyn Processor + Send + Sync>> {
        &self.items
    }

    pub async fn get_item(
        &self,
        sc_file: &shared::sc_file::ScFile,
    ) -> Option<Arc<dyn Processor + Send + Sync>> {
        for processor in &self.items {
            if processor.can_process(sc_file).await {
                print!("> Selected processor: {}\n", processor.name());
                return Some(processor.clone());
            }
        }
        None
    }
}
