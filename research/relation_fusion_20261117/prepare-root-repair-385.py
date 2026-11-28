from pathlib import Path
import json,hashlib,shutil,datetime
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';d=s/'root-fixture-repair';d.mkdir()
manifest=(s/'root-algebra-tests/Cargo.toml').read_text().replace('relation-root-algebra-tests','relation-root-fixture-repair');manifest+='\n[[bin]]\nname = "root-fixture-repair"\npath = "main.rs"\n';(d/'Cargo.toml').write_text(manifest);(d/'lib.rs').write_text('pub use crypto_lib;\n')
lock=(s/'root-algebra-tests/Cargo.lock').read_text();assert lock.count('name = "relation-root-algebra-tests"')==1;(d/'Cargo.lock').write_text(lock.replace('name = "relation-root-algebra-tests"','name = "relation-root-fixture-repair"'))
old=(repo/'src/cryptanalysis/curve_aliases.json').read_text();duplicates=[]
def unique(pairs):
 names=[k for k,v in pairs];assert len(names)==len(set(names)),'duplicate JSONkeys';return dict(pairs)
v=json.loads(old,object_pairs_hook=unique);beforekeys=list(v['aliases']);sortedv=dict(v);sortedv['aliases']=dict(sorted(v['aliases'].items()));new=json.dumps(sortedv,indent=0,ensure_ascii=False)+'\n';assert json.loads(new)==v;assert beforekeys!=sorted(beforekeys)
for folder,text in [('old',old),('sorted',new)]:
 p=d/folder;p.mkdir();shutil.copy2(repo/'src/cryptanalysis/curve_id.rs',p/'curve_id.rs');(p/'curve_aliases.json').write_text(text)
(d/'main.rs').write_text('''pub use crypto_lib::{binary_ecc, hash};
#[allow(dead_code)]
#[path = "old/curve_id.rs"]
mod legacy;
#[allow(dead_code)]
#[path = "sorted/curve_id.rs"]
mod sorted;
use serde_json::json;
fn main() {
    let mode = std::env::args().nth(1).expect("aliases-before or aliases-after");
    assert!(["aliases-before", "aliases-after"].contains(&mode.as_str()));
    let table: serde_json::Value = serde_json::from_str(include_str!("old/curve_aliases.json")).unwrap();
    let map = table["aliases"].as_object().unwrap();
    let resolve: fn(&str)->Option<&'static str> = if mode == "aliases-before" { legacy::resolve } else { sorted::resolve };
    let mut misses = vec![];
    for (key,value) in map {
        assert_eq!(sorted::normalise(key), *key, "reference key must already be normalized");
        let expected = value.as_str();
        let got = resolve(key);
        if got != expected { misses.push(json!({"name":key,"expected":expected,"actual":got})); }
        let between=format!("{key}~");
        assert_eq!(resolve(&between), map.get(&between).and_then(|v| v.as_str()), "between {key}");
    }
    for key in ["", "no-such-curve", "icv1-fp20-t0-00000000", "a\\\"b", "a\\\\b", "secp256k", "!", "~~~~", "0"] {
        assert_eq!(resolve(key), map.get(key).and_then(|v|v.as_str()));
    }
    println!("{}",json!({"schema":"root-alias-order-qualification/v1","mode":mode,"aliases_checked":map.len(),"mismatches":misses,"all_values_preserved":true,"performance_comparison":false}));
    if !misses.is_empty() { std::process::exit(1); }
}
''')
r={'schema':'root-repair-prototype-source/v1','recorded_before_compile_and_calls':True,'root_source':'ROOT_UNIT_SOURCE_348.json','purpose':'Qualify an ordering-only alias-table correction using byte-identical productioncurve_id module and sameactual crypto native352dependency closure. No root production source/data changed yet.','reference':'scripts/curve_id.py alias_map_text requires aliases sorted bykey; existing original curve_id equalitytest retainedas reference.','alias_values_and_metadata_equal':True,'original_alias_order_bad':True,'files':{str(p.relative_to(repo)):hashlib.sha256(p.read_bytes()).hexdigest() for p in d.rglob('*') if p.is_file()},'performance_observations_collected':0,'15x_admitted':False};(s/'ROOT_REPAIR_PROTOTYPE_SOURCE_385.json').write_text(json.dumps(r,indent=2)+'\n')
p=repo/'docs/session-logs/2026-11-23.md';p.write_text(p.read_text()+f'''\n## {datetime.datetime.now(datetime.timezone.utc).isoformat()} —384exact failures classified;385ordering-only prototype

### What was done (commands, files, features)
Both384isolated tests fail101 as before. Binaryradical test reaches coverageassertion n19yes10/no190, no equivalence/witness mismatch before that assertion. CurveID fails generated-31bit-2147483647 lookup None versus existingmapvalue; tableline172isoutoforder amonggenerated-24bit aliases. Reference scripts/curve_id.py alias_map_text explicitly sorts aliases. New385smallharness copies productioncurve_id.rs byte-exact into old/sorted namespaces and uses semanticallyidentical, fullysorted aliasJSON copy. All root production files/data remain unchanged.

### Decisions made (and why)
Do not lower yes>10 or remove the rare-case test. Prepare deterministic rational-point-sum positive controls while retaining every old random case, then qualify before any root test edit. For curveID, preserve every alias/value/metadata and the binary-search algorithm; restore generator-required ordering instead of replacing it with slower HashMap fallback or changing identities. KB unavailable: scripts/research-kb status reports no executable; fallback sources are existing module contracts, scripts/curve_id.py alias_map_text, and koblitz_ic_leads::lambda::add/points for established group-law point controls. No novelty or new curve-fact claim.

### Open threads (unfinished, pick up next time)
Run385old negative and sorted positive lookup against allmap keys/between/unknown inputs; then qualify true-planted S4 controls through actual public root oracles, preserve all oldcases/assertions. Root348 source remains pinned until prototype evidence justifies edits. Protocol decision pending; no actual speed measurements.

### Errors (what went wrong, how it was handled)
Full380unqualified/incomplete and384failures retained. Earlier path guesses point.rs/field.rs were wrong; actualbinary point/group sources found byrg. This is validation/fixture repair, not an established runtime15× improvement.\n''')
print(json.dumps({'prototype_sources_pinned':len(r['files']),'alias_entries':len(v['aliases']),'all_alias_values_and_metadata_preserved':True,'root_source_unchanged':True}))
