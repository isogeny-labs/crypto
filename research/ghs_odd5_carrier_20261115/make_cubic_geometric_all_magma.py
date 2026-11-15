#!/usr/bin/env python3
"""Freeze the full Magma row list from the immutable degree-11 screen."""
import json
from pathlib import Path

P=Path(__file__).resolve().parent
screen=json.loads((P/'screen.json').read_text())
traces={-26,-22,-14,-10,10,14,22,26}
rows=[(r[6],r[0]) for r in screen['rows'] if r[6] in traces]
assert len(rows)==84
source=(P/'cubic_geometric_replay.magma').read_text()
old='rows := [<-26,28967>, <26,35518>];'
assert source.count(old)==1
literal='rows := ['+', '.join('<%d,%d>'%(t,m) for t,m in rows)+'];'
source=source.replace(old,literal)
target=P/'cubic_geometric_all.magma'
if target.exists():
    raise SystemExit('refusing to overwrite existing Magma source')
target.write_text(source)
print(target)
