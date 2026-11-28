import pickle,re,collections,sys
edges,selfc=pickle.load(open('prog.pkl','rb'))
S=lambda s: re.sub(r'crypto_lib::cryptanalysis::|core::iter::adapters::|alloc::vec::|core::slice::iter::|core::iter::traits::iterator::Iterator','',s)
incoming=collections.defaultdict(list)
for (a,b),(n,c) in edges.items(): incoming[b].append((a,n))
ROOT='crypto_lib::cryptanalysis::f4_fp::f4_with_arithmetic'
frac=collections.defaultdict(float); frac[ROOT]=1.0
funcs=set(incoming)
for it in range(60):
    for f in funcs:
        if f==ROOT: continue
        tot=sum(n for a,n in incoming[f])
        if tot==0: continue
        frac[f]=sum(n*frac[a] for a,n in incoming[f] if a!=f)/max(1,sum(n for a,n in incoming[f] if a!=f))
SH={'__rustc::__rust_alloc':'alloc','__rustc::__rust_alloc_zeroed':'zalloc','__rustc::__rust_realloc':'realloc','__rustc::__rust_dealloc':'dealloc'}
# treat allocator plumbing as transparent: attribute up to first non-plumbing caller
PLUMB=re.compile(r'raw_vec|finish_grow|do_reserve|grow_one|try_allocate_in|__rustc::|spec_from_elem|drop_glue::<alloc|hashbrown::raw|fallible_with_capacity|reserve_rehash')
def attribute(fn,n,depth=0):
    # distribute n calls from fn upward until a non-plumbing function
    if not PLUMB.search(fn) or depth>4 or fn==ROOT:
        yield fn,n; return
    inc=[(a,m) for a,m in incoming[fn] if a!=fn]
    tot=sum(m for a,m in inc)
    if tot==0: yield fn,n; return
    for a,m in inc:
        yield from attribute(a,n*m/tot,depth+1)
res=collections.defaultdict(lambda: collections.defaultdict(float))
tot=collections.defaultdict(float)
for (a,b),(n,c) in edges.items():
    if b in SH:
        k=SH[b]
        for site,w in attribute(a,n):
            w2=w*frac[site]
            res[site][k]+=w2; tot[k]+=w2
print('F4-attributed totals (prog file, frac-weighted):',{k:round(v) for k,v in tot.items()})
print('per F4 call:',{k:round(v/104,1) for k,v in tot.items()})
rows=sorted(res.items(),key=lambda kv:-(kv[1]['alloc']+kv[1]['zalloc']+kv[1]['realloc']))
print('%9s %8s %8s %9s  site'%('alloc','zalloc','realloc','dealloc'))
for s,d in rows[:45]:
    print('%9d %8d %8d %9d  %s'%(d['alloc'],d['zalloc'],d['realloc'],d['dealloc'],S(s)[:200]))
print('frac of a few:',{S(k)[:50]:round(frac[k],3) for k in ['<crypto_lib::cryptanalysis::f4_fp::Cols>::get','crypto_lib::cryptanalysis::f4_fp::normalise','<alloc::raw_vec::RawVec<[usize; 4]>>::grow_one']})
