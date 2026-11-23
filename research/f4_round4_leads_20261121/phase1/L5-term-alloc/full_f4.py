import pickle,re,collections
edges,selfc=pickle.load(open('prog.pkl','rb'))
incoming=collections.defaultdict(list)
for (a,b),(n,c) in edges.items(): incoming[b].append((a,n))
ROOTS={'crypto_lib::cryptanalysis::f4_fp::f4_with_arithmetic','<rayon_core::registry::WorkerThread>::wait_until_cold'}
frac=collections.defaultdict(float)
for r in ROOTS: frac[r]=1.0
for it in range(80):
    for f in incoming:
        if f in ROOTS: continue
        inc=[(a,n) for a,n in incoming[f] if a!=f]
        t=sum(n for a,n in inc)
        if t: frac[f]=sum(n*frac[a] for a,n in inc)/t
f4=sum(selfc[f]*frac[f] for f in selfc)
print('F4 self-weighted total (main+worker): %.4g'%f4)
cats=collections.defaultdict(float)
for f,c in selfc.items():
    w=c*frac[f]
    if re.search(r'_rjem_|^je_|arena_|tcache|extent|^sz_|emap|^pa_|^pac_|^hpa|^sec_|^san|^edata|^eset|^bin_|^large_|^prof_|base_alloc|^ecache|^ehooks|^psset|bitmap|^witness|^rtree|^tsd|do_rallocx',f): cats['jemalloc']+=w
    elif re.search(r'__rustc::__rust_(alloc|dealloc|realloc|alloc_zeroed|no_alloc)',f): cats['rust shims']+=w
    elif re.search(r'raw_vec|finish_grow',f): cats['RawVec grow']+=w
    elif re.search(r'memcpy|memmove',f): cats['memcpy']+=w
    elif re.search(r'memset',f): cats['memset']+=w
for k,v in cats.items(): print('  %-12s %12d %6.2f%%'%(k,v,100*v/f4))
print('  sum %d %.2f%%'%(sum(cats.values()),100*sum(cats.values())/f4))
SH={'__rustc::__rust_alloc':'alloc','__rustc::__rust_alloc_zeroed':'zalloc','__rustc::__rust_realloc':'realloc','__rustc::__rust_dealloc':'dealloc'}
tot=collections.defaultdict(float)
for (a,b),(n,c) in edges.items():
    if b in SH: tot[SH[b]]+=n*frac[a]
print('F4 alloc calls (main+worker):',{k:round(v) for k,v in tot.items()},'per call',{k:round(v/104) for k,v in tot.items()})
pickle.dump(dict(frac),open('frac_full.pkl','wb'))
