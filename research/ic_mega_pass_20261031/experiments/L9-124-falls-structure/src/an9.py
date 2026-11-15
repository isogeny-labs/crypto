"""L9 analysis helpers: load macdump dumps, canonical pivots, bidegree classification, swap test,
K-relation search (which K-valued polynomials in a,b of bit-degree<=3 lie in W^(r)_low (x) K)."""
import sys, os, numpy as np, itertools, json
from sys9 import *
def load(pref,rd):
    monos=[int(x) for x in open(pref+'_monos.txt')]
    b=open('%s_r%d.bin'%(pref,rd),'rb').read(); nl,nr=np.frombuffer(b[:16],dtype=np.int64)
    WL=(nl+63)//64; a=np.frombuffer(b[16:],dtype=np.uint64).reshape(nr,WL)
    rows=[int.from_bytes(r.tobytes(),'little') for r in a]
    return monos,rows
class Basis:
    """GF(2) echelon, pivot = lowest set bit (same convention as macclose)."""
    def __init__(s): s.p={}
    def red(s,r):
        while r:
            lb=r&-r; q=s.p.get(lb)
            if q is None: return r
            r^=q
        return 0
    def add(s,r):
        r=s.red(r)
        if r: s.p[r&-r]=r; return True
        return False
    def __len__(s): return len(s.p)
    def full(s):
        """make reduced echelon (each pivot bit occurs in its own row only); then nf() is linear & canonical"""
        ps=sorted(s.p,key=lambda x:-x.bit_length())  # process from highest pivot
        for lb in sorted(s.p,key=lambda x:x.bit_length(),reverse=True):
            r=s.p[lb]
            for lb2 in s.p:
                if lb2!=lb and s.p[lb2]&lb: s.p[lb2]^=r
        s.pl=list(s.p.items()); return s
    def nf(s,x):
        for lb,r in s.pl:
            if x&lb: x^=r
        return x
def basis(rows):
    B=Basis()
    for r in rows: B.add(r)
    return B
def bideg(m,k):
    return (bin(m&((1<<k)-1)).count('1'), bin(m>>k).count('1'))
def swapmono(m,k): return ((m&((1<<k)-1))<<k)|(m>>k)
def poly_of(row,monos):
    out=[];i=0
    while row:
        lb=row&-row; out.append(monos[lb.bit_length()-1]); row^=lb
    return out
def row_of(poly,idx):
    r=0
    for m in poly: r^=1<<idx[m]
    return r
