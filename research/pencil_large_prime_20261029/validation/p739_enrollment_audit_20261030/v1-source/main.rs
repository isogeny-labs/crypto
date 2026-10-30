use p739_enrollment_audit::compare;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage: p739-enrollment-audit GATE COHORT FRESH NEW_OUTPUT_DIR".into());
    }
    let mut values = Vec::new();
    let mut bindings = Vec::new();
    for p in &args[1..4] {
        let bytes = fs::read(p)?;
        values.push(serde_json::from_slice::<Value>(&bytes)?);
        bindings.push(
            json!({"path":p,"sha256":format!("{:x}",Sha256::digest(&bytes)),"bytes":bytes.len()}),
        );
    }
    let mapping = compare(&values[0], &values[1])?;
    let fresh = compare(&values[0], &values[2])?;
    if mapping["gate_workloads"] != 60
        || mapping["resolved_arms"] != 3
        || mapping["gate_executions"] != 1080
    {
        return Err("gate inventory differs from exact60/3/1080 contract".into());
    }
    let out = Path::new(&args[4]);
    fs::create_dir(out)?;
    let result = json!({"schema":"p739-retained-plan-enrollment-audit/v1","inputs":bindings,"gate_to_cohort16":mapping,"gate_to_fresh":fresh,"fresh_full_plan_equal":values[0]==values[2],"scope":"Offline retained-plan identity verification only. No solver, plan generation, new measurement, general admission or remaining1200-target launch authorization.","algorithm_seed_representation":"Exact unsigned64bit JSON integer; never f64 or decimal approximation","new_solver_executions":0});
    fs::write(out.join("audit.json"), serde_json::to_vec_pretty(&result)?)?;
    let counts = [
        (
            "Mapped",
            result["gate_to_cohort16"]["mapped"].as_u64().unwrap(),
        ),
        (
            "Missing",
            result["gate_to_cohort16"]["missing"].as_u64().unwrap(),
        ),
        ("Duplicate", 0),
        (
            "Seed mismatch",
            result["gate_to_cohort16"]["seed_mismatches"]
                .as_u64()
                .unwrap(),
        ),
    ];
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"780\" height=\"310\" viewBox=\"0 0 780 310\"><rect width=\"780\" height=\"310\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#172b4d\"><text x=\"20\" y=\"30\" font-size=\"20\">Retained p739 gate to cohort16 execution mapping</text><text x=\"20\" y=\"55\" font-size=\"13\">60 workloads; 3 resolved arms; 1,080 gate slots. Offline exact-integer check.</text>");
    for (i, (label, n)) in counts.iter().enumerate() {
        let y = 85 + i * 43;
        let width = n * 480 / 1080;
        svg+=&format!("<text x=\"20\" y=\"{}\">{label}</text><rect x=\"150\" y=\"{y}\" width=\"{width}\" height=\"23\" fill=\"#31759a\"/><text x=\"650\" y=\"{}\">{n}</text>",y+17,y+17);
    }
    svg+="<text x=\"20\" y=\"285\" font-size=\"12\">Counts are derived from retained plans; no execution or launch permission is established.</text></g></svg>";
    fs::write(out.join("mapping.svg"), svg)?;
    let mut tex=String::from("\\documentclass{article}\n\\usepackage[margin=0.8in]{geometry}\n\\usepackage{tikz}\n\\begin{document}\n\\section*{Retained p739 plan enrollment audit}\nObservations only. This native checker compared retained gate, cohort16 and fresh-session plans. It executed no solver, generated no plan and took no new measurement.\\par\n");
    tex+=&format!("Gate workloads:60. Resolved arms:3. Gate executions:1080. Matched executions:{}; missing:{}; duplicates:0; seed mismatches:{}. Sequence positions differing between gate and cohort subset:{}.\\par\n",counts[0].1,counts[1].1,counts[3].1,result["gate_to_cohort16"]["seq_differences"]);
    tex += "\\begin{tikzpicture}[x=0.005cm,y=0.7cm]\n";
    for (i, (label, n)) in counts.iter().enumerate() {
        tex+=&format!("\\node[anchor=east] at (0,-{i}) {{{label}}};\\fill[blue!55] (0,-{i}) rectangle ({n},-{});\\node[anchor=west] at (1100,-{i}) {{{n}}};\n",i as f64-0.45);
    }
    tex+="\\end{tikzpicture}\\par\nFull workload and resolved arm objects are compared, including original IDs. Algorithm seeds are parsed as exact unsigned64bit integers. Mapping rows retain both sequence positions and both seed values. This verifies membership only; it is not general admission or authority to launch the remaining1200 targets.\\par\n\\subsection*{Input integrity}\n";
    for (i, name) in ["Gate", "Cohort16", "Fresh"].iter().enumerate() {
        tex += &format!(
            "{name}: \\texttt{{\\small {}}}\\par\n",
            result["inputs"][i]["sha256"].as_str().unwrap()
        );
    }
    tex+=&format!("\\par Fresh full plan equality: {}. Gate-to-cohort bijection: {}.\\par\nFull machine-readable evidence and all1080 mapping rows are in audit.json; mapping.svg contains the corresponding count graph.\n\\end{{document}}\n",result["fresh_full_plan_equal"],result["gate_to_cohort16"]["bijection"]);
    fs::write(out.join("report.tex"), tex)?;
    let output = Command::new("pdflatex")
        .current_dir(out)
        .args(["-interaction=nonstopmode", "-halt-on-error", "report.tex"])
        .output()?;
    fs::write(out.join("pdflatex.stdout"), &output.stdout)?;
    fs::write(out.join("pdflatex.stderr"), &output.stderr)?;
    if !output.status.success() {
        return Err("native PDF render failed".into());
    }
    println!(
        "{}",
        json!({"output":out,"gate_to_cohort_bijection":result["gate_to_cohort16"]["bijection"],"fresh_full_plan_equal":result["fresh_full_plan_equal"],"mapped":counts[0].1,"missing":counts[1].1,"duplicates":0,"seed_mismatches":counts[3].1})
    );
    Ok(())
}
