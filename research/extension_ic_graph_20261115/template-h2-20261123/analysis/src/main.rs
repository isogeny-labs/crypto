//! Analyze frozen records and draw their row counts; no algebraic solver.
use serde_json::{json, Value};
use std::{env, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn require(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}
fn load(root: &Path, name: &str) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(root.join(name))?)?)
}
fn number(v: &Value) -> Result<u64> {
    v.as_u64().ok_or_else(|| "integer record field".into())
}
fn exclusive(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    f.write_all(bytes)?;
    f.sync_all()?;
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    require(args.len() == 2, "usage: REPORT_PACKET")?;
    let root = Path::new(&args[1]);
    let generic = load(root, "probe-01-summary.json")?;
    require(
        generic["outcome"] == "construction_refused" && generic["cases_applied"] == 0,
        "generic outcome",
    )?;
    let low = load(root, "koszul-summary.json")?;
    let seven = load(root, "degree7-summary.json")?;
    let reference = load(root, "saved-reference-shape.json")?;
    for record in [&generic, &low, &seven, &reference] {
        require(
            record["input_sha256"]
                == "e63a6126b3309bc7e387347f4f8629d78f12897a7c55c928b7152e75c21e76f1",
            "every record binds the same original input",
        )?;
    }
    require(
        reference["cases"]
            .as_array()
            .ok_or("shape cases")?
            .iter()
            .all(|r| r["max_minimal_leading_degree"] == 7),
        "required reference degree",
    )?;
    let rows = low["degrees"]
        .as_array()
        .ok_or("low degrees")?
        .iter()
        .chain(seven["degrees"].as_array().ok_or("seven degree")?.iter())
        .collect::<Vec<_>>();
    require(rows.len() == 3, "three recorded degree controls")?;
    let mut derived = Vec::new();
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1040\" height=\"430\" viewBox=\"0 0 1040 430\"><rect width=\"1040\" height=\"430\" fill=\"white\"/><g font-family=\"DejaVu Sans,sans-serif\" fill=\"#18323d\"><text x=\"28\" y=\"36\" font-size=\"25\">Principal-syzygy pruning and the required degree</text><text x=\"28\" y=\"65\" font-size=\"16\">Gray: enumerated rows · colored: retained rows · exposed saved cases 0 and 1</text>");
    let mut accepted = 0usize;
    for (i, row) in rows.iter().enumerate() {
        let degree = number(&row["degree"])?;
        require(degree == 5 + i as u64, "frozen degree order")?;
        let shape = &row["shape"];
        let (full, kept, columns) = (
            number(&shape["enumerated"])?,
            number(&shape["kept"])?,
            number(&shape["columns"])?,
        );
        require(kept <= full, "row subset")?;
        let cases = row["cases"].as_array().ok_or("case records")?;
        require(cases.len() == 2, "two original case outcomes")?;
        let mut products = 0u128;
        for (case_index, c) in cases.iter().enumerate() {
            require(
                c["case"] == case_index && c["degree"] == degree,
                "case identity",
            )?;
            let cost = if degree == 7 {
                require(
                    c["accepted"] == true
                        && c["ideal_equal_saved_f4"] == true
                        && c["basis_size"] == 40
                        && c["rank"] == 1652,
                    "accepted complete basis",
                )?;
                accepted += 1;
                number(&c["elimination"]["modular_products"])?
            } else {
                require(
                    c["accepted"] == false
                        && c["refusal"]["reason"] == "leading ideal changed or incomplete template",
                    "retained refusal",
                )?;
                number(&c["refusal"]["attempted"]["modular_products"])?
            };
            products = products
                .checked_add(u128::from(cost))
                .ok_or("product total overflow")?;
        }
        let cells = u128::from(kept)
            .checked_mul(u128::from(kept + columns))
            .ok_or("cell total overflow")?;
        derived.push(json!({"degree":degree,"enumerated_rows":full,"retained_rows":kept,"omitted_rows":full-kept,
            "columns":columns,"coefficient_provenance_cells":cells.to_string(),"two_case_elimination_products":products.to_string(),
            "accepted_cases":if degree==7 {2} else {0},"adequate_reference_degree":degree>=7}));
        let y = 108 + i * 87;
        let color = if degree == 7 { "#246b70" } else { "#b56846" };
        svg.push_str(&format!("<text x=\"28\" y=\"{}\" font-size=\"19\">Degree {degree}</text><rect x=\"150\" y=\"{y}\" width=\"{}\" height=\"25\" fill=\"#abbcc6\"/><rect x=\"150\" y=\"{y}\" width=\"{}\" height=\"25\" fill=\"{color}\"/><text x=\"{}\" y=\"{}\" font-size=\"16\">{kept} / {full}</text><text x=\"150\" y=\"{}\" font-size=\"15\">{}</text>",y+19,full as f64*0.24,kept as f64*0.24,163.0+full as f64*0.24,y+19,y+49,
            if degree==7 {"2/2 accepted; exact ideals match saved F4"} else {"0/2 accepted; bound below the required minimal leading degree 7"}));
    }
    require(accepted == 2, "only two accepted basis cases")?;
    svg.push_str("<text x=\"28\" y=\"396\" font-size=\"15\">Construction viability only. All failures retained; no runtime or IC/rho-parity claim.</text></g></svg>\n");
    let result = json!({"schema":"nagao-template-report-analysis/v1","accepted_basis_cases":accepted,
        "rows":derived,"generic_construction_refused":true,"counter_scope":"reported modular products; incomplete machine cost",
        "required_reference_degree":7,"signed_validation":false});
    exclusive(
        &root.join("derived-observation.json"),
        format!("{}\n", serde_json::to_string_pretty(&result)?).as_bytes(),
    )?;
    exclusive(&root.join("row-counts.svg"), svg.as_bytes())?;
    println!("three degree records checked; two accepted exact-ideal comparisons; SVG written");
    Ok(())
}
