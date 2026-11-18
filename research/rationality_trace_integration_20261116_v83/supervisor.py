from pathlib import Path
import os,select,signal,subprocess,time
P=Path(__file__).resolve().parent

def invoke(command,label,seconds=8,mode='pidfd'):
 begin=time.monotonic_ns();deadline=begin+int(seconds*1e9);ready=P/(label+'.ready');fd=None;reaped=False;timedout=False;rss=[];child=None
 try:
  with(P/(label+'.stdout')).open('xb')as out,(P/(label+'.stderr')).open('xb')as err:
   child=subprocess.Popen(command+[str(ready)],stdout=out,stderr=err,start_new_session=True)
   if mode=='pidfd':
    if not hasattr(os,'pidfd_open'):raise RuntimeError('pidfd unavailable; no silent polling fallback')
    fd=os.pidfd_open(child.pid,0)
   while True:
    left=(deadline-time.monotonic_ns())/1e9
    if left<=0:
     timedout=True
     try:os.killpg(child.pid,signal.SIGKILL)
     except ProcessLookupError:pass
     pid,status,usage=os.wait4(child.pid,0);reaped=True;end=time.monotonic_ns();break
    if mode=='pidfd':
     event=select.select([fd],[],[],min(.005,left))[0]
     if event:pid,status,usage=os.wait4(child.pid,0);reaped=True;end=time.monotonic_ns();break
    elif mode=='legacy':
     pid,status,usage=os.wait4(child.pid,os.WNOHANG)
     if pid:reaped=True;end=time.monotonic_ns();break
    else:raise ValueError('observer mode')
    if ready.exists():
     try:
      for line in Path('/proc/'+str(child.pid)+'/status').read_text().splitlines():
       if line.startswith('VmRSS:'):rss.append(int(line.split()[1]));break
     except FileNotFoundError:pass
    if mode=='legacy':time.sleep(min(.005,left))
   child.returncode=os.waitstatus_to_exitcode(status)
 finally:
  if fd is not None:os.close(fd)
  if child is not None and not reaped:
   try:os.killpg(child.pid,signal.SIGKILL)
   except ProcessLookupError:pass
   os.wait4(child.pid,0)
 timedout=timedout or end>deadline
 result={'command':command,'observer':mode,'wall_seconds':(end-begin)/1e9,'parent_end_ns':end,'outer_timeout':timedout,'exit_code':child.returncode,'wait4_peak_rss_kib':usage.ru_maxrss,'post_exec_sample_peak_kib':max(rss,default=None),'post_exec_samples':len(rss),'user_seconds':usage.ru_utime,'system_seconds':usage.ru_stime,'stdout':(P/(label+'.stdout')).read_text(),'stderr':(P/(label+'.stderr')).read_text()}
 result['supervisor_seconds']=(time.monotonic_ns()-begin)/1e9;return result
