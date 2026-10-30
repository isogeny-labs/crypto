//! Native registration of frozen, exactly ordered experimental curves.
use crypto_lib::cryptanalysis::{curve_id, three_leads::StudyInstance};
use num_bigint::BigUint;
use serde_json::{json, Value};
fn digits(mut x: u64, p: u64, n: usize) -> Vec<u64> {
    (0..n)
        .map(|_| {
            let a = x % p;
            x /= p;
            a
        })
        .collect()
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let path = &args[1];
    let mut registry: Value =
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let rows = registry["curves"].as_array_mut().unwrap();
    let mut count = 0;
    for manifest in &args[2..] {
        let fixtures: Vec<StudyInstance> =
            serde_json::from_str(&std::fs::read_to_string(manifest).unwrap()).unwrap();
        for i in fixtures {
            i.validate().unwrap();
            let f = &i.curve.f;
            let order = BigUint::from(i.order);
            let id = if i.curve.binary {
                curve_id::binary(
                    f.n as u32,
                    &f.modulus.into(),
                    &i.curve.c[2].into(),
                    &i.curve.c[0].into(),
                    &order,
                    None,
                )
            } else if f.n == 1 {
                curve_id::prime(
                    &f.p.into(),
                    &i.curve.c[1].into(),
                    &i.curve.c[0].into(),
                    &order,
                )
            } else {
                curve_id::extension(
                    f.p,
                    &digits(f.modulus, f.p, f.n),
                    &digits(i.curve.c[1], f.p, f.n),
                    &digits(i.curve.c[0], f.p, f.n),
                    &order,
                )
            }
            .unwrap();
            if rows.iter().any(|r| r["slug"] == id.slug) {
                continue;
            }
            let parts: Vec<_> = id.icv1.split(':').collect();
            rows.push(json!({"slug":id.slug,"icv1":id.icv1,"family":if f.p==2{"binary"}else if f.n==1{"prime"}else{"extension"},"params":{"p":f.p,"degree":f.n,"modulus":f.modulus,"a":if i.curve.binary{i.curve.c[2]}else{i.curve.c[1]},"b":i.curve.c[0]},"model_json":id.model_json,"trace":parts[2],"order":i.order.to_string(),"j":parts[4],"end":parts[5],"aliases":[],"standard_names":[],"sources":[manifest],"representations":[],"ec1_unresolved":"Explicit synthetic study fixture; ICV1 model and prime subgroup/order witness recorded in the source manifest."}));
            count += 1;
        }
    }
    std::fs::write(
        path,
        serde_json::to_string_pretty(&registry).unwrap() + "\n",
    )
    .unwrap();
    println!("registered {count} new exact curve models");
}
