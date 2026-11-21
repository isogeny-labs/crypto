import sys,re
# aggregate self cost per instruction address for fn matching pattern
f=sys.argv[1]; pat=sys.argv[2]; COL=int(sys.argv[3])
cur=None; addr=0; names={}; costs={}
infn=False; calling=False
for line in open(f,errors='replace'):
    line=line.rstrip('\n')
    m=re.match(r'^fn=\((\d+)\)(?: (.*))?',line)
    if m:
        if m.group(2): names[m.group(1)]=m.group(2)
        cur=names.get(m.group(1),''); infn = pat in cur; continue
    if line.startswith(('cfn=','cfi=','cob=','fl=','fi=','fe=','ob=')):
        m=re.match(r'^cfn=\((\d+)\)(?: (.*))?',line)
        if m and m.group(2): names[m.group(1)]=m.group(2)
        continue
    if line.startswith('calls='):
        calling=True; continue
    m=re.match(r'^(0x[0-9a-f]+|\+\d+|-\d+|\*)\s+(\S+)\s+((?:\d+\s*)+)',line)
    if m:
        a=m.group(1)
        if a.startswith('0x'): addr=int(a,16)
        elif a=='*': pass
        else: addr+=int(a)
        if calling: calling=False; continue
        if infn: costs[addr]=costs.get(addr,0)+(lambda v: int(v[COL]) if COL < len(v) else 0)(m.group(3).split())
for a in sorted(costs): print(hex(a),costs[a])
