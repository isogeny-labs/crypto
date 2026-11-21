import sys,re,collections
# self Ir per instruction and conditional-jump counts for one function (callgrind, positions: instr line)
path,fname=sys.argv[1],sys.argv[2]; UPD=int(sys.argv[3])
cost=collections.Counter(); jc=[]
names={}; cur=None; last=0; lastl=0; skip=False; pend=None
def adv(tok,last):
    if tok=='*': return last
    if tok.startswith('+'): return last+int(tok[1:],0)
    if tok.startswith('-'): return last-int(tok[1:],0)
    return int(tok,0)
for line in open(path):
    line=line.rstrip('\n')
    m=re.match(r'^(c?fn)=\((\d+)\)(?: (.*))?$',line)
    if m:
        if m.group(3): names[m.group(2)]=m.group(3)
        if m.group(1)=='fn': cur=names[m.group(2)]
        continue
    if line.startswith('calls='): skip=True; continue
    if line.startswith(('jump=','jcnd=')):
        k,rest=line.split('=',1); t=rest.split()
        tgt=adv(t[1],last)
        if UPD: last=tgt
        pend=(k,t[0],tgt); skip=True; continue
    if line and (line[0].isdigit() or line[0] in '+-*'):
        t=line.split(); a=adv(t[0],last); last=a; lastl=adv(t[1],lastl)
        if pend:
            if cur==fname: jc.append((a,)+pend)
            pend=None; skip=False; continue
        if skip: skip=False; continue
        if cur==fname and len(t)>2: cost[a]+=int(t[2])
for a in sorted(cost): print('I',hex(a),cost[a])
for a,k,c,tg in jc: print('J',hex(a),k,c,hex(tg))
