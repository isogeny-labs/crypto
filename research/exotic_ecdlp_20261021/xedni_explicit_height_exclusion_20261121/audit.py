"""Exact invariant, x-coordinate, and rational-inequality certificate."""

import hashlib
import json
from fractions import Fraction as F
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parent
O=None


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def add(P,Q):
    if P is O:return Q
    if Q is O:return P
    x1,y1=P;x2,y2=Q
    if x1==x2 and y1==-y2:return O
    slope=((3*x1*x1-94*x1+751)/(2*y1) if P==Q else (y2-y1)/(x2-x1))
    x3=slope*slope+47-x1-x2
    return (x3,slope*(x1-x3)-y1)


def mul(n,P):
    R=O
    while n:
        if n&1:R=add(R,P)
        P=add(P,P);n//=2
    return R


def main():
    p=json.loads((HERE/"protocol.json").read_text())
    r=json.loads((HERE/"result.json").read_text())
    assert sha(ROOT/"xedni_rank1_generator_certificate_20261121"/"result.json")==p["source_generator_sha256"]
    assert sha(ROOT/"xedni_rank1_generator_certificate_20261121"/"lift-class-result.json")==p["source_lift_class_sha256"]
    a2,a4,a6=-47,751,-3997
    b2,b4,b6=4*a2,2*a4,4*a6
    b8=4*a2*a6-a4*a4
    D=-(b2*b2*b8+8*b4**3+27*b6*b6-9*b2*b4*b6)
    c4=b2*b2-24*b4
    j=F(c4**3,D)
    assert (D,b2,j)==(-2835200,-188,F(1362944,11075))
    assert (r["discriminant"],r["b2"],r["j_invariant"])==(str(D),str(b2),str(j))
    Q=(F(17),F(10))
    P=mul(65,Q)
    x,y=P
    assert y*y==x**3-47*x*x+751*x-3997
    num_bits=abs(x.numerator).bit_length()
    den_bits=x.denominator.bit_length()
    assert (num_bits,den_bits)==(2894,2891)
    assert (num_bits,den_bits)==(r["witness_x_numerator_bits"],r["witness_x_denominator_bits"])
    assert hashlib.sha256(str(x).encode()).hexdigest()==r["witness_x_sha256"]
    # For nonnegative logarithmic heights: h(Delta)<22, h_oo(j)<7,
    # h_oo(b2/12)<4, h(j)<21, using log(2)<1 and exact bit/ratio checks.
    assert abs(D)<2**22 and max(abs(j.numerator),j.denominator)<2**21
    assert abs(j)<2**7 and F(abs(b2),12)<2**4
    mu_upper=F(22,12)+F(7,12)+F(4,2)+F(1,2)
    branch_lower_upper=2*(F(21,24)+mu_upper+1)
    branch_upper_upper=2*(mu_upper+2)
    assert mu_upper==F(59,12) and max(branch_lower_upper,branch_upper_upper)<14
    # log(2)>2/3 and log(2)<7/10 are checked in the Lean module.
    height_lower=F(441)*(F(2893)*F(2,3)-14)/4225-14
    cap_height_upper=F(260)*F(7,10)
    assert height_lower==F(785218,4225) and cap_height_upper==182
    assert height_lower>cap_height_upper
    print(json.dumps({"schema":"xedni-explicit-height-exclusion-audit-v1","status":"PASS",
        "discriminant":D,"j_invariant":str(j),"x65_numerator_bits":num_bits,
        "x65_denominator_bits":den_bits,"certified_silverman_upper":14,
        "target_naive_height_lower_rational":str(height_lower),
        "height_cap_upper_rational":str(cap_height_upper),
        "excluded_H_at_most":"2^130"},indent=2,sort_keys=True))


if __name__=="__main__":main()
