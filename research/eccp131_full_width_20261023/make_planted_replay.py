#!/usr/bin/env python3
"""Emit a separate PARI/GP point-equation replay for planted controls."""
import json
from pathlib import Path

here = Path(__file__).parent
v = json.loads((here/'runs/replay.json').read_text())
p = int(v['parameters']['p'],16)
a = int(v['parameters']['a'],16)
b = int(v['parameters']['b'],16)
base = [(int(x,16),int(y,16)) for x,y in v['screen']['base']]
controls = v['screen']['planted_controls']
assert len(controls) == 8
lines = ['\\ Independent PARI/GP replay from published-curve coordinates',
         f'p={p};a={a};b={b};E=ellinit([a,b],p);']
for i,c in enumerate(controls):
    lines.append('T=[0];')
    for j,s in c['terms']:
        x,y = base[j]
        lines.append(f'T=elladd(E,T,ellmul(E,[Mod({x},p),Mod({y},p)],{s}));')
    x,y = (int(z,16) for z in c['point'])
    lines.append(f'if(T!=[Mod({x},p),Mod({y},p)],error("planted {i} mismatch"));')
lines.append('print("PLANTED_REPLAY_OK 8");')
(here/'runs/planted_replay.gp').write_text('\n'.join(lines)+'\n')
