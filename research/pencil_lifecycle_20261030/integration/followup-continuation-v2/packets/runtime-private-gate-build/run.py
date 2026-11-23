"""One bounded software build/check attempt. Never executes gate candidate."""
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
    out=P/'attempt1';out.mkdir();start=time.monotonic();commands=[]
    compiler=Path('/usr/bin/gcc').resolve()
    result={'schema':'private.gate.candidate-build/v1','source_sha256':H(SOURCE),'compiler_path':str(compiler),'compiler_sha256':H(compiler),'frozen_plan_sha256':H(P/'frozen-plan.json'),'runtime_admitted':False,'image_admitted':False,'candidate_executed':False,'commands':commands}
    pending_signal=None
    def interrupted(signum,frame):
        nonlocal pending_signal
        if pending_signal is None:
            pending_signal=signum
            raise RuntimeError('software validation signal '+str(signum))
    signal.signal(signal.SIGTERM,interrupted);signal.signal(signal.SIGINT,interrupted)
    def run(label,argv,cap):
        cap=min(cap,170-(time.monotonic()-start))
        if cap<=0:raise RuntimeError('validation deadline')
        entry={'label':label,'argv':argv,'cap_seconds':cap};commands.append(entry)
        proc=None;stdout=stderr=b''
        try:
            prior_mask=signal.pthread_sigmask(signal.SIG_BLOCK,{signal.SIGTERM,signal.SIGINT})
            try:
                entry['launch_uncertain']=True
                proc=subprocess.Popen(argv,cwd=P,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
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
        if proc.returncode:raise RuntimeError(label+' failed')
        return stdout
    try:
        run('compiler-version',[str(compiler),'--version'],5)
        target=run('compiler-target',[str(compiler),'-dumpmachine'],5).decode().strip()
        if not target.startswith('x86_64-'):raise RuntimeError('compiler target')
        result['compiler_target']=target
        run('candidate-build',[str(compiler),'-static','-O2','-Wall','-Wextra','-Werror',str(SOURCE),'-o',str(out/'gate-candidate.bin')],60)
        elf=run('candidate-elf',['/usr/bin/readelf','-h','-l','-d',str(out/'gate-candidate.bin')],5)
        if b'Advanced Micro Devices X86-64' not in elf or b'INTERP' in elf or b'(NEEDED)' in elf:raise RuntimeError('static ELF contract')
        result['candidate_sha256']=H(out/'gate-candidate.bin');result['candidate_bytes']=(out/'gate-candidate.bin').stat().st_size
        run('helper-build',[str(compiler),'-static','-O2','-Wall','-Wextra','-Werror',str(P/'controls.c'),'-o',str(out/'helper-controls.bin')],60)
        raw=run('helper-controls',[str(out/'helper-controls.bin')],25)
        result['helper_binary_sha256']=H(out/'helper-controls.bin');result['helper_controls']=json.loads(raw)
        result['status']='candidate_built_helper_controls_passed_unadmitted'
    except BaseException as e:
        result['status']='failed';result['error']=type(e).__name__+': '+str(e);raise
    finally:
        result['pending_signal']=pending_signal
        result['elapsed_seconds']=time.monotonic()-start
        (out/'receipt.json').write_text(json.dumps(result,indent=2)+'\n')
if __name__=='__main__':main()
