#!/usr/bin/env python3
"""Audit the finite cover census against elliptic trace and descent criteria."""
from pathlib import Path
from collections import Counter
import hashlib
import json

here=Path(__file__).parent
screen_path=here/'SCREEN.json'
three_path=here/'THREE_BRANCH.json'
positions_path=here/'POSITIONS.json'
verify_path=here/'VERIFIED.json'
verify_three_path=here/'VERIFIED_THREE.json'
screen=json.loads(screen_path.read_text())
three=json.loads(three_path.read_text())
positions=json.loads(positions_path.read_text())
verified=json.loads(verify_path.read_text())
verified_three=json.loads(verify_three_path.read_text())
assert screen['equations']==len(screen['records'])==1093
assert three['equations']==len(three['records'])==91
assert verified['checked']==16
assert verified_three['checked']==3
assert positions['PGL2_F3_order']==24
assert positions['quadratic_pair_orbit_size']==3

def elliptic_trace_possible(t):
    return t%3!=0 or t in (0,-27,27)

def descended_fifth_trace(s):
    return s**5-15*s**3+45*s

descended={descended_fifth_trace(s) for s in range(-3,4)}
assert descended=={-31,-27,-2,0,2,27,31}
assert set(r['trace_t'] for r in three['records'])=={-24,-9,24}
assert not any(elliptic_trace_possible(r['trace_t']) for r in three['records'])

split=[r for r in screen['records'] if r['trace_pair'] is not None]
nonsplit=[r for r in screen['records'] if r['trace_pair'] is None]
assert len(split)==801 and len(nonsplit)==292
eligible_components=[(r['case'],r['exponents'],t) for r in split
                     for t in r['trace_pair'] if elliptic_trace_possible(t)]
assert len(eligible_components)>0
assert all(t in descended for _,_,t in eligible_components)
assert {t for _,_,t in eligible_components}=={-31,2,31}
out={'schema':'ghs.cyclic11-screen-audit/v1',
     'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
     'inputs_sha256':{p.name:hashlib.sha256(p.read_bytes()).hexdigest()
                      for p in (screen_path,three_path,positions_path,verify_path,verify_three_path)},
     'genus5_equations':91,'genus10_equations':1093,
     'genus10_split':len(split),'genus10_nonsplit':len(nonsplit),
     'genus5_trace_histogram':dict(sorted(Counter(r['trace_t']
                                                 for r in three['records']).items())),
     'genus10_trace_pair_histogram':screen['trace_pair_histogram'],
     'eligible_component_traces':sorted({t for _,_,t in eligible_components}),
     'descended_fifth_traces':sorted(descended),
     'eligible_non_descended_factors':0}
print(json.dumps(out,sort_keys=True))
