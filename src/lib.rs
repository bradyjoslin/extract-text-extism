use extism_pdk::*;

use lol_html::{element, HtmlRewriter, Settings};


fn scraper(content: String) -> Result<String, anyhow::Error> {
    let mut output = vec![];

    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                element!("*", |el| {
                    Ok(el.remove_and_keep_content())
                })
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c)
    );

    rewriter.write(content.as_bytes())?;
    rewriter.end()?;

    let results = String::from_utf8(output)?;
    info!("{}", results);
    Ok(results.to_string())
}


#[plugin_fn]
pub fn extract(input: String) -> FnResult<String> {
    let text = scraper(input)?;
    Ok(text)
}
