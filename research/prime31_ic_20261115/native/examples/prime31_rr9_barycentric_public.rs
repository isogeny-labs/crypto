//! Public-only B8 RR9 barycentric extraction and original-curve replay.
//! Completes 256 signed-fibre algebraic controls; NOT a B31 unknown-log solver.
#[path = "prime31_rr9_public_macaulay.rs"]
mod parent;

fn main() {
    println!("{}", serde_json::to_string_pretty(
        &parent::public_b8_barycentric_controls()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime31_public_residual_barycentric_positive_and_two_negatives() {
        let result = parent::public_b8_barycentric_controls();
        assert_eq!(result["p"], 2_147_483_647u64);
        assert_eq!(result["n"], 2_147_446_831u64);
        assert_eq!(result["base_x"], serde_json::json!([0,1,2,3,5,6,8,9]));
        assert_eq!(result["positive_residual"], serde_json::json!([1_911_579_686u64, 548_820_096u64]));
        assert_eq!(result["positive_chart_count"], 1);
        assert_eq!(result["positive_four_by_three_system_rank"], 3);
        assert_eq!(result["positive_first_chart_coefficients"],
            serde_json::json!([455835358u64, 518652820u64, 754716693u64,
                               932383545u64, 62510441u64, 1884059825u64, 1142049404u64]));
        assert_eq!(result["positive_original_curve_replay"], true);
        assert_eq!(result["negatives"][0]["charts"], 0);
        assert_eq!(result["negatives"][1]["charts"], 0);
        assert_eq!(result["sign_masks_enumerated_per_residual"], 256);
        assert_eq!(result["method_is_support_enumeration"], true);
        assert_eq!(result["nonenumerative_extraction"], false);
        assert_eq!(result["full_b31_unknown_ecdlp_solves"], 0);
        assert_eq!(result["pcc"], "unverified");
    }
}
