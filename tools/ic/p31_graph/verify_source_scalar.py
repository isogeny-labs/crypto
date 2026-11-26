"""Calibrate the p31 graph kernel and check the public scalar on the source Jacobian.

Run with ``sage -python verify_source_scalar.py DATA_ROOT --output RECEIPT``.
"""
import argparse
import hashlib
import json
import struct
import time
from pathlib import Path

from sage.all import GF, HyperellipticCurve, PolynomialRing, ZZ, gcd

parser = argparse.ArgumentParser()
parser.add_argument("data_root", type=Path)
parser.add_argument("--kernel", type=Path)
parser.add_argument("--output", type=Path)
args = parser.parse_args()
start = time.perf_counter()
root = args.data_root
graph = root / "Picard full graph 25k boundary"
later = root / "Picard graph rank completion 20261123"
cfg_path = root / "Picard native collector/input_2147483647.json"
source_path = root / "Picard checkpoint collector/public_source_instance.json"
kernel_path = args.kernel or later / "graph_kernel_24999.json"
cfg = json.loads(cfg_path.read_bytes())
source = json.loads(source_path.read_bytes())
kernel = json.loads(kernel_path.read_bytes())
setup = json.loads((graph / "setup.json").read_bytes())
p, ell = cfg["p"], cfg["ell"]
assert p == source["p"] == setup["p"] == 2147483647
assert ell == source["ell"] == kernel["ell"] == 11832483282198303559
assert ZZ(ell).is_prime()
assert hashlib.sha256((graph / "fb25k.bin").read_bytes()).hexdigest() == setup["fb25k_sha256"]
input_fields = list(map(int, (graph / "input25k.txt").open().readline().split()))
assert input_fields[5:8] == [ell, cfg["mu"], cfg["zeta"]]
assert input_fields[9:10] == [25000]
assert len(kernel["kernel"]) == 25000 and kernel["kernel"][0] == 1
logs = kernel["kernel"]
zeta, mu = cfg["zeta"], cfg["mu"]
zp = [1, zeta, zeta * zeta % p]

def canonical(point):
    x, y = point
    representative = min(y * z % p for z in zp)
    j = next(j for j in range(3) if representative * zp[j] % p == y)
    return (x, representative), pow(mu, j, ell)

wanted = {canonical(point)[0] for group in cfg["split_input_points"] for point in group}
index = {}
with (graph / "fb25k.bin").open("rb") as factor_base:
    for i in range(25000):
        point = struct.unpack("<II", factor_base.read(8))
        if point in wanted:
            index[point] = i
assert set(index) == wanted

def score(points):
    return sum(coefficient * logs[index[point]] for point, coefficient in map(canonical, points)) % ell

base_score = score(cfg["split_input_points"][0])
target_score = score(cfg["split_input_points"][1])
result = {
    "schema": "p31-graph-source-scalar-check-v1", "p": p, "ell": ell,
    "base_score": base_score, "target_score": target_score,
    "kernel_sha256": hashlib.sha256(kernel_path.read_bytes()).hexdigest(),
    "factor_base_sha256": setup["fb25k_sha256"],
    "transfer_config_sha256": hashlib.sha256(cfg_path.read_bytes()).hexdigest(),
    "source_instance_sha256": hashlib.sha256(source_path.read_bytes()).hexdigest(),
}
assert base_score, "graph kernel has zero source-base score"
candidate = cfg["alpha"] * target_score * pow(cfg["beta"] * base_score % ell, -1, ell) % ell
F = GF(p)
R = PolynomialRing(F, "x")
f = R(source["source_polynomial"])
assert f.degree() == 9 and gcd(f, f.derivative()) == 1
J = HyperellipticCurve(f).jacobian()(F)
O = J(0)
B = J([R(c) for c in source["base"]])
Q = J([R(c) for c in source["target"]])
assert B != O and Q != O and ell * B == O and ell * Q == O
answer = next((v for v in (candidate, (-candidate) % ell) if v * B == Q), None)
result.update(candidate=candidate, recovered_source_scalar=answer,
              verified=answer is not None, seconds=time.perf_counter() - start)
if args.output:
    args.output.write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result))
assert answer is not None
