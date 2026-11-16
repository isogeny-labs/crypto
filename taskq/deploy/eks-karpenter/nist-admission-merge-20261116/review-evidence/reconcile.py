import collections, hashlib, importlib.util, json, pathlib, re, subprocess

root = pathlib.Path('/workspace/crypto-nist-admission-20261116')
def git(*args):
    return subprocess.check_output(['git', *args], cwd=root)
def write(path, data):
    dest = root / path
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_bytes(data if isinstance(data, bytes) else data.encode())
def digest(data): return hashlib.sha256(data).hexdigest()
head = git('rev-parse', 'HEAD').decode().strip()
base = git('rev-parse', 'origin/main').decode().strip()
common = git('merge-base', head, base).decode().strip()
assert head == '3c783772f4cadf36d9e14cedd944c797c3f8bb24'
assert base == 'a1c98c8e85c32f910b12d5119ca0cd52f15391d6'
custody = 'taskq/deploy/eks-karpenter/nist-admission-merge-20261116'
ledger = 'pcc/ledger.jsonl'
old = git('show', f'{head}:{ledger}')
main = git('show', f'{base}:{ledger}')
shared = git('show', f'{common}:{ledger}')
assert old.startswith(shared) and main.startswith(shared)
suffix = old[len(shared):]
rows = [json.loads(x) for x in suffix.splitlines()]
assert len(rows) == 1 and rows[0]['body']['claim_id'] == 'PCC-20261116-50d4da'
write(f'{custody}/ORIGINAL-BRANCH-SUFFIX.jsonl', suffix)
write(ledger, main)
spec = importlib.util.spec_from_file_location('pcc_reconcile', root/'tools/pcc/pcc.py')
pcc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(pcc)
mapping=[]
for row in rows:
    new=pcc.append_entry(root/ledger, row['type'], row['body'], row.get('sig'))
    assert new['body']==row['body'] and new['type']==row['type'] and new.get('sig')==row.get('sig')
    mapping.append({'original_seq':row['seq'],'original_hash':row['hash'], 'current_seq':new['seq'],'current_hash':new['hash']})
note=pcc.append_entry(root/ledger, 'note', {'text':'Base-merge metadata reconciliation; original branch envelope retained verbatim and in immutable parent; claim body/type/signature unchanged; no standing promotion.', 'original_parent':head,'main_parent':base,'reconciliation':f'{custody}/RECONCILIATION.json'})
errors=[]
entries=pcc.read_ledger(root/ledger)
pcc.check_chain(entries, errors)
pcc.check_prefix(pcc.read_ledger_text(main.decode()), entries, errors)
assert not errors, errors
assert (root/ledger).read_bytes().startswith(main)
log='docs/session-logs/2026-11-16.md'
oldlog=git('show',f'{head}:{log}')
mainlog=git('show',f'{base}:{log}')
sharedlog=git('show',f'{common}:{log}')
assert oldlog.startswith(sharedlog)
addition=oldlog[len(sharedlog):]
write(f'{custody}/ORIGINAL-BRANCH-LOG-ADDITIONS.md',addition)
anchor=b'## 2026-11-16T16:10Z '
assert mainlog.count(anchor)==1
merged=mainlog.replace(anchor,addition.lstrip(b'\n')+b'\n'+anchor,1)
def blocks(data):
    return collections.Counter(s.strip() for s in re.split(rb'(?m)(?=^## )',data) if s.strip())
mb=blocks(merged)
for parent in (oldlog,mainlog):
    assert not (blocks(parent)-mb), 'journal block or multiplicity lost'
handoff='''\n## 2026-11-16T18:20Z — NIST admission base merge custody

### What was done (commands, files, features)

Merged main a1c98c8 into the admission branch after Copilot request6922900423
remained unanswered for over one hour; takeover6925030551 recorded beforehand.
Retained every journal block from both parents with multiplicities. Replayed the
original admission claim body through canonical pcc.append_entry after main's
exact ledger prefix; original envelope and mapping are retained in the custody
directory and immutable parent. This is metadata reconciliation only.

### Decisions made (and why)

The original main-only read-only workflow, five mock controls, evidence and claim
subject remain unchanged. No standing promotion or scientific execution is
asserted. The user's no-run-specific-cap instruction persists, with shared lab
controls and finite attempt limits. No rebase or force-push is used.

### Open threads (unfinished, pick up next time)

Check CI and obtain fresh independent review for the new merge head before
landing. Then dispatch the read-only workflow on main and inspect actual access.
No curve sample, nonce, paid job, image or scientific upload has started. Existing
G/b reports were reviewed read-only: no complete separator in the fixed catalog;
exact RNG-state replay remains prospective, with no new computation admitted.

### Errors (what went wrong, how it was handled)

Only the journal and ledger conflicted. Neither history was selected over the
other. Initial sparse historical source read failed its promisor proxy fetch;
connected raw reads succeeded, without credential or proxy changes. Hosted
inherited startup failures are separate from successful required checks and the
completed old-head Codex review. Those receipts do not approve the merge head.
'''
write(log,merged+handoff.encode())
record={'schema':'nist.admission.merge-custody/v1','original_parent':head,'main_parent':base,'common_parent':common,'original_suffix_sha256':digest(suffix),'main_ledger_entries':len(main.splitlines()),'current_ledger_entries':len(entries),'main_exact_byte_prefix':True,'claim_mapping':mapping,'note_seq':note['seq'],'journal_branch_addition_bytes':len(addition),'journal_parent_block_multisets_retained':True,'claim_standing':'unverified','scientific_execution':False}
write(f'{custody}/RECONCILIATION.json',json.dumps(record,indent=2)+'\n')
write(f'{custody}/README.md','''# Admission merge custody

This directory records the merge of main into the read-only NIST admission PR.
It preserves the original branch ledger envelope and journal additions verbatim.
RECONCILIATION.json maps the original claim envelope to its canonical appended
envelope after main, followed by one audit note. Claim body/type/signature and
all original preparation files stay unchanged; standing remains unverified.
Neither conflict was resolved by selecting a claim or evidence side.

This merge is repository metadata work, not an AWS identity read or scientific
execution. Required checks and independent review must apply to its new head.
No historical preparation or proof subject is rewritten.
''')
original_paths=git('diff','--name-only',common,head).decode().splitlines()
bindings=[]
for path in original_paths:
    if path in (ledger,log): continue
    original=git('show',f'{head}:{path}')
    current=(root/path).read_bytes()
    assert current==original,path
    bindings.append({'path':path,'sha256':digest(current),'unchanged':True})
write(f'{custody}/UNCHANGED-SUBJECTS.json',json.dumps(bindings,indent=2)+'\n')
print(json.dumps(record,indent=2))
