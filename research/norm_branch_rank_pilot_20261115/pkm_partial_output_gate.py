#!/usr/bin/env python3
"""Optimistic m=8 PKM partial-useful-output exponent model; no measurement.

B=p^beta, per-call cost B^gamma, and useful independent output <= B^delta.
This is a necessary optimistic cost gate, not a measured attack.
"""
import json
from fractions import Fraction


def exponent(beta, gamma, delta):
    return max((gamma+1-delta)*beta, 1+(gamma-7)*beta, 2*beta)


def optimum(gamma, delta):
    assert gamma >= 0 and 0 <= delta <= 1
    if gamma >= 7:
        return Fraction(0), Fraction(1)
    h = max(gamma+1-delta, Fraction(2))
    b = Fraction(1)/(7-gamma+h)
    assert 0 <= b <= Fraction(1,7)
    e = exponent(b,gamma,delta)
    assert e == b*h == 1+b*(gamma-7)
    return b,e


def run():
    rows=[]
    deltas=(Fraction(0),Fraction(1,4),Fraction(1,2),Fraction(3,4),Fraction(1))
    gammas=(Fraction(14,5),Fraction(31,10),Fraction(33,10),Fraction(69,20),Fraction(7,2))
    for delta in deltas:
        for gamma in gammas:
            beta,e=optimum(gamma,delta)
            rows.append(dict(delta=str(delta),gamma=str(gamma),
                optimal_beta=str(beta),projected_exponent=str(e),
                beats_half=e<Fraction(1,2),rank_yield_gate=str(3+delta/2)))
    for delta in deltas:
        for gamma in (Fraction(0),Fraction(1),Fraction(2),
                      Fraction(29,10),Fraction(33,10),Fraction(7,2),
                      Fraction(6),Fraction(7),Fraction(8)):
            b,e=optimum(gamma,delta)
            for i in range(4097):
                assert exponent(Fraction(i,4096*7),gamma,delta)>=e
            if gamma<7:
                assert exponent(b,gamma,delta)==e
    assert optimum(Fraction(2),Fraction(1))[1]==Fraction(2,7)
    assert optimum(Fraction(7,2),Fraction(1))[1]==Fraction(1,2)
    assert optimum(Fraction(3),Fraction(0))[1]==Fraction(1,2)
    assert optimum(Fraction(3),Fraction(1,2))[1]<Fraction(1,2)
    return dict(standing='[pcc:unverified]',
        model='optimistic output-rank-aware sparse PKM m=8',
        work='max(B^(1-delta), p/B^7)*B^gamma + B^2',
        assumptions=['B=p^beta','c(B)=B^gamma',
          'independent useful output per call <= B^delta',
          '0<=delta<=1','no additional setup cost',
          'per-call work includes failures and witness extraction'],
        formula='h=max(2,gamma+1-delta); beta=1/(7-gamma+h); E=h/(7-gamma+h) for gamma<7',
        beat_rho_necessary='gamma < 3 + delta/2',
        rows=rows,verification='exact rational intersection and 4097-cell beta-grid checks')


if __name__=='__main__':
    print(json.dumps(run(),indent=2,sort_keys=True))
