import hashlib,json,pathlib,shutil
base=pathlib.Path('/tmp/pr-followup-20261018/crypto/leaderboard-before');out=pathlib.Path('/tmp/pr-followup-20261018/crypto/leaderboard');root=pathlib.Path('/workspace/crypto');after=out/'after';after.mkdir(exist_ok=True)
files={'leaderboard.json':'docs/ic/leaderboard.json','LEADERBOARD.md':'docs/ic/LEADERBOARD.md','ic-leaderboard.html':'docs/ic-leaderboard.html'}
manifest=[]
for name,rel in files.items():
 shutil.copyfile(root/rel,after/name)
 b=(base/name).read_bytes();a=(root/rel).read_bytes()
 manifest.append({'path':rel,'before_bytes':len(b),'after_bytes':len(a),'before_sha256':hashlib.sha256(b).hexdigest(),'after_sha256':hashlib.sha256(a).hexdigest(),'changed':a!=b})
b=json.loads((base/'leaderboard.json').read_text());a=json.loads((root/'docs/ic/leaderboard.json').read_text());changes=[];lengths=[]
def numeric(v):return isinstance(v,(int,float)) and not isinstance(v,bool)
def ptr(path,k):return path+'/'+str(k).replace('~','~0').replace('/','~1')
def add_leaves(value,path,kind):
 if isinstance(value,dict):
  for k,v in value.items():add_leaves(v,ptr(path,k),kind)
 elif isinstance(value,list):
  for i,v in enumerate(value):add_leaves(v,ptr(path,i),kind)
 else:changes.append({'path':path,'kind':kind,'before':value if kind=='removed' else None,'after':value if kind=='added' else None})
def walk(x,y,path=''):
 if type(x)!=type(y):changes.append({'path':path,'kind':'type_changed','before':x,'after':y});return
 if isinstance(x,dict):
  for k in sorted(x.keys()|y.keys()):
   if k not in x:add_leaves(y[k],ptr(path,k),'added')
   elif k not in y:add_leaves(x[k],ptr(path,k),'removed')
   else:walk(x[k],y[k],ptr(path,k))
 elif isinstance(x,list):
  if len(x)!=len(y):lengths.append({'path':path,'before':len(x),'after':len(y)})
  for i in range(min(len(x),len(y))):walk(x[i],y[i],ptr(path,i))
  for i in range(len(y),len(x)):add_leaves(x[i],ptr(path,i),'removed')
  for i in range(len(x),len(y)):add_leaves(y[i],ptr(path,i),'added')
 elif x!=y:changes.append({'path':path,'kind':'changed','before':x,'after':y})
walk(b,a)
num=[x for x in changes if numeric(x['before']) or numeric(x['after'])]
sourcechanges=[x for x in changes if x['path'].startswith('/sources/')]
unchanged=[k for k in a if k in b and a[k]==b[k]]
summary={'leaf_changes':changes,'leaf_change_count':len(changes),'numeric_changes':num,'numeric_change_count':len(num),'source_changes':sourcechanges,'source_change_count':len(sourcechanges),'array_length_changes':lengths,'unchanged_top_level_sections':unchanged,'changed_top_level_sections':[k for k in a if k not in b or a[k]!=b[k]],'files':manifest}
(out/'semantic-delta.json').write_text(json.dumps(summary,indent=2,ensure_ascii=False)+'\n')
print(json.dumps({k:v for k,v in summary.items() if k not in {'leaf_changes','numeric_changes','files'}},indent=2,ensure_ascii=False))
print('ALL_LEAF_CHANGES')
for x in changes:print(json.dumps(x,ensure_ascii=False))
