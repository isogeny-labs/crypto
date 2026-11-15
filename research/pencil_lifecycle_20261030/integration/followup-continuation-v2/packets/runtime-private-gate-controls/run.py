"""One bounded software build/check attempt. Executes candidate only as an unprivileged non-PID1 rejection control."""
from pathlib import Path
import hashlib,json,os,subprocess,time,signal
P=Path(__file__).resolve().parent
SOURCE=P.parent/'runtime-private-qualifier/gate.c'
H=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    frozen=json.loads((P/'frozen-plan.json').read_text())
    for name,digest in frozen['files'].items():
        if H(P/name)!=digest:raise RuntimeError('frozen plan changed '+name)
    if H(SOURCE)!='6548d8d41a37975bb30c1a225583113ea6c25056753b3d455d90f1608d8e65bc':raise RuntimeError('gate source mismatch')
    if os.getpid()==1 or os.getuid()==0:raise RuntimeError('refuse privileged/PID1 software controls')
    admission=json.loads((P/'admission.json').read_text())
    if admission.get('proposal_manifest_sha256')!=H(P/'proposal.json') or admission.get('software_admitted') is not True:raise RuntimeError('software admission binding')
    candidate=P.parent/'runtime-private-gate-build-v2/gate-candidate.bin'
    if H(candidate)!='830406f4c2ddaf900ff4f95ca85f5b1c111b71c9d4d8bf6467ffa46853ae88e9':raise RuntimeError('candidate binding')
    out=P/'attempt1';out.mkdir();start=time.monotonic();commands=[]
    # Existing qualified private musl toolchain; verify every retained binding.
    toolchain=json.loads((P/'compiler-bindings.json').read_text())
    preflight={'status':'checking','checked':0,'manifest_sha256':H(P/'compiler-bindings.json')}
    try:
        for entry in toolchain:
            path=Path(entry['path'])
            if str(path.resolve())!=entry['resolved'] or H(path)!=entry['sha256']:raise RuntimeError('toolchain binding '+str(path))
            preflight['checked']+=1
        preflight['status']='verified'
    except BaseException as error:
        preflight.update(status='failed',error=type(error).__name__+': '+str(error));raise
    finally:(out/'toolchain-preflight.json').write_text(json.dumps(preflight,indent=2)+'\n')
    compiler=Path('/var/tmp/pencil-musl-private-20261031/prefix/bin/musl-gcc').resolve()
    tmp=out/'tmp';tmp.mkdir(mode=0o700)
    child_env={'PATH':'/usr/bin:/bin','LC_ALL':'C','REALGCC':'/usr/bin/gcc','TMPDIR':str(tmp)}
    result={'schema':'private.gate.combined-software-controls/v1','gate_source_sha256':H(SOURCE),'compiler_path':str(compiler),'compiler_sha256':H(compiler),'toolchain_manifest_sha256':H(P/'compiler-bindings.json'),'toolchain_files_verified':len(toolchain),'child_environment':child_env,'frozen_plan_sha256':H(P/'frozen-plan.json'),'runtime_admitted':False,'image_admitted':False,'candidate_executed':False,'actual_kernel_container_qualification':False,'held_release_controls_passed':False,'launch_uncertain':False,'pending_signals':[], 'proposal_manifest_sha256':H(P/'proposal.json'),'commands':commands}
    pending_signal=None
    def interrupted(signum,frame):
        nonlocal pending_signal
        if pending_signal is None:
            pending_signal=signum
            raise RuntimeError('software validation signal '+str(signum))
    signal.signal(signal.SIGTERM,interrupted);signal.signal(signal.SIGINT,interrupted)
    def run(label,argv,cap,expected=0):
        cap=min(cap,170-(time.monotonic()-start))
        if cap<=0:raise RuntimeError('validation deadline')
        entry={'label':label,'argv':argv,'cap_seconds':cap};commands.append(entry)
        proc=None;stdout=stderr=b''
        try:
            prior_mask=signal.pthread_sigmask(signal.SIG_BLOCK,{signal.SIGTERM,signal.SIGINT})
            try:
                entry['launch_uncertain']=True
                proc=subprocess.Popen(argv,cwd=P,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True,env=child_env)
                entry.update(pid=proc.pid,pgid=proc.pid,launch_uncertain=False)
            finally:signal.pthread_sigmask(signal.SIG_SETMASK,prior_mask)
            stdout,stderr=proc.communicate(timeout=cap)
        except BaseException as error:
            entry['error']=type(error).__name__+': '+str(error)
            if proc is not None:
                try:
                    try:os.killpg(proc.pid,signal.SIGKILL)
                    except ProcessLookupError:pass
                    stdout,stderr=proc.communicate(timeout=2)
                except BaseException as cleanup_error:
                    entry['cleanup_error']=type(cleanup_error).__name__+': '+str(cleanup_error)
            raise
        finally:
            entry['exit_code']=None if proc is None else proc.poll()
            entry['reap_confirmed']=proc is not None and proc.poll() is not None
            (out/(label+'.stdout')).write_bytes(stdout);(out/(label+'.stderr')).write_bytes(stderr)
        if proc.returncode!=expected:raise RuntimeError(label+' unexpected exit')
        return stdout
    try:
        run('compiler-version',[str(compiler),'--version'],5)
        target=run('compiler-target',[str(compiler),'-dumpmachine'],5).decode().strip()
        if not target.startswith('x86_64-'):raise RuntimeError('compiler target')
        result['compiler_target']=target
        import shutil
        shutil.copyfile(candidate,out/'gate-candidate.bin');os.chmod(out/'gate-candidate.bin',0o555)
        result['gate_binary_sha256']=H(out/'gate-candidate.bin')
        if result['gate_binary_sha256']!=H(candidate):raise RuntimeError('candidate copy')
        elf=run('candidate-elf',['/usr/bin/readelf','-h','-l','-d',str(out/'gate-candidate.bin')],5)
        if b'Advanced Micro Devices X86-64' not in elf or b'INTERP' in elf or b'(NEEDED)' in elf:raise RuntimeError('static ELF contract')
        run('parser-build',[str(compiler),'-static','-std=c11','-O2','-Wall','-Wextra','-Werror','-march=x86-64','-mtune=generic','-DFIXTURES="'+str(P/'fixtures')+'"',str(P/'parser-controls.c'),'-o',str(out/'parser-controls.bin')],60)
        raw=run('parser-controls',[str(out/'parser-controls.bin')],5)
        if not raw.endswith(b'ALL_CONTROLS_PASS checks=45\n'):raise RuntimeError('parser controls count/result')
        result['parser_checks']=45;result['parser_binary_sha256']=H(out/'parser-controls.bin')
        result['candidate_executed']=True
        raw=run('pristine-host-negative',[str(out/'gate-candidate.bin'),'Q1','01234567-89ab-cdef-0123-456789abcdef'],5,expected=126)
        if raw or (out/'pristine-host-negative.stderr').read_bytes()!=b'qualification gate: PID/UID/GID boundary\n':raise RuntimeError('host rejection boundary')
        result['pristine_host_negative_passed']=True
        run('helper-build',[str(compiler),'-static','-std=c11','-O2','-Wall','-Wextra','-Werror','-march=x86-64','-mtune=generic',str(P/'controls.c'),'-o',str(out/'helper-controls.bin')],60)
        raw=run('helper-controls',[str(out/'helper-controls.bin')],25)
        result['helper_binary_sha256']=H(out/'helper-controls.bin');result['helper_controls']=json.loads(raw)
        h=result['helper_controls']
        if h.get('status')!='helper_controls_passed' or h.get('checks')!=154 or h.get('partial_lengths')!=37 or h.get('whole_gate_executed') is not False or h.get('uid_gid_mocked') is not True:raise RuntimeError('held release control result')
        if pending_signal is not None or time.monotonic()-start>=170 or any(c.get('launch_uncertain') or not c.get('reap_confirmed') for c in commands):raise RuntimeError('terminal software boundary')
        result['held_release_controls_passed']=True
        result['status']='fixed_software_checks_passed_pending_independent_review'
    except BaseException as e:
        result['status']='failed';result['error']=type(e).__name__+': '+str(e);raise
    finally:
        result['pending_signal']=pending_signal
        result['pending_signals']=[] if pending_signal is None else [pending_signal]
        result['launch_uncertain']=any(c.get('launch_uncertain',True) or not c.get('reap_confirmed',False) for c in commands)
        result['elapsed_seconds']=time.monotonic()-start
        (out/'receipt.json').write_text(json.dumps(result,indent=2)+'\n')
if __name__=='__main__':main()
