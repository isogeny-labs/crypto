"""Reviewed reversible ledger preservation; no scientific search or deletion."""
from pathlib import Path
import fcntl,hashlib,json,os,subprocess,sys
from lab.runner import Laboratory
R=Path('/workspace/generalized_roots');C=R/'research/autolab_cycle_20261115_v6'
L=R/'lab/ledger';D=R/'lab/ledger_sealed_20261115_v6';P=R/'lab/research_program.json'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def put(p,x):
    with p.open('x')as f:json.dump(x,f,indent=2);f.write('\n')
assert not D.exists() and not(C/'PRE_SEAL_RECEIPT.json').exists()
assert not(L/'PROGRAM_BEFORE_SEAL.json').exists()
with(L/'.lock').open('a+b')as lock:
    fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
    lab=Laboratory(L).load(recover=False);records=lab.journal();state=json.loads((L/'state.json').read_text());old=json.loads(P.read_text())
    assert records[-1]['state']==state==lab.state,'terminal state mismatch'
    assert sha(P)==state['program']['manifest_sha256'],'unregistered manifest edit'
    assert all(x['status']!='running'for x in state['experiments'].values())
    for eid,item in state['experiments'].items():
        assert not(item['status']=='pending'and(L/'attempts'/eid/'0001').exists()),'orphan attempt'
    validation=lab.validate();assert validation['integrity']=='passed',validation
    receipt=dict(schema='sealed-ledger-parent/v1',sealed_path=str(D.relative_to(R)),journal_sha256=sha(L/'journal.jsonl'),journal_bytes=(L/'journal.jsonl').stat().st_size,state_sha256=sha(L/'state.json'),program_sha256=sha(P),journal_records=len(records),terminal_record_sha256=records[-1]['record_sha256'],terminal_state_equal=True,all_attempts_preserved=True,attempt_count=sum(len(x['attempts'])for x in state['experiments'].values()),experiment_status={eid:x['status']for eid,x in state['experiments'].items()},prior_parent=old.get('sealed_parent'),validation=validation,program_before_seal='PROGRAM_BEFORE_SEAL.json',fingerprint_guard='New registrations must reject fingerprints found in this sealed epoch as well as the active ledger.')
    put(C/'PRE_SEAL_RECEIPT.json',receipt)
    os.rename(L,D)
    os.rename(P,D/'PROGRAM_BEFORE_SEAL.json')
    new={k:v for k,v in old.items()if k!='experiments'}
    new.update(experiments=[],epoch='20261115-v6-dual-obstruction-sealed-parent',sealed_parent=receipt)
    put(P,new)
    assert sha(D/'journal.jsonl')==receipt['journal_sha256']
    assert sha(D/'state.json')==receipt['state_sha256']
    assert sha(D/'PROGRAM_BEFORE_SEAL.json')==receipt['program_sha256']
    fcntl.flock(lock,fcntl.LOCK_UN)
init=json.loads(subprocess.check_output([sys.executable,'-m','lab.runner','init','--ledger','lab/ledger','--manifest','lab/research_program.json'],cwd=R))
resume=json.loads(subprocess.check_output([sys.executable,'-m','lab.runner','resume','--ledger','lab/ledger','--manifest','lab/research_program.json'],cwd=R))
sealed=Laboratory(D).load(recover=False).validate();assert sealed['integrity']=='passed'
put(C/'SEAL_RECEIPT.json',dict(parent=receipt,init=init,resume=resume,post_rename_readonly_validation=sealed,no_files_deleted=True,scientific_goal_complete=False))
print(json.dumps(dict(sealed=str(D),preserved_attempts=receipt['attempt_count'],old_journal_records=len(records),new_journal_records=resume['journal_records'],integrity=sealed['integrity'])))
