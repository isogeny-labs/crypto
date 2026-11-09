#[cfg(test)]
mod projective_f17_semantic_probe_tests {
    use super::*;

    #[test]
    fn author_dbl2b0_zero_weight_counterexample() {
        let j = Jacobian {
            f: Field::new(17, 1, 0),
            polynomial: vec![2, 1, 0, 0, 0, 1],
        };
        let d = Divisor { degree: 2, u: [5, 6, 1], v: [0, 0] };
        assert!(j.valid(d));
        assert_eq!(j.add(d, d), Divisor::O);
        let wd = weight_degree_two(d, 1, 1, &j.f);
        let (raw, label) = double22_weighted(wd, &j.polynomial, &j.f);
        assert_eq!(label, "b_zero_identity");
        assert_eq!(raw, ResultPoint::Identity);
        let (dispatched, dispatch_label) = add22_weighted(wd, wd, &j.polynomial, &j.f);
        assert_eq!(dispatch_label, "same_u_inverse");
        assert_eq!(dispatched, Some(ResultPoint::Identity));
        assert_eq!(normalize(raw, &j.f), Divisor::O);
        println!("F17_TORSION_JSON {}", serde_json::json!({
            "p":17,"f":[2,1,0,0,0,1],"u":[5,6,1],"v":[0,0],
            "input_weighted":wd,"rust_direct_branch":label,"rust_dispatch_branch":dispatch_label,"rust_raw":"Identity",
            "rust_normalized":"Identity","cantor":"Identity"
        }));
    }
}
