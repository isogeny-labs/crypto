"""Independent integer replay of compact p31 PP edges from the 25k base."""
import hashlib
import json
import math
import struct
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
EDGE = HERE / "edges25k_fullspace.bin"
INPUT = HERE / "input25k.txt"
FB = HERE / "fb25k.bin"
SOURCE = HERE.parent / "Picard full graph pilot" / "audit_edges.py"
fields = list(map(int, INPUT.open().readline().split()))
p, tw, a, b, c, ell, mu, zeta, seed, B, tree_cap, pair_cap, fixed, count = fields
assert p == 2147483647 and B == count == 25000 and fixed == 1
assert pair_cap == 3*B*(B-1)//2
fb = list(struct.iter_unpack("<II", FB.read_bytes()))
assert len(fb) == B
fb_x = {x for x, y in fb}
assert len(fb_x) == B
cutoff = 4*p//5
helper = "MASK =" + SOURCE.read_text().split("MASK =", 1)[1].split("positions=sorted", 1)[0]
scope = dict(math=math, p=p, tw=tw, a=a, b=b, c=c, ell=ell, mu=mu, zeta=zeta,
             seed=seed, B=B, fb=fb, fb_x=fb_x)
exec(helper, scope)
assert EDGE.stat().st_size % 12 == 0
records = EDGE.stat().st_size // 12
emitter = json.loads((HERE / "emitter25k.json").read_text())
assert emitter["pairs"] == pair_cap == 937462500 and emitter["selected_pp"] == records
start = time.perf_counter()
positions = sorted({0, records-1, *[(records-1)*i//1023 for i in range(1024)]})
with EDGE.open("rb") as stream:
    for position in positions:
        stream.seek(position*12)
        ordinal, x0, x1 = struct.unpack("<III", stream.read(12))
        assert 0 <= ordinal < pair_cap and x0 < cutoff and x1 < cutoff
        assert scope["replay"](ordinal) == (x0, x1)
digest = hashlib.sha256()
with EDGE.open("rb") as stream:
    while block := stream.read(1 << 24):
        digest.update(block)
result = {
    "status": "independent_25k_line_replay_passed",
    "p": p, "ell": ell, "B": B,
    "pair_horizon": pair_cap,
    "edge_records": records,
    "edge_sha256": digest.hexdigest(),
    "input_sha256": hashlib.sha256(INPUT.read_bytes()).hexdigest(),
    "fb_sha256": hashlib.sha256(FB.read_bytes()).hexdigest(),
    "sampled_records_replayed": len(positions),
    "sampled_curve_point_identities_checked": True,
    "all_ordinals_and_cutoffs_checked_by_direct_filter": False,
    "elapsed_internal_seconds": time.perf_counter()-start,
    "weighted_relation_rank_tested": False,
    "original_curve_scalar_tested": False,
}
(HERE / "audit25k.json").write_text(json.dumps(result, indent=2)+"\n")
print(json.dumps(result))
