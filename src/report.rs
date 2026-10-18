use std::fs;
use std::io;
use std::path::Path;

use crate::asset::Mime;
use crate::html;

/// A content block in a report.
#[derive(Clone, Debug)]
pub enum Block {
    Heading { level: u8, text: String },
    Paragraph(String),
    /// Trusted HTML fragment; inserted without escaping.
    Html(String),
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    List { ordered: bool, items: Vec<String> },
    Image {
        mime: String,
        data: Vec<u8>,
        alt: String,
    },
}

/// Builder for a self-contained HTML report.
#[derive(Clone, Debug)]
pub struct Report {
    pub(crate) title: String,
    pub(crate) meta: Vec<(String, String)>,
    pub(crate) blocks: Vec<Block>,
    pub(crate) styles: Vec<String>,
    pub(crate) scripts: Vec<String>,
}

impl Report {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            meta: Vec::new(),
            blocks: Vec::new(),
            styles: Vec::new(),
            scripts: Vec::new(),
        }
    }

    pub fn meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.meta.push((key.into(), value.into()));
        self
    }

    pub fn heading(mut self, level: u8, text: impl Into<String>) -> Self {
        self.blocks.push(Block::Heading {
            level: level.clamp(1, 6),
            text: text.into(),
        });
        self
    }

    pub fn paragraph(mut self, text: impl Into<String>) -> Self {
        self.blocks.push(Block::Paragraph(text.into()));
        self
    }

    /// Append a trusted HTML fragment (not escaped).
    pub fn html(mut self, raw: impl Into<String>) -> Self {
        self.blocks.push(Block::Html(raw.into()));
        self
    }

    pub fn table<H, R, C>(mut self, headers: H, rows: R) -> Self
    where
        H: IntoIterator,
        H::Item: Into<String>,
        R: IntoIterator,
        R::Item: IntoIterator<Item = C>,
        C: Into<String>,
    {
        let headers: Vec<String> = headers.into_iter().map(Into::into).collect();
        let rows: Vec<Vec<String>> = rows
            .into_iter()
            .map(|row| row.into_iter().map(Into::into).collect())
            .collect();
        self.blocks.push(Block::Table { headers, rows });
        self
    }

    pub fn list<I, S>(mut self, ordered: bool, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.blocks.push(Block::List {
            ordered,
            items: items.into_iter().map(Into::into).collect(),
        });
        self
    }

    pub fn image(mut self, bytes: impl Into<Vec<u8>>, mime: &str, alt: impl Into<String>) -> Self {
        self.blocks.push(Block::Image {
            mime: mime.to_string(),
            data: bytes.into(),
            alt: alt.into(),
        });
        self
    }

    pub fn image_png(self, bytes: impl Into<Vec<u8>>, alt: impl Into<String>) -> Self {
        self.image(bytes, Mime::Png.as_str(), alt)
    }

    pub fn image_jpeg(self, bytes: impl Into<Vec<u8>>, alt: impl Into<String>) -> Self {
        self.image(bytes, Mime::Jpeg.as_str(), alt)
    }

    /// Extra CSS appended after the default theme.
    pub fn style(mut self, css: impl Into<String>) -> Self {
        self.styles.push(css.into());
        self
    }

    /// Extra JavaScript appended into an inline `<script>` at the end of `<body>`.
    pub fn script(mut self, js: impl Into<String>) -> Self {
        self.scripts.push(js.into());
        self
    }

    /// Render a complete self-contained HTML document.
    pub fn render(&self) -> String {
        html::render(self)
    }

    /// Write the report to `path` as UTF-8 HTML.
    pub fn write_to(&self, path: impl AsRef<Path>) -> io::Result<()> {
        fs::write(path, self.render())
    }
}
