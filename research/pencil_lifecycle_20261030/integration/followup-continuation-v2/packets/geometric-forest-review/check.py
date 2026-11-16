import pathlib,json,hashlib,collections
src=pathlib.Path('/var/tmp/pencil-kummer-geometric-cycle-20261031');out=pathlib.Path(__file__).parent
rows=[json.loads(s) for s in (src/'archive.jsonl').read_text().splitlines() if json.loads(s)['p']==739];seen=set();counts=collections.Counter();parent={};edges=[];dispositions=[];cycles=0

def find(x):
 parent.setdefault(x,x)
 while parent[x]!=x:x=parent[x]
 return x
for row in rows:
 for c in row['cases']:
  d=c['coordinate_witness']['jacobian_expected'];status='degree_skip'
  if d['degree']==2:
   key=(tuple(d['u']),min(tuple(d['v']),tuple(-z%739 for z in d['v'])))
   if key in seen:status='duplicate'
   else:
    seen.add(key);roots=[x for x in range(739) if (x*x+d['u'][1]*x+d['u'][0])%739==0]
    status='nonsplit_or_repeated'
    if len(roots)==2:
     status='edge';a,b=roots;ra,rb=find(a),find(b);cycles+=ra==rb
     if ra!=rb:parent[ra]=rb
     edges.append({'xs':roots,'target_index':row['target_index'],'case_index':c['case_index']})
  counts[status]+=1;dispositions.append({'target_index':row['target_index'],'case_index':c['case_index'],'status':status})
producer=json.loads((src/'selection.json').read_text());assert dispositions==producer['observations'];components=len({find(x) for x in parent});assert len(edges)==len(parent)-components;assert cycles==0
result={'schema':'research.component-review/v1','archive_sha256':hashlib.sha256((src/'archive.jsonl').read_bytes()).hexdigest(),'records':len(rows),'cases':len(dispositions),'counts':dict(counts),'vertices':len(parent),'components':components,'edges':len(edges),'cycles':cycles,'dispositions_match':True,'scope':'deduplicated degree-two distinct-root edge graph only; duplicate relations and known-answer label assignments are excluded, not shown incapable of constraints','predecessor_attempts':0,'scientific_trials':0}
(out/'result.json').write_text(json.dumps(result,indent=2)+'\n');(out/'edges.json').write_text(json.dumps(edges,indent=2)+'\n');print(json.dumps(result))
