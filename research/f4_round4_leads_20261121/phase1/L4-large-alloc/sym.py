import sys,subprocess,re
txt,binp=sys.argv[1],sys.argv[2]
base=None
for l in open(txt+'.maps'):
    if binp.split('/')[-1] in l:
        a=int(l.split('-')[0],16); off=int(l.split()[2],16)
        if base is None or a-off<base: base=a-off
sites=[]
for l in open(txt):
    if l.startswith('site'):
        t=l.split(); kind=int(t[2]); n=int(t[4]); b=int(t[6]); mx=int(t[8]); fr=[int(x,16) for x in t[10:] if x!='(nil)']
        sites.append((b,n,mx,kind,fr))
sites.sort(reverse=True)
allf=sorted({f-base for s in sites for f in s[4] if f>base and f-base<0x10000000})
out=subprocess.run(['addr2line','-f','-C','-i','-e',binp]+[hex(f-1) for f in allf],capture_output=True,text=True).stdout
# with -i, variable lines; redo one at a time for simplicity
names={}
for f in allf:
    o=subprocess.run(['addr2line','-f','-C','-i','-e',binp,hex(f-1)],capture_output=True,text=True).stdout.strip().split('\n')
    pairs=[(o[i],o[i+1]) for i in range(0,len(o)-1,2)]
    names[f]=pairs
tot=sum(s[0] for s in sites)
skip=re.compile(r'alloc::|core::|__rust|__rdl|RawVec|raw_vec|<alloc|std::|hashbrown|rayon')
for b,n,mx,kind,fr in sites[:25]:
    print(f"== bytes {b/1e6:.1f} MB ({100*b/tot:.1f}%) n {n} max {mx} kind {['malloc','calloc','realloc','memalign'][kind]}")
    shown=0
    for f in fr:
        for fn,loc in names.get(f-base,[]):
            if 'cryptanalysis' in loc or 'examples' in loc:
                print("    ",fn[:110],"@",loc.split('/home/user/crypto/')[-1]); shown+=1
        if shown>=4: break
