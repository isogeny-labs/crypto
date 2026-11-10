//! Import the mathematical records and straight-line programs, not web prose.
//! Network retrieval uses curl; parsing, hashing and generation are native Rust.
//!
//! Two EFD sections are supported. `g12o` is the characteristic-two section
//! imported by `binary_ecc::efd`; `g1p` is the genus-1 large-characteristic
//! section imported by `prime_ecc::efd`. The record/program layout is shared,
//! so one importer keeps both catalogues byte-reproducible.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Copy)]
enum Characteristic {
    Two,
    Prime,
}

struct Section {
    name: &'static str,
    base: &'static str,
    representations: usize,
    formulas: usize,
    schema: &'static str,
    characteristic: Characteristic,
}

static BINARY: Section = Section {
    name: "g12o",
    base: "https://www.hyperelliptic.org/EFD/g12o/",
    representations: 19,
    formulas: 135,
    schema: "efd-binary-catalogue/v1",
    characteristic: Characteristic::Two,
};

static PRIME: Section = Section {
    name: "g1p",
    base: "https://www.hyperelliptic.org/EFD/g1p/",
    representations: 33,
    formulas: 471,
    schema: "efd-prime-catalogue/v1",
    characteristic: Characteristic::Prime,
};

impl Section {
    fn parse(name: &str) -> Result<&'static Self, String> {
        match name {
            "g12o" => Ok(&BINARY),
            "g1p" => Ok(&PRIME),
            other => Err(format!("unknown EFD section {other}; expected g12o or g1p")),
        }
    }
}

fn links(s: &str) -> BTreeSet<String> {
    s.split("href=\"")
        .skip(1)
        .filter_map(|x| x.split('"').next())
        .map(str::to_owned)
        .collect()
}

fn fetch(section: &Section, cache: &Path, path: &str) -> Result<String, String> {
    if path.contains("..") || path.starts_with('/') || path.contains(':') {
        return Err(format!("unsafe source path: {path}"));
    }
    let dest = cache.join(path);
    if let Ok(s) = fs::read_to_string(&dest) {
        return Ok(s);
    }
    let url = format!("{}{path}", section.base);
    let result = Command::new("curl")
        .args([
            "--fail",
            "--silent",
            "--show-error",
            "--location",
            "--max-time",
            "45",
            "--retry",
            "2",
            &url,
        ])
        .output()
        .map_err(|e| e.to_string())?;
    if !result.status.success() {
        return Err(format!(
            "fetch {url}: {}",
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    let s = String::from_utf8(result.stdout).map_err(|e| e.to_string())?;
    fs::create_dir_all(dest.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(dest, &s).map_err(|e| e.to_string())?;
    Ok(s)
}

fn source(section: &Section, path: &str, text: String) -> Value {
    json!({"url":format!("{}{path}", section.base), "sha256":format!("{:x}",Sha256::digest(text.as_bytes())), "text":text})
}

fn values(s: &str, prefix: &str) -> Vec<String> {
    s.lines()
        .filter_map(|l| l.strip_prefix(prefix).map(str::to_owned))
        .collect()
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 4 {
        return Err("usage: efd-import CACHE_DIRECTORY OUTPUT_JSON [g12o|g1p]".into());
    }
    let section = Section::parse(args.get(3).map(String::as_str).unwrap_or("g12o"))?;
    let cache = PathBuf::from(&args[1]);
    let index = fetch(section, &cache, "index.html")?;
    let pages: Vec<_> = links(&index)
        .into_iter()
        .filter(|p| p.starts_with("auto-") && p.ends_with(".html") && p.matches('-').count() >= 2)
        .collect();
    if pages.len() != section.representations {
        return Err(format!(
            "expected {} representations, found {}",
            section.representations,
            pages.len()
        ));
    }
    let mut representations = Vec::new();
    let mut paths = BTreeSet::new();
    let mut models = BTreeSet::new();
    for page in pages {
        let html = fetch(section, &cache, &page)?;
        let ls = links(&html);
        let var_path = ls
            .iter()
            .find(|s| s.starts_with("data/") && s.ends_with("/variables"))
            .ok_or_else(|| format!("no variables for {page}"))?;
        let text = fetch(section, &cache, var_path)?;
        let id = var_path
            .trim_start_matches("data/")
            .trim_end_matches("/variables");
        models.insert(id.split('/').next().unwrap().to_owned());
        paths.extend(
            ls.iter()
                .filter(|s| s.starts_with("auto-code/") && s.ends_with(".op3"))
                .cloned(),
        );
        representations.push(json!({"id":id,"page_url":format!("{}{page}", section.base),
            "variables":values(&text,"variable "),"relations":values(&text,"satisfying "),
            "assumptions":values(&text,"assume "),"source":source(section,var_path,text)}));
    }
    let mut model_records = Vec::new();
    for model in models {
        let page = format!("auto-{model}.html");
        let html = fetch(section, &cache, &page)?;
        let ls = links(&html);
        let path = ls
            .iter()
            .find(|s| s.starts_with("data/") && s.ends_with("/coordinates"))
            .ok_or_else(|| format!("no model coordinates for {model}"))?;
        let text = fetch(section, &cache, path)?;
        model_records.push(json!({"id":model,"source":source(section,path,text)}));
    }
    let paths: Vec<_> = paths.into_iter().collect();
    let results = std::thread::scope(|scope| {
        let handles:Vec<_>=paths.chunks(paths.len().div_ceil(4)).map(|chunk|{
            let cache=&cache;
            scope.spawn(move|| -> Result<Vec<Value>,String> {
                let mut formulas=Vec::new();
                for op_path in chunk {
                    let id=op_path.trim_start_matches("auto-code/").trim_end_matches(".op3");
                    let data_path=format!("data/{id}");
                    let record=fetch(section,cache,&data_path)?;
                    let program=fetch(section,cache,op_path)?;
                    let parts:Vec<_>=id.split('/').collect();
                    if parts.len()!=4 { return Err(format!("invalid formula id {id}")); }
                    formulas.push(json!({"id":id,"representation":format!("{}/{}",parts[0],parts[1]),
                        "operation":parts[2],"name":parts[3],"assumptions":values(&record,"assume "),
                        "record":source(section,&data_path,record),"program":source(section,op_path,program),
                        "upstream_verification_url":format!("{}auto-sage/{id}.sage", section.base)}));
                }
                Ok(formulas)
            })
        }).collect();
        handles
            .into_iter()
            .map(|h| h.join().map_err(|_| "import worker panicked".to_owned())?)
            .collect::<Result<Vec<_>, String>>()
    })?;
    let mut formulas: Vec<_> = results.into_iter().flatten().collect();
    formulas.sort_by(|a, b| a["id"].as_str().cmp(&b["id"].as_str()));
    if formulas.len() != section.formulas {
        return Err(format!(
            "expected {} formulas, found {}",
            section.formulas,
            formulas.len()
        ));
    }
    let date = Command::new("date")
        .args(["-u", "+%F"])
        .output()
        .map_err(|e| e.to_string())?;
    if !date.status.success() {
        return Err("could not record retrieval date".into());
    }
    let retrieved = String::from_utf8(date.stdout).map_err(|e| e.to_string())?;
    let characteristic = match section.characteristic {
        Characteristic::Two => json!(2),
        Characteristic::Prime => json!("p"),
    };
    let out = json!({"schema":section.schema,
        "source_url":format!("{}index.html", section.base),
        "retrieved":retrieved.trim(),"characteristic":characteristic,
        "models":model_records,"representations":representations,"formulas":formulas});
    let dest = PathBuf::from(&args[2]);
    fs::create_dir_all(dest.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(
        &dest,
        format!("{}\n", serde_json::to_string_pretty(&out).unwrap()),
    )
    .map_err(|e| e.to_string())?;
    eprintln!(
        "imported {} formulas in {} representations to {}",
        out["formulas"].as_array().unwrap().len(),
        out["representations"].as_array().unwrap().len(),
        dest.display()
    );
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
