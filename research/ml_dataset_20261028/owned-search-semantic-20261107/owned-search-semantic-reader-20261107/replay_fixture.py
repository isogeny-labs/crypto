#!/usr/bin/env python3
"""Reinspect the prior software-only genuine Rust serializer fixture."""
from pathlib import Path
import argparse,json
from semantic_reader import inspect_selected,structural_summary
p=argparse.ArgumentParser();p.add_argument('fixture',type=Path);a=p.parse_args()
with (a.fixture/'snapshot.bin').open('rb') as f:whole=inspect_selected([f])
paths=sorted(a.fixture.glob('chunk-*.bin'))
def streams():
    for path in paths:
        with path.open('rb') as f:yield f
iterator=streams()
try:chunked=inspect_selected(iterator)
finally:iterator.close()
assert whole==chunked
native=whole['retained']['continuation']['partitions'][0]['search']['nodes'][0]['native']
print(json.dumps({'fixture':str(a.fixture),'chunk_count':len(paths),'whole_and_chunked_equal':True,'inspection':structural_summary(whole),'native_basis_rows':len(native['basis']),'native_batches':len(native['batches']),'fixture_scope':'Actual Rust serializer over deliberately synthetic retained objects; neither authenticated original inputs nor valid row proof is asserted.'},indent=2))
