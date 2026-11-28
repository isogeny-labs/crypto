from pathlib import Path
import json,hashlib,datetime
r=Path('/workspace/relation-supervisor-live');s=r/'research/relation_fusion_20261117';d=s/'root-fixture-repair'
assert not (s/'ROOT_GEOMETRIC_SOURCE_386.json').exists()
(d/'main-385.rs').write_bytes((d/'main.rs').read_bytes())
src=(r/'src/cryptanalysis/binary_radical_root.rs').read_text();start=src.index('    fn factored_subspace_oracle_decides_as_subspace_oracle() {');body=src[start:];assert body.endswith('    }\n}\n');body=body[:-2]
body=body.replace('    fn factored_subspace_oracle_decides_as_subspace_oracle() {','pub fn run() {\n        let mut reports = vec![];',1)
body=body.replace('                assert!(yes > 10 && no > 10, "n={n}: yes={yes} no={no}");','''                let legacy_yes = yes;
                let legacy_no = no;
                let controls = geometric_controls(&base.span, b, &gf);
                for (xs, xr) in &controls {
                    assert_eq!(eval_s4_general(xs[0], xs[1], xs[2], *xr, b, &gf), 0);
                    let (want, _) = base.decompose(*xr, &gf);
                    let (got, _) = fac.decompose(*xr, &gf);
                    assert!(want.is_some(), "geometric control missing in original oracle");
                    let [a, c, d] = got.expect("geometric control missing in factored oracle");
                    assert!(fac.contains(a) && fac.contains(c) && fac.contains(d));
                    assert_eq!(eval_s4_general(a, c, d, *xr, b, &gf), 0);
                    yes += 1;
                }
                assert!(yes > 10 && no > 10, "n={n}: yes={yes} no={no}");
                reports.push(serde_json::json!({"n":n,"l":l,"trial":trial,"b":b,"basis":basis,"legacy_cases":200,"legacy_yes":legacy_yes,"legacy_no":legacy_no,"geometric_controls":controls,"combined_yes":yes,"combined_no":no}));''')
body=body[:-6]+'''        println!("{}", serde_json::json!({"schema":"root-geometric-positive-controls/v1","trials":reports,"legacy_cases":2400,"geometric_positive_controls":144,"original_seeds_and_assertions_preserved":true,"counts_are_not_yield_estimates":true,"performance_comparison":false}));
    }
'''
pre='''use crypto_lib::cryptanalysis::binary_radical_root::FactoredSubspaceOracle;
use crypto_lib::cryptanalysis::semaev_decomp::{Gf2, quartic_in_x3_general};
use rand::{rngs::StdRng, Rng, SeedableRng};
fn field(n: u32) -> Gf2 {
    Gf2::new(&crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible(n).expect("irreducible"))
}
// Lift points only over the existing factor-base span; no new RNG draws or seeds.
// Target selection uses group addition, independently of either decomposition oracle.
fn geometric_controls(span: &[u64], b: u64, gf: &Gf2) -> Vec<([u64;3],u64)> {
    use crypto_lib::cryptanalysis::ic_boundary::ArtinSchreier;
    use crypto_lib::cryptanalysis::koblitz_ic_leads::lambda::add;
    use std::collections::BTreeSet;
    let ash = ArtinSchreier::new(gf);
    let on_curve = |(x,y): (u64,u64)| gf.sqr(y) ^ gf.mul(x,y) == gf.mul(gf.sqr(x),x) ^ gf.sqr(x) ^ b;
    let mut points = vec![];
    for &x in span.iter().filter(|&&x| x != 0) {
        let rhs = x ^ 1 ^ gf.mul(b, gf.sqr(gf.inv(x)));
        if let Some(z) = ash.solve(rhs) {
            let y = gf.mul(x,z);
            assert!(on_curve((x,y)) && on_curve((x,y^x)));
            points.push((x,y)); points.push((x,y^x));
        }
    }
    let mut seen = BTreeSet::new(); let mut out = vec![];
    for i in 0..points.len() {
        for j in i+1..points.len() {
            for k in j+1..points.len() {
                let (p,q,r) = (points[i],points[j],points[k]);
                if p.0==q.0 || p.0==r.0 || q.0==r.0 { continue; }
                let sum = add(gf,1,add(gf,1,Some(p),Some(q)),Some(r));
                if let Some(t) = sum {
                    assert!(on_curve(t));
                    if span.contains(&t.0) || !seen.insert(t.0) { continue; }
                    assert_eq!(crypto_lib::cryptanalysis::semaev_decomp::eval_s4_general(p.0,q.0,r.0,t.0,b,gf),0);
                    out.push(([p.0,q.0,r.0],t.0));
                    if out.len()==12 { return out; }
                }
            }
        }
    }
    panic!("not enough distinct nondegenerate geometric controls: n={} count={}",gf.n,out.len());
}
'''
(d/'geometric.rs').write_text(pre+body)
main=(d/'main.rs').read_text().replace('pub use crypto_lib::{binary_ecc, hash};','pub use crypto_lib::{binary_ecc, hash, cryptanalysis};\nmod geometric;').replace('    assert!(["aliases-before", "aliases-after"].contains(&mode.as_str()));','    if mode == "geometric" { geometric::run(); return; }\n    assert!(["aliases-before", "aliases-after"].contains(&mode.as_str()));')
(d/'main.rs').write_text(main)
files={str(p.relative_to(r)):hashlib.sha256(p.read_bytes()).hexdigest() for p in d.rglob('*') if p.is_file()}
out={'schema':'root-geometric-positive-controls-source/v1','before_compile_and_execution':True,'root_source':'ROOT_UNIT_SOURCE_348.json','source_files':files,'preserve_original_seeded_cases_per_trial':200,'trials':12,'geometric_controls_per_trial':12,'seed_policy':'Keep original StdRng seeds900+n, no new RNG draws for controls, deterministic point lifts and group sums on original bases/curves','boundary':'Correctness fixture qualification through actual public production oracles; not a speed/yield experiment','root_files_changed':False,'performance_comparison':False,'15x_admitted':False}
(s/'ROOT_GEOMETRIC_SOURCE_386.json').write_text(json.dumps(out,indent=2)+'\n')
p=r/'docs/session-logs/2026-11-23.md';p.write_text(p.read_text()+f'''\n## {datetime.datetime.now(datetime.timezone.utc).isoformat()} —385alias outcome;386geometric-control qualification prepared

### What was done (commands, files, features)
385before exits1 with exactly the inherited generated31key miss; sorted385after exits0 on all45664 aliases, between-key and unknown inputs. All identities/metadata unchanged and production lookup code byte-identical. Prepared386 deterministic nondegenerate rational-point-sum controls on original12 field/basis trials, retaining all200 seeded cases/trial and original yes>10/no>10 assertions. Source pins recorded before compile/calls. [pcc:unverified]

### Decisions made (and why)
Use existing lambda::add and ArtinSchreier APIs; enumerate only points over existing small span, no whole-field scan. Require12 distinct targets outside span with3 distinct inputx, exact curve/S4 checks, and witnesses from both actual original oracles. Separate legacy/random counts from positive controls; no yield or performance conclusion. Conductor unavailable(command not found); documented docs/CONDUCTOR.md no-binary mode applies, no other agent here. KB unavailable; existing group-law/oracle/generator references reviewed.

### Open threads (unfinished, pick up next time)
Run386, retain any failure; integrate only qualified fixture/data repairs, then validate actual-source tests without overwriting frozen root executable. Publish378onward to samePR4989. Protocol question still unanswered, no actual comparative timing.

### Errors (what went wrong, how it was handled)
Full380 remains incomplete/failed; both384failures retained. No root348 files/data edited at this step;385/386are prototype/API scope only.
''')
print(json.dumps({'source_pins':len(files),'root_files_changed':False}))
