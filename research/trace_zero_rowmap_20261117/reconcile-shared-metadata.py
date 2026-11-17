"""Git/PCC byte custody only; no scientific computation or claim-file edits.

Keep the entire original fork in a deterministic archive, keep actual main's
ledger byte prefix, and register the immutable unsigned claim subjects through
the maintained PCC API. Retain both journal suffixes in timestamp order.
"""
import collections
import datetime
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import re
import subprocess
import sys
import tarfile

root = Path(__file__).resolve().parents[2]
p = Path(__file__).resolve().parent
def git(*a):
    return subprocess.check_output(['git', *a], cwd=root)
branch = git('rev-parse', 'HEAD').decode().strip()
main = git('rev-parse', 'MERGE_HEAD').decode().strip()
base = git('merge-base', branch, main).decode().strip()
ledger_path = 'pcc/ledger.jsonl'
journal_path = 'docs/session-logs/2026-11-17.md'
conflicts = set(git('diff','--name-only','--diff-filter=U').decode().splitlines())
assert conflicts == {ledger_path, journal_path}, conflicts
files = {}
for label, revision in [('branch',branch),('main',main),('common',base)]:
    files[label+'-ledger.jsonl'] = git('show',revision+':'+ledger_path)
    files[label+'-journal.md'] = git('show',revision+':'+journal_path)
assert files['branch-ledger.jsonl'].startswith(files['common-ledger.jsonl'])
assert files['main-ledger.jsonl'].startswith(files['common-ledger.jsonl'])
assert files['branch-journal.md'].startswith(files['common-journal.md'])
assert files['main-journal.md'].startswith(files['common-journal.md'])

spec = importlib.util.spec_from_file_location('rowmap_pcc',root/'tools/pcc/pcc.py')
pcc = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = pcc
spec.loader.exec_module(pcc)
old = pcc.read_ledger_text(files['branch-ledger.jsonl'].decode())
canonical = pcc.read_ledger_text(files['main-ledger.jsonl'].decode())
common = pcc.read_ledger_text(files['common-ledger.jsonl'].decode())
for entries in [old,canonical,common]:
    errors = []
    pcc.check_chain(entries,errors)
    assert not errors, errors
suffix = old[len(common):]
for e in suffix:
    assert e['type'] == 'claim' and 'sig' not in e
    data = git('show',branch+':'+e['body']['path'])
    assert (root/e['body']['path']).read_bytes() == data
    assert pcc.claim_digest(json.loads(data)) == e['body']['claim_sha256']

def pack():
    out = io.BytesIO()
    with tarfile.open(fileobj=out, mode='w:xz',preset=6,format=tarfile.GNU_FORMAT) as tf:
        for name,data in sorted(files.items()):
            info=tarfile.TarInfo(name)
            info.size=len(data); info.mode=0o644; info.mtime=0
            tf.addfile(info,io.BytesIO(data))
    return out.getvalue()
archive = p/'shared-history.tar.xz'
assert not archive.exists(), 'Refuse to replace custody evidence'
packed = pack()
assert packed == pack()
archive.write_bytes(packed)
with tarfile.open(archive) as tf:
    assert set(tf.getnames()) == set(files)
    for name,data in files.items():
        assert tf.extractfile(name).read() == data

def blocks(data):
    text=data.decode()
    starts=list(re.finditer(r'(?m)^## \d{4}-\d\d-\d\dT\S+',text))
    assert starts and not text[:starts[0].start()].strip()
    return [text[m.start():starts[i+1].start() if i+1<len(starts) else None].strip()
            for i,m in enumerate(starts)]
prefix=files['common-journal.md']
ours=blocks(files['branch-journal.md'][len(prefix):])
theirs=blocks(files['main-journal.md'][len(prefix):])
counts=collections.Counter(ours) | collections.Counter(theirs)
merged=list(counts.elements())
merged.sort(key=lambda b: datetime.datetime.fromisoformat(b.split()[1].replace('Z','+00:00')))
assert collections.Counter(merged) == counts
(root/journal_path).write_bytes(prefix+b'\n'+('\n\n'.join(merged)+'\n').encode())

(root/ledger_path).write_bytes(files['main-ledger.jsonl'])
present={(e['type'],pcc.sha256_obj(e['body'])):e for e in canonical}
registrations=[]
for e in suffix:
    key=(e['type'],pcc.sha256_obj(e['body']))
    already=key in present
    new=present[key] if already else pcc.append_entry(root/ledger_path,e['type'],e['body'])
    assert new['body'] == e['body']
    registrations.append({'claim':e['body']['claim_id'],'old_entry':e['hash'],
                          'new_entry':new['hash'],'already_on_main':already})
assert (root/ledger_path).read_bytes().startswith(files['main-ledger.jsonl'])
errors=[]
pcc.check_chain(pcc.read_ledger(root/ledger_path),errors)
assert not errors,errors
receipt={'schema':'rowmap-shared-history/v1','branch':branch,'main':main,'common':base,
         'archive_sha256':hashlib.sha256(packed).hexdigest(),
         'archive_members':[{'archive_member':name,'bytes':len(data),
                             'sha256':hashlib.sha256(data).hexdigest()}
                            for name,data in sorted(files.items())],
         'complete_archive_extraction_and_identical_repack':True,
         'canonical_main_ledger_byte_prefix_preserved':True,
         'claim_files_byte_identical':True,'registrations':registrations,
         'journal_branch_suffix_blocks':len(ours),'journal_main_suffix_blocks':len(theirs),
         'journal_result_suffix_blocks':len(merged),'journal_multiset_max_preserved':True,
         'pcc_api_sha256':hashlib.sha256((root/'tools/pcc/pcc.py').read_bytes()).hexdigest(),
         'standing':'Unsigned metadata custody; original fork entries/timestamps archived whole. New registrations do not assert a signed verdict.'}
(p/'shared-history.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({'claims_registered':len(registrations),'main_prefix':True,
                  'journal_blocks':len(merged),'archive_bytes':len(packed)}))
