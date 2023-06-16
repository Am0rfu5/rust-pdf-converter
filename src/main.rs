use lopdf::{Document, Object, Stream};
use std::path::Path;
use std::io::{self, Write};
use leptess::LepTess;
use std::fs::File;


/**
 * Extracts images from a PDF file and saves them to disk.
 */
fn extract_images_from_pdf(pdf_path: &str) -> Vec<String> {
    let relative_path = Path::new(pdf_path);
    let doc = Document::load(pdf_path).unwrap();
    let mut image_paths = Vec::new();

     for page_id in doc.page_iter() {
         let page = doc.get_page_content(page_id).unwrap();
        for &xobject_id in page.resources.xobject_id_iter() {
            if let Ok(Object::Stream(Stream { data, .. })) = doc.get_object(xobject_id) {
                // Assume images are in JPEG format; you may need to adjust based on your PDF
                let image_path = format!("image_{}.jpg", xobject_id.0);
                let mut file = File::create(&image_path).unwrap();
                file.write_all(&data).unwrap();
                image_paths.push(image_path);
            }
        }
    }

    image_paths
}

/**
 * Performs OCR on a list of images and saves the results to disk.
 */
fn perform_ocr_on_images(image_paths: Vec<String>) {
    let mut lt = LepTess::new(None, "eng").unwrap();

    for image_path in image_paths {
        lt.set_image_file(&image_path).unwrap();
        let text = lt.get_utf8_text().unwrap();
        let text_file_path = format!("{}.txt", image_path);
        std::fs::write(text_file_path, text).unwrap();
    }
}

fn main() {
    print!("What file would you like to parse? ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let pdf_path = input.trim();
    let image_paths = extract_images_from_pdf(pdf_path);
}
