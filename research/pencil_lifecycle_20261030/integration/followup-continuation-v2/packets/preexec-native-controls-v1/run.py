#!/usr/bin/env python3
"""Prospective fixed software checks; root review/admission required. No Docker."""
from pathlib import Path
import argparse,hashlib,json,os,signal,subprocess,time
D=Path(__file__).resolve().parent
H=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def require(x,m):
 if not x:raise RuntimeError(m)
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--admission',required=True);ap.add_argument('--out',required=True);a=ap.parse_args()
 adm=json.loads(Path(a.admission).read_text());require(adm.get('admitted') is True,'not admitted');require(adm.get('proposal_manifest_sha256')==H(D/'manifest.json'),'proposal binding');require(adm.get('total_seconds')==300 and adm.get('compile_seconds')==120 and adm.get('control_seconds')==20,'fixed caps')
 out=Path(a.out);require(out.is_absolute() and out.parent==Path('/var/tmp') and out.name.startswith('pencil-preexec-native-execution-') and not out.exists(),'new fixed private execution path');out.mkdir(mode=0o700);tmp=out/'tmp';tmp.mkdir(mode=0o700)
 start=time.monotonic();deadline=start+300;receipt={'status':'started','scientific_execution':False,'docker_actions':False,'gate_binary_sha256':None,'controls_binary_sha256':None,'commands':[]};child=None;child_name=None
 def interrupted(signum,frame):raise RuntimeError('interrupted signal '+str(signum))
 signal.signal(signal.SIGTERM,interrupted)
 signal.signal(signal.SIGINT,interrupted)
 def verify():
  for n,v in json.loads((D/'manifest.json').read_text())['files'].items():require(H(D/n)==v,'proposal file changed '+n)
  for row in json.loads((D/'compiler-bindings.json').read_text()):require(H(Path(row['path']))==row['sha256'],'compiler input changed '+row['path'])
  for p in (D/'fixtures').iterdir():require(p.is_file() and not p.is_symlink() and p.stat().st_nlink==1 and p.stat().st_mode&0o777==0o444,'fixture identity '+str(p))
 def run(name,cmd,cap,expected=0):
  nonlocal child,child_name
  remain=deadline-time.monotonic();require(remain>0,'shared deadline');(out/(name+'.command.json')).write_text(json.dumps(cmd,indent=2))
  began=time.monotonic();timeout=False
  child_name=name
  child=subprocess.Popen(cmd,cwd=out,env={'PATH':'/usr/bin:/bin','LC_ALL':'C','REALGCC':'/usr/bin/gcc','TMPDIR':str(tmp)},stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
  try:so,se=child.communicate(timeout=min(cap,remain))
  except subprocess.TimeoutExpired:
   timeout=True;os.killpg(child.pid,signal.SIGKILL);so,se=child.communicate()
  (out/(name+'.stdout')).write_bytes(so);(out/(name+'.stderr')).write_bytes(se);r={'name':name,'exit_code':child.returncode,'timeout':timeout,'elapsed_seconds':time.monotonic()-began};(out/(name+'.receipt.json')).write_text(json.dumps(r,indent=2));receipt['commands'].append(r);child=None
  require(not timeout and r['exit_code']==expected,name+' failed');return so,se
 try:
  verify();require(os.getpid()!=1,'host PID1 forbidden');(out/'admission.json').write_text(json.dumps(adm,indent=2))
  cc='/var/tmp/pencil-musl-private-20261031/prefix/bin/musl-gcc';base=[cc,'-std=c11','-O2','-Wall','-Wextra','-Werror','-static','-march=x86-64','-mtune=generic']
  run('compile-gate',base+[str(D/'gate.c'),'-Wl,-Map='+str(out/'gate.map'),'-o',str(out/'gate')],120)
  run('compile-controls',base+['-DFIXTURES="'+str(D/'fixtures')+'"',str(D/'controls.c'),'-Wl,-Map='+str(out/'controls.map'),'-o',str(out/'controls')],120)
  for name in ['gate','controls']:
   so,_=run(name+'-elf',['/usr/bin/readelf','-h','-l','-d',str(out/name)],10)
   require(b'Advanced Micro Devices X86-64' in so and b'INTERP' not in so and b'NEEDED' not in so,'static AMD64 ELF gate')
   run(name+'-disassembly',['/usr/bin/objdump','-dr',str(out/name)],10)
  so,se=run('fixed-controls',[str(out/'controls')],20)
  require(so==b'PASS nonce\nPASS mountinfo\nPASS exact-values\nPASS bounded-reader\nPASS host-negative-and-fixed-cases\nALL_CONTROLS_PASS checks=45\n','control markers')
  require(se==b'qualification gate: PID/UID/GID boundary\n'*3+b'qualification gate: fixed case and nonce required\n'*3,'control stderr')
  so,se=run('pristine-host-negative',[str(out/'gate'),'Q1','01234567-89ab-cdef-0123-456789abcdef'],20,126)
  require(so==b'' and se==b'qualification gate: PID/UID/GID boundary\n','host negative boundary')
  verify();receipt.update(status='fixed_software_checks_passed_pending_independent_review',gate_binary_sha256=H(out/'gate'),controls_binary_sha256=H(out/'controls'),actual_kernel_container_qualification=False)
 except BaseException as e:
  receipt.update(status='failed_retained_no_retry',error=type(e).__name__+': '+str(e))
 finally:
  if child is not None:
   if child.poll() is None:os.killpg(child.pid,signal.SIGKILL)
   so,se=child.communicate();(out/(child_name+'.stdout')).write_bytes(so);(out/(child_name+'.stderr')).write_bytes(se);(out/(child_name+'.interrupted.json')).write_text(json.dumps({'exit_code':child.returncode,'interrupted':True}))
  receipt['elapsed_seconds']=time.monotonic()-start;(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
 return 0 if receipt['status']=='fixed_software_checks_passed_pending_independent_review' else 1
if __name__=='__main__':raise SystemExit(main())
