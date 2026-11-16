"""Mirror published consumer evidence and queue, without changing producer science."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,shutil,subprocess
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target';head='69ce10b8042819385d5444522ca51a7c7255aa47';e=s/'evidence/public-n4-norm-point-result-01';e.mkdir()
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=c,text=True).strip()==head and not subprocess.check_output(['git','status','--porcelain'],cwd=c)
receipt=Path('/workspace/scratch/ct73-norm-point-reuse-native-01-publication-01/receipt.json');assert json.loads(receipt.read_text())['head']==head and json.loads(receipt.read_text())['original_exit']==0;shutil.copyfile(receipt,e/'consumer-normal-publication.json')
exports=['pcc/claims/PCC-20261115-36f906.json','pcc/claims/PCC-20261115-46008b.json','pcc/claims/PCC-20261115-f8d381.json','pcc/claims/PCC-20261115-b72f12.json','experiments/chained-transfer-public-target/evidence/fixed-chain-norm-point-native-01/outcome.json','experiments/chained-transfer-public-target/evidence/fixed-chain-norm-point-native-01/literal-selected-vectors.json','experiments/chained-transfer-public-target/evidence/fixed-chain-norm-point-native-01/whole-unfiltered-raw-inventory.json','experiments/chained-transfer-public-target/evidence/fixed-chain-norm-point-native-01/runtime-custody.json','experiments/chained-transfer-public-target/evidence/fixed-chain-norm-point-native-01/exact-original-checkpoint13.json','experiments/chained-transfer-public-target/reports/norm-point-reuse-result-01/REPORT.md','experiments/chained-transfer-public-target/reports/norm-point-reuse-result-01/render-inputs.json','experiments/chained-transfer-public-target/reports/norm-point-reuse-result-01/visual-inspection.json','experiments/chained-transfer-public-target/fixed-jacobian-chain-evaluator-02/PROOF_MAP.md']
rows=[]
for name in exports:
 p=c/name;b=p.read_bytes();assert b==subprocess.check_output(['git','show',head+':'+name],cwd=c);dest=e/Path(name).name;assert not dest.exists();dest.write_bytes(b);rows.append(dict(consumer_path=name,consumer_commit=head,sha256=hashlib.sha256(b).hexdigest(),bytes=len(b),local_export=str(dest.relative_to(r))))
(e/'source-exports.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),consumer_head=head,whole_exports=rows,scope='exact published consumer evidence projection; no independent mathematical or signed promotion'),indent=2)+'\n');shutil.copyfile(__file__,e/'original-record-command.py')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Exact published consumer69ce10b mirror: norm-point native46008b at51dff2bb/build01/exactoriginal checkpoint13/all28/localgate0 passes one unfilteredtest/all14 phases/whole695 records except prospective48counterchanges/four actual whole omissions101/nooutput/all1099guards/three compiler-runtime binaries. Samefull672-twooutgoing/fourhalves-corrections-cofactors/144mixed120public96additions/full13-169norms-inverses/independent oldellipticchecks. Conditional3rootproofs/bothoriginalgates/all3bindings36f906 and inspected59050B reportf8d381; exactpublished consumer claim/wholeinventory/literalvector/proofmap/reportmetadata exports retained. Sourcecalls unpriced; allmathematical/geometric/program/calibrated/signing gaps explicit. Mirror only,unsigned; genuineNEW23 andactualstrict25CI still pending; no root-global-green or ECDLP advantage inference.']
for p in sorted(e.iterdir()):cmd+=['--proof','attestation:'+str(p.relative_to(r))]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();Path('/workspace/scratch/ct86-root-norm-point-mirror.claim.txt').write_text(claim+'\n');print(claim)
state='''# Norm-point reuse result mirrored; next original-prime and cost tasks

'''+claim+''': exact published consumer69ce10b evidence exports mirror unsigned native46008b at51dff2bb. Full695 mathematical records match under the prospective48 counter transformations; one unfiltered test0/all14 phases expected/four whole omissions101/no output/all1099 guards/three actual binaries exact. Same entire finite population and independent elliptic/full-cofactor checks. Three conditional Lean roots pass both original gates and all3 exact bindings36f906; the inspected two-page reportf8d381 is59,050B. [pcc:unverified]

## What was done
Direct norm transport changes169 calls2→1 plus1doubling; literal13 calls4→2 plus2doublings; retained13 calls4→2 plus1doubling. Every other selected counter and mathematical field remains exact. The negative halving comparison survives. Complete primitive setup/reuse and known public initial elliptic lift comparisons remain open. [pcc:unverified]

## Decisions made
Preserve and publish root metadata without editing the343 producer scientific inputs, original tools/workflows/policies or vault. Continue from existing all-six HLP tables for originalE2/E3/E4 before selecting any original-prime successor. Freeze a separate protocol before implementation; do not revise the fixed D12/D24/D67 negative or transport the old prime by twisting. [pcc:unverified]

## Open threads
Actual strict25 replay43674526503 at recorded cadence; complete primitive cost protocol and known-initial-lift baseline; separate original-prime/prime-field endpoint mechanisms; genuineNEW23 of all whole new roots/source/binaries/prepared copies/failures/reports/checkpoint13 before canonical candidatefixture/replay. [pcc:unverified]

## Errors
CI25 relative self-copy failed before mathematics, staging optional-empty subset and source-authoring syntax/legacy cleanup-schema preflight failures retained. One obsolete415311792B derived root cache RLIB removed only outside all44386 guards/every current+legacy proofsubject/no compilers/retained executable and all Git statuses exact; no source/raw/exe/prepared copies/current caches removed. Root124/clock101/main-ledger/signing/review/conflicts/KB-vault/calibratedadmission and fullcold120 obligations remain open. [pcc:unverified]

---

'''
for p in [s/'QUEUE.md',s/'CHECKPOINT.md',s/'PUBLIC_TARGET_CHECKPOINT.md',r/'.resume/RESUME.md',r/'docs/session-logs/2026-11-15.md']:p.write_text(state+p.read_text())
