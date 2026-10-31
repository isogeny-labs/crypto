#!/usr/bin/env python3
"""Emit a PARI/GP check of the exact published ECCp-131 group and points."""
import json
from pathlib import Path

here = Path(__file__).parent
v = {k:int(x,16) for k,x in json.loads((here/'runs/replay.json').read_text())['parameters'].items()}
p,a,b,n = (v[k] for k in ('p','a','b','n'))
gx,gy,qx,qy = (v[k] for k in ('gx','gy','qx','qy'))
text = f'''\\ Exact published curve and target identity, independent PARI/GP check
p={p};a={a};b={b};n={n};
E=ellinit([a,b],p);
G=[Mod({gx},p),Mod({gy},p)];Q=[Mod({qx},p),Mod({qy},p)];
if(!isprime(p)||!isprime(n),error("prime check"));
if(ellcard(E)!=n,error("curve order"));
if(!ellisoncurve(E,G)||!ellisoncurve(E,Q),error("point off curve"));
if(ellmul(E,G,n)!=[0]||ellmul(E,Q,n)!=[0],error("subgroup order"));
print("IDENTITY_OK");
'''
(here/'runs/identity_check.gp').write_text(text)
