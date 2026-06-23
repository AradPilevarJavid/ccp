use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let templates_dir = manifest_dir.join("templates");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let output_path = out_dir.join("builtin_templates.rs");

    println!("cargo:rerun-if-changed={}", templates_dir.display());

    let mut templates = Vec::new();
    if templates_dir.is_dir() {
        for entry in fs::read_dir(&templates_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|extension| extension.to_str()) != Some("tree") {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
                continue;
            };
            templates.push((stem.to_string(), path));
        }
    }

    templates.sort_by(|left, right| left.0.cmp(&right.0));

    let mut generated = String::from("const BUILTIN_TEMPLATES: &[(&str, &str)] = &[\n");
    for (name, path) in templates {
        generated.push_str(&format!(
            "    ({name:?}, include_str!({path:?})),\n",
            name = name,
            path = path.display().to_string()
        ));
    }
    generated.push_str("];\n");

    fs::write(output_path, generated)
}
