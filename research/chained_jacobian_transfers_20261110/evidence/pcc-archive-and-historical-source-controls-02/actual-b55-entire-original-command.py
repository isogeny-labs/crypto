"""Whole actual data claim replay in an isolated current-source fixture."""
from pathlib import Path
import hashlib,json,lzma,os,shutil,subprocess,sys

source=Path('/workspace/crypto');out=Path('/dev/shm/ct185-actual-historical-workflow-data-replay-01');out.mkdir()
repo=out/'repo';repo.mkdir()
shutil.copytree(source/'tools/pcc',repo/'tools/pcc',ignore=shutil.ignore_patterns('__pycache__'))
shutil.copytree(source/'pcc/claims',repo/'pcc/claims')
for name in ['pcc/keyring.json','pcc/ledger.jsonl']:
 destination=repo/name;destination.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source/name,destination)
claim=json.loads((repo/'pcc/claims/PCC-20261116-b55dd4.json').read_text())
for proof in claim['proofs']:
 if proof.get('kind')=='attestation':
  subject=proof['subject'];name=subject['path'];destination=repo/name
  assert not Path(name).is_absolute()
  if name=='.github/workflows/pcc-gate.yml':continue
  destination.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source/name,destination)
  assert hashlib.sha256(destination.read_bytes()).hexdigest()==subject['sha256']
protocol='research/chained_jacobian_transfers_20261110/PCC_HISTORICAL_WORKFLOW_ADDRESS_02.json'
manifest=json.loads((source/protocol).read_text())
canonical=repo/manifest['canonical_path'];canonical.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source/manifest['canonical_path'],canonical)
workflow=repo/'.github/workflows/pcc-gate.yml';workflow.parent.mkdir(parents=True);shutil.copyfile(source/'.github/workflows/pcc-gate.yml',workflow)
subprocess.run(['git','init','--quiet',str(repo)],check=True,capture_output=True)
subprocess.run(['git','add','--','.github/workflows/pcc-gate.yml'],cwd=repo,check=True,capture_output=True)
subprocess.run(['git','-c','user.name=Historical source data control','-c','user.email=historical-source-data-control@example.invalid','commit','--quiet','-m','actual new workflow fixture, not a published source commit'],cwd=repo,check=True,capture_output=True)
before=workflow.read_bytes()
def verify(label,expected):
 result=subprocess.run(['python3','tools/pcc/pcc.py','verify','PCC-20261116-b55dd4'],cwd=repo,capture_output=True)
 (out/(label+'.stdout.xz')).write_bytes(lzma.compress(result.stdout));(out/(label+'.stderr.xz')).write_bytes(lzma.compress(result.stderr));(out/(label+'.exit')).write_text(str(result.returncode)+'\n')
 assert result.returncode==expected,label
 print(label,result.returncode,flush=True)
verify('actual-current-source-is-not-historical-data',1)
sys.path.insert(0,str(source/'tools/pcc'))
import prepare_historical_workflow_data_02 as views
receipt=out/'whole-current-workflow-hold/receipt.json'
views.prepare(repo,manifest,receipt)
verify('entire-original-b55dd4-data-claim-in-historical-view',2)
views.restore(receipt)
assert workflow.read_bytes()==before and not workflow.is_symlink()
verify('restored-actual-current-source-is-not-historical-data',1)
assert (source/'.github/workflows/pcc-gate.yml').read_bytes()==before
(out/'complete-observation.json').write_text(json.dumps(dict(scope='actual full b55dd4 statement/data replay in isolated fixture, not actual current-source execution verification',before_original_verifier_exit=1,historical_data_view_original_verifier_exit=2,unsigned_not_covered=True,after_original_verifier_exit=1,actual_current_workflow_restored_regular_and_exact=True,main_workspace_workflow_unmodified_by_replay=True,main_original_verifier_keyring_claim_ledger_unmodified=True,isolated_fixture=str(repo)),indent=2)+'\n')
