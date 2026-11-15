import subprocess,re,json,hashlib
from pathlib import Path
path='docs/session-logs/2026-11-15.md'
def git(*args): return subprocess.check_output(['git',*args],text=True).strip()
def read(ref):
    data=subprocess.check_output(['git','show',f'{ref}:{path}'],text=True)
    parts=re.split(r'(?m)(?=^## )',data)
    prefix=''.join(s for s in parts if not s.startswith('## '))
    blocks=[s for s in parts if s.startswith('## ')]
    headers=[s.splitlines()[0] for s in blocks]
    assert len(headers)==len(set(headers))
    return data,prefix,blocks,dict(zip(headers,blocks))
head,main=git('rev-parse','HEAD'),git('rev-parse','origin/main')
base=git('merge-base',head,main)
_,op,ob,od=read(head)
_,mp,mb,md=read(main)
assert op==mp,'journal prefix requires manual review'
for h in od.keys()&md.keys():
    assert od[h].rstrip()==md[h].rstrip(),f'nontrivial common-block difference: {h}'
extra=[s for s in ob if s.splitlines()[0] not in md]
assert extra,'no own additions to preserve'
anchor=next(s.splitlines()[0] for s in reversed(ob[:ob.index(extra[0])]) if s.splitlines()[0] in md)
i=next(i for i,s in enumerate(mb) if s.splitlines()[0]==anchor)+1
def stamp(block):
    h=block.splitlines()[0]
    m=re.search(r'(?:T|\s)(\d\d):(\d\d)(?::(\d\d))?(?:Z|\s|$)',h)
    assert m,f'unparsed tail timestamp: {h}'
    return tuple(int(s or 0) for s in m.groups())
tail=sorted(mb[i:]+extra,key=stamp)
merged=mp+''.join(s.rstrip()+'\n\n' for s in mb[:i]+tail)
_,_,_,check=('',None,None,{s.splitlines()[0]:s for s in re.split(r'(?m)(?=^## )',merged) if s.startswith('## ')})
for original in (md,od):
    for h,s in original.items(): assert check[h].rstrip()==s.rstrip(),f'lost block: {h}'
out=Path('/workspace/scratch/critical-pair-journal-preview001.md')
with out.open('x') as f:f.write(merged)
receipt={'head':head,'main':main,'merge_base':base,'main_blocks':len(md),'own_blocks':len(od),'own_extra':len(extra),'merged_blocks':len(check),'anchor':anchor,'main_all_blocks_preserved':True,'own_all_blocks_preserved':True,'normalization':'trailing whitespace at block boundaries only','preview_sha256':hashlib.sha256(merged.encode()).hexdigest()}
with Path('/workspace/scratch/critical-pair-journal-preview001.json').open('x') as f:json.dump(receipt,f,indent=2);f.write('\n')
print(json.dumps(receipt,indent=2))
