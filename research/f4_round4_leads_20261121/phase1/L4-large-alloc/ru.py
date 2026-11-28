import os,sys,json,time,subprocess
# usage: ru.py label [ENV=VAL ...] -- cmd...
label=sys.argv[1]; i=sys.argv.index('--'); envs=sys.argv[2:i]; cmd=sys.argv[i+1:]
env=dict(os.environ)
for e in envs:
    k,v=e.split('=',1); env[k]=v
out=open(os.path.join(os.path.dirname(__file__),label+'.out'),'w'); err=open(os.path.join(os.path.dirname(__file__),label+'.err'),'w')
t=time.time(); p=subprocess.Popen(cmd,env=env,stdout=out,stderr=err)
_,st,ru=os.wait4(p.pid,0)
r=dict(label=label,envs=envs,status=st,wall_s=time.time()-t,user_s=ru.ru_utime,sys_s=ru.ru_stime,minflt=ru.ru_minflt,majflt=ru.ru_majflt,maxrss_kb=ru.ru_maxrss,nvcsw=ru.ru_nvcsw,nivcsw=ru.ru_nivcsw)
print(json.dumps(r)); open(os.path.join(os.path.dirname(__file__),'rusage.jsonl'),'a').write(json.dumps(r)+'\n')
