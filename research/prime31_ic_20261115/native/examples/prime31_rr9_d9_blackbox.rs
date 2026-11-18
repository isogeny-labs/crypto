//! B8 RR9 D9 public-residual-only negative rank gate.
//! Uses existing native Rust elliptic curve arithmetic and public RR9 quadrics.
//! Usage: prime31_rr9_d9_blackbox <frozen_negative_index:0|1>
//! No complete ECDLP or sign-independent positive extraction is claimed.
#[path = "prime31_rr9_public_macaulay.rs"]
mod parent;

fn main() {
    let case: usize = std::env::args().nth(1)
        .expect("specify frozen negative case 0 or 1")
        .parse().expect("integer case");
    let result = parent::rr9_d9_blackbox_controls(case);
    println!("{}", serde_json::to_string_pretty(&result).expect("serializable result"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime31_rr9_d9_sparse_operator_and_rank_falsifiers() {
        let got = parent::rr9_d9_blackbox_smoke();
        assert_eq!(got["schema"], "prime31.rr9.d9-blackbox-smoke.v1");
        assert_eq!(got["curve_p"], 2147483647);
        assert_eq!(got["order_n"], 2147446831);
        assert_eq!(got["macaulay_d9_rows"], 27456);
        assert_eq!(got["macaulay_d9_columns"], 11440);
        assert!(got["d9_nnz"].as_u64().unwrap() > 27000);
        assert_eq!(got["d3_sparse_equals_independent_dense"], true);
        assert_eq!(got["rank2_positive_gate"], true);
        assert_eq!(got["singular_negative_gate"], true);
        assert_eq!(got["original_positive_d9_kernel_test"], true);
        assert_eq!(got["complete_unknown_ecdlp_solves"], 0);
    }
}
