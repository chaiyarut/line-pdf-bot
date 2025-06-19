use lopdf::Document;
use std::error::Error;

pub fn remove_page_2(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let mut doc = Document::load(input_path)?;
    let pages = doc.get_pages();
    let page_numbers: Vec<u32> = pages.keys().cloned().collect();

    let mut new_doc = Document::with_version("1.5");
    for (i, page_no) in page_numbers.iter().enumerate() {
        if i == 1 { continue; }
        let page_id = pages[page_no];
        new_doc.add_object(*doc.get_object(page_id)?);
    }

    new_doc.compress();
    new_doc.save(output_path)?;
    Ok(())
}
