//! Source-addressed textbook inventory and fail-closed proof lookup. No arithmetic experiments.
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn digest(path: &Path) -> Result<String> {
    Ok(hash(&fs::read(path)?))
}
fn read<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn write<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    fs::write(path, format!("{}\n", serde_json::to_string_pretty(value)?))?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Item {
    id: String,
    number: String,
    kind: String,
    chapter: u32,
    line_start: usize,
    line_end: usize,
    byte_start: usize,
    byte_end: usize,
    sha256: String,
    keywords: Vec<String>,
    bracketed_reference_candidates: Vec<u32>,
    status: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Inventory {
    schema: String,
    source_sha256: String,
    source_bytes: usize,
    source_lines: usize,
    body_end_byte: usize,
    counts: BTreeMap<String, usize>,
    items: Vec<Item>,
    coverage_note: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Rule {
    id: String,
    source: String,
    title: String,
    statement: String,
    hypotheses: Vec<String>,
    declaration: String,
    module: String,
    relation: String,
    fidelity_note: String,
    aliases: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Correction {
    source: String,
    #[serde(default = "default_correction_origin")]
    origin: String,
    issue: String,
    required_action: String,
}
fn default_correction_origin() -> String {
    "editors_errata".into()
}
#[derive(Debug, Serialize, Deserialize)]
struct Corrections {
    url: String,
    checked_on: String,
    corrections: Vec<Correction>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Receipt {
    schema: String,
    files: BTreeMap<String, String>,
    declarations: BTreeSet<String>,
    command: String,
    check_scope: String,
}

fn inventory(text: &str) -> Result<Inventory> {
    let anchor = Regex::new(
        r"(?m)^(\d+\.\d+\.\d+) (Theorem|Proposition|Lemma|Corollary|Definition|Conjecture|Remark|Algorithm|Example|Problem|Construction|Question|Observation|Fact)\b",
    )?;
    let refs = Regex::new(r"\[([0-9][0-9,; –-]*)\]")?;
    let words = Regex::new(r"[A-Za-z][A-Za-z-]{3,}")?;
    let body_end = text
        .find("\nBibliography\n")
        .ok_or("missing bibliography boundary")?
        + 1;
    // Exclude the table of contents and two subsection headings whose first
    // word is also a statement label. Plural headings are deliberately excluded.
    let body_start = text.find("2.1.1 Definition").unwrap_or(0);
    let anchors: Vec<_> = anchor
        .captures_iter(&text[..body_end])
        .filter(|c| {
            let rest = &text[c.get(0).unwrap().end()..];
            c.get(0).unwrap().start() >= body_start
                && !(&c[2] == "Construction"
                    && (rest.starts_with(" by ") || rest.starts_with(" of ")))
        })
        .collect();
    if anchors.is_empty() {
        return Err("no numbered statements found".into());
    }
    let stop: BTreeSet<_> = "that this with from have then there which over field fields finite theorem proposition lemma corollary definition remark example proof following defined where such when some given also only into each every than these been does will polynomial polynomials integer integers number elements element positive characteristic denote denote called respectively handbook suppose assume let and for the are has its all can not iff see any theorems definitions examples remarks".split_whitespace().collect();
    let mut items = Vec::new();
    let mut seen = BTreeSet::new();
    let mut counts = BTreeMap::new();
    for (i, cap) in anchors.iter().enumerate() {
        let number = cap[1].to_owned();
        if !seen.insert(number.clone()) {
            return Err(format!("duplicate statement {number}").into());
        }
        let start = cap.get(0).unwrap().start();
        let end = anchors
            .get(i + 1)
            .map_or(body_end, |c| c.get(0).unwrap().start());
        let body = &text[start..end];
        let mut frequencies = BTreeMap::<String, usize>::new();
        for w in words.find_iter(body) {
            let w = w.as_str().to_lowercase();
            if !stop.contains(w.as_str()) {
                *frequencies.entry(w).or_default() += 1;
            }
        }
        let mut ranked: Vec<_> = frequencies.into_iter().collect();
        ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let mut keywords: Vec<_> = ranked.into_iter().take(12).map(|x| x.0).collect();
        keywords.sort();
        let bracketed_reference_candidates: BTreeSet<_> = refs
            .captures_iter(body)
            .flat_map(|c| {
                c[1].split(|c: char| !c.is_ascii_digit())
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<_>>()
            })
            .collect();
        let kind = cap[2].to_owned();
        let status = if kind.starts_with("Conjecture") {
            "conjecture_unchecked_current_status"
        } else if kind.starts_with("Problem") || kind.starts_with("Question") {
            "problem_unreviewed"
        } else if kind.starts_with("Definition") || kind.starts_with("Notation") {
            "definition_unreviewed"
        } else {
            "extracted_unreviewed"
        }
        .to_owned();
        *counts.entry(kind.clone()).or_default() += 1;
        items.push(Item {
            id: format!("HFF2013:{number}"),
            chapter: number.split('.').next().unwrap().parse()?,
            number,
            kind,
            line_start: text[..start].bytes().filter(|x| *x == b'\n').count() + 1,
            line_end: text[..end].bytes().filter(|x| *x == b'\n').count(),
            byte_start: start,
            byte_end: end,
            sha256: hash(body.as_bytes()),
            keywords,
            bracketed_reference_candidates: bracketed_reference_candidates.into_iter().collect(),
            status,
        });
    }
    Ok(Inventory { schema: "finite-field-kb.inventory/v1".into(), source_sha256: hash(text.as_bytes()),
        source_bytes: text.len(), source_lines: text.lines().count(), body_end_byte: body_end, counts, items,
        coverage_note: "Numbered item spans end at the next recognized numbered item or bibliography. They may contain intervening headings/prose; they are source coordinates, not certified statement boundaries. Unnumbered prose, formulas, tables, subclaims and exercises still require a semantic coverage review. Extraction never certifies truth.".into() })
}

fn validate_rules(inv: &Inventory, rules: &[Rule], corrections: &Corrections) -> Result<()> {
    let mut ids = BTreeSet::new();
    let mut decls = BTreeSet::new();
    for rule in rules {
        if !ids.insert(&rule.id) || !decls.insert(&rule.declaration) {
            return Err("duplicate rule or declaration".into());
        }
        let item = inv
            .items
            .iter()
            .find(|i| i.id == rule.source)
            .ok_or("rule source absent from inventory")?;
        if item.kind.starts_with("Conjecture") || item.kind.starts_with("Problem") {
            return Err("cannot certify a conjecture/problem by annotation".into());
        }
        if !["equivalent", "part", "definition_consequence", "corrected"]
            .contains(&rule.relation.as_str())
        {
            return Err("unknown fidelity relation".into());
        }
        if [
            rule.title.as_str(),
            rule.statement.as_str(),
            rule.fidelity_note.as_str(),
        ]
        .iter()
        .any(|s| s.trim().is_empty())
        {
            return Err("missing semantic review".into());
        }
        if rule.hypotheses.is_empty() {
            return Err("explicit hypotheses required".into());
        }
        if corrections
            .corrections
            .iter()
            .any(|c| c.source == rule.source)
            && rule.relation != "corrected"
        {
            return Err("erratum requires an explicit corrected statement".into());
        }
        if !rule.declaration.starts_with("FiniteFieldHandbook.")
            || rule.module != "FiniteFieldHandbook"
        {
            return Err("unrecognized Lean namespace/module".into());
        }
    }
    for c in &corrections.corrections {
        if !inv.items.iter().any(|i| i.id == c.source) {
            return Err(format!("erratum source absent: {}", c.source).into());
        }
    }
    Ok(())
}

const BOUND_FILES: &[&str] = &[
    "../../tools/finite-field-kb/src/main.rs",
    "../../tools/finite-field-kb/Cargo.toml",
    "../../tools/finite-field-kb/Cargo.lock",
    "../../tools/lean/AuditAxioms.lean",
    "inventory.json",
    "source.json",
    "rules.json",
    "corrections.json",
    "lean/FiniteFieldHandbook.lean",
    "lean/PrintAxioms.lean",
    "lean/Usage.lean",
    "lean/ExportDependencies.lean",
    "lean/lean-toolchain",
    "lean/lakefile.toml",
    "lean/lake-manifest.json",
    "verify.sh",
    "receipts/build.log",
    "receipts/audit.log",
    "receipts/axioms.log",
    "receipts/usage.log",
    "receipts/dependencies.json",
    "receipts/build.exit",
    "receipts/audit.exit",
    "receipts/axioms.exit",
    "receipts/usage.exit",
    "receipts/dependencies.exit",
    "receipts/inputs.json",
];
fn input_snapshot(root: &Path) -> Result<BTreeMap<String, String>> {
    BOUND_FILES
        .iter()
        .filter(|name| !name.starts_with("receipts/"))
        .map(|name| Ok(((*name).to_owned(), digest(&root.join(name))?)))
        .collect()
}
fn prepare(root: &Path) -> Result<()> {
    let snapshot = input_snapshot(root)?;
    fs::create_dir_all(root.join("receipts"))?;
    // Prevent a new input snapshot from reusing a previous run's successful exits.
    for name in ["build", "audit", "axioms", "usage", "dependencies"] {
        let path = root.join(format!("receipts/{name}.exit"));
        if path.exists() {
            fs::remove_file(path)?;
        }
    }
    write(&root.join("receipts/inputs.json"), &snapshot)
}
fn recorded_declarations(root: &Path) -> Result<BTreeSet<String>> {
    let before: BTreeMap<String, String> = read(&root.join("receipts/inputs.json"))?;
    if before != input_snapshot(root)? {
        return Err("inputs differ from the snapshot taken before the proof checks".into());
    }
    for name in ["build", "audit", "axioms", "usage", "dependencies"] {
        if fs::read_to_string(root.join(format!("receipts/{name}.exit")))?.trim() != "0" {
            return Err(format!("{name} did not succeed").into());
        }
    }
    let audit = fs::read_to_string(root.join("receipts/audit.log"))?;
    if !audit.contains("0 failing, 0 trusted")
        || audit
            .lines()
            .any(|l| l.starts_with("FAIL") || l.starts_with("TRUST"))
    {
        return Err("audit not clean".into());
    }
    let declarations: BTreeSet<_> = audit
        .lines()
        .filter_map(|line| {
            let mut p = line.split_whitespace();
            if p.next()? == "ok" {
                p.next().map(str::to_owned)
            } else {
                None
            }
        })
        .collect();
    if declarations.is_empty() {
        return Err("empty audit".into());
    }
    let statements = fs::read_to_string(root.join("receipts/axioms.log"))?;
    for declaration in &declarations {
        if !statements.contains(&format!("'{declaration}'"))
            || !statements.contains(&format!("{declaration} :"))
        {
            return Err(format!("missing explicit statement/axioms for {declaration}").into());
        }
    }
    Ok(declarations)
}
fn seal(root: &Path) -> Result<Receipt> {
    let declarations = recorded_declarations(root)?;
    let rules: Vec<Rule> = read(&root.join("rules.json"))?;
    for r in &rules {
        if !declarations.contains(&r.declaration) {
            return Err(format!("not audited: {}", r.declaration).into());
        }
    }
    let mut files = BTreeMap::new();
    for name in BOUND_FILES {
        files.insert((*name).into(), digest(&root.join(name))?);
    }
    Ok(Receipt { schema: "finite-field-kb.proof-receipt/v1".into(), files, declarations,
        command: "knowledge/finite-fields/verify.sh".into(),
        check_scope: "Recorded local Lean build, transitive axiom audit and explicit #print axioms; file hashes prevent stale lookup. This is not an independent semantic review or a signature authenticating a remote producer.".into() })
}
fn verified(root: &Path) -> Result<BTreeSet<String>> {
    let receipt: Receipt = read(&root.join("receipts/verification.json"))?;
    if receipt.schema != "finite-field-kb.proof-receipt/v1" {
        return Err("unsupported receipt".into());
    }
    let expected: BTreeSet<_> = BOUND_FILES.iter().map(|s| s.to_string()).collect();
    if receipt.files.keys().cloned().collect::<BTreeSet<_>>() != expected {
        return Err("receipt file set mismatch".into());
    }
    for name in BOUND_FILES {
        if receipt.files[*name] != digest(&root.join(name))? {
            return Err(format!("stale proof receipt: {name}").into());
        }
    }
    if receipt.declarations != recorded_declarations(root)? {
        return Err("receipt declarations differ from audit".into());
    }
    Ok(receipt.declarations)
}
fn run(args: &[String]) -> Result<()> {
    let command = args.first().map(String::as_str).unwrap_or("help");
    if command == "help" {
        println!("finite-field-kb ingest SOURCE ROOT | check ROOT [SOURCE] | search ROOT QUERY [--all] | show ROOT ID [--source SOURCE] | cite ROOT RULE_ID | stats ROOT | prepare ROOT | seal ROOT");
        return Ok(());
    }
    if command == "ingest" {
        let source = args.get(1).ok_or("SOURCE required")?;
        let root = Path::new(args.get(2).ok_or("ROOT required")?);
        let text = fs::read_to_string(source)?;
        let inv = inventory(&text)?;
        fs::create_dir_all(root)?;
        write(&root.join("inventory.json"), &inv)?;
        println!(
            "{} numbered items; {} lines; sha256 {}",
            inv.items.len(),
            inv.source_lines,
            inv.source_sha256
        );
        return Ok(());
    }
    let root = PathBuf::from(args.get(1).ok_or("ROOT required")?);
    let inv: Inventory = read(&root.join("inventory.json"))?;
    let rules: Vec<Rule> = read(&root.join("rules.json"))?;
    let corrections: Corrections = read(&root.join("corrections.json"))?;
    validate_rules(&inv, &rules, &corrections)?;
    if command == "prepare" {
        prepare(&root)?;
        println!("Input snapshot recorded; prior exit statuses cleared");
        return Ok(());
    }
    if command == "stats" {
        let source_items = rules.iter().map(|r| &r.source).collect::<BTreeSet<_>>();
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "extracted_items": inv.items.len(), "counts": inv.counts,
                "mapped_rules": rules.len(), "distinct_source_items_with_mappings": source_items.len(),
                "semantic_review": "implementing-agent review recorded in each fidelity note; independent review pending",
                "whole_book_status": "partial; unnumbered claims, tables, exercises and full item fidelity review remain open",
                "receipt": verified(&root).map(|s| format!("{} declarations; current hashes",s.len())).unwrap_or_else(|e|format!("unavailable: {e}"))
            }))?
        );
        return Ok(());
    }
    if command == "cite" {
        let id = args.get(2).ok_or("RULE_ID required")?;
        let rule = rules
            .iter()
            .find(|r| r.id == *id)
            .ok_or("unknown rule ID; use search first")?;
        if !verified(&root)?.contains(&rule.declaration) {
            return Err("rule not checked".into());
        }
        let item = inv
            .items
            .iter()
            .find(|i| i.id == rule.source)
            .ok_or("missing source item")?;
        println!("{} This follows from {} (Mullen–Panario, Handbook of Finite Fields, 2013), Lean `{}`. Assumptions: {} Source text lines {}–{}, source SHA-256 {}. Scope: {} {} Local kernel and axiom checks are recorded; independent source-fidelity review is pending.", rule.statement, rule.source, rule.declaration, rule.hypotheses.join("; "), item.line_start, item.line_end, inv.source_sha256, rule.relation, rule.fidelity_note);
        return Ok(());
    }
    if command == "seal" {
        let receipt = seal(&root)?;
        write(&root.join("receipts/verification.json"), &receipt)?;
        println!("Sealed {} audited declarations", receipt.declarations.len());
        return Ok(());
    }
    if command == "check" {
        if let Some(source) = args.get(2) {
            let text = fs::read_to_string(source)?;
            let regenerated = inventory(&text)?;
            if serde_json::to_value(&inv)? != serde_json::to_value(regenerated)? {
                return Err("inventory differs from source regeneration".into());
            }
        }
        let checked = verified(&root)?;
        for rule in &rules {
            if !checked.contains(&rule.declaration) {
                return Err("rule lacks proof receipt".into());
            }
        }
        println!("{} extracted items; {} mapped rules; {} audited declarations; {} errata records; current hashes OK", inv.items.len(), rules.len(), checked.len(), corrections.corrections.len());
        return Ok(());
    }
    if command == "search" {
        let query = args.get(2).ok_or("QUERY required")?.to_lowercase();
        let all = args.iter().any(|x| x == "--all");
        let checked = if all {
            BTreeSet::new()
        } else {
            verified(&root)?
        };
        let terms: Vec<_> = query.split_whitespace().collect();
        let mut found = 0;
        for rule in &rules {
            let hay = format!(
                "{} {} {} {} {} {}",
                rule.id,
                rule.source,
                rule.title,
                rule.statement,
                rule.hypotheses.join(" "),
                rule.aliases.join(" ")
            )
            .to_lowercase();
            if terms.iter().all(|t| hay.contains(t)) && (all || checked.contains(&rule.declaration))
            {
                println!(
                    "{} | {} | {} | {} | {}",
                    rule.id,
                    rule.source,
                    rule.title,
                    rule.declaration,
                    if all {
                        "proof status not checked (--all)"
                    } else {
                        "recorded kernel check; see hypotheses and fidelity"
                    }
                );
                found += 1;
            }
        }
        if all {
            for item in &inv.items {
                let hay =
                    format!("{} {} {}", item.id, item.kind, item.keywords.join(" ")).to_lowercase();
                if terms.iter().all(|t| hay.contains(t)) {
                    println!(
                        "{} | {} | {} | lines {}-{} | {}",
                        item.id,
                        item.kind,
                        item.status,
                        item.line_start,
                        item.line_end,
                        item.keywords.join(", ")
                    );
                    found += 1;
                }
            }
        }
        println!("{found} matches");
        return Ok(());
    }
    if command == "show" {
        let id = args.get(2).ok_or("ID required")?;
        let rule = rules.iter().find(|r| r.id == *id);
        let source_id = rule.map_or(id.as_str(), |r| r.source.as_str());
        let item = inv
            .items
            .iter()
            .find(|i| i.id == source_id || i.number == source_id)
            .ok_or("unknown ID")?;
        println!("{}", serde_json::to_string_pretty(item)?);
        for r in rules.iter().filter(|r| r.source == item.id) {
            println!("{}", serde_json::to_string_pretty(r)?);
            match verified(&root) {
                Ok(checked) if checked.contains(&r.declaration) => {
                    println!("proof: recorded kernel check, hashes current")
                }
                Ok(_) => println!("proof: unavailable"),
                Err(e) => println!("proof: unavailable ({e})"),
            }
            if let Ok(statements) = fs::read_to_string(root.join("receipts/axioms.log")) {
                let start_marker = format!("{} :", r.declaration);
                if let Some(start) = statements.find(&start_marker) {
                    let rest = &statements[start..];
                    let end = rest.find("\nFiniteFieldHandbook.").unwrap_or(rest.len());
                    println!(
                        "Recorded Lean statement and axioms (consult proof status above):\n{}",
                        &rest[..end]
                    );
                }
            }
        }
        for c in corrections
            .corrections
            .iter()
            .filter(|c| c.source == item.id)
        {
            println!(
                "CORRECTION/CLARIFICATION ({}): {}. Required: {}. Editor reference: {}",
                c.origin, c.issue, c.required_action, corrections.url
            );
        }
        if let Some(i) = args.iter().position(|s| s == "--source") {
            let path = args.get(i + 1).ok_or("--source requires a path")?;
            let text = fs::read_to_string(path)?;
            if hash(text.as_bytes()) != inv.source_sha256 {
                return Err("source hash mismatch".into());
            }
            let body = text.get(item.byte_start..item.byte_end).ok_or("bad span")?;
            if hash(body.as_bytes()) != item.sha256 {
                return Err("span hash mismatch".into());
            }
            println!("{body}");
        }
        return Ok(());
    }
    Err(format!("unknown command {command}").into())
}
fn main() {
    if let Err(e) = run(&env::args().skip(1).collect::<Vec<_>>()) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn spans_and_conjectures() {
        let text = "Contents\n2.1.1 Basic definitions\n2.1.1 Definition Café α.\n2.1.2 Theorem [12, 34] Frobenius.\n2.1.3 Conjecture Unknown.\nBibliography\n2.1.4 Theorem Not body.\n";
        let i = inventory(text).unwrap();
        assert_eq!(i.items.len(), 3);
        assert_eq!(i.items[0].line_start, 3);
        assert_eq!(i.items[1].bracketed_reference_candidates, vec![12, 34]);
        assert!(i.items[2].status.starts_with("conjecture"));
        for item in &i.items {
            assert_eq!(
                hash(&text.as_bytes()[item.byte_start..item.byte_end]),
                item.sha256
            );
        }
    }
    #[test]
    fn duplicates_fail() {
        assert!(inventory("2.1.1 Lemma A\n2.1.1 Theorem B\nBibliography\n").is_err());
    }
    #[test]
    fn missing_boundary_fails() {
        assert!(inventory("2.1.1 Theorem A").is_err());
    }
    #[test]
    fn mathematical_ellipses_are_not_contents() {
        let i = inventory("2.1.1 Theorem Let a1, . . . , an satisfy P.\nBibliography\n").unwrap();
        assert_eq!(i.items.len(), 1);
    }
    #[test]
    fn construction_headings_are_not_statements() {
        let i = inventory("3.2.1 Construction by composition\n3.2.1 Theorem P\n3.2.2 Construction (Paley) Q\nBibliography\n").unwrap();
        assert_eq!(i.items.len(), 2);
    }
    #[test]
    fn promotion_fails_without_receipts() {
        assert!(verified(Path::new("/no-such-proof-receipt")).is_err());
    }
    fn with_receipt_fixture(test: impl FnOnce(&Path)) {
        static NEXT_FIXTURE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let sequence = NEXT_FIXTURE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let base = env::temp_dir().join(format!(
            "finite-field-kb-test-{}-{unique}-{sequence}",
            std::process::id()
        ));
        let root = base.join("knowledge/finite-fields");
        fs::create_dir_all(&root).unwrap();
        for file in BOUND_FILES {
            let path = root.join(file);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(
                path,
                if file.ends_with(".exit") {
                    "0\n"
                } else {
                    "fixture\n"
                },
            )
            .unwrap();
        }
        fs::write(root.join("rules.json"), "[]\n").unwrap();
        fs::write(root.join("receipts/audit.log"), "ok    FiniteFieldHandbook.test  []\naudited 1 theorems in [FiniteFieldHandbook]; 0 failing, 0 trusted (native_decide)\n").unwrap();
        fs::write(root.join("receipts/axioms.log"), "FiniteFieldHandbook.test : True\n'FiniteFieldHandbook.test' does not depend on any axioms\n").unwrap();
        write(
            &root.join("receipts/inputs.json"),
            &input_snapshot(&root).unwrap(),
        )
        .unwrap();
        let receipt = seal(&root).unwrap();
        write(&root.join("receipts/verification.json"), &receipt).unwrap();
        test(&root);
        fs::remove_dir_all(base).unwrap();
    }
    #[test]
    fn changed_proof_source_invalidates_lookup() {
        with_receipt_fixture(|root| {
            assert!(verified(root).is_ok());
            fs::write(root.join("lean/FiniteFieldHandbook.lean"), "changed\n").unwrap();
            assert!(verified(root)
                .unwrap_err()
                .to_string()
                .contains("stale proof receipt"));
            assert!(seal(root).is_err());
        });
    }
    #[test]
    fn prepare_cannot_reuse_previous_success_statuses() {
        with_receipt_fixture(|root| {
            prepare(root).unwrap();
            assert!(seal(root).is_err());
        });
    }
    #[test]
    fn changed_rule_hypotheses_invalidate_lookup() {
        with_receipt_fixture(|root| {
            fs::write(root.join("rules.json"), "[{}]\n").unwrap();
            assert!(verified(root).is_err());
        });
    }
    #[test]
    fn missing_dependency_pin_invalidates_lookup() {
        with_receipt_fixture(|root| {
            fs::remove_file(root.join("lean/lake-manifest.json")).unwrap();
            assert!(verified(root).is_err());
        });
    }
    #[test]
    fn arbitrary_receipt_path_is_rejected() {
        with_receipt_fixture(|root| {
            let mut receipt: Receipt = read(&root.join("receipts/verification.json")).unwrap();
            receipt.files.insert("/etc/passwd".into(), "unused".into());
            write(&root.join("receipts/verification.json"), &receipt).unwrap();
            assert!(verified(root)
                .unwrap_err()
                .to_string()
                .contains("file set mismatch"));
        });
    }
    #[test]
    fn failed_build_cannot_be_sealed() {
        with_receipt_fixture(|root| {
            fs::write(root.join("receipts/build.exit"), "1\n").unwrap();
            assert!(seal(root).is_err());
        });
    }
    #[test]
    fn axioms_must_be_printed_for_every_declaration() {
        with_receipt_fixture(|root| {
            fs::write(root.join("receipts/axioms.log"), "").unwrap();
            assert!(seal(root).is_err());
        });
    }
    #[test]
    fn conjecture_cannot_be_promoted() {
        let i = inventory("2.1.1 Conjecture C\nBibliography\n").unwrap();
        let rule = Rule {
            id: "c".into(),
            source: "HFF2013:2.1.1".into(),
            title: "C".into(),
            statement: "C".into(),
            hypotheses: vec!["H".into()],
            declaration: "FiniteFieldHandbook.c".into(),
            module: "FiniteFieldHandbook".into(),
            relation: "equivalent".into(),
            fidelity_note: "read".into(),
            aliases: vec![],
        };
        assert!(validate_rules(
            &i,
            &[rule],
            &Corrections {
                url: "u".into(),
                checked_on: "d".into(),
                corrections: vec![]
            }
        )
        .is_err());
    }
}
