import sys,re,collections
# parse callgrind: edges caller->callee with call counts and inclusive cost
names={}
def nm(s):
    m=re.match(r'\((\d+)\)(?:\s+(.*))?',s)
    if m:
        i=m.group(1)
        if m.group(2): names[i]=m.group(2)
        return names.get(i,i)
    return s
edges=collections.defaultdict(lambda:[0,0])
selfc=collections.defaultdict(int)
cur=None;pend=None;expect=False
for line in open(sys.argv[1],errors='replace'):
    line=line.rstrip('\n')
    if line.startswith('fn='): cur=nm(line[3:]);continue
    if line.startswith('cfn='): pend=nm(line[4:]);continue
    if line.startswith('calls='):
        cnt=int(line[6:].split()[0]);expect=True;continue
    if line and (line[0].isdigit() or line[0] in '+-*'):
        parts=line.split()
        if len(parts)>=2:
            c=int(parts[1])
            if expect:
                e=edges[(cur,pend)];e[0]+=cnt;e[1]+=c;expect=False
            else: selfc[cur]+=c
        continue
import pickle
pickle.dump((dict(edges),dict(selfc)),open(sys.argv[2],'wb'))
