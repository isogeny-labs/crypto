import json, glob, os, re, sys, math
E=os.environ['TMPDIR']+'/e2'
def itm(n):
    e=n-1; bits=e.bit_length(); return (bits-1)+bin(e).count('1')-(1 if False else 0) if False else (bits-1)+sum((e>>i)&1 for i in range(bits-1))
def cr(n): return (8+itm(n))/5.0
def lg(n): return 3*(itm(n)+2)/5.0
def load(d,tag):
    s=json.loads([l for l in open(f'{d}/{tag}.stdout') if l.startswith('{')][-1])
    recs=[json.loads(l) for l in open(f'{d}/{tag}.jsonl')]
    assert s['targets_solved']==s['targets']==len(recs), tag
    assert all(r.get('recovered_matches_published')!=False for r in recs), tag
    return s,recs
def costs(n,s,recs):
    c,l=cr(n),lg(n)
    s0=s['index_pair_candidates']*c+s['rank_root_calls_total']*c+s['rank_relations']*l
    ds=[(r.get('root_calls',r['probes']/2))*c+r.get('relation_checks',1)*l for r in recs]
    return s0,sum(ds)/len(ds),ds
def parse(tag):
    m=re.match(r'n(\d+)_K(\d+)_N(\d+)_T(\d+)_s(\d)_r(\d+)',tag); return tuple(map(int,m.groups()))
if __name__=='__main__':
    d=sys.argv[1]
    for f in sorted(glob.glob(d+'/*.stdout')):
        tag=os.path.basename(f)[:-7]; n,K,N,T,sym,r=parse(tag)
        try: s,recs=load(d,tag)
        except Exception as e: print(tag,'ERR',e); continue
        s0,dd,_=costs(n,s,recs); print(f'{tag} S0={s0:.4g} d={dd:.4g} S0d={s0*dd:.4g} S={s0+dd:.4g}')
