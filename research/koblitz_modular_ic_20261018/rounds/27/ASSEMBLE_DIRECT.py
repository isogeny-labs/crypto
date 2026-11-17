#!/usr/bin/env python3
"""Assemble literal existing coefficients; all field arithmetic stays native."""
import hashlib,json
from pathlib import Path
here=Path(__file__).resolve().parent
prior=Path('/home/ec2-user/crypto-modular-ic/research/koblitz_modular_ic_20261018/rounds/15/runs')
out=here/'source-data';out.mkdir(exist_ok=True)
header=(here/'NODE-header.sing').read_text().split('proc hexField')[1].split('list millerP')[0]
hexproc='proc hexField'+header
binary='''ring binaryDirect27=2,(z),lp;
proc binaryBits(string encoded){
 poly value=0;poly nibble;int pos;int digit;int bit;
 string alphabet="0123456789abcdef";
 for(pos=1;pos<=size(encoded);pos++){
  for(digit=0;digit<16;digit++){if(encoded[pos]==alphabet[digit+1]){break;}}
  if(digit==16){ERROR("bad original bits");}
  nibble=0;for(bit=0;bit<4;bit++){if((digit div (2^bit))%2==1){nibble=nibble+z^bit;}}
  value=value*z^4+nibble;
 }
 return(value);
}
'''
rows=[];manifest=[]
for fn in [1,2]:
 raw=(prior/f'm16-function-{fn}.json').read_bytes();manifest.append((hashlib.sha256(raw).hexdigest(),f'm16-function-{fn}.json'))
 (out/f'm16-function-{fn}.json').write_bytes(raw);data=json.loads(raw)
 raw=(prior/f'm15-function-{fn}.json').read_bytes();manifest.append((hashlib.sha256(raw).hexdigest(),f'm15-function-{fn}.json'))
 (out/f'm15-function-{fn}.json').write_bytes(raw);norm=json.loads(raw)
 for p in data['points']:
  if p['stratum']=='ordinary':rows.append((fn,p,norm))
with (out/'direct-input-template.sing').open('w')as f:
 f.write(binary)
 for index,(fn,p,norm) in enumerate(rows):
  degree=p['closed_degree'];primes={436:[2,109],12:[2,3],284:[2,71],83:[83]}[degree]
  f.write(f'''setring binaryDirect27;
poly modulus{index}=binaryBits("{p['x_minpoly_bits']}");
if(deg(modulus{index})!={degree}){{ERROR("direct modulus degree mismatch");}}
poly square{index}=z;int stage{index};
for(stage{index}=1;stage{index}<={degree};stage{index}++){{
 square{index}=reduce(square{index}^2,ideal(modulus{index}));
''')
  for prime in primes:f.write(f' if(stage{index}=={degree//prime} && gcd(square{index}+z,modulus{index})!=1){{ERROR("direct Rabin gcd failed");}}\n')
  f.write(f'''}}
if(square{index}!=z){{ERROR("direct Rabin final failed");}}
ring pairField{index}=(2,beta),(X),lp;
''')
  f.write(hexproc)
  f.write(f'''minpoly=hexField("{p['x_minpoly_bits']}");
number firstX=beta;number firstY=hexField("{p['y_residue_bits']}");
if(firstY^2+firstX*firstY+firstX^3+firstX^2+1!=0){{ERROR("actual first point not on E49");}}
number normA=hexField("{norm['A_bits']}");number normB=hexField("{norm['B_bits']}");
if(normA+normB*firstY!=0){{ERROR("actual recovered norm not zero at support");}}
''')
  denominator='firstY' if fn==1 else 'firstX'
  f.write(f'''number denominator27={denominator};
if(denominator27==0){{ERROR("actual ordinary inverse denominator zero");}}
number secondY=1/denominator27;
poly imageCubic=X^3+X^2+secondY*X+1+secondY^2;
list imageFactors=factorize(imageCubic);
poly rebuilt=1;int factorIndex;int linearCount=0;int totalDegree=0;number imageRoot;
print("DIRECT27_SUPPORT function={fn} degree={degree} multiplicity={p['multiplicity']}");
for(factorIndex=1;factorIndex<=size(imageFactors[1]);factorIndex++){{
 rebuilt=rebuilt*imageFactors[1][factorIndex]^imageFactors[2][factorIndex];
 totalDegree=totalDegree+deg(imageFactors[1][factorIndex])*imageFactors[2][factorIndex];
 print("CUBIC_FACTOR degree="+string(deg(imageFactors[1][factorIndex]))+" multiplicity="+string(imageFactors[2][factorIndex]));
 if(deg(imageFactors[1][factorIndex])==1){{
  linearCount=linearCount+1;imageRoot=leadcoef(subst(imageFactors[1][factorIndex],X,0))/leadcoef(imageFactors[1][factorIndex]);
  if(secondY^2+imageRoot*secondY+imageRoot^3+imageRoot^2+1!=0){{ERROR("second image root fails target curve");}}
  print("SECOND_IMAGE_LINEAR_ROOT");imageRoot;
 }}
}}
if(rebuilt!=imageCubic || totalDegree!=3){{ERROR("actual image cubic factor reconstruction failed");}}
print("DIRECT27_RATIONAL_SECOND_X_COUNT");linearCount;
print("DIRECT27_IMAGE_CUBIC_FACTORIZATION");imageFactors;
''')
  f.write('kill hexField;\n')
 f.write('print("DIRECT27_ALL_CHECKS_PASS");quit;\n')
(out/'direct-original-inputs.sha256').write_text(''.join(h+'  '+n+'\n'for h,n in manifest))
