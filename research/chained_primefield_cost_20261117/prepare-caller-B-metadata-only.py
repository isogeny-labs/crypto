"""Prospective publication/logging repair only; no scientific source change."""
from pathlib import Path
from datetime import datetime,timezone
import ast,difflib,hashlib,json,shutil
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_primefield_cost_20261117';old=json.loads((s/'SOURCE_LOCK_16.json').read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
for row in old['files']:assert sha(s/row['path'])==row['sha256'],row['path']
with (r/'docs/session-logs/2026-11-17.md').open('a') as f:f.write('\n\n---\n\n'+datetime.now(timezone.utc).isoformat()+'\n# Preserve first16A publication failure before metadata-only B revision\n\n## What was done\nA8f3d314c caller1: published Cargo.lock absent because ignored; exact-public-source guard stops before copy/deps/compiler/native. Complete available36-member50652B packet57d41266 retains all26 declaredsource/lock files and failedmetadata.85 original source hashes pass only afterward; copied/dependency/tool after guards absent. The metadata helper lost originalGitstderr, with a separate explicitlypostcall read-only diagnostic kept; do not upgrade it. [pcc:unverified]\n\n## Decisions made\nFreeze separate B helper/caller/lock, force-add exact unchanged Cargo.lock and retain full failed Git metadata stdout/stderr before raising. Add staged closure check before publication. All25 originalA hashes, mathematical expressions,12 hand oracles,106snapshotrows, controls,limits/floors remain unchanged. [pcc:unverified]\n\n## Open threads\nPublic exact B source and first compile/native, completeAnegative report/final reports/resultqueue/regularPR. CurrentregularPR remains prospectiveC,sourceAcheckpoint intact. [pcc:unverified]\n\n## Errors\nThe ignore omission was caught by the published-source guard, not compiler or arithmetic. No scientific16 failure/result yet. Error-capture gap remains documented; postcall diagnostic is not firstcall evidence. [pcc:unverified]\n')
before=(s/'custody_metadata16.py').read_text();after=before.replace("study/'SOURCE_LOCK_16.json'","study/'SOURCE_LOCK_16B.json'")
needle=" def blob(path):return subprocess.run(['git','cat-file','blob',ctx['source_commit']+':'+path.relative_to(repo).as_posix()],cwd=repo,env=E,stdout=subprocess.PIPE,stderr=subprocess.PIPE,check=True).stdout"
replacement=""" def blob(path):
  argv=['git','cat-file','blob',ctx['source_commit']+':'+path.relative_to(repo).as_posix()]
  result=subprocess.run(argv,cwd=repo,env=E,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
  if result.returncode!=0:
   for suffix,b in [('stdout',result.stdout),('stderr',result.stderr)]:
    with (h/('FAILED_PUBLIC_GIT16.'+suffix)).open('xb') as stream:stream.write(b)
   write_new(h/'FAILED_PUBLIC_GIT16.json',{'argv':argv,'exit':result.returncode,'cwd':str(repo),'original_full_stdout':'FAILED_PUBLIC_GIT16.stdout','original_full_stderr':'FAILED_PUBLIC_GIT16.stderr'})
   raise subprocess.CalledProcessError(result.returncode,argv,output=result.stdout,stderr=result.stderr)
  return result.stdout"""
assert after.count(needle)==1;after=after.replace(needle,replacement);ast.parse(after);(s/'custody_metadata16B.py').write_text(after)
shell_before=(s/'run_first_unit16.sh').read_text();shell_after=shell_before.replace('META16="$STUDY16/custody_metadata16.py"','META16="$STUDY16/custody_metadata16B.py"');assert shell_before!=shell_after;(s/'run_first_unit16B.sh').write_text(shell_after)
protocol='''# Prospective source16B: publication completeness and failure bytes

First sourceA8f3d314c stops before copying,compilation or arithmetic: Cargo.lock
is ignored and absent from the published tree, despite its exact physical hash
being in SOURCE_LOCK_16.json. Preserve the whole first-negative packet and all
sourceA inputs. Force-add that same unchanged Cargo.lock; do not edit an ignore
rule, dependency, arithmetic expression, oracle, population or resource limit.
[pcc:unverified]

Separate metadata helper B selects SOURCE_LOCK_16B.json and writes complete
failed Git stdout/stderr/argv/exit before raising. A lost the first such stderr;
its later diagnostic check remains explicitly post-call. Separate shell B only
selects the new helper. All native source and all original16 criteria stay
literal. Check every execution-lock path against the staged Git blob before
publication, then publish/read back B before its first scientific call.
[pcc:unverified]
'''
(s/'PROSPECTIVE_PUBLICATION_REPAIR_16B.md').write_text(protocol)
diff=''.join(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile='frozen-A/custody_metadata16.py',tofile='new-B/custody_metadata16B.py'))+''.join(difflib.unified_diff(shell_before.splitlines(True),shell_after.splitlines(True),fromfile='frozen-A/run_first_unit16.sh',tofile='new-B/run_first_unit16B.sh'))
metadata={'prepared':datetime.now(timezone.utc).isoformat(),'first_source_A':'8f3d314c9f6ba549f27d660776888b65b63e29f1','all25_original_A_file_hashes_still_equal':True,'only_full_metadata_caller_diff':diff,'native_code_or_oracle_changes':0,'B_scientific_compiler_native_calls':0,'original_first_metadata_stderr_gap_preserved':True}
(s/'ACTUAL_PUBLICATION_REPAIR_16B.json').write_text(json.dumps(metadata,indent=2)+'\n')
paths=[row['path'] for row in old['files'] if row['path'] not in ['custody_metadata16.py','run_first_unit16.sh']]+['custody_metadata16B.py','run_first_unit16B.sh','PROSPECTIVE_PUBLICATION_REPAIR_16B.md','ACTUAL_PUBLICATION_REPAIR_16B.json','SOURCE_LOCK_16.json']
rows=[{'path':n,'bytes':(s/n).stat().st_size,'sha256':sha(s/n),'mode':(s/n).stat().st_mode&0o7777} for n in sorted(paths)]
(s/'SOURCE_LOCK_16B.json').write_text(json.dumps({'schema':'exact-native-cost16B-source/v1','prepared':datetime.now(timezone.utc).isoformat(),'unchanged_public_protocol':'dd6223faae3ff638eb2582e8f40ceca41c5a142c','frozen_A':'8f3d314c9f6ba549f27d660776888b65b63e29f1','files':rows,'same_native_filter':'nonsplit::map::tests_14::frozen_prime_cost_16','expected_new_test':1,'expected_inherited_compiled_filtered':15,'all_original_math_criteria_limits_unchanged':True,'B_scientific_calls_before_freeze':0},indent=2)+'\n');shutil.copy2(Path(__file__),s/'prepare-caller-B-metadata-only.py');print(json.dumps({'B_source_files':len(rows),'native_changes':0,'all_A_source_hashes_preserved':True},indent=2))
