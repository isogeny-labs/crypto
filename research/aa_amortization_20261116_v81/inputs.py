import json,hashlib
def materialize(n,d):
 out=[]
 for delta in range(n):
  v=dict(d['base']);v['beta']=[x+delta for x in d['base']['beta']];v['shift_high']=d['base']['shift_high']-delta;v['translation_delta']=delta;out.append(v)
 return out

def digest(v):return hashlib.sha256(json.dumps(v,sort_keys=True,separators=(',',':')).encode()).hexdigest()
