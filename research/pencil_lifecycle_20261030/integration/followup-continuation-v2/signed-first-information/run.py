from common import *
from certificates import solve
import csv,copy,io

def boundary(p,n,rows):
 end=solve(p,n,rows)
 if end['kind']=='undetermined':return None,end,None
 assert end['kind']=='determined'
 lo=0;hi=len(rows)
 while hi-lo>1:
  mid=(hi+lo)//2;c=solve(p,n,rows[:mid]);assert c['kind']!='inconsistent'
  if c['kind']=='determined':hi=mid
  else:lo=mid
 return hi,solve(p,n,rows[:hi]),solve(p,n,rows[:hi-1])

def main():
 sources={str(f.relative_to(P.parent)):h(f.read_bytes()) for f in [P/'PLAN.md',P/'run.py',P/'common.py',W/'certificates.py',W/'check_certificate.py']}
 controls=[]
 for name,n,rows,want in [('empty',0,[],None),('immediate',0,[[1,5]],1),('delayed',1,[[1,0,3],[1,1,1]],2),('undetermined',1,[[1,1,3]],None)]:
  first,c,prev=boundary(7,n,rows);assert first==want and check_boundary(7,n,rows,first,c,prev)
  if first:
   bad=copy.deepcopy(c);bad['scalar']=(bad['scalar']+1)%7;assert not check_boundary(7,n,rows,first,bad,prev)
   assert not check_boundary(7,n,rows,0,c,prev)
  controls.append({'name':name,'passed':True})
 inputs,bindings=load_inputs();records=[];members=[]
 with tarfile.open(P/'boundaries.tar.gz','w:gz') as t:
  for name,item in sorted(inputs.items()):
   d=item['system'];p=d['modulus'];n=d['nuisance_columns'];rows=d['rows'];first,c,prev=boundary(p,n,rows)
   assert check_boundary(p,n,rows,first,c,prev)
   if first:assert c['scalar']==d['retained_scalar']
   origin=d['row_origins'][first-1] if first else None;terminal=item['terminal'];attempt=origin['attempt'] if origin else None
   r={'stream':name,'kind':c['kind'],'total_rows':len(rows),'first_row_count':first,'first_sequence':origin['sequence'] if origin else None,'first_attempt':attempt,'native_terminal_attempt':terminal['attempts'],'attempts_after_first':terminal['attempts']-attempt if attempt is not None else None,'rows_after_first':len(rows)-first if first else None,'scalar':c.get('scalar'),'native_scalar':terminal['recovered']}
   artifact={'schema':'signed.first-information/v1','source_stream':name,'witness_sha256':item['witness_sha256'],'first_row_count':first,'certificate':c,'predecessor':prev,'record':r}
   raw=(json.dumps(artifact,separators=(',',':'))+'\n').encode();fn=name.replace('.jsonl','.boundary.json');info=tarfile.TarInfo(fn);info.size=len(raw);t.addfile(info,io.BytesIO(raw));members.append({'file':fn,'sha256':h(raw),'bytes':len(raw)});records.append(r);print(json.dumps(r),flush=True)
 assert all(h((P.parent/f).read_bytes())==v for f,v in sources.items())
 summary={'schema':'signed.first-information.summary/v1','records':records,'systems':len(records),'determined':sum(r['kind']=='determined' for r in records),'undetermined':sum(r['kind']=='undetermined' for r in records),'controls':controls,'input_bindings':bindings,'source_hashes':sources,'collector_invocations':0,'independent_review':'pending'}
 (P/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');(P/'boundary-manifest.json').write_text(json.dumps({'members':members},indent=2)+'\n')
 with (P/'systems.csv').open('w',newline='') as f:
  w=csv.DictWriter(f,fieldnames=list(records[0]),lineterminator='\n');w.writeheader();w.writerows(records)
if __name__=='__main__':main()
