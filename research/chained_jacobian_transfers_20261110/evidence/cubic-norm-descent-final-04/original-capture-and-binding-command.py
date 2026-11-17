from pathlib import Path
from datetime import datetime,timezone
import hashlib,importlib.util,json,shutil,subprocess,sys
r=Path('/workspace/crypto');p=r/'research/chained_jacobian_transfers_20261110';g=p/'evidence/cubic-norm-descent-final-04';g.mkdir()
source=p/'lean/CubicNormDescent.lean'
assert Path('/dev/shm/ct272-cubic-norm-complete-dual-original-gates/Root435.exit').read_text().strip()=='0'
assert Path('/dev/shm/ct274-cubic-norm-complete-Consumer-gate/actual.exit').read_text().strip()=='0'
assert source.read_bytes()==Path('/dev/shm/ct272-cubic-norm-complete-dual-original-gates/entire-final-source.lean').read_bytes()
assert source.read_bytes()==Path('/workspace/cryptanalysis/.resume/cubic-norm-proof-source-20261116-02/CubicNormDescent.lean').read_bytes()
shutil.copyfile(source,g/'CubicNormDescent.lean')
for label,folder in [('whole-final-Root435','/dev/shm/ct272-cubic-norm-complete-dual-original-gates'),('whole-final-Consumer434','/dev/shm/ct274-cubic-norm-complete-Consumer-gate'),('whole-intermediate-eight-and-first-Consumer-root-failure','/dev/shm/ct267-cubic-norm-final-dual-gates'),('whole-correct-intermediate-Consumer-eight','/dev/shm/ct269-cubic-norm-correct-Consumer-root'),('whole-first-coordinate-compile','/dev/shm/ct270-cubic-norm-original-arithmetic-model'),('whole-second-coordinate-compile','/dev/shm/ct271-cubic-norm-final-original-model')]:
    target=g/label;shutil.copytree(folder,target)
    for f in sorted(target.rglob('*.lean')):f.rename(f.with_name(f.name+'.txt'))
for label,repo in [('Root',r),('Consumer',Path('/workspace/cryptanalysis'))]:
    q=g/('entire-original-'+label+'-proof-tools');q.mkdir()
    for name in ['proof-gate.sh','AuditAxioms.lean','trusted-native.txt']:
        f=repo/'tools/lean'/name
        if f.exists():shutil.copyfile(f,q/(name+'.txt'))
common=Path('/workspace/cryptanalysis/experiments/chained-transfer-lp-presentation-01/historical-01/original-source/common.rs')
assert hashlib.sha256(common.read_bytes()).hexdigest()=='832fd28810909194875a9438e164ec1b45dcefbaa83257b4e3fad3095ef865d6'
shutil.copyfile(common,g/'entire-original-common.rs.txt')
shutil.copyfile(__file__,g/'original-capture-and-binding-command.py')
receipt=dict(at=datetime.now(timezone.utc).isoformat(),actual_final_source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),
    Root435_original_gate_exit=0,Consumer434_original_gate_exit=0,actual_audited_declarations_each=15,actual_trusted_each=0,
    mathematical_named_statements=12,scope='commutative-ring determinant/square identities and exact Fin223 schoolbook arithmetic; no field/Jacobian/global geometry or execution-proof backend',
    original_common_sha256=hashlib.sha256(common.read_bytes()).hexdigest(),
    prior_compile_failures='abstract-ring zero simplification; two finite-model numeral/association attempts; wrong Consumer-contained target path; every whole source/log retained',
    imports='Std.Tactic and Init.GrindInstances.Ring.Fin only',
    KB_tool_observation=dict(query='odd degree finite field quadratic twist obstruction prime field descent nonsquare',exit=127,source='tool-observed wrapper response; executable unavailable; no absence/novelty inference'),
    native_historical_population_executed=False,paid_build_or_job_launched=False)
(g/'actual-complete-proof-scope.json').write_text(json.dumps(receipt,indent=2)+'\n')
statement='Concrete prime-field cubic-norm descent proof component: both unchanged original proof gates pass15 audited declarations each,0 trusted, on byte-identical final source, Lean4.35 Root and4.34.1 Consumer (source copy inside actual Consumer configured root). Twelve named statements prove generic determinant/norm-square and unit square-root descent, canonical Fin223 nonroots12/24 and units93/158, all-coefficient cubic square exclusions and exact original Kf.mul natural/modular square-model correspondence and exclusions; positive square4 control retained. Kernel reduction checks223 base residues, not223^3 triples; generic identities use commutative rings and explicitly stated unit hypotheses. Entire original common.rs SHA832fd288 is preserved for source interpretation; no finite-field/Jacobian construction, binary/word-overflow execution proof, global polarized-model impossibility, new map or cost claim. Full first failed abstract-ring compile, both failed finite-model compiles, wrong Consumer-root call and interim8-declaration passes remain complete. Only propext/Classical.choice/Quot.sound occur; no trusted native_decide or signed standing. No native full historical population/paid launch; exhausted budgets still stop paid work.'
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement',statement]
for f in sorted(g.rglob('*')):
    if f.is_file():cmd+=['--proof','attestation:'+f.relative_to(r).as_posix()]
data=subprocess.check_output(cmd,cwd=r,text=True).strip()
spec=importlib.util.spec_from_file_location('original_pcc_binding',r/'tools/pcc/pcc.py');m=importlib.util.module_from_spec(spec);sys.modules[spec.name]=m;spec.loader.exec_module(m)
names=['determinant_formula','norm_square','norm_of_constant_square','recover_base_square_root','base_nonroots','actual_constant_units','no_cubic_square_root12','no_cubic_square_root24','square_constant_control','original_square_coordinates','original_square_ne12','original_square_ne24']
bindings=[]
for name in names:
    qualified='CubicNormDescent.'+name
    actual=m.lean_find_declaration([g/'CubicNormDescent.lean'],qualified);assert actual is not None
    normalized=actual[1]
    proof=dict(path=(g/'CubicNormDescent.lean').relative_to(r).as_posix(),theorem=qualified,decl_sha256=hashlib.sha256(normalized.encode()).hexdigest())
    cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','theorem','--claim-type','interpretive','--statement',normalized,
     '--scope','Exact stated generic CommRing or canonical Fin223 arithmetic; original natural/modular square model only. Field/Jacobian/geometry/native word execution/cost correspondence separate. Immutable source, original dual-gate15/0 data '+data+'; unsigned until original standing.', '--proof','lean:'+json.dumps(proof)]
    claim=subprocess.check_output(cmd,cwd=r,text=True).strip()
    bindings.append(dict(theorem=qualified,claim=claim,normalized_statement=normalized,proof=proof))
(g/'exact-twelve-statement-bindings.json').write_text(json.dumps(dict(data=data,bindings=bindings),indent=2)+'\n')
print(data)
print(json.dumps(dict(source_sha256=receipt['actual_final_source_sha256'],named_statements=len(bindings),audited_each=15,trusted_each=0)))
text=f'''\n---\n\n{datetime.now(timezone.utc).isoformat()}\n# Concrete cubic norm obstruction and original square arithmetic\n\n## What was done\n{data}: final exact source passes both unchanged original Lean4.35/4.34.1 gates15 audited/0 trusted. Twelve exact original PCC declaration bindings pin the determinant/norm-square/unit descent, Fin223 nonroots/units, all-coefficient cubic exclusions and original natural/modular Kf.mul square model. Positive square4 control retained; all earlier failed/interim sources/logs are whole. [pcc:unverified]\n\n## Decisions made\nThis strengthens the previously written nonsquare12/24 implication in the actual z^3=3 coefficient presentation, without constructing fields/Jacobians/global polarized identifications or claiming binary/cost standing. Publish source/proof map/readable inspected report and focused dual-gate CI. Native fixed60 and paid work remain blocked by budgete42949. [pcc:unverified]\n\n## Open threads\nRoot4411 genuine successor16929 (4 metadata/removal paths for two files in one directory) must be independently collected/guarded before changing its source or merging; no force/rechain. Consumer040 changed-source CI due08:15; Root currentCI due08:25. Full scientificNEW28/OCI/ABI/known future admission and actual global Jacobian/program bridges remain open. [pcc:unverified]\n\n## Errors\nFirst abstract-ring simp failureae8acc, both model numeral/association failures and first wrong Consumer root692220 remain whole. Recently fetched whole Git pack metadata storage7cb04a preserves all bytes and restores40MiB overlay headroom; no raw/source deletion. KB wrapper query127 is an access gap, not novelty. [pcc:unverified]\n'''
with (r/'docs/session-logs/2026-11-16.md').open('a') as f:f.write(text)
with (r/'.resume/RESUME.md').open('a') as f:f.write(text)
