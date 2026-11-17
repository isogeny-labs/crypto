from research.residue_quotient_20261116_v99b.kernel import basis,apply
from research.simple_prefix_lift_20261116_v117b.kernel import normalize
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.field_correction_basis_20261117_v128c.kernel import audited_lift
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic

def evaluate_family(base,columns,x):
 result=list(base)
 for i,column in enumerate(columns):
  if x>>i&1:result=[a^b for a,b in zip(result,column)]
 return result

def filtration(columns,n):
 lower=[];row=0
 for i,col in enumerate(columns):
  lower.append(sum(((col[j]>>k)&1)<<(5*k+j)for k in range(n)for j in range(5)))
  row|=(((col[0]^col[1])>>n)&1)<<i
 piv,K=basis(lower);aug=[a|(((row>>i)&1)<<(5*n))for i,a in enumerate(lower)];pa,_=basis(aug);witness=next((v for v in K if(row&v).bit_count()%2),None);assert len(pa)-len(piv)==int(witness is not None)
 if witness is not None:assert apply(lower,witness)==0 and(row&witness).bit_count()%2==1
 return {'rank':len(piv),'augmented_rank':len(pa),'kernel_dimension':len(K),'witness':witness,'lower_columns':lower,'row':row,'kernel':K}

def numeric_correction(base,columns,x,d,n):
 F=evaluate_family(base,columns,x);coeff=[[(v>>k)&1 for k in range((5-j)*(d-1)+1)]for j,v in enumerate(F)];return audited_lift(coeff,d,Arithmetic(2,[0,1]))['audit'][n-1]['correction'][1]

def actual_family(cfg,chart):
 raw=raw_family(matrix(cfg['matrix']),cfg['modulus']);origin=raw(0);base=normalize(raw(chart['particular']),18);columns=[normalize([a^b for a,b in zip(raw(v),origin)],18)for v in chart['columns']];assert [v&1 for v in base]==[0,0,1,1,1,1];assert all(not any(v&1 for v in col)for col in columns);return base,columns
