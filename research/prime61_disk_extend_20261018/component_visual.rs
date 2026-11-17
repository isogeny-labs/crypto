//! Render the exact relation witnesses; no arithmetic search or timing.
use serde_json::Value;
use std::{fs, path::Path};
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 3, "RELATIONS_JSONL OUTPUT_SVG");
    let rows: Vec<Value> = fs::read_to_string(&a[1])
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let summary = rows.iter().find(|v| v["kind"] == "summary").unwrap();
    assert_eq!(summary["result"], "PASS");
    assert_eq!(summary["subgroup_order"], 545296208u64);
    let degrees = [3, 7, 11, 13, 19, 23, 53, 61];
    let relations: Vec<_> = rows.iter().filter(|v| v["kind"] == "relation").collect();
    assert_eq!(relations.len(), 8);
    let mut s = String::from("<svg xmlns='http://www.w3.org/2000/svg' width='880' height='580' viewBox='0 0 880 580'><rect width='880' height='580' fill='white'/><g font-family='sans-serif' fill='#162b40'><text x='28' y='30' font-size='22' font-weight='bold'>Exact selected-subgroup certificate</text><text x='28' y='57' font-size='15'>H = &lt;G&gt; × &lt;T0,T1,T2,T3&gt; ≅ C34081013 × C2^4</text><rect x='28' y='74' width='824' height='46' rx='5' fill='#e1eee9'/><text x='45' y='104' font-size='20' font-weight='bold'>|H| = 34,081,013 × 16 = 545,296,208</text><text x='80' y='150' font-size='15'>Degree</text><text x='270' y='150' font-size='15'>Exponent of G</text><text x='535' y='150' font-size='15'>Torsion mask</text>");
    for (i, r) in relations.iter().enumerate() {
        assert_eq!(r["left"], r["right"]);
        assert_eq!(r["index"], i + 1);
        let y = 179 + i * 30;
        s += &format!("<path d='M45 {} H820' stroke='#e5ebef'/><text x='105' y='{y}' font-size='15'>{}</text><text x='285' y='{y}' font-size='15'>{}</text><text x='570' y='{y}' font-size='15'>{}</text>",y+9,degrees[i],r["odd_exponent"],r["torsion_mask"]);
    }
    s += "<text x='28' y='435' font-size='14'>G = A7²; T = [A3, A7^34081013, A11^34081013, A61^34081013].</text><text x='28' y='459' font-size='14'>All 16 torsion products distinct; eight exact relations checked without a class-number oracle.</text><rect x='28' y='476' width='824' height='54' rx='5' fill='#f7f3e8'/><text x='43' y='499' font-size='14'>Standard proper-class-group and CM bridges are outside the partial Lean proof.</text><text x='43' y='520' font-size='14'>Whole-class equality remains uncertified. The experiment has enumerated a prefix.</text><text x='28' y='554' font-size='12'>Sources: results/class-relations.jsonl, verify_relations.gp, COMPONENT-CERTIFICATE.md.</text><text x='28' y='573' font-size='12'>Finite coordinate bijection: ComponentCoordinates.lean. Run 2026-10-18; exact witnesses, no interval.</text></g></svg>";
    fs::write(Path::new(&a[2]), s).unwrap();
}
