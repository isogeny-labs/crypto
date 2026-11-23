import sys, os, json, subprocess
HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE,'..','..','N1-gf2-d4-profile','src'))
import lib as N1
from lib import system, enc
BIN5=os.path.join(HERE,'gf2close5')
def call5(mode,N,d,eqs,cap=None,chunk=1024,progress_log=None):
    env=dict(os.environ)
    if cap: env['CPUCAP']=str(cap)
    if progress_log: env['PROGRESS']='1'
    err=open(progress_log,'w') if progress_log else subprocess.PIPE
    r=subprocess.run([BIN5,str(chunk)],input=enc(mode,N,d,eqs),stdout=subprocess.PIPE,stderr=err,text=True,env=env)
    if r.returncode: raise RuntimeError(str(r.returncode)+' '+str(r.stderr))
    return json.loads(r.stdout)
