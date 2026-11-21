"""Source and file custody only, before any new scientific implementation."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,shutil
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_primefield_cost_20261117';old=r/'research/chained_cubic_inverse_20261117'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
original=json.loads((old/'ORIGINAL_INPUTS_BEFORE_CODE_15.json').read_text());lock=json.loads((old/'SOURCE_LOCK_15B.json').read_text());guards={x['original_path']:x for x in original['original_source_files']}
for row in lock['files']:
 p=old/row['path'];assert sha(p)==row['sha256'];guards[str(p)]={'original_path':str(p),'bytes':p.stat().st_size,'sha256':sha(p)}
for p in [old/'SOURCE_LOCK_15B.json',old/'first-frozen-15B-success-02/23-first-frozen-arithmetic.stdout',old/'first-frozen-15B-success-02/ACTUAL_FIRST15B_SUCCESS.json',old/'first-frozen-15B-success-02/19-first-frozen-Lean-proof.stdout']:
 guards[str(p)]={'original_path':str(p),'bytes':p.stat().st_size,'sha256':sha(p)}
registry=Path('/home/agent/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/num-bigint-0.4.8');checksum=json.loads((s/'OFFICIAL_NUM_BIGINT_SOURCE_CUSTODY.json').read_text());assert checksum['sha256']=='c89e69e7e0f03bea5ef08013795c25018e101932225a656383bd384495ecc367'
copies={'fp_poly15B.rs':old/'vendor/fp_poly.rs','CubicField15B.rs':old/'src/nonsplit.rs','Field_map15B.rs':old/'src/nonsplit_map.rs','num-bigint-048-power.rs':registry/'src/biguint/power.rs','num-bigint-048-monty.rs':registry/'src/biguint/monty.rs','num-bigint-048-BigUint.rs':registry/'src/biguint.rs','num-bigint-048-Cargo.lock-source-package-checksum.json':s/'OFFICIAL_NUM_BIGINT_SOURCE_CUSTODY.json'}
(s/'primary-inputs').mkdir(exist_ok=False)
for name,p in copies.items():
 dst=s/'primary-inputs'/name;shutil.copy2(p,dst);assert dst.read_bytes()==p.read_bytes();guards[str(p)]={'original_path':str(p),'bytes':p.stat().st_size,'sha256':sha(p)}
(s/'original-complete15B-scientific-snapshot.stdout').write_bytes((old/'first-frozen-15B-success-02/23-first-frozen-arithmetic.stdout').read_bytes())
for row in guards.values():assert sha(Path(row['original_path']))==row['sha256']
metadata={'schema':'prospective-prime-field-cost16-inputs/v1','prepared':datetime.now(timezone.utc).isoformat(),'task_tracking_commit':'014387238b9aee0afeafd37b2ce34fe475013160','successful_mathematical_source15B':'fe5d0da839e0f9496f7b0326feebbedb9b80cc64','actual_scientific_result15':'57199d2b3db5d1e64fa5627c7e97d6edb0fd0269','original_source_files':list(guards.values()),'entire_prior_dependency_archive':original['entire_prior_dependency_archive'],'exact_prebuilt_Rust1991_dependencies':original['exact_prebuilt_Rust1991_dependencies'],'dependency_source_scope':'same exact prebuilt binary dependencies, registry0.4.8 source checked against Cargo.lock; no new pre-call dependency build is claimed','completed_Lean_proof_reused':'PCC-20261117-69b890; source/actual full output guarded, not rerun for a new theorem','frozen_scientific_snapshot':{'path':'original-complete15B-scientific-snapshot.stdout','bytes':(s/'original-complete15B-scientific-snapshot.stdout').stat().st_size,'sha256':sha(s/'original-complete15B-scientific-snapshot.stdout')},'new_scientific_code_compiler_and_native_calls':0}
(s/'ORIGINAL_INPUTS_BEFORE_NEW_CODE_16.json').write_text(json.dumps(metadata,indent=2)+'\n');shutil.copy2(Path(__file__),s/'prepare-prospective-inputs-metadata-only.py')
(s/'SOURCES_CHECKED_16.md').write_text('''# Sources checked before cost16 approach/code

KB/theorem/Conductor tools are not loaded. Exact status wrapper and Conductor
calls both return127, retained in KB_CONDUCTOR_STATUS_16.json. Intended queries
and the source fallback are recorded there; no novelty/absence inference.
[pcc:unverified]

- Successful15B Field/CubicField/FpPoly/closed-fibre source and full native output:
  preserved primary files and input guard manifest. The inner Euclidean inverse
  may change its prime-field work when its input changes from a² to a.
- Actual num-bigint0.4.8 registry source and Cargo.lock checksum c89e69e7: public
  BigUint::modpow delegates to power::modpow, which selects monty_modpow for odd
  modulus. Therefore I remains atomic; no guessed internal exponentiation count.
  Public source: https://docs.rs/num-bigint/0.4.8/src/num_bigint/biguint/power.rs.html
- Exact FpPoly source exposes16 operator sites; CubicField.c adds the u64
  remainder needed inside inversion. Separate inverse-scope and whole-backend
  budgets prevent uninstrumented Fp2/u64 work from being called full map cost.
- Current AGENTS implementation/custody and cost/admission sections; original
  /workspace/crypto/docs/CONDUCTOR.md documents hooks allowing edits when absent.
  No new benchmark/admission/budget exception is created.

All formulae and old results are reused, without a new field/Picard theorem or
claim of fastest arithmetic. The twelve hand oracles and one counter mutant
are fixed before code; all106 prior scientific rows stay in the population.
[pcc:unverified]
''')
print(json.dumps({'original_guards':len(guards),'primary_copies':len(copies),'scientific_calls':0,'snapshot_bytes':metadata['frozen_scientific_snapshot']['bytes']},indent=2))
