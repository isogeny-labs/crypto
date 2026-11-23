//! Native identities derived from the exact constructor used by the benchmark.
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_frobenius_factor_base, KoblitzCurve,
};
use serde_json::json;
fn main() {
    let mut curves = Vec::new();
    for n in [17u32, 19, 23] {
        let kc = KoblitzCurve::new(1, n).unwrap();
        let fb = build_frobenius_factor_base(&kc, 0).unwrap();
        let id = kc.curve_id();
        curves.push(json!({"n":n,"slug":id.slug,"icv1":id.icv1,"model_json":id.model_json,
            "group_order":kc.group_order.to_str_radix(10),"subgroup_order":kc.subgroup_order.to_str_radix(10),"cofactor":kc.cofactor.to_str_radix(10),
            "factor_base_points":fb.points.len(),"factor_base_divisor_index":0,"boolean_variables":2*fb.subspace_basis.len(),
            "subspace_basis":fb.subspace_basis.iter().map(|v|v.raw_bits().to_vec()).collect::<Vec<_>>(),"generator":format!("{:?}",kc.generator())}));
    }
    println!("{}", serde_json::to_string_pretty(&curves).unwrap());
}
