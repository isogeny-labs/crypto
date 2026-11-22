import sys,json,time,subprocess,hashlib
from .common import P,finish
START=time.monotonic();group=sys.argv[3];out={'group':group,'runs':[],'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());panel=json.loads((P/'PANEL.json').read_text());budget=210 if group=='binary'else 120
 for fi,spec in enumerate(panel[group]):
  for ci,case in enumerate(spec['cases']):
   expected=None
   for rep in range(d['cold_repetitions']):
    methods=['direct','radical','dispatch']if group=='binary'else['direct','radical']
    if rep%2:methods.reverse()
    for method in methods:
     remaining=budget-(time.monotonic()-START)
     if remaining<2:raise TimeoutError('frozen panel budget; retain all completed rows')
     command=[sys.executable,'-m','research.discriminant_roots_20261116_v107b.child',str(P/'PANEL.json'),group,str(fi),str(ci),method];begin=time.monotonic();r=subprocess.run(command,capture_output=True,text=True,timeout=min(25,remaining));wall=time.monotonic()-begin;assert r.returncode==0,(command,r.stderr);value=json.loads(r.stdout);roots=value['result']['roots']
     if expected is None:expected=roots
     assert value['result']['complete']and roots==expected
     out['runs'].append({'field':spec['field']['name'],'case':case['name'],'method':method,'rep':rep,'wall_seconds':wall,'peak_rss_kib':value['peak_rss_kib'],'roots':roots,'branch':value['result']['branch'],'stdout_sha256':hashlib.sha256(r.stdout.encode()).hexdigest()})
 finish(out,None,START,'Fresh-process exact quintic-root/multiplicity comparisons across frozen fields; all setup, extraction, verification and output charged.','Use fastest specialized comparator per case, retain losses and all field families; scoped kernel results never qualify original F4/F5/F6 gate.',budget=budget+6)
except Exception as e:finish(out,repr(e),START,'Bounded cold exact-root kernel panel.','Preserve partial runs and diagnose changed field/branch cost before any new experiment.',budget=(216 if group=='binary'else 126))
