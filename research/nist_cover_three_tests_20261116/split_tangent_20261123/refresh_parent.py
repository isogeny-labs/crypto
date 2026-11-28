"""Metadata-only stack refresh; no curve arithmetic or verifier trust changes."""
from pathlib import Path
from collections import Counter
import hashlib, importlib.util, json, re, subprocess
root=Path.cwd()
archive=Path('research/nist_cover_three_tests_20261116/split_tangent_20261123/evidence/parent-refresh-v1')
assert not archive.exists()
parent='lane/p192-recorded-pool-hypotheses-20261123'
ledger=Path('pcc/ledger.jsonl'); journal=Path('docs/session-logs/2026-11-23.md')
head_ledger=subprocess.check_output(['git','show','HEAD:'+str(ledger)])
working=ledger.read_bytes();assert working.startswith(head_ledger)
tail=working[len(head_ledger):];entries=[json.loads(line) for line in tail.splitlines()]
assert len(entries)==1 and entries[0]['type']=='claim' and 'sig' not in entries[0]
head_journal=subprocess.check_output(['git','show','HEAD:'+str(journal)])
working_journal=journal.read_bytes();assert working_journal.startswith(head_journal)
journal_tail=working_journal[len(head_journal):]
archive.mkdir();(archive/'unpublished-ledger-tail.jsonl').write_bytes(tail);(archive/'unpublished-journal-tail.txt').write_bytes(journal_tail)
claims={e['body']['path']:Path(e['body']['path']).read_bytes() for e in entries}
incoming=subprocess.check_output(['git','show',parent+':'+str(ledger)])
subprocess.run(['git','restore','--source=HEAD','--worktree',str(ledger),str(journal),'docs/ACTIVE.md'],check=True)
merge=subprocess.run(['git','merge','--no-stat','--no-edit',parent],capture_output=True,text=True)
(archive/'merge.stdout').write_text(merge.stdout);(archive/'merge.stderr').write_text(merge.stderr)
assert merge.returncode in (0,1)
active=Path('docs/ACTIVE.md');active.write_text(re.sub(r'^(<<<<<<< .*|=======|>>>>>>> .*)\n','',active.read_text(),flags=re.M))
assert not re.search(r'^(<<<<<<<|=======|>>>>>>>)',active.read_text(),re.M)
if subprocess.run(['git','rev-parse','--quiet','--verify','MERGE_HEAD'],capture_output=True).returncode==0:
 subprocess.run(['git','add','docs/ACTIVE.md'],check=True)
 subprocess.run(['git','-c','core.editor=true','commit','--no-edit'],check=True)
assert ledger.read_bytes()==incoming,'canonical parent ledger before append'
spec=importlib.util.spec_from_file_location('pcc_api',root/'tools/pcc/pcc.py');pcc=importlib.util.module_from_spec(spec);spec.loader.exec_module(pcc)
mapping=[]
for old in entries:
 assert Path(old['body']['path']).read_bytes()==claims[old['body']['path']]
 new=pcc.append_entry(ledger,old['type'],old['body']);mapping.append({'old':old['hash'],'new':new['hash'],'body_unchanged':old['body']==new['body']})
(archive/'registration-map.json').write_text(json.dumps(mapping,indent=2)+'\n')
def subject(path):return {'path':str(path),'sha256':hashlib.sha256(path.read_bytes()).hexdigest()}
pcc.append_entry(ledger,'note',{'text':'Parent merged the advanced main metadata. Preserve the unpublished child registration envelope and append its identical immutable claim payload after the complete canonical parent prefix. No scientific payload or signature changes.','original':subject(archive/'unpublished-ledger-tail.jsonl'),'registration_map':subject(archive/'registration-map.json'),'parent_revision':subprocess.check_output(['git','rev-parse',parent],text=True).strip()})
pattern=re.compile(r'(?=^## \d{4}-\d\d-\d\d)',re.M)
def parts(text):p=pattern.split(text);return p[0],p[1:]
def norm(block):return '\n'.join(line.rstrip() for line in block.strip().splitlines())
def stamp(block):
 h=re.search(r'(\d{4}-\d\d-\d\d)[T ](\d\d:\d\d)',block.splitlines()[0]);return h.group(1)+'T'+h.group(2) if h else ''
prefix,main_blocks=parts(journal.read_text());_,owned=parts(journal_tail.decode());merged=list(main_blocks)
for block in owned:
 position=next((i for i,other in enumerate(merged) if stamp(other)>stamp(block)),len(merged));merged.insert(position,block)
assert Counter(map(norm,merged))==Counter(map(norm,main_blocks))+Counter(map(norm,owned))
journal.write_text(prefix+''.join(merged))
active.write_text(active.read_text().replace('stacked on PR #5093).','PR #5101, stacked on PR #5093).',1))
(archive/'check.json').write_text(json.dumps({'claim_payloads_unchanged':len(claims),'journal_blocks_restored':len(owned),'journal_multiset_preserved':True,'canonical_parent_prefix_preserved':True},indent=2)+'\n')
print((archive/'check.json').read_text())
