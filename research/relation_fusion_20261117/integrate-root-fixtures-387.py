from pathlib import Path
import json,hashlib,datetime,struct
r=Path('/workspace/relation-supervisor-live');s=r/'research/relation_fusion_20261117';d=s/'root-fixture-repair'
assert not (s/'ROOT_FIXTURE_SOURCE_387.json').exists()
j=json.loads((s/'development-01/root-geometric-386.stdout').read_text());assert len(j['trials'])==12;assert all(v['legacy_cases']==200 and len(v['geometric_controls'])==12 for v in j['trials']);assert sum(v['legacy_yes']+v['legacy_no'] for v in j['trials'])==2400
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
(s/'ROOT_GEOMETRIC_OUTCOME_386.json').write_text(json.dumps({'schema':'root-geometric-positive-controls-outcome/v1','source':'ROOT_GEOMETRIC_SOURCE_386.json','exit_code':0,'launch_path_error_exit127_preserved':True,'legacy_cases':2400,'geometric_positive_controls':144,'all_original_assertions_pass':True,'root_files_changed':False,'raw_stdout_sha256':sha(s/'development-01/root-geometric-386.stdout'),'raw_stderr_sha256':sha(s/'development-01/root-geometric-386.stderr'),'performance_comparison':False,'15x_admitted':False},indent=2)+'\n')
p=r/'src/cryptanalysis/binary_radical_root.rs';old=p.read_text();(d/'binary_radical_root-before-387.rs').write_text(old)
g=(d/'geometric.rs').read_text();a=g.index('fn geometric_controls(');b=g.index('pub fn run()');helper=g[a:b].replace('fn geometric_controls(', '    fn geometric_positive_controls(',1)
# Keep all production code and the original200case loop byte-for-byte except its misleading comment.
needle='    #[test]\n    fn factored_subspace_oracle_decides_as_subspace_oracle() {'
assert old.count(needle)==1
new=old.replace(needle,'''    // Deterministic rational-point sums give genuine positive controls without
    // changing the original seeded stream. Counts here are correctness coverage,
    // not a relation-yield estimate. Require distinct inputs/targets outside V.
'''+helper+'\n'+needle)
needle='                assert!(yes > 10 && no > 10, "n={n}: yes={yes} no={no}");';assert new.count(needle)==1
extra='''                // The bounded scan above can miss every rational root at larger n.
                // Add genuine positives after all original cases and RNG draws.
                for (xs, xr) in geometric_positive_controls(&base.span, b, &gf) {
                    assert_eq!(eval_s4_general(xs[0], xs[1], xs[2], xr, b, &gf), 0);
                    let (want, _) = base.decompose(xr, &gf);
                    let (got, _) = fac.decompose(xr, &gf);
                    assert!(want.is_some(), "geometric control missing in original oracle");
                    let [a, c, d] = got.expect("geometric control missing in factored oracle");
                    assert!(fac.contains(a) && fac.contains(c) && fac.contains(d));
                    assert_eq!(eval_s4_general(a, c, d, xr, b, &gf), 0);
                    yes += 1;
                }
'''
new=new.replace(needle,extra+needle).replace('// Plant targets as x of an S₄ root over the base, half the time.','// Attempt a bounded S₄-root scan over the base, half the time.')
assert new.split('#[cfg(test)]')[0]==old.split('#[cfg(test)]')[0];p.write_text(new)
a=r/'src/cryptanalysis/curve_aliases.json';assert a.read_bytes()==(d/'old/curve_aliases.json').read_bytes();a.write_bytes((d/'sorted/curve_aliases.json').read_bytes());assert json.loads(a.read_bytes())==json.loads((d/'old/curve_aliases.json').read_bytes())
(d/'actual-module-tests.rs').write_text('''//! Compile the actual changed source modules and their unchanged original tests.
//! Other public APIs link to frozen source348/default-features-false library352.
//! This qualifies these modules, not the full root-crate libtest target.
pub use crypto_lib::{binary_ecc, hash, cryptanalysis};
pub use crypto_lib::cryptanalysis::semaev_decomp;
#[path = "../../../src/cryptanalysis/binary_radical_root.rs"]
mod binary_radical_root;
#[path = "../../../src/cryptanalysis/curve_id.rs"]
mod curve_id;
''')
build=Path('/workspace/crypto-root-unoptimized-181/release/build');fp=json.loads((build/'relation-root-fixture-repair/5839ab0629ab7e89/fingerprint/bin-root-fixture-repair.json').read_text())
deps={}
for row in fp['deps']:
 name=row[1]
 if name not in ['crypto_lib','rand','num_bigint','num_traits','serde_json']:continue
 fingerprint=row[3].to_bytes(8,'little').hex();candidates=[]
 for f in build.glob(f'*/**/fingerprint/lib-{name}'):
  if f.read_text().strip()==fingerprint:
   candidates+=list((f.parent.parent/'out').glob(f'lib{name}-*.rlib'))
 assert len(candidates)==1,(name,candidates);deps[name]=str(candidates[0])
args=['/home/agent/.cargo/bin/rustc','--edition=2021','--test',str(d/'actual-module-tests.rs'),'--crate-name','root_fixture_modules_387','-C','opt-level=0','-C','codegen-units=16','-C','debuginfo=0','-C','debug-assertions=off','-o','/workspace/root-fixture-modules-387']
for name,path in deps.items():args+=['--extern',f'{name}={path}']
for out in sorted(build.glob('*/*/out')):args+=['-L',f'dependency={out}']
context={'schema':'actual-root-fixture-module-source/v1','recorded_before_compile_and_tests':True,'source_parent':'ROOT_UNIT_SOURCE_348.json','root_changes':{str(p.relative_to(r)):sha(p) for p in [p,a]},'scope':'Compile actual source files with all7 original module tests. Public dependencies link to frozen root352 default-features-false library. Not a full root rebuild or full suite pass.','original_200_case_loop_per_trial_preserved':True,'original_coverage_assertion_preserved':True,'production_oracle_source_bytes_unchanged':True,'all_alias_values_and_metadata_preserved':True,'test_harness_sha256':sha(d/'actual-module-tests.rs'),'extern_rlib_sha256':{k:{'path':v,'sha256':sha(Path(v))} for k,v in deps.items()},'rustc_command':args,'performance_comparison':False,'15x_admitted':False}
(s/'ROOT_FIXTURE_SOURCE_387.json').write_text(json.dumps(context,indent=2)+'\n')
p=r/'docs/session-logs/2026-11-23.md';p.write_text(p.read_text()+f'''\n## {datetime.datetime.now(datetime.timezone.utc).isoformat()} —386controls pass;387actual module integration prepared

### What was done (commands, files, features)
386 exits0: all2400 original cases and144 geometric positives pass both production public oracles with exact witness checks. Original seeded trial counts are separately retained in rawJSON, including n23random-base legacyyes0 and1. Corrected root cfg(test) fixture by adding deterministic point controls after every original draw, preserving originalasserts, and restored alias generator ordering without changing any45664values/metadata. Prospective387pins actual changed modules, harness and exact cached externrlibs. [pcc:unverified]

### Decisions made (and why)
No full root rebuild fits current398MiBfree. Compile the two actual changed source modules and all7 original unit tests against frozen actual root352dependencies via rustc; this checks source/includes directly and never overwrites frozen557MiB350ELF. Scope remains module qualification, not full-suitepass. Rootproduction oracle bytes before cfg(test) are identical; onlytestfixture andaliasordering change. No actual comparative timing or yield claim.

### Open threads (unfinished, pick up next time)
Compile/run387 actual module tests, retain diagnostics, archive and publish378onward onPR4989; full8216suite remains unqualified from380. Pendingcanonicalprotocol decision/access unchanged.

### Errors (what went wrong, how it was handled)
First386 launch exit127 because cargo absent from PATH, rawpathfailure retained; explicit /home/agent/.cargo/bin route succeeds without sourcechange. All380/384failures remain. No blanketcachedeletion or frozenartifact overwrite.
''')
print(json.dumps({'changed_root_files':list(context['root_changes']),'externs':list(deps),'source_pinned':True}))
