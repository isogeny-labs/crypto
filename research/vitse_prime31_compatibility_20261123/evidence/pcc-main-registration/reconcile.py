#!/usr/bin/env python3
"""Preserve and re-register unpublished PCC envelopes after concurrent main."""
from pathlib import Path
import hashlib, importlib.util, json, subprocess, sys

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[3]
ARCHIVE = HERE.parent / 'pcc-before-main-integration'
MAIN = '26761272d78ff7108e4829808f2d2fa8ecc296f3'

def sha(data):
    return hashlib.sha256(data).hexdigest()

def relative(path):
    return str(path.relative_to(REPO))

def write_new(path, value):
    with path.open('x', encoding='utf-8') as f:
        json.dump(value, f, indent=2, sort_keys=True)
        f.write('\n')

manifest = json.loads((ARCHIVE / 'manifest.json').read_text())
ledger_path = REPO / 'pcc/ledger.jsonl'
main_ledger = subprocess.check_output(['git', 'show', MAIN + ':pcc/ledger.jsonl'], cwd=REPO)
assert ledger_path.read_bytes() == main_ledger
assert sha(main_ledger) == manifest['main_ledger_sha256']
assert sha((ARCHIVE / 'ledger-tail.jsonl').read_bytes()) == manifest['local_tail_sha256']
assert not (HERE / 'registration-map.json').exists()

spec = importlib.util.spec_from_file_location('official_pcc', REPO / 'tools/pcc/pcc.py')
pcc = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = pcc
spec.loader.exec_module(pcc)
old_entries = [json.loads(line) for line in (ARCHIVE / 'ledger-tail.jsonl').read_text().splitlines()]
claims = []
for old in old_entries:
    assert old['type'] == 'claim'
    path = REPO / old['body']['path']
    data = path.read_bytes()
    assert data == (ARCHIVE / 'claims' / path.name).read_bytes()
    assert sha(data) == manifest['claim_payloads'][relative(path)]['sha256']
    claim = json.loads(data)
    assert pcc.claim_digest(claim) == old['body']['claim_sha256']
    claims.append((old, claim))

mapping = []
for old, claim in claims:
    new = pcc.append_entry(ledger_path, 'claim', dict(old['body']))
    mapping.append({'claim_id': claim['id'], 'claim_sha256': pcc.claim_digest(claim),
                    'claim_file_sha256': sha((REPO / old['body']['path']).read_bytes()),
                    'original_envelope': {'seq': old['seq'], 'hash': old['hash'], 'at': old['at']},
                    'current_envelope': {'seq': new['seq'], 'hash': new['hash'], 'at': new['at']}})
map_path = HERE / 'registration-map.json'
write_new(map_path, {'canonical_base_commit': MAIN, 'canonical_prefix_sha256': sha(main_ledger),
                     'original_tail': {'path': relative(ARCHIVE / 'ledger-tail.jsonl'),
                                       'sha256': manifest['local_tail_sha256']},
                     'registrations': mapping, 'standing': 'unsigned; no independent verdict'})
note = pcc.append_entry(ledger_path, 'note', {
    'text': 'Concurrent main added a registration after the common ledger prefix while this study remained local. Preserve the complete current-main prefix and the five original unpublished registration envelopes in the pinned archive; register the identical immutable claim payloads in new append-only envelopes. Original claim JSON, producer commits and archived envelopes remain unchanged. No signed verification is asserted.',
    'canonical_base_commit': MAIN,
    'original_prefix_commit': manifest['original_parent'],
    'original_prefix_sha256': manifest['ledger_base_sha256'],
    'original_tail': {'path': relative(ARCHIVE / 'ledger-tail.jsonl'), 'sha256': manifest['local_tail_sha256']},
    'registration_mapping': {'path': relative(map_path), 'sha256': sha(map_path.read_bytes())}
})
write_new(HERE / 'receipt.json', {'canonical_base_commit': MAIN,
          'main_prefix_preserved': ledger_path.read_bytes().startswith(main_ledger),
          'registrations': len(mapping), 'note_seq': note['seq'], 'note_hash': note['hash'],
          'ledger_sha256_after': sha(ledger_path.read_bytes()),
          'pcc_tool_sha256': sha((REPO / 'tools/pcc/pcc.py').read_bytes()),
          'script_sha256': sha(Path(__file__).read_bytes())})
print(json.dumps({'registrations': len(mapping), 'last_seq': note['seq'], 'last_hash': note['hash']}))
