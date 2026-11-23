//! Evidence-bound, observation-only report. No solver or price implementation.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

pub type Result<T> = std::result::Result<T, String>;
pub fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn text<'a>(v: &'a Value, key: &str) -> Result<&'a str> {
    v.get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| format!("missing string {key}"))
}
fn bound(v: &Value) -> Result<Value> {
    let bytes = fs::read(text(v, "path")?).map_err(|e| e.to_string())?;
    if hash(&bytes) != text(v, "sha256")? {
        return Err("source hash mismatch".into());
    }
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}
pub fn build(request: &Value) -> Result<Value> {
    if text(request, "schema")? != "pencil-native-review-report-request/v1" {
        return Err("unsupported request schema".into());
    }
    let summary = bound(&request["plan_summary"])?;
    let fields = [
        "expected_workloads",
        "expected_measured_pairs",
        "expected_executions",
        "expected_warmup_executions",
        "observed_executions",
    ];
    let mut counts = serde_json::Map::new();
    for key in fields {
        counts.insert(
            key.into(),
            json!(summary[key]
                .as_u64()
                .ok_or_else(|| format!("missing count {key}"))?),
        );
    }
    if summary["observed_executions"] != json!(0) {
        return Err("this preparation report requires zero observed executions".into());
    }
    let cells = summary["aggregate_cells"]
        .as_array()
        .ok_or("missing cells")?;
    if cells.len() != 36
        || cells.iter().any(|c| {
            !c.get("algorithm_ratio_of_sums").is_some_and(Value::is_null)
                || !c
                    .get("ratio_of_sums_including_calibration")
                    .is_some_and(Value::is_null)
        })
    {
        return Err("expected 36 explicitly null ratio cells".into());
    }
    let mut evidence = vec![
        json!({"id":"plan","binding":request["plan_summary"],"scope":"named plan-only input; not historical experiments"}),
    ];
    let mut observations = vec![];
    let mut ids = std::collections::BTreeSet::new();
    ids.insert("plan".to_string());
    for item in request["observations"]
        .as_array()
        .ok_or("missing observations")?
    {
        let id = text(item, "id")?;
        if !ids.insert(id.to_string()) {
            return Err("duplicate evidence id".into());
        }
        let scope = text(item, "scope")?;
        let value = bound(&item["source"])?;
        let mut observed = serde_json::Map::new();
        for (label, pointer) in item["fields"].as_object().ok_or("missing fields")? {
            let pointer = pointer.as_str().ok_or("pointer must be string")?;
            let selected = value
                .pointer(pointer)
                .ok_or_else(|| format!("missing evidence pointer {pointer}"))?;
            observed.insert(label.clone(), selected.clone());
        }
        evidence.push(json!({"id":id,"binding":item["source"],"scope":scope}));
        observations.push(json!({"id":id,"scope":scope,"values":observed}));
    }
    Ok(
        json!({"schema":"pencil-accounting-review-report/v2","title":text(request,"title")?,"snapshot":text(request,"snapshot")?,"scope":"Public synthetic known-answer mathematical research. Evidence observations only; no judgments, new cohort executions or qualification.","observation_boundary":"Zero executions refers to this named plan-only input, not historical experiments. Per-source software scopes are distinct and counts are never pooled.","cohort":counts,"ratios":cells,"observations":observations,"evidence":evidence,"historical_report":"validation/accounting/reports/review-20261029T1906Z; retained unchanged, Python renderer superseded for new reports","interpretation":"No empirical cost ratio is available in this snapshot. Software validation and prepared inputs do not constitute independent protocol, binary, resource or launch admission.","renderer_source_sha256":hash(include_bytes!("lib.rs")),"request":request}),
    )
}
fn escape(s: &str, tex: bool) -> String {
    let mut out = String::new();
    for c in s.chars() {
        out.push_str(match (tex, c) {
            (true, '\\') => "\\textbackslash{}",
            (true, '_') => "\\_",
            (true, '&') => "\\&",
            (true, '%') => "\\%",
            (true, '#') => "\\#",
            (true, '{') => "\\{",
            (true, '}') => "\\}",
            (true, '$') => "\\$",
            (true, '^') => "\\textasciicircum{}",
            (true, '~') => "\\textasciitilde{}",
            (false, '&') => "&amp;",
            (false, '<') => "&lt;",
            (false, '>') => "&gt;",
            (false, '\"') => "&quot;",
            _ => {
                out.push(c);
                continue;
            }
        });
    }
    out
}
pub fn svg(report: &Value) -> String {
    let c = &report["cohort"];
    let labels = [
        format!("{} prescribed workloads", c["expected_workloads"]),
        format!("{} planned execution slots", c["expected_executions"]),
        format!(
            "{} observed executions in this input",
            c["observed_executions"]
        ),
        format!("{} prescribed measured pairs", c["expected_measured_pairs"]),
        "36 cost cells: all ratios null".into(),
    ];
    let mut out=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"440\" viewBox=\"0 0 1000 440\"><rect width=\"1000\" height=\"440\" fill=\"white\"/><g font-family=\"DejaVu Sans,sans-serif\" fill=\"#183042\"><text x=\"35\" y=\"35\" font-size=\"23\">Preparation evidence; no empirical comparison</text>");
    for (i, label) in labels.iter().enumerate() {
        let y = 60 + i * 62;
        out+=&format!("<rect x=\"35\" y=\"{y}\" width=\"930\" height=\"48\" rx=\"6\" fill=\"#eef4f7\" stroke=\"#486276\"/><text x=\"55\" y=\"{}\" font-size=\"20\">{}</text>",y+31,escape(label,false));
    }
    out+="<text x=\"35\" y=\"400\" font-size=\"17\">Source: plan evidence binding in report.json. Diagram used because no empirical ratios exist.</text></g></svg>";
    out
}
pub fn tex(report: &Value) -> String {
    let mut out=String::from("\\documentclass[11pt]{article}\n\\usepackage[margin=0.7in]{geometry}\n\\usepackage{tikz,longtable,hyperref}\n\\setlength{\\parindent}{0pt}\\setlength{\\parskip}{7pt}\n\\begin{document}\n\\raggedright\\sloppy\n");
    out += &format!(
        "{{\\LARGE {}}}\\\\[5pt]\n{}\\\\\n{}\n\n{}\n",
        escape(report["title"].as_str().unwrap_or(""), true),
        escape(report["snapshot"].as_str().unwrap_or(""), true),
        escape(report["scope"].as_str().unwrap_or(""), true),
        escape(report["observation_boundary"].as_str().unwrap_or(""), true)
    );
    out += "\\section*{Plan-only observations}\n\\begin{center}\\begin{tikzpicture}\n";
    for (i, (label, key)) in [
        ("Prescribed workloads", "expected_workloads"),
        ("Planned execution slots", "expected_executions"),
        ("Observed executions in this input", "observed_executions"),
        ("Prescribed measured pairs", "expected_measured_pairs"),
    ]
    .iter()
    .enumerate()
    {
        out+=&format!("\\node[draw,rounded corners,fill=blue!4,minimum width=15cm,minimum height=0.8cm] at (0,-{}) {{{}: {}}};\n",i,escape(label,true),report["cohort"][key]);
    }
    out+="\\node[draw,rounded corners,fill=yellow!10,minimum width=15cm,minimum height=0.8cm] at (0,-4) {36 cost sensitivity cells: all ratios null};\n\\end{tikzpicture}\\end{center}\nDiagram rather than a performance graph: this input contains no observed empirical ratios. Counts are read from source [plan]; no rates or uncertainty estimates are inferred.\n";
    out += &format!(
        "Warmup execution slots prescribed: {}. {}\n",
        report["cohort"]["expected_warmup_executions"],
        escape(report["interpretation"].as_str().unwrap_or(""), true)
    );
    out += "\\section*{Scoped source observations}\n";
    for o in report["observations"].as_array().unwrap() {
        out += &format!(
            "\\textbf{{[{}]}} {}\\\\\n",
            escape(o["id"].as_str().unwrap(), true),
            escape(o["scope"].as_str().unwrap(), true)
        );
        for (k, v) in o["values"].as_object().unwrap() {
            out += &format!(
                "{}: {}\\\\\n",
                escape(k, true),
                escape(&v.to_string(), true)
            );
        }
    }
    out+="\\section*{Evidence and limits}\nNo software test count is an empirical target count. Missing costs stay null. Resource authorization is limited to its explicit source scope; no budget, deadline, infrastructure approval or independent review is inferred. The prior 19:06 report and its Python generator remain historical artifacts; new reports use this native renderer.\n\\small\n";
    for e in report["evidence"].as_array().unwrap() {
        out += &format!(
            "\\textbf{{[{}]}} {}\\\\\n\\texttt{{{}}}\\\\\nSHA256: \\texttt{{{}}}\\\\\n",
            escape(e["id"].as_str().unwrap(), true),
            escape(e["scope"].as_str().unwrap(), true),
            escape(e["binding"]["path"].as_str().unwrap(), true).replace('/', "/\\allowbreak{}"),
            e["binding"]["sha256"].as_str().unwrap()
        );
    }
    out += "\\end{document}\n";
    out
}
pub fn write_report(request: &Value, out: &Path) -> Result<()> {
    let report = build(request)?;
    fs::create_dir(out).map_err(|e| e.to_string())?;
    for (name, bytes) in [
        (
            "report.json",
            serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
        ),
        ("pipeline.svg", svg(&report).into_bytes()),
        ("report.tex", tex(&report).into_bytes()),
        (
            "request.json",
            serde_json::to_vec_pretty(request).map_err(|e| e.to_string())?,
        ),
    ] {
        fs::write(out.join(name), bytes).map_err(|e| e.to_string())?;
    }
    let run = std::process::Command::new("pdflatex")
        .args([
            "-no-shell-escape",
            "-interaction=nonstopmode",
            "-halt-on-error",
            "report.tex",
        ])
        .current_dir(out)
        .output()
        .map_err(|e| e.to_string())?;
    fs::write(out.join("build.stdout.log"), &run.stdout).map_err(|e| e.to_string())?;
    fs::write(out.join("build.stderr.log"), &run.stderr).map_err(|e| e.to_string())?;
    if !run.status.success() {
        return Err("pdflatex failed; raw logs retained".into());
    }
    let mut names = fs::read_dir(out)
        .map_err(|e| e.to_string())?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::io::Result<Vec<_>>>()
        .map_err(|e| e.to_string())?;
    names.sort();
    let mut manifest = String::new();
    for path in names {
        if path.is_file() {
            manifest += &format!(
                "{}  {}\n",
                hash(&fs::read(&path).map_err(|e| e.to_string())?),
                path.file_name().unwrap().to_string_lossy()
            );
        }
    }
    fs::write(out.join("SHA256SUMS"), manifest).map_err(|e| e.to_string())?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (std::path::PathBuf, Value) {
        let d = std::env::temp_dir().join(format!(
            "native-report-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&d).unwrap();
        let summary = json!({"expected_workloads":1260,"expected_measured_pairs":6300,"expected_executions":22680,"expected_warmup_executions":3780,"observed_executions":0,"aggregate_cells":vec![json!({"algorithm_ratio_of_sums":null,"ratio_of_sums_including_calibration":null});36]});
        fs::write(d.join("plan.json"), summary.to_string()).unwrap();
        fs::write(d.join("test.json"), b"{\"passed\":7,\"failed\":0}").unwrap();
        let bind = |name: &str| json!({"path":d.join(name),"sha256":hash(&fs::read(d.join(name)).unwrap())});
        let r = json!({"schema":"pencil-native-review-report-request/v1","title":"Software fixture & scope","snapshot":"software only","plan_summary":bind("plan.json"),"observations":[{"id":"unit","scope":"seven isolated software tests; not cohort targets","source":bind("test.json"),"fields":{"passed":"/passed","failed":"/failed"}}]});
        (d, r)
    }
    #[test]
    fn preserves_scoped_counts_and_nulls() {
        let (d, r) = fixture();
        let x = build(&r).unwrap();
        assert_eq!(x["observations"][0]["values"]["passed"], 7);
        assert_eq!(x["cohort"]["observed_executions"], 0);
        assert!(tex(&x).contains("Software fixture \\& scope"));
        assert!(svg(&x).contains("36 cost cells: all ratios null"));
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn hash_mismatch_refused() {
        let (d, r) = fixture();
        fs::write(d.join("test.json"), "{}").unwrap();
        assert!(build(&r).unwrap_err().contains("hash mismatch"));
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn absent_pointer_not_zero() {
        let (d, mut r) = fixture();
        r["observations"][0]["fields"]["passed"] = json!("/unknown");
        assert!(build(&r).unwrap_err().contains("missing evidence pointer"));
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn missing_scope_refused() {
        let (d, mut r) = fixture();
        r["observations"][0]["scope"] = json!("");
        assert!(build(&r).is_err());
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn positive_ratios_not_reported_as_empty() {
        let (d, mut r) = fixture();
        let path = d.join("plan.json");
        let mut v: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        v["aggregate_cells"][0]["algorithm_ratio_of_sums"] = json!(1);
        fs::write(&path, v.to_string()).unwrap();
        r["plan_summary"]["sha256"] = json!(hash(&fs::read(&path).unwrap()));
        assert!(build(&r).unwrap_err().contains("null ratio"));
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn duplicate_scopes_are_not_pooled() {
        let (d, mut r) = fixture();
        let item = r["observations"][0].clone();
        r["observations"].as_array_mut().unwrap().push(item);
        assert!(build(&r).unwrap_err().contains("duplicate evidence"));
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn native_pdf_render_and_write_once() {
        let (d, r) = fixture();
        let output = d.join("rendered");
        write_report(&r, &output).unwrap();
        assert!(fs::read(output.join("report.pdf"))
            .unwrap()
            .starts_with(b"%PDF-"));
        assert!(fs::read_to_string(output.join("SHA256SUMS"))
            .unwrap()
            .contains("report.pdf"));
        assert!(write_report(&r, &output).is_err());
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn historical_report_subset_preserves_validation_scopes() {
        let (d, mut r) = fixture();
        let bytes = include_bytes!("../tests/fixtures/historical-validation.json");
        let old: Value = serde_json::from_slice(bytes).unwrap();
        let path = d.join("historical.json");
        fs::write(&path, bytes).unwrap();
        r["observations"] = json!([{
            "id":"historical-producer", "scope":"Historical 19:06 producer suite only, not current tests or empirical targets",
            "source":{"path":path,"sha256":hash(bytes)},
            "fields":{"passed":"/software_tests/0/passed","ignored":"/software_tests/0/ignored"}
        }, {
            "id":"historical-root", "scope":"Historical root release suite, a separate validation scope",
            "source":{"path":path,"sha256":hash(bytes)},
            "fields":{"passed":"/full_root_release_tests/passed"}
        }]);
        let report = build(&r).unwrap();
        assert_eq!(
            report["observations"][0]["values"]["passed"],
            old["software_tests"][0]["passed"]
        );
        assert_eq!(
            report["observations"][1]["values"]["passed"],
            old["full_root_release_tests"]["passed"]
        );
        assert_eq!(report["cohort"]["observed_executions"], 0);
        fs::remove_dir_all(d).unwrap();
    }
}
