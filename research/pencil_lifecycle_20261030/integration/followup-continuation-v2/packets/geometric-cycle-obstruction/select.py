from pathlib import Path
import json,hashlib,collections
D=Path(__file__).parent;P=Path('/home/ec2-user/crypto-pr2304-persist/research/kummer_coordinates_20261022/results/target-gate-16.jsonl');raw=P.read_bytes();(D/'archive.jsonl').write_bytes(raw)
vs=[json.loads(x) for x in raw.splitlines() if json.loads(x)['p']==739];assert len(vs)<=60
G=vs[0]['cases'][1]['coordinate_witness']['jacobian_expected'];assert all(v['cases'][1]['coordinate_witness']['jacobian_expected']==G for v in vs)
p=739;r=vs[0]['r'];seen=set();forest=collections.defaultdict(list);edges=[];chosen=None;observed=[]
for v in vs:
 for c in v['cases']:
  d=c['coordinate_witness']['jacobian_expected'];record={'target_index':v['target_index'],'case_index':c['case_index']};observed.append(record)
  if d['degree']!=2:record['status']='degree_skip';continue
  canon=(tuple(d['u']),min(tuple(d['v']),tuple((-x)%p for x in d['v'])))
  if canon in seen:record['status']='duplicate';continue
  seen.add(canon);disc=(d['u'][1]**2-4*d['u'][0])%p;root=pow(disc,(p+1)//4,p)
  if root*root%p!=disc or root==0:record['status']='nonsplit_or_repeated';continue
  xs=[(root-d['u'][1])*pow(2,-1,p)%p,(-root-d['u'][1])*pow(2,-1,p)%p];ys=[(d['v'][0]+d['v'][1]*x)%p for x in xs];signs=[1 if y<=p//2 else -1 for y in ys];e={'divisor':d,'scalar':(c['a']+c['b']*v['planted'])%r,'xs':xs,'signs':signs,**record};idx=len(edges);edges.append(e);record['status']='edge'
  start,end=xs;queue=[start];prev={start:None}
  for x in queue:
   for y,j in forest[x]:
    if y not in prev:prev[y]=(x,j);queue.append(y)
  if end in prev:
   path=[];x=end
   while prev[x] is not None:x,j=prev[x];path.append(j)
   chosen=[edges[j] for j in reversed(path)]+[e];break
  forest[start].append((end,idx));forest[end].append((start,idx))
 if chosen:break
out={'schema':'fixed-archived-geometric-cycle-selection/v1','archive_sha256':hashlib.sha256(raw).hexdigest(),'records_available':len(vs),'cases_inspected':len(observed),'observations':observed,'planted':vs[0]['planted'],'p':p,'r':r,'G':G,'cycle':chosen,'status':'selected' if chosen else 'no_cycle_in_bound','planted_label_construction':True}
(D/'selection.json').write_text(json.dumps(out,indent=2));print(out['status'],len(observed),'cases',len(chosen or []),'edges')
