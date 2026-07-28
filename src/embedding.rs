use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

pub struct Embedder {
    model: TextEmbedding,
}

impl Embedder {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let model = TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2))?;
        Ok(Self { model })
    }

    pub fn embed(&mut self, texts: &[&str]) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        let texts_vec: Vec<String> = texts.iter().map(|s| s.to_string()).collect();
        let embeddings = self.model.embed(texts_vec, None)?;
        Ok(embeddings.into_iter().map(|e| e.to_vec()).collect())
    }
}
