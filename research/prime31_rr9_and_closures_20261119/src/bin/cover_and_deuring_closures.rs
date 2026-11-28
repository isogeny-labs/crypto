use prime31_rr9_and_closures::cover_and_deuring::{
    run_part_a_cover_audit, run_part_b_deuring_rarity_audit,
};
use serde_json::json;

fn main() {
    let (toy_rows, pic1009, p192_closure) = run_part_a_cover_audit();
    let deuring = run_part_b_deuring_rarity_audit();

    let out = json!({
        "lead_p1_4_part_a_cover_closure": {
            "toy_nist_cover_ladder": toy_rows,
            "explicit_f1009_plane_quartic_pic0_audit": pic1009,
            "nist_p192_native_verification_and_gap_closure": p192_closure,
        },
        "lead_p1_4_part_b_deuring_vertical_isogeny_rarity": deuring,
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
