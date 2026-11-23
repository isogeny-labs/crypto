#!/usr/bin/env python3
"""alpha* = H^{-1}((1 - 3/m)/omega'): the largest fall-degree slope per unknown bit
compatible with the parity gate s < m - 3 under T >= M(N, alpha N)^{omega'}.
Lays out a derived table; computes no research result from data."""
import math
def H(a): return -a*math.log2(a)-(1-a)*math.log2(1-a)
def Hinv(h, lo=1e-12, hi=0.5):
    for _ in range(200):
        mid=(lo+hi)/2
        if H(mid)<h: lo=mid
        else: hi=mid
    return lo
if __name__=="__main__":
    print("m | omega'=1 | omega'=2 | omega'=3")
    for m in (4,5,6,8,10):
        print(m, *[f"{Hinv((1-3/m)/w):.4f}" for w in (1,2,3)])
