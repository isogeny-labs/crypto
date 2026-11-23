import json,math
R=json.load(open('RESULTS.json'));A={}
for n,c in R['cells'].items():
  cs=c['curves'];a={'identity_mismatch_total':sum(x['identity_mismatch'] for x in cs)}
  for l in (2,3,4):
    xs=[x[f'l{l}'] for x in cs if f'l{l}' in x]
    if xs: o=sum(x['obs'] for x in xs);e=sum(x['null'] for x in xs);a[f'exact_l{l}']=dict(curves=len(xs),obs=o,null=round(e,1),ratio=round(o/e,4),z=round((o-e)/math.sqrt(e),2),per_curve=[round(x['obs']/x['null'],3) for x in xs])
    s=[x[f'l{l}_sis'] for x in cs if f'l{l}_sis' in x]
    if s: o=sum(x['est'] for x in s);e=sum(x['null'] for x in s);se=math.sqrt(sum(x['se']**2 for x in s));a[f'sis_l{l}']=dict(est=round(o),null=round(e),ratio=round(o/e,4),ratio_se=round(se/e,4))
  for k in ('sigma_ok','sigma_zero','betaV_closed','nV'):
    v=[x[l][k] for x in cs for l in('l2','l3') if l in x and k in x[l]]
    if v:a[k]=sum(v)
  sh=[(x['l3']['obs'],x['shuffle_l3'],x['l4']['obs'],x['shuffle_l4'],x['sis_l4']) for x in cs if 'shuffle_l3' in x]
  if sh:a['shuffle_vs_obs']=sh
  pl=[x['planted'] for x in cs if 'planted' in x]
  if pl:a['planted']=pl
  a['lstar']=[(x['lstar'],x['lstar_shuffled'],x['lstar_formula']) for x in cs if 'lstar' in x]
  A[n]=a
R['aggregate']=A;json.dump(R,open('RESULTS.json','w'),indent=1);print(json.dumps(A,indent=0))
