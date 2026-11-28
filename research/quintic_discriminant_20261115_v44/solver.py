"""Inversion-free binary quintic discriminant and exact adaptive support."""
from research.certified_support_20261115_v44 import solver as old
from rootfinder import binaryfield as b

def repeated(f,H):
 assert len(H)==6 and H[-1]==1
 e,d,c,bb,a,_=H
 C=f._raw_mul(a,bb)^c;E=f._raw_mul(a,d)^e
 delta=f._raw_square(E)^f._raw_mul(bb,f._raw_mul(E,C))^f._raw_mul(d,f._raw_square(C))
 return delta==0

def supported(f,H,lam):
 # R=0 is always sufficient. Only repeated H require the nilpotence fallback.
 repeat=repeated(f,H);rem=old.remainder(f,H,lam)
 if not repeat:return not rem
 for _ in range(3):rem=b._squaremod(rem,H,f)
 return not rem
