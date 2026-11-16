#!/usr/bin/env sage
"""Independent point-count and Weil-polynomial replay for two trace-minus14 carriers."""
import hashlib
import json
from pathlib import Path

masks=(9735,17371)
rows=[]
for mask in masks:
 counts=[]
 for r in range(1,6):
  K=GF(3^r,name='u')
  coeff=[(mask//(3^i))%3 for i in range(11)]
  n=1
  for x in K:
   v=x^11+sum(K(coeff[i])*x^i for i in range(11))
   n+=1 if v==0 else (2 if v.is_square() else 0)
  counts.append(int(n))
 traces=[3^(i+1)+1-counts[i] for i in range(5)]
 c=[1]
 for n in range(1,6):
  numerator=-sum(c[n-j]*traces[j-1] for j in range(1,n+1))
  assert numerator%n==0
  c.append(numerator//n)
 full=c+[0]*5
 for i in range(5):full[10-i]=3^(5-i)*c[i]
 assert full==[1,0,0,0,0,14,0,0,0,0,243]
 assert counts==[4,10,28,82,314]
 rows.append({'fmask':mask,'point_counts_F3_powers_1_through_5':counts,
              'frobenius_power_sums_1_through_5':traces,
              'L_polynomial_coefficients':full})
source=Path(__file__).with_suffix('')
print(json.dumps({'schema':'ghs.trace-minus14.carrier-pointcount-replay/v1',
                  'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
                  'rows':rows},sort_keys=True,default=int))
