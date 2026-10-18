use crate::asset::data_uri;
use crate::report::{Block, Report};

pub(crate) fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

const DEFAULT_CSS: &str = r#"
:root {
  --bg: #1b1b1b;
  --panel: #242424;
  --fg: #efebdc;
  --muted: #817f68;
  --accent: #e75a50;
  --border: #3a3a3a;
}
* { box-sizing: border-box; }
body {
  margin: 0;
  font-family: ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, sans-serif;
  background: var(--bg);
  color: var(--fg);
  line-height: 1.5;
}
.report {
  max-width: 56rem;
  margin: 0 auto;
  padding: 1.5rem 1.25rem 3rem;
}
header.page {
  border-bottom: 1px solid var(--border);
  padding-bottom: 1rem;
  margin-bottom: 1.5rem;
}
header.page h1 {
  margin: 0 0 0.5rem;
  font-size: 1.6rem;
}
.meta {
  display: flex;
  flex-wrap: wrap;
  gap: 0.35rem 1.25rem;
  color: var(--muted);
  font-size: 0.9rem;
}
.meta dt { font-weight: 600; color: var(--accent); display: inline; }
.meta dd { display: inline; margin: 0; }
h1, h2, h3, h4, h5, h6 { line-height: 1.25; margin: 1.4rem 0 0.6rem; }
p { margin: 0.6rem 0; }
table {
  width: 100%;
  border-collapse: collapse;
  margin: 0.75rem 0 1.25rem;
  font-size: 0.95rem;
}
th, td {
  border: 1px solid var(--border);
  padding: 0.45rem 0.65rem;
  text-align: left;
  vertical-align: top;
}
th { background: var(--panel); }
tr:nth-child(even) td { background: #1f1f1f; }
img.report-img {
  display: block;
  max-width: 100%;
  height: auto;
  margin: 0.75rem 0 1.25rem;
  border: 1px solid var(--border);
  border-radius: 6px;
}
ul, ol { margin: 0.5rem 0 1rem; padding-left: 1.4rem; }
li { margin: 0.25rem 0; }
.block-html { margin: 0.75rem 0 1.25rem; }
"#;

pub(crate) fn render(report: &Report) -> String {
    let title = escape(&report.title);
    let mut meta_html = String::new();
    if !report.meta.is_empty() {
        meta_html.push_str("<dl class=\"meta\">\n");
        for (k, v) in &report.meta {
            meta_html.push_str(&format!(
                "  <div><dt>{}</dt> <dd>{}</dd></div>\n",
                escape(k),
                escape(v)
            ));
        }
        meta_html.push_str("</dl>\n");
    }

    let mut body = String::new();
    for block in &report.blocks {
        body.push_str(&render_block(block));
    }

    let mut styles = String::from(DEFAULT_CSS);
    for css in &report.styles {
        styles.push('\n');
        styles.push_str(css);
    }

    let mut scripts = String::new();
    for js in &report.scripts {
        if !scripts.is_empty() {
            scripts.push('\n');
        }
        scripts.push_str(js);
    }

    let script_tag = if scripts.is_empty() {
        String::new()
    } else {
        format!("<script>\n{scripts}\n</script>\n")
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8"/>
<meta name="viewport" content="width=device-width, initial-scale=1"/>
<title>{title}</title>
<style>
{styles}
</style>
</head>
<body>
<article class="report">
<header class="page">
  <h1>{title}</h1>
  {meta_html}</header>
{body}</article>
{script_tag}</body>
</html>
"#
    )
}

fn render_block(block: &Block) -> String {
    match block {
        Block::Heading { level, text } => {
            let level = (*level).clamp(1, 6);
            format!("<h{level}>{}</h{level}>\n", escape(text))
        }
        Block::Paragraph(text) => format!("<p>{}</p>\n", escape(text)),
        Block::Html(raw) => format!("<div class=\"block-html\">{raw}</div>\n"),
        Block::Table { headers, rows } => {
            let mut s = String::from("<table>\n<thead><tr>");
            for h in headers {
                s.push_str(&format!("<th>{}</th>", escape(h)));
            }
            s.push_str("</tr></thead>\n<tbody>\n");
            for row in rows {
                s.push_str("<tr>");
                for cell in row {
                    s.push_str(&format!("<td>{}</td>", escape(cell)));
                }
                s.push_str("</tr>\n");
            }
            s.push_str("</tbody>\n</table>\n");
            s
        }
        Block::List { ordered, items } => {
            let tag = if *ordered { "ol" } else { "ul" };
            let mut s = format!("<{tag}>\n");
            for item in items {
                s.push_str(&format!("  <li>{}</li>\n", escape(item)));
            }
            s.push_str(&format!("</{tag}>\n"));
            s
        }
        Block::Image { mime, data, alt } => {
            let src = data_uri(mime, data);
            format!(
                "<img class=\"report-img\" src=\"{src}\" alt=\"{}\"/>\n",
                escape(alt)
            )
        }
    }
}
