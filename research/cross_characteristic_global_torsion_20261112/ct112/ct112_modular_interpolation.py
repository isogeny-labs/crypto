#!/usr/bin/env python3
"""CT112 independent modular source-reconstruction replay, not the global proof."""
import json
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent.parent / "ct109"))
import ct109_modular_probe as m

def ck(ok, msg):
    if not ok:
        raise ArithmeticError(msg)

def interpolate(F, basis, target):
    p, n, d = F.p, len(basis), 60
    rows = [[basis[j][i] if i < len(basis[j]) else 0 for j in range(n)]
            + [target[i] if i < len(target) else 0] for i in range(d)]
    for j in range(n):
        i = next((i for i in range(j, d) if rows[i][j]), None)
        ck(i is not None, "singular power basis")
        rows[j], rows[i] = rows[i], rows[j]
        inv = pow(rows[j][j], -1, p)
        rows[j] = [v * inv % p for v in rows[j]]
        for k in range(d):
            if k != j and rows[k][j]:
                x = rows[k][j]
                rows[k] = [(a - x * b) % p for a, b in zip(rows[k], rows[j])]
    ck(all(not any(r[:n]) and r[-1] % p == 0 for r in rows[n:]),
       "not in line subfield")
    return [r[-1] % p for r in rows[:n]]

def run(p, fixture):
    F = m.PolyMod(p)
    xs = [F.prod(F.rem(a), F.inv(b)) for a, b in zip(m.nums, m.dens)]
    h = [[1]]
    for x in xs:
        nxt = [[0] for _ in range(len(h) + 1)]
        for j, z in enumerate(h):
            nxt[j] = F.minus(nxt[j], F.prod(z, x))
            nxt[j + 1] = F.plus(nxt[j + 1], z)
        h = nxt
    s = F.prod([-1], h[4])
    ck(s == F.invariant(), "trace mismatch")
    basis = [[1]]
    for _ in range(11):
        basis.append(F.prod(basis[-1], s))
    C = [[int(v) for v in row] for row in fixture["numerators_low_to_high"]]
    L = int(fixture["common_denominator"])
    ck(L % p != 0, "nonunit denominator")
    invL = pow(L, -1, p)
    for j in range(4):
        coeff = interpolate(F, basis, h[j])
        ck(coeff == [v * invL % p for v in C[j]],
           "interpolation mismatch j=%d p=%d" % (j, p))
    if p == 59:
        for j, val in enumerate([27, 19, 45, 31]):
            evaluated = sum(v * pow(32, i, p) for i, v in enumerate(C[j]))
            ck(evaluated * invL % p == val, "source subgroup coefficient")
    return p

if __name__ == "__main__":
    fixture = json.loads((Path(__file__).parent / "reconstructed.json").read_text())
    if sys.argv[1:] == ["--full"]:
        primes = [n for n in range(13, 1104)
                  if all(n % d for d in range(2, int(n ** .5) + 1))]
        ck(len(primes) == 180, "unexpected full prime count")
    else:
        primes = [int(a) for a in sys.argv[1:]] or [13, 17, 59, 1151, 1201]
    good, excluded = [], []
    for p in primes:
        try:
            good.append(run(p, fixture))
        except ArithmeticError as e:
            if p == 827 and str(e) in ("singular power basis", "nonunit denominator"):
                excluded.append(p)
            else:
                raise
    print(json.dumps({
        "validated_primes": good, "excluded_bad_prime": excluded,
        "global_certificate_in_separate_verifier": True
    }, sort_keys=True, separators=(",", ":")))
