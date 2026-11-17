"""Metadata/hash/I-O only, no scientific arithmetic or replay."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,shutil,subprocess,tarfile,zipfile
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_cubic_inverse_20261117';f=r/'research/chained_richelot_map_20261117/compiler-revision-F'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
rows=[]
for item in json.loads((f/'SOURCE_LOCK_14F.json').read_text())['files']:
 p=f/item['path'];assert sha(p)==item['sha256'];rows.append(dict(item,original_path=str(p)))
lean=Path('/home/agent/.elan/toolchains/leanprover--lean4---v4.35.0');tools=[]
for p in [lean/'bin/lean',lean/'lib/lean/Init.olean',lean/'lib/lean/Init/Grind/Ring/Field.olean',lean/'lib/lean/Std/Tactic.olean']:
 tools.append({'path':str(p),'bytes':p.stat().st_size,'sha256':sha(p)})
orig=s/'primary-inputs';orig.mkdir(exist_ok=False)
for p in [lean/'src/lean/Init/Grind/Ring/Field.lean',Path('/workspace/cryptanalysis/experiments/chained-transfer-reference/CLOSED_FIBRES.md')]:shutil.copy2(p,orig/p.name)
data={'time':datetime.now(timezone.utc).isoformat(),'prospective_before_new_code':True,'original_F_public_source':'f35867831245560527428e3562ab55d74eefb5a0','full_old_F_and_native14_checkpoint':'265b97cabcc41ad60ba86d2d769b4cc24836c4ea','all_original_F_declared_files':rows,'old_input_and_full_dependency_manifest':json.loads((f/'ORIGINAL_INPUTS_BEFORE_CODE_14.json').read_text()),'pinned_Lean_compiler_and_direct_imports':tools,'Lean_version':'Lean4.35.0, eb6cafb7290e8bd5f2937522d8d804f58f02bafc','no_new_scientific_compiler_or_arithmetic_call':True,'scientific_code_languages':['Rust','Lean'],'Python_role':'metadata/hash/I-O only'}
(s/'ORIGINAL_INPUTS_BEFORE_NEW_CODE_15.json').write_text(json.dumps(data,indent=2)+'\n')
root=Path('/dev/net/chained-transfers-owned-storage-20261117/main12-complete-provider-custody');out=r/'research/chained_richelot_balanced_chart_20261117/provider-main12-full-ZIP-custody-16';out.mkdir(exist_ok=False)
record=json.loads((root/'ACTUAL_COMPLETE_MAIN12_PROVIDER_CUSTODY.json').read_text());p=Path(record['original_complete_provider_ZIP']['retained_path'])
dep=data['old_input_and_full_dependency_manifest']['entire_prior_dependency_archive']
with zipfile.ZipFile(p) as z,tarfile.open(r/dep['path']) as t:
 a=t.extractfile('all-native-tests.entire-stdout.txt').read();b=z.read('all-native-tests.entire-stdout.txt')
 left,aa=a.rsplit(b'; finished in ',1);right,bb=b.rsplit(b'; finished in ',1)
 assert left==right and aa.endswith(b's\n\n') and bb.endswith(b's\n\n')
 record.update(all_native_output_bytes_equal_except_exact_last_harness_elapsed_literal=True,unchanged_prefix_bytes=len(left),unchanged_prefix_sha256=hashlib.sha256(left).hexdigest(),elapsed_literal_local=aa.decode().strip(),elapsed_literal_provider=bb.decode().strip(),normalization_status='post-comparison diagnosis only; raw inequality/full9-line diff retained; not a benchmark or timing ratio')
 (out/'FULL_RAW_STDOUT_DIFF.json').write_text(json.dumps({'entire_raw_unified_diff':(root/'FULL_NATIVE_STDOUT_DIFF.txt').read_text()},indent=2)+'\n')
 for name in ['actual-phases.txt','compiler-qualifier-after.entire-stdout.txt','compiler-version.entire-stdout.txt']:
  (out/(name+'.json')).write_text(json.dumps({'provider_ZIP_member':name,'entire_raw_content':z.read(name).decode()},indent=2)+'\n')
for name in ['ALL_FULL_PROVIDER_ZIP_MEMBERS.json','CONNECTOR_BYTE_DOWNLOAD_FAILURE.json']:
 if (root/name).is_file():shutil.copy2(root/name,out/name)
(out/'ACTUAL_COMPLETE_MAIN12_PROVIDER_CUSTODY.json').write_text(json.dumps(record,indent=2)+'\n')
with (r/'docs/session-logs/2026-11-17.md').open('a') as log:log.write('\n\n---\n\n'+datetime.now(timezone.utc).isoformat()+'''
# Publish14 normally; track15 first and recover full main12 provider bytes

## What was done
Regular4544 payload2160eb2a is ready; normal MERGE auto-merge enabled09:36:15Z. All268 remote filenames equal local diff, all3 original lockfiles present, no workflows/tags/new inline compressed datasets. Artifacts/prose/whitespace pass; canonical data3252c2/report775e23/custodyc94926 remain unsigned. ACTIVE-onlyda590ab7 records15 and main12 custody before new work and is pushed to the open lane; work continues on owned source15 branch. Latest observed PR update09:39:25Z, first CI no earlier10:29:25Z. [pcc:unverified]

Main12 full original artifact14298461626 now downloaded via freshly supplied connector bytes:7291140B SHA6fec718f/265members, every full member hashed/CRC passed. All9 source members equal the full original local archive; all13 provider phase exits0. Both820434B native stdouts differ only final harness elapsed3.01s versus0.90s; the full scientific/result prefix is byte-identical, raw inequality and complete9-line diff preserved. This is custody/same-producer comparison, not independent arithmetic or a timing gain. [pcc:unverified]

## Decisions made
Close the missing-ZIP byte-inventory gap with full retained provider container and exact records, without rerunning12. Freeze15 protocol before new code:64 cubic/8 noncubic primitive inverses,2 expected zero rejections,8 sign/3 constructor controls,17 forward+17 dual closed comparisons and explicit boundary-call counters. Use standard Lean4.35 Field under default limits, not a new custom field axiom or actual Rust field instance. [pcc:unverified]

## Open threads
Publish15 prospective packet/PDF, then exact native/Lean sources and first fixed unit. Update persistent3904 with regular4544/current15/main12 custody; preserve CI cadence and futureDec1 checks. Original fullEKS/paidSTOP/vault/signed/generalfield-Picard/fullsolvercost remain. [pcc:unverified]

## Errors
CLI provider redirect403 retained as empty transport output; first connector signed link expired before use and returned403. Refreshed connector download is consumed immediately and full digest matches; signed URLs are never committed. No native experiment was repeated. Full raw stdout equality is false, solely the recorded harness elapsed literal; do not conceal it or compute a speed ratio. [pcc:unverified]
''')
print(json.dumps({'pinned_F_inputs':len(rows),'pinned_Lean_inputs':len(tools),'main12_source_equalities':len(record['full_source_member_equalities']),'main12_exact_scientific_prefix_bytes':len(left),'raw_stdout_equal':False}))
