// Built by the validator against the library the replay binaries used (commit edea86713):
//   rustc --edition 2021 -O -o instances instances.rs \
//     --extern crypto_lib=target/release/deps/libcrypto_lib-338c039ba089a57d.rlib -L dependency=target/release/deps
//   ./instances > instances.jsonl && python3 check_instances.py instances.jsonl instance_checks.json
// Validator scratch harness (not part of the repository): regenerate the E19 / E24 / E25
// instances exactly as the pipelines do, and print the curve, generator, target, planted
// scalar and group data as JSON lines, so an independent Python implementation can check
// [d]G = Q, [n]G = O, the j-invariant's field of definition, and psi on the GLS curves.
use crypto_lib::cryptanalysis::gaudry_quartic::{E4, Pt4};
use crypto_lib::cryptanalysis::quartic_folds::{
    generate_gls_klein_instance, generate_klein_instance, generate_translation_instance,
    FoldInstance4,
};

fn e(v: &E4) -> String {
    format!("[{},{},{},{}]", v.0[0], v.0[1], v.0[2], v.0[3])
}
fn pt(v: &Pt4) -> String {
    format!("{{\"x\":{},\"y\":{},\"inf\":{}}}", e(&v.x), e(&v.y), v.inf)
}
fn line(exp: &str, seed: u64, i: &FoldInstance4) {
    let gls = match i.gls {
        Some((cx, cy)) => format!("{{\"cx\":{},\"cy\":{}}}", e(&cx), e(&cy)),
        None => "null".into(),
    };
    let f = i.curve().f;
    println!(
        "{{\"experiment\":\"{exp}\",\"p\":{},\"seed\":{seed},\"fp4_c\":{},\"a\":{},\"b\":{},\"n\":{},\"group_order\":{},\"cofactor\":{},\"g\":{},\"q\":{},\"d\":{},\"lambda\":{},\"gls\":{gls}}}",
        i.p, f.c, e(&i.a), e(&i.b), i.n, i.group_order, i.cofactor, pt(&i.g), pt(&i.q), i.d, i.lambda
    );
}

fn main() {
    for &p in &[53u64, 101, 149, 197, 269] {
        for s in 1..=2u64 {
            match generate_translation_instance(p, s, 4) {
                Ok(i) => line("e19", s, &i),
                Err(err) => eprintln!("e19 {p} {s}: {err}"),
            }
        }
    }
    for &p in &[101u64, 149, 197, 269, 397, 521] {
        for s in 1..=2u64 {
            match generate_klein_instance(p, s, 16) {
                Ok(k) => line("e24", s, &k.inst),
                Err(err) => eprintln!("e24 {p} {s}: {err}"),
            }
        }
    }
    for &p in &[101u64, 149, 197, 269, 397] {
        for s in 1..=2u64 {
            match generate_gls_klein_instance(p, s, 16) {
                Ok(k) => line("e25", s, &k.inst),
                Err(err) => eprintln!("e25 {p} {s}: {err}"),
            }
        }
    }
}
