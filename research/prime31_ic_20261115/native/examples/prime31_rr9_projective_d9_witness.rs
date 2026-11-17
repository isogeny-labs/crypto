//! Bounded projective degree-9 positive-witness gate, #4550.
//! Uses the original pinned prime31 Rust arithmetic and existing public
//! RR9 coefficient quadrics. Does NOT compute D9 rank, extract an unknown
//! public residual support or recover an unknown discrete logarithm.
#[path = "prime31_rr9_public_macaulay.rs"]
mod parent;

fn main() {
    println!("{}", serde_json::to_string_pretty(
        &parent::projective_degree_nine_witness_controls()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frozen_prime31_d9_sparse_evaluation_null_witness() {
        let j=parent::projective_degree_nine_witness_controls();
        assert_eq!(j["p"],2_147_483_647u64);
        assert_eq!(j["n"],2_147_446_831u64);
        assert_eq!(j["base_x"],serde_json::json!([0,1,2,3,5,6,8,9]));
        assert_eq!(j["positive_residual"],
                   serde_json::json!([1_911_579_686u64,548_820_096u64]));
        assert_eq!(j["degree"],9);
        assert_eq!(j["macaulay_columns"],11_440);
        assert_eq!(j["macaulay_rows_checked"],27_456);
        assert_eq!(j["all_D9_rows_annihilated"],true);
        assert_eq!(j["evaluation_at_h_equals_one_nonzero"],true);
        assert_eq!(j["degree_nine_rank_upper_bound_for_positive"],11_439);
        assert_eq!(j["negative_degree_nine_rank"],serde_json::Value::Null);
        assert_eq!(j["complete_unknown_ecdlp_solves"],0);
        assert_eq!(j["public_only_non_enumerative_extractor_implemented"],false);
    }
}
