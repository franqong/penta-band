use pulldown_cmark::{Options, Parser, html};
use ropey::Rope;

pub struct LyricsDocument {
    source: Rope,
}

impl LyricsDocument {
    pub fn new(markdown: &str) -> Self {
        Self {
            source: Rope::from_str(markdown),
        }
    }

    pub fn markdown(&self) -> String {
        self.source.to_string()
    }

    pub fn render_html(&self) -> String {
        let mut output = String::new();
        let options = Options::all();
        html::push_html(&mut output, Parser::new_ext(&self.markdown(), options));
        output
    }
}
