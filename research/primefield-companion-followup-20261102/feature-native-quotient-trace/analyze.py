"""Strict prospective collector parser; workload data must come from admitted run."""
import csv,json,sys
from pathlib import Path
FIXTURES=[(17,19),(41,47),(97,79),(193,191),(257,281)]
def analyze(path):
 rows=list(csv.reader(Path(path).read_text().splitlines()));cursor=0;targets=[];hist={}
 for p,r in FIXTURES:
  for k in range(r):
   if cursor>=len(rows):raise ValueError('missing target')
   row=rows[cursor];cursor+=1
   if len(row)!=7 or row[0]!='target':raise ValueError('target shape')
   pp,rr,kk,calls,count,dropped=map(int,row[1:])
   if (pp,rr,kk)!=(p,r,k) or calls<0 or not 0<=count<=100000 or dropped!=0:raise ValueError('target metadata')
   total=[0,0]
   for index in range(count):
    if cursor>=len(rows):raise ValueError('missing event')
    row=rows[cursor];cursor+=1
    if len(row)!=14 or row[0]!='quotient' or row[12]!='true':raise ValueError('event shape or failed quotient')
    pp,rr,kk,idx,n,d,s,w,route,m,inv=map(int,row[1:12]);modulus=int(row[13])
    if (pp,rr,kk,idx,modulus)!=(p,r,k,index,p):raise ValueError('event attribution')
    if n<1 or d<0 or not 0<=s<=min(d,n-1) or w<0 or m<0 or inv not in (0,1):raise ValueError('event domain')
    expected=0 if route==0 else n if n<=2 else 3 if w<=n-1 else 4
    if route!=expected or (route==0 and (n,d)!=(1,0)):raise ValueError('dispatch mismatch')
    if w<s*(s+1)//2 or w>s*(n-1):raise ValueError('support weight')
    if route==3 and m>2*n-1:raise ValueError('linear product bound')
    key=f'{p}:{n}:{d}:{s}:{w}:{route}';hist[key]=hist.get(key,0)+1;total[0]+=m;total[1]+=inv
   targets.append({'p':p,'r':r,'target_index':k,'source_calls':calls,'quotients':count,'quotient_products':total[0],'quotient_inversions':total[1]})
 if cursor!=len(rows):raise ValueError('extra rows')
 return {'status':'complete_fixed_panel','targets':targets,'histogram_p_n_d_support_weight_route':hist,'scope':'source Fp quotient stages only; no scalar-field or whole-source cost'}
if __name__=='__main__':
 result=analyze(sys.argv[1]);out=Path(sys.argv[2]);out.open('x').write(json.dumps(result,indent=2)+'\n')
