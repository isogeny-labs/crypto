use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: report INPUT_RECEIPT NEW_OUTPUT_DIR".into());
    }
    let bytes = fs::read(&args[1])?;
    let v: Value = serde_json::from_slice(&bytes)?;
    if v["passed"] != true
        || v["solver_executions_performed"] != 0
        || v["cohort_targets"] != 1260
        || v["cohort_planned_executions"] != 22680
        || v["cohort_measured_executions"] != 18900
        || v["cohort_warmup_executions"] != 3780
    {
        return Err("unexpected preparation receipt".into());
    }
    let out = Path::new(&args[2]);
    fs::create_dir(out)?;
    let hash = format!("{:x}", Sha256::digest(&bytes));
    let scope="Preparation only: one native plan-only invocation and retained-slot verification. No empirical solves or new cost result. Clock qualification, scientific admission, budgets and analysis/export integration remain pending.";
    let report = json!({"schema":"pencil-disjoint-preparation-report/v1","source":{"path":args[1],"sha256":hash},"workloads":1260,"slots":22680,"measured_arm_slots":18900,"warmup_slots":3780,"measured_pairs":6300,"solver_executions":0,"plan_only_invocations":v["plan_only_invocations"],"scope":scope,"partition":v["files"],"qualified_cost_ratio":null});
    fs::write(out.join("report.json"), serde_json::to_vec_pretty(&report)?)?;
    let svg="<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"840\" height=\"260\"><rect width=\"840\" height=\"260\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#172b4d\"><text x=\"20\" y=\"30\" font-size=\"21\">Disjoint enrollment preparation: planned slots</text><text x=\"20\" y=\"65\">Original seven-plan set = enrolled eight-plan set: 22,680 slots</text><text x=\"20\" y=\"106\">Measured</text><rect x=\"130\" y=\"86\" width=\"500\" height=\"28\" fill=\"#31759a\"/><text x=\"650\" y=\"106\">18,900</text><text x=\"20\" y=\"156\">Warmup</text><rect x=\"130\" y=\"136\" width=\"100\" height=\"28\" fill=\"#7397a9\"/><text x=\"250\" y=\"156\">3,780</text><text x=\"20\" y=\"205\">1,260 workloads; 6,300 measured pairs. No empirical solves.</text><text x=\"20\" y=\"235\">Preparation does not grant scientific, timing, resource or execution admission.</text></g></svg>";
    fs::write(out.join("enrollment.svg"), svg)?;
    let tex = format!(
        r"\documentclass{{article}}
\usepackage[margin=0.8in]{{geometry}}
\usepackage{{tikz}}
\begin{{document}}
\section*{{Disjoint enrollment preparation}}
{scope}\par
The retained seven original plans are partitioned into eight enrolled plans: p739 gate (60 workloads), cohort16 remainder (120), and six unchanged plans (180 each). Complete workload objects, methods and exact integer slot identities were checked by native preparation. Sequence and workload-array positions may differ and remain recorded in the mappings.\par
\begin{{center}}\begin{{tabular}}{{lr}}Workloads & 1,260\\Planned slots & 22,680\\Measured arm slots & 18,900\\Warmup slots & 3,780\\Measured candidate/reference pairs & 6,300\\Empirical solver executions & 0\end{{tabular}}\end{{center}}
\begin{{tikzpicture}}[x=0.0003cm,y=1cm]
\node[anchor=east] at (0,0) {{Measured}};\fill[blue!55] (0,-0.2) rectangle (18900,0.2);\node[anchor=west] at (19500,0) {{18,900}};
\node[anchor=east] at (0,-0.7) {{Warmup}};\fill[blue!30] (0,-0.9) rectangle (3780,-0.5);\node[anchor=west] at (4400,-0.7) {{3,780}};
\end{{tikzpicture}}
\subsection*{{Evidence and limits}}
The request and native outputs are retained byte-for-byte. The certificate and mappings preserve both original and enrolled identities. The separate upper gate is not included in this partition. Earlier p739 observations and their invalid clock status are not changed. This packet contains no cost estimate and authorizes no execution.\par
Source receipt SHA-256:\par\texttt{{\small {hash}}}\par
Machine-readable observations: report.json. Graph: enrollment.svg. Raw evidence and software-test receipts are bound by the packet manifest.
\end{{document}}
"
    );
    fs::write(out.join("report.tex"), tex)?;
    let status = Command::new("pdflatex")
        .current_dir(out)
        .args([
            "-no-shell-escape",
            "-interaction=nonstopmode",
            "-halt-on-error",
            "report.tex",
        ])
        .output()?;
    fs::write(out.join("pdflatex.stdout"), status.stdout)?;
    fs::write(out.join("pdflatex.stderr"), status.stderr)?;
    if !status.status.success() {
        return Err("PDF render failed".into());
    }
    Ok(())
}
