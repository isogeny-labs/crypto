import subprocess,re,json,hashlib,collections
from pathlib import Path
path='docs/session-logs/2026-11-15.md'
def git(*args):return subprocess.check_output(['git',*args],text=True).strip()
def read(ref):
    data=subprocess.check_output(['git','show',f'{ref}:{path}'],text=True)
    parts=re.split(r'(?m)(?=^## )',data)
    return ''.join(s for s in parts if not s.startswith('## ')),[s for s in parts if s.startswith('## ')]
def canon(s):return s.rstrip()
head,main=git('rev-parse','HEAD'),git('rev-parse','origin/main');base=git('merge-base',head,main)
op,ob=read(head);mp,mb=read(main);assert op==mp,'journal prefixes differ'
om=collections.Counter(map(canon,ob));mm=collections.Counter(map(canon,mb));remaining=mm.copy();extra=[];extra_indices=[]
for j,s in enumerate(ob):
    k=canon(s)
    if remaining[k]>0:remaining[k]-=1
    else:extra.append(s);extra_indices.append(j)
assert extra,'no own additions'
anchor=next(canon(s) for s in reversed(ob[:extra_indices[0]]) if mm[canon(s)]>0)
i=max(j for j,s in enumerate(mb) if canon(s)==anchor)+1
def stamp(block):
    m=re.search(r'(?:T|\s)(\d\d):(\d\d)(?::(\d\d))?(?:Z|\s|$)',block.splitlines()[0])
    assert m,'unparsed tail timestamp'
    return tuple(int(s or 0) for s in m.groups())
tail=sorted(mb[i:]+extra,key=stamp);blocks=mb[:i]+tail
merged=mp+''.join(canon(s)+'\n\n' for s in blocks);merged_counts=collections.Counter(map(canon,blocks))
assert not (mm-merged_counts),'incoming complete block lost'
assert not (om-merged_counts),'own complete block lost'
assert len(blocks)==len(mb)+sum((om-mm).values())
with Path('/workspace/scratch/critical-pair-journal-preview002.md').open('x') as f:f.write(merged)
receipt={'head':head,'main':main,'merge_base':base,'main_blocks':len(mb),'own_blocks':len(ob),'own_extra':len(extra),'merged_blocks':len(blocks),'main_duplicate_headers':len(mb)-len(set(s.splitlines()[0] for s in mb)),'own_duplicate_headers':len(ob)-len(set(s.splitlines()[0] for s in ob)),'anchor':anchor.splitlines()[0],'main_all_complete_blocks_and_multiplicities_preserved':True,'own_all_complete_blocks_and_multiplicities_preserved':True,'normalization':'trailing whitespace at block boundaries only','preview_sha256':hashlib.sha256(merged.encode()).hexdigest(),'first_preview_failure':'incorrect uniqueness assumption for historical duplicate headers; no preview or journal was written'}
with Path('/workspace/scratch/critical-pair-journal-preview002.json').open('x') as f:json.dump(receipt,f,indent=2);f.write('\n')
print(json.dumps(receipt,indent=2))
