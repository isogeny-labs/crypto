import json,subprocess,os,sys,hashlib,time
B='/home/user/bisect'; WT='/home/user/wt/bis'
env=dict(os.environ,CARGO_TARGET_DIR='/home/user/tgt',CARGO_PROFILE_RELEASE_OPT_LEVEL='1',CARGO_PROFILE_RELEASE_LTO='false',
         CARGO_PROFILE_RELEASE_CODEGEN_UNITS='16',CARGO_PROFILE_RELEASE_DEBUG='false')
fp=subprocess.check_output(['git','-C','/home/user/crypto','log','--first-parent','--format=%H','2245b75..origin/main'],text=True).split()
commits=['2245b7596f3fe04586c33b53a7cac97185fadba2']+fp[::-1]
json.dump(commits,open(f'{B}/commits.json','w'))
cache={}
if os.path.exists(f'{B}/cache.json'): cache=json.load(open(f'{B}/cache.json'))
def log(*a):
    print(time.strftime('%H:%M:%S'),*a,flush=True)
def sig(i):
    c=commits[i]
    if c in cache: return cache[c]
    subprocess.run(['git','-C',WT,'checkout','-q','--detach','-f',c],check=True)
    t=time.time()
    import re
    for attempt in range(6):
        b=subprocess.run(['cargo','build','--release','--bin','ecbench'],cwd=WT,env=env,capture_output=True,text=True)
        miss=re.findall(r"couldn't read `([^`]+)`",b.stderr)+re.findall(r"couldn't read ([^:\s]+): No such file",b.stderr)
        if b.returncode==0 or not miss: break
        sp=subprocess.check_output(['git','-C',WT,'rev-parse','--git-path','info/sparse-checkout'],text=True).strip()
        sp=sp if sp.startswith('/') else WT+'/'+sp
        with open(sp,'a') as f:
            for m in miss:
                rel=os.path.relpath(os.path.realpath(m),WT); f.write('/'+os.path.dirname(rel)+'/\n'); log('add sparse',rel)
        subprocess.run(['git','-C',WT,'sparse-checkout','reapply'],check=True)
    if b.returncode:
        log('BUILD FAIL',i,c[:9]); open(f'{B}/buildfail-{c[:9]}.log','w').write(b.stderr[-5000:]); cache[c]=None
    else:
        out=f'{B}/runs/{i:03d}-{c[:9]}'
        subprocess.run(['rm','-rf',out])
        r=subprocess.run(['/home/user/tgt/release/ecbench','run','--spec',f'{B}/f4only.json','--out',out,'--cpus','none','--allow-busy','--quiet'],cwd=WT,env=env,capture_output=True,text=True)
        if r.returncode: log('RUN FAIL',i,c[:9],r.stderr[-500:]); cache[c]=None
        else:
            s=subprocess.check_output(['python3',f'{B}/sig.py',out])
            open(f'{out}/sig.json','wb').write(s)
            cache[c]=hashlib.sha256(s).hexdigest()[:12]
            for f in ['records.jsonl']: pass
    json.dump(cache,open(f'{B}/cache.json','w'),indent=1)
    log('eval',i,c[:9],cache[c],f'{time.time()-t:.0f}s')
    return cache[c]
def near(i,lo,hi):
    # nearest buildable index to i within (lo,hi)
    for d in range(0,hi-lo):
        for j in (i+d,i-d):
            if lo<j<hi and sig(j) is not None: return j
    return None
changes=[]
def bis(lo,hi):
    slo,shi=sig(lo),sig(hi)
    if slo==shi: return
    if hi-lo==1:
        changes.append((lo,hi)); log('CHANGE',lo,hi,commits[hi][:9],slo,'->',shi); return
    m=near((lo+hi)//2,lo,hi)
    if m is None: changes.append((lo,hi)); log('UNRESOLVED',lo,hi); return
    bis(lo,m); bis(m,hi)
bis(0,len(commits)-1)
json.dump([dict(lo=a,hi=b,lo_sha=commits[a],hi_sha=commits[b],sig_lo=cache[commits[a]],sig_hi=cache[commits[b]]) for a,b in changes],open(f'{B}/changes.json','w'),indent=1)
log('DONE',changes)
