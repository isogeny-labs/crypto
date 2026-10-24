import subprocess,sys,json,os
X='/Volumes/SSD990-2/cargo-target-ic-norm/release/examples/koblitz_orbit_dlp_fast_online'
def run(n,K,norm=0,fr=1,seed=7,tg='tg.txt',extra={}):
    env=dict(os.environ,KIC_NORM=str(norm),**extra)
    if fr is not None: env['KIC_FREE_RELATIONS']=str(fr)
    import resource
    r0=resource.getrusage(resource.RUSAGE_CHILDREN)
    p=subprocess.run([X,f'construct:{n}:0:{K}',tg,str(seed),'o.jsonl'],env=env,capture_output=True,text=True)
    if p.returncode: return {'err':p.stderr[-300:]}
    r1=resource.getrusage(resource.RUSAGE_CHILDREN)
    d=json.loads(p.stdout.strip().splitlines()[-1]); d['cpu_s']=(r1.ru_utime+r1.ru_stime)-(r0.ru_utime+r0.ru_stime); return d
if __name__=='__main__':
    n=int(sys.argv[1])
    for spec in sys.argv[2:]:
        K,norm,fr=map(int,spec.split(','))
        d=run(n,K,norm,fr)
        if 'err' in d: print(spec,d['err']); continue
        t=d['timing_ms']
        print(spec,'| pts',d['factor_base_points'],'orb',d.get('orbit_representatives'),'setup %.0f index %.0f rank %.0f total %.0f'%(t['setup_total'],t['index_build'],t['rank_stage'],t['process_total']),'rankprobes',d.get('rank_probes'),{k:d[k] for k in d if 'verif' in k or 'recover' in k or k=='ok'})
def ok(path='o.jsonl'):
    rows=[json.loads(l) for l in open(path) if l.strip()]
    t=[r for r in rows if r.get('kind')=='compact_orbit_dlp_target']
    return sum(1 for r in t if r.get('recovered_matches_published') and r.get('group_verified')),len(t)
