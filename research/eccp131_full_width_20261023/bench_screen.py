#!/usr/bin/env python3
"""Paired exact-curve 115-column relation-screen stage measurement."""
import argparse
import hashlib
import json
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(HERE.parent/'eccp131_structured_ic_20261022'))
import screen  # noqa: E402


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--rep', type=int, required=True, choices=range(3))
    ap.add_argument('--arm', required=True, choices=('tower','small_a','small_b'))
    args = ap.parse_args()
    replay_path = HERE/'runs/replay.json'
    replay = json.loads(replay_path.read_text())
    vals = screen.parameters()
    assert {k:hex(v) for k,v in vals.items()} == replay['parameters']
    p,a,n = vals['p'],vals['a'],vals['n']
    for P in ((vals['gx'],vals['gy']),(vals['qx'],vals['qy'])):
        assert screen.mul(n,P,p,a) is None
    base_start = time.perf_counter()
    if args.arm == 'tower':
        base = [P for x in replay['tower']['x'] if (P := screen.lift(int(x,16),vals)) is not None]
        assert [[hex(x),hex(y)] for x,y in base] == replay['screen']['base']
    else:
        base = screen.small_base(115,vals)
    base_wall = time.perf_counter()-base_start
    assert len(base) == 115

    scalar_draws = []
    original_random = screen.random.Random
    class TrackedRandom(original_random):
        def randrange(self,*r):
            value = super().randrange(*r)
            if r == (1,n):
                scalar_draws.append(value)
            return value
    original_add = screen.add
    add_calls = [0]
    def counted_add(*r):
        add_calls[0] += 1
        return original_add(*r)
    screen.random.Random = TrackedRandom
    screen.add = counted_add
    try:
        screen_start = time.perf_counter()
        result = screen.screen_base(base,vals,131,256,256)
        screen_wall = time.perf_counter()-screen_start
    finally:
        screen.random.Random = original_random
        screen.add = original_add
    assert len(scalar_draws) == 511 and len(result['planted_controls']) == 8
    out = {'schema':'eccp131-equal-column-screen-bench/v1','rep':args.rep,'arm':args.arm,
           'source_sha256':{'replay.json':sha(replay_path),'screen.py':sha(HERE.parent/'eccp131_structured_ic_20261022/screen.py'),
                            'curve.rs':sha(ROOT/'src/ecc/curve.rs'),'params.rs':sha(ROOT/'src/bin/ic/params.rs')},
           'columns':len(base),'seed':131,'setup_candidates':256,'target_candidates':256,
           'candidate_scalar_sha256':hashlib.sha256(json.dumps(scalar_draws,separators=(',',':')).encode()).hexdigest(),
           'base_prepare_seconds':base_wall,'screen_seconds':screen_wall,'ec_add_calls':add_calls[0],
           'result':result}
    print(json.dumps(out,separators=(',',':')))


if __name__=='__main__':
    main()
