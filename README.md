# reportslib

Rust library for generating **self-contained** static HTML reports.

Each report is a single `.html` file with inline CSS, inline JavaScript, and
images encoded as `data:` base64 URIs. Library markup does not reference
external files.

```toml
reportslib = { path = "../reportslib" }
```

```rust
use reportslib::Report;

Report::new("Session summary")
    .meta("Generated", "2026-10-18")
    .heading(2, "Overview")
    .paragraph("Hits recorded during the session.")
    .table(["Device", "Hits"], [["sensor-01", "12"]])
    .image_png(png_bytes, "overlay")
    .write_to("report.html")?;
```

See `examples/basic.rs`.
