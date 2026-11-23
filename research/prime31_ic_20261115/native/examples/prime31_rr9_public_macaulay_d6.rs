//! Separate prospective degree-six B8 RR9 Macaulay test, no support enumeration in candidate.
//! Parent source is byte-identical for the published degree-five gate except
//! for the new public degree_six_controls() wrapper appended on this branch.
#[path = "prime31_rr9_public_macaulay.rs"]
mod parent;

fn main() {
    println!("{}", serde_json::to_string_pretty(&parent::degree_six_controls()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frozen_prime31_degree_six_public_rr9_panel() {
        let v = parent::degree_six_controls();
        assert_eq!(v["p"], 2_147_483_647u64);
        assert_eq!(v["n"], 2_147_446_831u64);
        assert_eq!(v["degree"], 6);
        assert_eq!(v["positive"]["macaulay"]["columns"], 1716);
        assert_eq!(v["positive"]["macaulay"]["rows_generated"], 2640);
        assert_eq!(v["positive"]["macaulay"]["unit_certificate"], false);
        assert_eq!(v["negatives"].as_array().unwrap().len(), 2);
        assert_eq!(v["complete_unknown_ecdlp_solves"], 0);
        for n in v["negatives"].as_array().unwrap() {
            let m = &n["macaulay"];
            assert_eq!(m["columns"], 1716);
            assert_eq!(m["rows_generated"], 2640);
            assert_eq!(m["candidate_chart_coefficients"], serde_json::Value::Null);
            if m["unit_certificate"] == true {
                assert_eq!(m["replay"]["identity_replayed"], true);
            }
        }
    }
}
