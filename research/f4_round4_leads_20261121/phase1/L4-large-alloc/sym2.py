import sys,subprocess,bisect,re
txt,binp=sys.argv[1],sys.argv[2]
base=None
for l in open(txt+'.maps'):
    if binp.split('/')[-1] in l:
        a=int(l.split('-')[0],16); off=int(l.split()[2],16)
        if base is None or a-off<base: base=a-off
syms=[]
for l in subprocess.run(['nm','-C','--defined-only',binp],capture_output=True,text=True).stdout.splitlines():
    p=l.split(' ',2)
    if len(p)==3 and p[1] in 'tTwW': syms.append((int(p[0],16),p[2]))
syms.sort(); addrs=[s[0] for s in syms]
def name(a):
    i=bisect.bisect_right(addrs,a)-1
    return syms[i][1] if i>=0 else '?'
sites=[]
for l in open(txt):
    if l.startswith('site'):
        t=l.split(); sites.append((int(t[6]),int(t[4]),int(t[8]),int(t[2]),[int(x,16) for x in t[10:] if x!='(nil)']))
sites.sort(reverse=True); tot=sum(s[0] for s in sites)
skip=re.compile(r'^(alloc::|core::|__rust|__rdl|<alloc|hashbrown|std::|<core)')
for b,n,mx,kind,fr in sites[:int(sys.argv[3]) if len(sys.argv)>3 else 30]:
    nm=[name(f-base-1) if f>base else 'lib' for f in fr]
    keep=[re.sub(r'::h[0-9a-f]{16}$','',x) for x in nm if not skip.match(x)][:4]
    print(f"{b/1e6:8.1f} MB {100*b/tot:5.1f}% n={n:6d} max={mx:8d} {['malloc','calloc','realloc','memalign'][kind]:8s} | "+" <- ".join(k[:70] for k in keep))
