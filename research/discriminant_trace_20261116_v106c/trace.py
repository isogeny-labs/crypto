import time,json,traceback
from .common import P,finish
from research.discriminant_roots_20261116_v106.kernel import Field,solve
START=time.monotonic();out={'H':[0,0,1,1,1,1],'field':{'p':3,'mod':[0,1]},'traces':{},'original_complete':False}
for method in ['direct','radical']:
 try:out['traces'][method]={'result':solve(out['H'],Field(3,[0,1]),method,1061611)}
 except Exception as e:out['traces'][method]={'error':repr(e),'traceback':traceback.format_exc()}
finish(out,None,START,'Narrow instrumented regression records exact failing code boundary; no repeated broad execution.','Repair the identified boundary in a new immutable source and rerun every independent oracle before timing.')
