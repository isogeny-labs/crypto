"""Construct the three degree-two quotient invariants from Magma matrices."""

import json
import re
from pathlib import Path

root = Path(__file__).resolve().parent
source = json.loads((root.parent / 'ghs_quartic_new_classes_q8_20261026' / 'search_results.json').read_text())
witness = next(w for w in source['witnesses'] if w['group_order'] == 554)
receipt = json.loads((root / '554_basechange_receipt.json').read_text())
stdout = receipt['response']['stdout']
assert receipt['response']['success']
assert 'extension_field_polynomial $.1^9 + $.1^4 + 1' in stdout
assert 'extension_automorphism_order 4' in stdout

K = GF(512, 'w', modulus='conway')
w = K.gen()
a = w^73
assert a^3+a+1 == 0
R = PolynomialRing(K, names=('x','y','z'))
x,y,z = R.gens()
exponents = [(i,j,4-i-j) for i in range(4,-1,-1) for j in range(4-i,-1,-1)]
F = sum(sum((a^bit for bit in range(3) if c & (1 << bit)), K(0)) * x^i*y^j*z^k
        for c,(i,j,k) in zip(witness['coefficients'],exponents))

def parse_matrix(block):
    rows = re.findall(r'^\[([^\]]+)\]$', block, re.M)
    assert len(rows) == 3, rows
    def elt(s):
        if s == '1': return K(1)
        if s == '0': return K(0)
        assert s.startswith('$.1^'), s
        return w^ZZ(s[4:])
    return matrix(K, [[elt(t) for t in row.split()] for row in rows])

blocks = re.findall(r'extension_generator_matrix\s*\n((?:\[[^\n]+\]\n){3})', stdout)
assert len(blocks) == 2
M1,M2 = [parse_matrix(block) for block in blocks]

def preserves(M):
    v=M*vector(R,[x,y,z])
    return F(*v)

assert all(preserves(M)==preserves(M).monomial_coefficient(x^4)*F for M in (M1,M2))
M1=M1/(M1*M1)[0,0].sqrt()
M2=M2/(M2*M2)[0,0].sqrt()
assert M1*M1 == identity_matrix(K,3)
assert M2*M2 == identity_matrix(K,3)
assert preserves(M1)==F and preserves(M2)==F

S = PolynomialRing(K,names=('X','Y','Z'))
X,Y,Z = S.gens()
Uring = PolynomialRing(K,names=('Y','Z'))
Y0,Z0=Uring.gens()

results=[]
for index,M in enumerate((M1,M2,M1*M2),1):
    N=M-identity_matrix(K,3)
    assert N.rank()==1 and N*N==0
    Xrow=next(vector(K,[1 if j==i else 0 for j in range(3)])
              for i in range(3) if vector(K,[1 if j==i else 0 for j in range(3)])*N != 0)
    Yrow=Xrow*N
    Zrow=next(row for row in N.left_kernel().basis()
              if matrix(K,[Xrow,Yrow,row]).rank()==3)
    T=matrix(K,[Xrow,Yrow,Zrow])
    assert T*M*T^-1 == matrix(K,[[1,1,0],[0,1,0],[0,0,1]])
    v=T^-1*vector(S,[X,Y,Z])
    H=F(*v)
    assert H(X+Y,Y,Z)==H
    c=H.monomial_coefficient(X^4)
    B2=H.monomial_coefficient(X^2*Y^2)*Y^2+H.monomial_coefficient(X^2*Y*Z)*Y*Z+H.monomial_coefficient(X^2*Z^2)*Z^2+c*Y^2
    C4=H(X=0)
    U=X^2+X*Y
    assert H==c*U^2+B2*U+C4
    b2=Uring(B2(X=0,Y=Y0,Z=Z0))
    c4=Uring(C4(X=0,Y=Y0,Z=Z0))
    assert c != 0
    u=polygen(K,'u')
    affine_count=sum(len((c*u^2+b2(t,1)*u+c4(t,1)).roots()) for t in K)
    infinity_count=len((c*u^2+b2(1,0)*u+c4(1,0)).roots())
    points=affine_count+infinity_count
    encode=lambda t: int(-1) if t==0 else int(t.log(w))
    results.append({'index':int(index),'matrix':[[encode(v) for v in row] for row in M],
                    'coordinate_transform':[[encode(v) for v in row] for row in T],
                    'c':str(c),'B2':str(b2),'C4':str(c4),
                    'c_exponent':encode(c),
                    'B2_exponents':[encode(b2.monomial_coefficient(Y0^i*Z0^(2-i))) for i in (2,1,0)],
                    'C4_exponents':[encode(c4.monomial_coefficient(Y0^i*Z0^(4-i))) for i in (4,3,2,1,0)],
                    'affine_points':int(affine_count),'infinity_points':int(infinity_count),
                    'quotient_points':int(points)})
    print('quotient',index,'points',points,'B2',b2,'C4',c4)

(root/'explicit_invariants.json').write_text(json.dumps({'schema':'ghs-q8-quartic-invariants/v1',
    'source_order':int(554),'field_modulus':'w^9+w^4+1','results':results},indent=2)+'\n')
