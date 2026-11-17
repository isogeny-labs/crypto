//! Presentation of retained native observations only; no solver or pricing implementation.
use serde_json::{json, Value as V};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path, process::Command};
pub type Result<T> = std::result::Result<T, String>;
pub fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn number(v: &V) -> Result<u64> {
    v.as_u64().ok_or("missing exact u64".into())
}
fn text(v: &V) -> Result<&str> {
    v.as_str()
        .filter(|s| !s.trim().is_empty())
        .ok_or("missing text".into())
}
fn bound(b: &V) -> Result<V> {
    let bytes = fs::read(text(&b["path"])?).map_err(|e| e.to_string())?;
    if hash(&bytes) != text(&b["sha256"])? {
        return Err("input hash mismatch".into());
    }
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}
fn fraction(v: &V) -> Result<String> {
    if v.is_null() {
        return Ok("null".into());
    }
    let ns = text(&v["numerator"])?;
    let ds = text(&v["denominator"])?;
    if ![ns, ds]
        .iter()
        .all(|s| s.bytes().all(|c| c.is_ascii_digit()))
        || ds.bytes().all(|c| c == b'0')
    {
        return Err("invalid exact fraction".into());
    }
    Ok(format!("{ns}/{ds}"))
}
pub fn build(request: &V) -> Result<V> {
    if request["schema"] != "pencil-native-observation-report-request/v1" {
        return Err("unsupported request".into());
    }
    for k in ["title", "run_id", "scope", "source_commit"] {
        text(&request[k])?;
    }
    let a = bound(&request["analysis"])?;
    if a["schema"] != "pencil-price-cohort-output/v1" {
        return Err("unsupported native analysis".into());
    }
    let expected = number(&a["expected_executions"])?;
    let workloads = number(&a["expected_workloads"])?;
    if workloads == 0
        || workloads.checked_mul(18) != Some(expected)
        || workloads.checked_mul(5) != Some(number(&a["expected_measured_pairs"])?)
    {
        return Err("invalid three-arm six-round plan geometry".into());
    }
    if a.get("qualified_ratios") != Some(&V::Null) {
        return Err("timing-qualified ratios outside report scope".into());
    }
    let observed = number(&a["observed_executions"])?;
    let missing = a["missing_executions"]
        .as_array()
        .ok_or("complete missing rows required, not summary")?
        .len() as u64;
    if observed.checked_add(missing) != Some(expected) {
        return Err("observed/missing denominator mismatch".into());
    }
    let pairs = number(&a["expected_measured_pairs"])?;
    let cells = a["aggregate_cells"].as_array().ok_or("cells absent")?;
    let mut keys = BTreeSet::new();
    if cells.len() != 36 {
        return Err("exact36 cells required".into());
    }
    for c in cells {
        if c.get("qualified_ratio") != Some(&V::Null) {
            return Err("qualified cell outside report scope".into());
        }
        let k = (
            number(&c["square_twice"])?,
            number(&c["inversion"])?,
            number(&c["native_tenths"])?,
        );
        if ![0, 1, 2].contains(&k.0)
            || ![1, 10, 100, 1000].contains(&k.1)
            || ![0, 1, 10].contains(&k.2)
            || !keys.insert(k)
        {
            return Err("wrong or duplicate price cell".into());
        }
        let admitted = number(&c["admitted_pairs"])?;
        let absent = number(&c["missing_or_ineligible_pairs"])?;
        if number(&c["expected_pairs"])? != pairs || admitted.checked_add(absent) != Some(pairs) {
            return Err("pair denominator mismatch".into());
        }
        let complete = c["complete"].as_bool().ok_or("complete flag absent")?;
        if complete != (admitted == pairs) {
            return Err("inconsistent complete flag".into());
        }
        for k in [
            "algorithm_ratio_of_sums",
            "ratio_of_sums_including_calibration",
        ] {
            if c.get(k).is_none() || complete == c[k].is_null() {
                return Err("ratio must be non-null exactly when complete".into());
            }
            fraction(&c[k])?;
        }
    }
    let outcomes = a["outcome_counts"]
        .as_object()
        .ok_or("outcome counts absent")?;
    let sum = outcomes.values().try_fold(0u64, |s, v| {
        s.checked_add(number(v)?)
            .ok_or_else(|| "outcome overflow".to_owned())
    })?;
    if sum != observed {
        return Err("outcome counts differ from observations".into());
    }
    let mut evidence = vec![request["analysis"].clone()];
    for b in request["sidecars"]
        .as_array()
        .ok_or("sidecars array required")?
    {
        bound(b)?;
        evidence.push(b.clone());
    }
    for note in request["notes"].as_array().ok_or("notes array required")? {
        text(note)?;
    }
    let resources = if request["resources"].is_null() {
        V::Null
    } else {
        evidence.push(request["resources"].clone());
        bound(&request["resources"])?
    };
    Ok(
        json!({"schema":"pencil-native-observation-report/v1","request":request,"evidence":evidence,"native_analysis":a,"resource_observations":resources,"presentation":"Observations only. Exact fractions and nulls retained. No statistical, speed, mathematical or timing qualification is inferred.","partial_sums_policy":"partial_admitted_sums_diagnostic_only remains a diagnostic field; it is never presented as a complete ratio.","renderer_source_sha256":hash(include_bytes!("lib.rs"))}),
    )
}
fn xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
fn tex(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\\' => "\\textbackslash{}".into(),
            '&' | '%' | '$' | '#' | '_' | '{' | '}' => format!("\\{c}"),
            '~' => "\\textasciitilde{}".into(),
            '^' => "\\textasciicircum{}".into(),
            _ => c.to_string(),
        })
        .collect()
}
fn csv(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}
fn breakable(s: &str) -> String {
    s.chars()
        .map(|c| format!("{}\\allowbreak{{}}", tex(&c.to_string())))
        .collect()
}
/// Floating point is used only for SVG coordinates, never for exported fractions.
fn ratio_svg(cells: &[V], request: &V) -> Result<Option<String>> {
    if cells
        .iter()
        .filter(|c| !c["algorithm_ratio_of_sums"].is_null())
        .count()
        < 2
    {
        return Ok(None);
    }
    let mut values = Vec::new();
    let mut maximum = 1.0f64;
    for c in cells {
        let mut pair = [None, None];
        for (j, key) in [
            "algorithm_ratio_of_sums",
            "ratio_of_sums_including_calibration",
        ]
        .iter()
        .enumerate()
        {
            if !c[key].is_null() {
                fraction(&c[key])?;
                let n = text(&c[key]["numerator"])?
                    .parse::<f64>()
                    .map_err(|e| e.to_string())?;
                let d = text(&c[key]["denominator"])?
                    .parse::<f64>()
                    .map_err(|e| e.to_string())?;
                let value = n / d;
                if !n.is_finite() || !d.is_finite() || !value.is_finite() {
                    return Err(
                        "fraction outside finite SVG projection range; exact inputs retained"
                            .into(),
                    );
                }
                maximum = maximum.max(value);
                pair[j] = Some(value);
            }
        }
        values.push(pair);
    }
    let mut svg = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1400\" height=\"1800\"><rect width=\"1400\" height=\"1800\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#183042\"><text x=\"20\" y=\"30\" font-size=\"22\">Derived finite-model cost ratios to rho — not timing</text><text x=\"20\" y=\"60\">Floating-point visual projection only. Exact fractions in JSON and CSV are authoritative.</text><text x=\"20\" y=\"90\">Blue: algorithm only. Orange: including calibration. Null values have no plotted position.</text>");
    for tick in 0..=4 {
        let x = 350 + tick * 200;
        svg += &format!("<line x1=\"{x}\" x2=\"{x}\" y1=\"130\" y2=\"1720\" stroke=\"#ddd\"/><text x=\"{x}\" y=\"120\">{:.4}</text>", maximum * tick as f64 / 4.0);
    }
    let reference_x = 350.0 + 800.0 / maximum;
    svg += &format!("<line x1=\"{reference_x}\" x2=\"{reference_x}\" y1=\"130\" y2=\"1720\" stroke=\"#777\" stroke-dasharray=\"4 4\"/>");
    for (i, (c, pair)) in cells.iter().zip(values).enumerate() {
        let y = 155 + i * 43;
        svg += &format!(
            "<g data-cell=\"{}\"><text x=\"20\" y=\"{y}\">Cell {}: S2={} I={} N10={}</text>",
            i + 1,
            i + 1,
            c["square_twice"],
            c["inversion"],
            c["native_tenths"]
        );
        for (j, key) in [
            "algorithm_ratio_of_sums",
            "ratio_of_sums_including_calibration",
        ]
        .iter()
        .enumerate()
        {
            let yy = y + j * 13;
            if let Some(value) = pair[j] {
                let x = 350.0 + 800.0 * value / maximum;
                let color = if j == 0 { "#2369a1" } else { "#b55c18" };
                svg += &format!("<circle data-series=\"{key}\" cx=\"{x}\" cy=\"{yy}\" r=\"4\" fill=\"{color}\"><title>{}</title></circle><text x=\"1180\" y=\"{yy}\" fill=\"{color}\">~{value:.6}</text>",xml(&fraction(&c[key])?));
            } else {
                svg += &format!(
                    "<text data-series=\"{key}\" x=\"1180\" y=\"{yy}\">null (unavailable)</text>"
                );
            }
        }
        svg += "</g>";
    }
    svg += "<text x=\"20\" y=\"1770\">Dashed guide: ratio 1. Price-cell ratios share observations; they are not independent samples.</text>";
    // Extend only the caption area; numerical projection coordinates are unchanged.
    svg = svg.replace("height=\"1800\"", "height=\"1920\"");
    for (i, key) in ["title", "run_id", "scope"].iter().enumerate() {
        let full = format!("{key}: {}", text(&request[key])?);
        let short = if full.chars().count() > 130 {
            format!("{}…", full.chars().take(129).collect::<String>())
        } else {
            full.clone()
        };
        svg += &format!(
            "<text x=\"20\" y=\"{}\"><title>{}</title>{}</text>",
            1810 + i * 30,
            xml(&full),
            xml(&short)
        );
    }
    svg += "</g></svg>";
    Ok(Some(svg))
}
pub fn render(report: &V, out: &Path) -> Result<()> {
    fs::create_dir(out).map_err(|e| e.to_string())?;
    let a = &report["native_analysis"];
    let r = &report["request"];
    let cells = a["aggregate_cells"].as_array().ok_or("cells")?;
    let title = text(&r["title"])?;
    let run = text(&r["run_id"])?;
    let counts = format!(
        "Workloads: {}; expected pairs per price cell: {}; slots: {}; observed: {}; missing: {}.",
        a["expected_workloads"],
        a["expected_measured_pairs"],
        a["expected_executions"],
        a["observed_executions"],
        a["missing_executions"].as_array().unwrap().len()
    );
    let mut svg=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"1600\"><rect width=\"1200\" height=\"1600\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#183042\"><text x=\"20\" y=\"30\" font-size=\"21\">{}</text><text x=\"20\" y=\"58\">Run: {}</text><text x=\"20\" y=\"86\">{}</text><text x=\"20\" y=\"114\">Recorded pair coverage per price cell (counts, not performance). Ratios remain exact in JSON/CSV/PDF.</text>",xml(title),xml(run),xml(&counts));
    let mut table = String::new();
    let mut grid=String::from("cell,square_twice,inversion,native_tenths,expected_pairs,admitted_pairs,missing_or_ineligible_pairs,complete,algorithm_ratio_exact,including_calibration_ratio_exact\n");
    let mut picture = String::new();
    for (i, c) in cells.iter().enumerate() {
        let admitted = number(&c["admitted_pairs"])?;
        let expected = number(&c["expected_pairs"])?;
        let width = if expected == 0 {
            0
        } else {
            u128::from(admitted) * 500 / u128::from(expected)
        };
        let y = 145 + i * 38;
        svg+=&format!("<text x=\"20\" y=\"{}\">Cell {}: S2={} I={} N10={}</text><rect x=\"330\" y=\"{y}\" width=\"500\" height=\"23\" fill=\"#eeeeee\"/><rect x=\"330\" y=\"{y}\" width=\"{width}\" height=\"23\" fill=\"#347f9e\"/><text x=\"850\" y=\"{}\">{admitted}/{expected} admitted; {} missing/ineligible</text>",y+17,i+1,c["square_twice"],c["inversion"],c["native_tenths"],y+17,c["missing_or_ineligible_pairs"]);
        let f = fraction(&c["algorithm_ratio_of_sums"])?;
        let g = fraction(&c["ratio_of_sums_including_calibration"])?;
        table += &format!(
            "{} & {}/{}/{} & {}/{} & {} & {} \\\\ \\hline\n",
            i + 1,
            c["square_twice"],
            c["inversion"],
            c["native_tenths"],
            admitted,
            expected,
            breakable(&f),
            breakable(&g)
        );
        grid += &format!(
            "{},{},{},{},{},{},{},{},{},{}\n",
            i + 1,
            c["square_twice"],
            c["inversion"],
            c["native_tenths"],
            expected,
            admitted,
            c["missing_or_ineligible_pairs"],
            c["complete"],
            csv(&f),
            csv(&g)
        );
        // Compact native TikZ graph, with the same count scale as SVG.
        let column = i / 18;
        let row = i % 18;
        picture+=&format!("\\begin{{scope}}[shift={{({},{})}}]\\node[anchor=east] at (0,0) {{{}}};\\fill[gray!15] (0,-0.1) rectangle (5,0.1);\\fill[blue!50] (0,-0.1) rectangle ({},0.1);\\node[anchor=west] at (5.1,0) {{{}/{}}};\\end{{scope}}\n",column*8,-(row as f64)*0.35,i+1,width as f64/100.0,admitted,expected);
    }
    svg+="<text x=\"20\" y=\"1550\">Gray: expected pairs; blue: admitted pairs. Incomplete or unavailable ratios stay null, never zero.</text></g></svg>";
    let diagram = cells
        .iter()
        .filter(|c| !c["algorithm_ratio_of_sums"].is_null())
        .count()
        < 2;
    let caption = if diagram {
        "Fewer than two comparable ratio cells are present. Diagram of planned scope and observed progress; no performance plot."
    } else {
        "Cell labels refer to the exact table. Gray is the expected denominator; blue is admitted pairs. This is a count graph, not a performance or uncertainty graph."
    };
    if diagram {
        svg=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"300\"><rect width=\"1200\" height=\"300\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#183042\"><text x=\"20\" y=\"35\">{}</text><rect x=\"20\" y=\"60\" width=\"1150\" height=\"60\" fill=\"#edf4f7\"/><text x=\"35\" y=\"95\">{}</text><text x=\"20\" y=\"160\">Arms: pencil, rho, rho_AA. Run: {}</text><text x=\"20\" y=\"220\">{}</text></g></svg>",xml(title),xml(&counts),xml(run),xml(caption));
        picture=format!("\\node[draw,text width=15cm,align=left] at (8,0) {{{}}};\\node[draw,text width=15cm,align=left] at (8,-1.2) {{Arms: pencil, rho, rho\\_AA. Planned slots are not observations.}};",tex(&counts));
    }
    let mut outcomes = String::new();
    for (k, v) in a["outcome_counts"].as_object().unwrap() {
        outcomes += &format!("{}: {}\\\\\n", tex(k), v)
    }
    if outcomes.is_empty() {
        outcomes = "No observed outcome rows.\\par\n".into();
    }
    let mut notes = String::new();
    for n in r["notes"].as_array().unwrap() {
        notes += &format!("{}\\par\n", tex(text(n)?));
    }
    if !report["resource_observations"].is_null() {
        notes+=&format!("\\subsection*{{Separate resource observations}}No conversion to algorithm cost units.\\par\\texttt{{\\small {}}}\\par\n",breakable(&serde_json::to_string(&report["resource_observations"]).map_err(|e|e.to_string())?));
    }
    let mut evidence = String::new();
    for b in report["evidence"].as_array().unwrap() {
        evidence += &format!(
            "\\texttt{{\\small {}}}\\par\\texttt{{\\small {}}}\\par\n",
            breakable(text(&b["path"])?),
            breakable(text(&b["sha256"])?)
        );
    }
    let doc = format!(
        r"\documentclass[10pt]{{article}}
\usepackage[margin=15mm]{{geometry}}\usepackage{{tikz,longtable,array,hyperref}}
\begin{{document}}\raggedright\sloppy\section*{{{}}}
Run: {}. Caller-supplied source commit: {}.\par
Observations only. No statistical, speed, mathematical or timing qualification is inferred.\par
{}\par {}\par
\subsection*{{Recorded outcomes}}
{} Missing rows remain missing; capped, timeout, error and other outcomes retain their original labels. Counts from different epochs are not pooled.\par
\subsection*{{Pair coverage by price cell}}
\begin{{tikzpicture}}[x=0.85cm,y=1cm,font=\scriptsize]
{}\end{{tikzpicture}}\par
{}\par
{}\newpage\section*{{All36 exact price cells}}
S2 is twice S/M; N10 is ten times the native event price. Incomplete/unavailable ratios are null, not zero. Partial admitted sums are retained in JSON as diagnostic only; no partial sum is promoted to a complete ratio.\par
\begin{{longtable}}{{r l l p{{4.2cm}} p{{4.2cm}}}}Cell&S2/I/N10&Admitted/expected&Method/rho exact&With calibration/rho exact\\\hline\endhead
{}\end{{longtable}}
\section*{{Evidence bindings}}{}\end{{document}}
",
        tex(title),
        tex(run),
        tex(text(&r["source_commit"])?),
        tex(text(&r["scope"])?),
        tex(&counts),
        outcomes,
        picture,
        tex(caption),
        notes,
        table,
        evidence
    );
    for (name, data) in [
        (
            "report.json",
            serde_json::to_string_pretty(report).map_err(|e| e.to_string())?,
        ),
        ("coverage.svg", svg),
        ("cost-grid-exact.csv", grid),
        ("report.tex", doc),
    ] {
        fs::write(out.join(name), data).map_err(|e| e.to_string())?;
    }
    if let Some(svg) = ratio_svg(cells, r)? {
        fs::write(out.join("finite-cost-ratios.svg"), svg).map_err(|e| e.to_string())?;
    }
    for pass in 1..=2 {
        let render = Command::new("pdflatex")
            .current_dir(out)
            .args([
                "-no-shell-escape",
                "-interaction=nonstopmode",
                "-halt-on-error",
                "report.tex",
            ])
            .output()
            .map_err(|e| e.to_string())?;
        fs::write(
            out.join(format!("pdflatex-pass{pass}.stdout")),
            render.stdout,
        )
        .map_err(|e| e.to_string())?;
        fs::write(
            out.join(format!("pdflatex-pass{pass}.stderr")),
            render.stderr,
        )
        .map_err(|e| e.to_string())?;
        if !render.status.success() {
            return Err("native PDF rendering failed".into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(observed: u64) -> V {
        let mut cells = vec![];
        for s in [0, 1, 2] {
            for i in [1, 10, 100, 1000] {
                for n in [0, 1, 10] {
                    cells.push(json!({"square_twice":s,"inversion":i,"native_tenths":n,"expected_pairs":5,"admitted_pairs":0,"missing_or_ineligible_pairs":5,"complete":false,"algorithm_ratio_of_sums":null,"ratio_of_sums_including_calibration":null,"qualified_ratio":null}));
                }
            }
        }
        json!({"schema":"pencil-price-cohort-output/v1","expected_workloads":1,"expected_executions":18,"expected_measured_pairs":5,"observed_executions":observed,"missing_executions":vec![json!({"fixture":true});(18-observed) as usize],"outcome_counts":if observed==0{json!({})}else{json!({"pencil:resource_exhausted":1,"rho:error":observed-1})},"aggregate_cells":cells,"qualified_ratios":null})
    }
    fn request(a: &V, dir: &Path) -> V {
        let bytes = serde_json::to_vec_pretty(a).unwrap();
        let p = dir.join("analysis.json");
        fs::write(&p, &bytes).unwrap();
        json!({"schema":"pencil-native-observation-report-request/v1","title":"Software fixture only","run_id":"fixture_not_a_run","scope":"Synthetic renderer test; zero solver executions","source_commit":"fixture","analysis":{"path":p,"sha256":hash(&bytes)},"sidecars":[],"notes":["No empirical result. Fixture numbers only."],"resources":null})
    }
    fn check(a: &V) -> Result<V> {
        let d = tempfile::tempdir().unwrap();
        build(&request(a, d.path()))
    }
    #[test]
    fn missing_and_failures_remain_explicit() {
        let a = fixture(2);
        let r = check(&a).unwrap();
        assert_eq!(r["native_analysis"], a);
        assert_eq!(
            r["native_analysis"]["outcome_counts"]["pencil:resource_exhausted"],
            1
        );
    }
    #[test]
    fn rejects_wrong_geometry() {
        let mut a = fixture(0);
        a["expected_measured_pairs"] = json!(4);
        assert!(check(&a).is_err());
        a = fixture(0);
        a["expected_executions"] = json!(17);
        assert!(check(&a).is_err());
    }
    #[test]
    fn rejects_duplicate_cell() {
        let mut a = fixture(0);
        a["aggregate_cells"][1] = a["aggregate_cells"][0].clone();
        assert!(check(&a).is_err());
    }
    #[test]
    fn rejects_qualified_values() {
        let mut a = fixture(0);
        a["qualified_ratios"] = json!([1]);
        assert!(check(&a).is_err());
        a = fixture(0);
        a["aggregate_cells"][0]["qualified_ratio"] = json!(1);
        assert!(check(&a).is_err());
    }
    #[test]
    fn rejects_inconsistent_nulls() {
        let mut a = fixture(0);
        let c = &mut a["aggregate_cells"][0];
        c["complete"] = json!(true);
        c["admitted_pairs"] = json!(5);
        c["missing_or_ineligible_pairs"] = json!(0);
        assert!(check(&a).is_err());
        a = fixture(0);
        a["aggregate_cells"][0]["algorithm_ratio_of_sums"] =
            json!({"numerator":"1","denominator":"2"});
        assert!(check(&a).is_err());
    }
    #[test]
    fn rejects_invalid_fraction() {
        assert!(fraction(&json!({"numerator":"3","denominator":"0"})).is_err());
        assert!(fraction(&json!({"numerator":"1e9","denominator":"2"})).is_err());
    }
    #[test]
    fn rejects_bad_hash() {
        let d = tempfile::tempdir().unwrap();
        let mut r = request(&fixture(0), d.path());
        r["analysis"]["sha256"] = json!("bad");
        assert!(build(&r).is_err());
    }
    #[test]
    fn ratio_caption_escapes_identity_and_retains_full_scope() {
        let mut a = fixture(0);
        for i in [0, 1] {
            a["aggregate_cells"][i]["algorithm_ratio_of_sums"] =
                json!({"numerator":"1","denominator":"2"});
            a["aggregate_cells"][i]["ratio_of_sums_including_calibration"] =
                json!({"numerator":"3","denominator":"4"});
        }
        let scope = "long scope & ".repeat(30);
        let request =
            json!({"title":"Software fixture <only>","run_id":"fixture&not-a-run","scope":scope});
        let svg = ratio_svg(a["aggregate_cells"].as_array().unwrap(), &request)
            .unwrap()
            .unwrap();
        assert!(svg.contains("title: Software fixture &lt;only&gt;"));
        assert!(svg.contains("run_id: fixture&amp;not-a-run"));
        assert!(svg.contains(&format!("<title>scope: {}</title>", xml(&scope))));
        assert!(svg.contains('…'));
        assert!(!svg.contains("Software fixture <only>"));
    }
    #[test]
    fn renders_missing_and_mixed_native_pdf() {
        let retained = std::env::var_os("REPORT_FIXTURE_DIR").map(std::path::PathBuf::from);
        let temp = tempfile::tempdir().unwrap();
        let base = retained.as_deref().unwrap_or(temp.path());
        fs::create_dir_all(base).unwrap();
        for mixed in [false, true] {
            let name = if mixed {
                "mixed-fixture"
            } else {
                "all-missing-fixture"
            };
            let input = base.join(format!("{name}-inputs"));
            fs::create_dir(&input).unwrap();
            let mut a = fixture(if mixed { 18 } else { 0 });
            if mixed {
                a["outcome_counts"] =
                    json!({"pencil:verified":6,"rho:verified":6,"rho_AA:verified":6});
                for index in [0, 1] {
                    let c = &mut a["aggregate_cells"][index];
                    c["complete"] = json!(true);
                    c["admitted_pairs"] = json!(5);
                    c["missing_or_ineligible_pairs"] = json!(0);
                    c["algorithm_ratio_of_sums"] = json!({"numerator":"90071992547409931234567890123456789","denominator":"70000000000000000000000000000000000"});
                    c["ratio_of_sums_including_calibration"] =
                        json!({"numerator":"3","denominator":"2"});
                }
            }
            let req = request(&a, &input);
            fs::write(
                input.join("request.json"),
                serde_json::to_vec_pretty(&req).unwrap(),
            )
            .unwrap();
            let r = build(&req).unwrap();
            let out = base.join(name);
            render(&r, &out).unwrap();
            assert_eq!(out.join("finite-cost-ratios.svg").exists(), mixed);
            if mixed {
                let svg = fs::read_to_string(out.join("finite-cost-ratios.svg")).unwrap();
                assert_eq!(svg.matches("data-cell=").count(), 36);
                assert_eq!(svg.matches("<circle ").count(), 4);
                assert_eq!(svg.matches("null (unavailable)").count(), 68);
                assert!(svg.contains(
                    "90071992547409931234567890123456789/70000000000000000000000000000000000"
                ));
                assert!(svg.contains("not timing"));
            }
            assert!(out.join("report.pdf").metadata().unwrap().len() > 1000);
            let csv = fs::read_to_string(out.join("cost-grid-exact.csv")).unwrap();
            assert_eq!(csv.lines().count(), 37);
            assert_eq!(
                csv.contains(
                    "90071992547409931234567890123456789/70000000000000000000000000000000000"
                ),
                mixed
            );
            assert_eq!(
                fs::read_to_string(out.join("coverage.svg"))
                    .unwrap()
                    .contains("no performance plot"),
                !mixed
            );
            assert!(render(&r, &out).is_err());
        }
    }
}
