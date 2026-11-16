import os,fcntl,errno,json,signal,time
from pathlib import Path
signal.alarm(15)
p=Path(__file__).parent
fd=None;pid=None;records=[]
def acquire(name):
 d=os.open(p/name,os.O_CREAT|os.O_RDWR,0o600)
 try:
  fcntl.flock(d,fcntl.LOCK_EX|fcntl.LOCK_NB)
  return d,{'acquired':True,'errno':None,'errno_name':None}
 except OSError as e:
  os.close(d)
  return None,{'acquired':False,'errno':e.errno,'errno_name':errno.errorcode.get(e.errno),'message':str(e)}
try:
 fd,first=acquire('baseline.lock');assert first['acquired'];os.close(fd);fd=None
 fd,second=acquire('baseline.lock');assert second['acquired'];os.close(fd);fd=None
 records.append({'control':'no_child_close_reacquire','first':first,'second':second})
 fd,initial=acquire('inherited.lock');assert initial['acquired']
 cloexec=bool(fcntl.fcntl(fd,fcntl.F_GETFD)&fcntl.FD_CLOEXEC)
 ready_r,ready_w=os.pipe();go_r,go_w=os.pipe()
 pid=os.fork()
 if pid==0:
  try:
   os.close(ready_r);os.close(go_w)
   os.fstat(fd)
   os.write(ready_w,b'R')
   if os.read(go_r,1)!=b'G':os._exit(91)
   os.close(fd);os.close(go_r);os.close(ready_w);os._exit(0)
  except BaseException:os._exit(92)
 os.close(ready_w);os.close(go_r)
 assert os.read(ready_r,1)==b'R'
 os.close(fd);fd=None
 probe,held=acquire('inherited.lock')
 if probe is not None:os.close(probe)
 records.append({'control':'child_holds_inherited_descriptor_parent_closed','child_pid':pid,'fd_cloexec':cloexec,'attempt':held})
 assert not held['acquired'] and held['errno'] in (errno.EAGAIN,errno.EWOULDBLOCK)
 os.write(go_w,b'G');os.close(go_w);os.close(ready_r)
 reaped,status=os.waitpid(pid,0);pid=None
 assert os.waitstatus_to_exitcode(status)==0
 fd,released=acquire('inherited.lock');assert released['acquired'];os.close(fd);fd=None
 records.append({'control':'child_closed_and_reaped','child_exit_code':os.waitstatus_to_exitcode(status),'attempt':released})
 print(json.dumps({'passed':True,'records':records,'scope':'deterministic flock mechanism; original test cause unproved','uname':list(os.uname())},indent=2),flush=True)
finally:
 if fd is not None:os.close(fd)
 if pid:
  try:os.kill(pid,signal.SIGKILL)
  except ProcessLookupError:pass
  os.waitpid(pid,0)
