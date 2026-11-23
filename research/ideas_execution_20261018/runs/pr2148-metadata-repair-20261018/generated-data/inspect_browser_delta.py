import collections,hashlib,importlib.util,json,pathlib
root=pathlib.Path('/workspace/crypto');p=pathlib.Path('/tmp/pr-followup-20261018/crypto/leaderboard')
spec=importlib.util.spec_from_file_location('browser_builder',root/'scripts/build_lab_browser.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
b=json.loads((p/'browser-before.json').read_text());a=json.loads((p/'browser-candidate.json').read_text())
def diff(before,after):
 out=[];lengths=[]
 def path(p,k):return p+'/'+str(k).replace('~','~0').replace('/','~1')
 def leaves(v,p,kind):
  if isinstance(v,dict):
   for k,x in v.items():leaves(x,path(p,k),kind)
  elif isinstance(v,list):
   for i,x in enumerate(v):leaves(x,path(p,i),kind)
  else:out.append({'path':p,'kind':kind,'before':v if kind=='removed' else None,'after':v if kind=='added' else None})
 def walk(x,y,p=''):
  if type(x)!=type(y):out.append({'path':p,'kind':'type_changed','before':x,'after':y});return
  if isinstance(x,dict):
   for k in sorted(x.keys()|y.keys()):
    if k not in x:leaves(y[k],path(p,k),'added')
    elif k not in y:leaves(x[k],path(p,k),'removed')
    else:walk(x[k],y[k],path(p,k))
  elif isinstance(x,list):
   if len(x)!=len(y):lengths.append({'path':p,'before':len(x),'after':len(y)})
   for i in range(min(len(x),len(y))):walk(x[i],y[i],path(p,i))
   for i in range(len(y),len(x)):leaves(x[i],path(p,i),'removed')
   for i in range(len(x),len(y)):leaves(y[i],path(p,i),'added')
  elif x!=y:out.append({'path':p,'kind':'changed','before':x,'after':y})
 walk(before,after);return out,lengths
raw,lengths=diff(b,a);stable,stablelengths=diff(module.comparable(b),module.comparable(a));num=lambda v:isinstance(v,(int,float)) and not isinstance(v,bool)
summary={'before_sha256':hashlib.sha256((p/'browser-before.json').read_bytes()).hexdigest(),'candidate_sha256':hashlib.sha256((p/'browser-candidate.json').read_bytes()).hexdigest(),'current_repository_browser_sha256':hashlib.sha256((root/'docs/browser/data.json').read_bytes()).hexdigest(),'repository_browser_unchanged':(root/'docs/browser/data.json').read_bytes()==(p/'browser-before.json').read_bytes(),'raw_leaf_changes':raw,'raw_leaf_change_count':len(raw),'raw_numeric_change_count':sum(num(x['before']) or num(x['after']) for x in raw),'raw_array_length_changes':lengths,'check_comparable_leaf_changes':stable,'check_comparable_leaf_change_count':len(stable),'check_comparable_numeric_change_count':sum(num(x['before']) or num(x['after']) for x in stable),'check_comparable_array_length_changes':stablelengths,'raw_changed_sections':sorted({x['path'].split('/')[1] for x in raw}),'raw_change_counts_by_section':dict(collections.Counter(x['path'].split('/')[1] for x in raw)),'counts_before':b['counts'],'counts_after':a['counts'],'check_semantics':'official build_lab_browser.comparable; ignores candidates[*].mentions/files/where only'}
(p/'browser-semantic-delta.json').write_text(json.dumps(summary,indent=2,ensure_ascii=False)+'\n')
print(json.dumps({k:v for k,v in summary.items() if k not in {'raw_leaf_changes'}},indent=2,ensure_ascii=False))
