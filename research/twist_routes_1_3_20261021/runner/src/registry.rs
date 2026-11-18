//! Canonical model metadata shared by the experiment executables.
use crate::{source_sha256, toy_field::inv};
use serde_json::{json, Value};
pub fn digest(bytes: &[u8]) -> String {
    hex::encode(source_sha256::sha256(bytes))
}
pub fn identity(p: u64, a: u64, b: u64, order: u128, modulus: Option<&[u64]>) -> Value {
    let j = 1728 % p * 4 % p * a % p * a % p * a % p
        * inv((4 * a % p * a % p * a + 27 * b % p * b) % p, p)
        % p;
    let (model, field, tag, q, family, params, jstr) = if let Some(md) = modulus {
        let hash = digest(
            format!(
                "fpk-modulus:{p}:{}",
                md.iter().map(u64::to_string).collect::<Vec<_>>().join(",")
            )
            .as_bytes(),
        );
        let field = format!("fpk-{p}-{}-{}", md.len(), &hash[..8]);
        let mut aa = vec!["0".to_string(); md.len()];
        aa[0] = a.to_string();
        let mut bb = aa.clone();
        bb[0] = b.to_string();
        let mut jj = aa.clone();
        jj[0] = j.to_string();
        let model = json!({"a":aa,"b":bb,"field":field,"form":"y^2=x^3+a*x+b","k":md.len().to_string(),"modulus":md.iter().map(u64::to_string).collect::<Vec<_>>(),"p":p.to_string(),"v":"1"});
        let params = json!({"p":p.to_string(),"k":md.len().to_string(),"a":aa,"b":bb,"modulus":md.iter().map(u64::to_string).collect::<Vec<_>>()});
        (
            model,
            field,
            format!("fp{}k{}", 64 - p.leading_zeros(), md.len()),
            (p as u128).pow(md.len() as u32),
            "extension",
            params,
            jj.join(","),
        )
    } else {
        let field = format!("fp-{p}");
        (
            json!({"a":a.to_string(),"b":b.to_string(),"field":field,"form":"y^2=x^3+a*x+b","p":p.to_string(),"v":"1"}),
            field,
            format!("fp{}", 64 - p.leading_zeros()),
            p as u128,
            "prime",
            json!({"p":p.to_string(),"a":a.to_string(),"b":b.to_string()}),
            j.to_string(),
        )
    };
    let trace = q as i128 + 1 - order as i128;
    let mj = serde_json::to_string(&model).unwrap();
    let hash = digest(mj.as_bytes());
    let tr = if trace < 0 {
        format!("m{}", -trace)
    } else {
        trace.to_string()
    };
    let slug = format!("icv1-{tag}-t{tr}-{}", &hash[..8]);
    json!({"slug":slug,"icv1":format!("ICV1:{field}:{trace}:{order}:{jstr}:unk:unk:r:{}",&hash[..12]),"family":family,"params":params,"model_json":mj,"trace":trace.to_string(),"order":order.to_string(),"j":jstr,"end":"unk","aliases":[],"standard_names":[],"sources":["research/twist_routes_1_3_20261021/execution-v1/fixtures.json"],"representations":[],"ec1_unresolved":"Exact field, whole order, source subgroup generator and verified tower-to-flat generator are frozen in the linked execution fixture; no cross-repository EC1 comparison is claimed."})
}
