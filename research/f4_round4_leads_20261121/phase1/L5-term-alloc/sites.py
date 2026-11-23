import pickle,collections,re,sys
edges,selfc=pickle.load(open(sys.argv[1],'rb'))
S=lambda s: re.sub(r'crypto_lib::cryptanalysis::|core::iter::adapters::|alloc::vec::|core::slice::iter::','',s)[:300]
for tgt in ['__rustc::__rust_alloc','__rustc::__rust_alloc_zeroed','__rustc::__rust_realloc','<alloc::raw_vec::RawVecInner>::finish_grow','<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>','__rustc::__rust_dealloc']:
    rows=[(n,c,a) for (a,b),(n,c) in edges.items() if b==tgt]
    rows.sort(key=lambda r:-r[0])
    print('==',tgt,sum(r[0] for r in rows), sum(r[1] for r in rows))
    for n,c,a in rows:
        if n>=500: print('  %9d %11d %s'%(n,c,S(a)))
for (a,b),(n,c) in sorted(edges.items(),key=lambda kv:-kv[1][0]):
    if 'grow_one' in b and n>=500: print('GROW %9d %s <- %s'%(n,S(b)[:60],S(a)))
