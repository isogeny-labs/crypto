# per-address self Ir of functions matching a regex, from a callgrind --dump-instr file
import sys,re,collections
path,pat=sys.argv[1],re.compile(sys.argv[2])
names={};cur=None;addr=0;line=0;skip=False
cost=collections.defaultdict(lambda:collections.Counter())
def pos(tok,prev):
    if tok=='*': return prev
    if tok[0] in '+-': return prev+int(tok,0)
    return int(tok,0)
for raw in open(path):
    l=raw.rstrip('\n')
    if not l: continue
    c=l[0]
    if l.startswith('fn='):
        m=re.match(r'fn=\((\d+)\)(?: (.*))?',l)
        i=m.group(1)
        if m.group(2): names[i]=m.group(2)
        cur=names[i];continue
    if l.startswith(('cfn=','cfi=','cfl=','cob=')):
        m=re.match(r'cfn=\((\d+)\) (.*)',l)
        if m: names[m.group(1)]=m.group(2)
        continue
    if l.startswith('calls='):
        skip=True;continue
    if l.startswith(('jump=','jcnd=')):
        continue
    if c.isdigit() or c in '+-*' or l.startswith('0x'):
        t=l.split()
        a=pos(t[0],addr); 
        if len(t)>1: ln=pos(t[1],line)
        addr=a
        if skip: skip=False; continue
        if len(t)>2 and cur and pat.search(cur):
            cost[cur][a]+=int(t[2])
        continue
for f,cn in cost.items():
    tot=sum(cn.values())
    print(f"## {tot:,} {f[:160]}")
    for a,v in sorted(cn.items(),key=lambda x:-x[1])[:int(sys.argv[3]) if len(sys.argv)>3 else 30]:
        print(f"  {a:#x} {v:,}")
