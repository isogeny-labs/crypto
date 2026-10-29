#!/usr/bin/env python3
"""Independent check of the Oakley Pratt certificates.

Reads oakley_pratt_certificates.json (the data behind lean/OakleyPrimality.lean)
and re-verifies every certificate with the Python standard library only:
the product of the listed factors is p - 1, a^(p-1) = 1 mod p, and
a^((p-1)/q) != 1 mod p for every listed q. Each listed q is either below the
small-prime bound and prime by trial division, or has its own certificate.
It also checks that the two roots are the integers named in OakleyOrders.lean.
Shares no code with the Lean proof. Exit status 0 means every check passed.
"""

import json
import math
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOTS = {
    "oakley3Prime": 3805993847215893016155463826195386266397436443,
    "oakley4Prime": 12259964326927110866866776214413170562013096250261263279,
}


def trial_prime(n: int) -> bool:
    if n < 2:
        return False
    return all(n % d for d in range(2, math.isqrt(n) + 1))


def main() -> int:
    data = json.loads((HERE / "oakley_pratt_certificates.json").read_text())
    bound = data["small_prime_bound"]
    certs = {int(p): c for p, c in data["certificates"].items()}
    lean = (HERE.parent / "lean" / "OakleyOrders.lean").read_text()
    for name, value in ROOTS.items():
        m = re.search(rf"def {name} : ℕ := (\d+)", lean)
        assert m and int(m.group(1)) == value, f"{name} differs from OakleyOrders.lean"
    proved: set[int] = set()

    def check(p: int) -> None:
        if p in proved:
            return
        if p < bound:
            assert trial_prime(p), f"{p} is not prime"
        else:
            c = certs[p]
            a, factors = c["witness"], c["p_minus_1_factors"]
            assert math.prod(factors) == p - 1, f"factors of {p}-1 do not multiply out"
            assert pow(a, p - 1, p) == 1, f"Fermat condition fails for {p}"
            for q in set(factors):
                assert pow(a, (p - 1) // q, p) != 1, f"witness {a} fails at q={q} for {p}"
                check(q)
        proved.add(p)

    for name, value in ROOTS.items():
        check(value)
        print(f"PASS {name} = {value} ({value.bit_length()} bits) prime by Pratt certificate")
    unused = set(certs) - proved
    assert not unused, f"unused certificates: {sorted(unused)}"
    print(f"PASS {len(certs)} certificates, {len(proved)} primes checked")
    return 0


if __name__ == "__main__":
    sys.exit(main())
