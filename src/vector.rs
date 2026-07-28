use std::sync::Arc;

use arrow_array::{
    Array, FixedSizeListArray, Float32Array, RecordBatch, StringArray,
};
use arrow_schema::{DataType, Field, Schema};
use futures::TryStreamExt;
use lancedb::connection::Connection;
use lancedb::query::{ExecutableQuery, QueryBase};

const TABLE_NAME: &str = "screenshots";
const EMBEDDING_DIM: i32 = 384;

pub struct SearchResult {
    pub file: String,
    pub content: String,
    pub score: f32,
}

pub struct VectorStore {
    db: Connection,
}

impl VectorStore {
    pub async fn connect(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let db = lancedb::connect(path).execute().await?;
        Ok(Self { db })
    }

    pub async fn insert(
        &self,
        file: &str,
        content: &str,
        embedding: Vec<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let schema = Arc::new(Schema::new(vec![
            Field::new("file", DataType::Utf8, false),
            Field::new("content", DataType::Utf8, false),
            Field::new(
                "vector",
                DataType::FixedSizeList(
                    Arc::new(Field::new("item", DataType::Float32, true)),
                    EMBEDDING_DIM,
                ),
                false,
            ),
        ]));

        let file_array = StringArray::from(vec![file]);
        let content_array = StringArray::from(vec![content]);
        let values = Float32Array::from(embedding);
        let field = Arc::new(Field::new("item", DataType::Float32, true));
        let vector_array = FixedSizeListArray::try_new(field, EMBEDDING_DIM, Arc::new(values), None)?;

        let batch = RecordBatch::try_new(
            schema,
            vec![Arc::new(file_array), Arc::new(content_array), Arc::new(vector_array)],
        )?;

        let tables = self.db.table_names().execute().await?;
        if tables.contains(&TABLE_NAME.to_string()) {
            let table = self.db.open_table(TABLE_NAME).execute().await?;
            table.add(vec![batch]).execute().await?;
        } else {
            self.db
                .create_table(TABLE_NAME, vec![batch])
                .execute()
                .await?;
        }

        Ok(())
    }

    pub async fn search(
        &self,
        query_embedding: Vec<f32>,
        k: usize,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let table = self.db.open_table(TABLE_NAME).execute().await?;

        let results = table
            .query()
            .nearest_to(query_embedding)?
            .limit(k)
            .execute()
            .await?
            .try_collect::<Vec<_>>()
            .await?;

        let mut search_results = Vec::new();

        for batch in &results {
            if batch.num_rows() == 0 {
                continue;
            }
            let files = batch
                .column_by_name("file")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>())
                .ok_or("missing 'file' column")?;
            let contents = batch
                .column_by_name("content")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>())
                .ok_or("missing 'content' column")?;
            let distances = batch
                .column_by_name("_distance")
                .and_then(|c| c.as_any().downcast_ref::<Float32Array>())
                .ok_or("missing '_distance' column")?;

            for i in 0..batch.num_rows() {
                search_results.push(SearchResult {
                    file: files.value(i).to_string(),
                    content: contents.value(i).to_string(),
                    score: distances.value(i),
                });
            }
        }

        Ok(search_results)
    }
}
