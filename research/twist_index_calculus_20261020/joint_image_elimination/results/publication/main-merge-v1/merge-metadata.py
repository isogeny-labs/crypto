"""Infrastructure only: fail-closed three-way JSON registry merge.

No field/curve arithmetic or scientific result is computed. Preserve complete
records and use the repository's alias serializer for generated lookup text.
"""
import hashlib
import json
import sys
from pathlib import Path

root = Path('/home/ec2-user/crypto-twist-ic')
receipt = Path(__file__).resolve().parent
docs = [json.loads((receipt/f'registry-{stage}.json').read_text()) for stage in (1,2,3)]
base, ours, theirs = docs
maps = []
for doc in docs:
    items = {entry['slug']: entry for entry in doc['curves']}
    assert len(items) == len(doc['curves']), 'duplicate curve slug'
    maps.append(items)
absent = object()
conflicts = []
def merge_item(key, old, left, right):
    if left == right:
        return left
    if left == old:
        return right
    if right == old:
        return left
    conflicts.append(key)
    return absent

curves = []
for slug in sorted(set().union(*(set(m) for m in maps))):
    entry = merge_item(slug, *(m.get(slug, absent) for m in maps))
    if entry is not absent:
        curves.append(entry)
metadata = {}
for key in sorted((set(base)|set(ours)|set(theirs))-{'curves'}):
    entry = merge_item('metadata:'+key, *(doc.get(key, absent) for doc in docs))
    if entry is not absent:
        metadata[key] = entry
if conflicts:
    (receipt/'unresolved.json').write_text(json.dumps({'conflicts':conflicts},indent=2)+'\n')
    raise SystemExit('Unresolved semantic conflicts; no repository files written')
assert set(maps[1]) | set(maps[2]) == {entry['slug'] for entry in curves}, 'lost existing identity'
rank = {'koblitz':0, 'subfield':1, 'binary':2, 'prime':3, 'extension':4}
curves.sort(key=lambda e: (rank[e['family']], int(e['params'].get('n',e['params'].get('m',0)) or 0),
                         int(e['params'].get('p',0)), e['params'].get('a',0) if e['family']=='koblitz' else 0,
                         e['slug']))
metadata['curves'] = curves
sys.path.insert(0,str(root/'scripts'))
import curve_id as cid
registry_text = json.dumps(metadata,indent=1,ensure_ascii=False)+'\n'
alias_text = cid.alias_map_text(metadata)
# Ensure all parent aliases which have the same meaning remain unchanged.
aliases = json.loads(alias_text)['aliases']
for stage in (2,3):
    previous = json.loads((receipt/f'aliases-{stage}.json').read_text())['aliases']
    for key,value in previous.items():
        assert aliases.get(key) == value, 'parent alias dropped or changed: '+key
paths = [root/'docs/curves/registry.json', root/'src/cryptanalysis/curve_aliases.json']
for path,text in zip(paths,[registry_text,alias_text]):
    path.write_text(text)
record = {'schema':'registry-three-way-merge/v1','status':'all-parent-records-and-aliases-preserved',
          'parents':(receipt/'parents.txt').read_text().splitlines(),
          'inputs':[{'file':f'registry-{i}.json','sha256':hashlib.sha256((receipt/f'registry-{i}.json').read_bytes()).hexdigest()} for i in (1,2,3)],
          'outputs':[{'path':str(path.relative_to(root)),'sha256':hashlib.sha256(path.read_bytes()).hexdigest()} for path in paths],
          'curve_counts':{'base':len(maps[0]),'ours':len(maps[1]),'theirs':len(maps[2]),'merged':len(curves)},
          'semantically_conflicting_records':[],
          'scope':'Metadata merge only; no research result, frozen evidence or ledger row changes. Alias text regenerated with scripts/curve_id.py alias_map_text.'}
(receipt/'summary.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps(record,indent=2))
