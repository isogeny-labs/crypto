use prime31_rr9_and_closures::{
    cover_and_deuring::{run_part_a_cover_audit, run_part_b_deuring_rarity_audit},
    p2_6_suite::run_lead_p2_6_suite,
    prime31_field::PRIME31,
};
use serde_json::json;
use std::path::PathBuf;

fn main() {
    let p2_6 = run_lead_p2_6_suite();
    let (toy_rows, pic1009, p192_closure) = run_part_a_cover_audit();
    let deuring = run_part_b_deuring_rarity_audit();

    let results = json!({
        "metadata": {
            "workstream": "Workstream 3 — Prime31 RR9 Non-Enumerative Extraction & Auxiliary Cover / Deuring Closures",
            "date": "2026-11-19",
            "branch": "cursor/prime31-rr9-and-closures-3858",
            "prime31_curve": {
                "p": PRIME31.p,
                "a": PRIME31.a,
                "b": PRIME31.b,
                "order_n": PRIME31.n,
                "generator_g": [PRIME31.g.0, PRIME31.g.1],
            },
            "zero_python_native_rust": true,
            "forbidden_tuple_enumeration_calls": p2_6.blinded_grader_controls.total_forbidden_calls,
        },
        "lead_p2_6_prime31_rr9_non_enumerative": p2_6,
        "lead_p1_4_part_a_auxiliary_cover_closure": {
            "toy_nist_cover_ladder": toy_rows,
            "explicit_f1009_plane_quartic_pic0_audit": pic1009,
            "nist_p192_native_verification_and_gap_closure": p192_closure,
        },
        "lead_p1_4_part_b_deuring_vertical_isogeny_rarity": deuring,
    });

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out_path = manifest_dir.join("RESULTS.json");
    let pretty = serde_json::to_string_pretty(&results).expect("serialize RESULTS.json");
    std::fs::write(&out_path, format!("{pretty}\n")).expect("write RESULTS.json");
    println!("Wrote deterministic RESULTS.json to {}", out_path.display());
}
