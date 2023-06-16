#[cfg(test)]

mod test_document {

    use lopdf::Document;
    use std::path::Path;
    use std::io::Write;
    use std::fs;
    use serde::{Serialize, Deserialize};

    struct PDFDocument {
        name: String,
        path: String,
        pages: Vec<Page>,
    }

    struct Database {
        documents: Vec<Document>,
    }

    fn save_to_file(&self) -> std::io::Result<()> {
        let data = serde_json::to_string(&self)?;
        let mut file: fs::File = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }   

    fn load_from_file() -> std::io::Result<Self> {
        let file_content = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }


}