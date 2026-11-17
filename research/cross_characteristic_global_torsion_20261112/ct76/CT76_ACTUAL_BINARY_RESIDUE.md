# CT76 — Exact binary Frobenius gate for the actual CT75 curve [pcc:unverified]

**Scope:** Fix the unchanged global CT75 model E/Q: y²+y=x³−x²−10x−20. Its good reduction at 2 is E₂: y²+y=x³+x², with exactly 5 F₂-rational points. The Frobenius trace is −2, determinant 2 and characteristic polynomial X²+2X+2. This is classical, not a novelty claim.

For every odd prime ell, the characteristic polynomial has discriminant −4≠0 mod ell, and the geometric ell-torsion Frobenius is conjugate to the companion matrix

    M = [[0, −2], [1, −2]] (mod ell).

The smallest degree of a binary extension containing **any** nonzero rational ell-torsion is f_some=min{f≥1 : det(M^f−I)=0 mod ell}. All geometric ell-torsion becomes rational at f_full=ord(M) in GL₂(F_ell). The identity det(M^f−I) ≡ #E₂(F_(2^f)) mod ell gives a second exact consistency gate.

## Results for all CT75 source torsion orders

| ell | Actual 11a1 binary f_some | Actual 11a1 f_full | CT55 alternative binary model f_some | CT55 f_full |
|---:|---:|---:|---:|---:|
| 3 | 8 | 8 | 1 | 2 |
| 5 | 1 | 4 | 8 | 8 |
| 7 | 24 | 24 | 12 | 12 |
| 11 | 40 | 40 | 5 | 10 |
| 13 | 6 | 12 | 24 | 24 |

**Precise consequence:** Keeping this exact global model, an order-7 point cannot be rational over its binary reduction before F_(2^24), and order-11 requires at least F_(2^40). For the order-5 subgroup some rational points already occur over F₂ but **full** five-torsion takes extension degree four. Neither bound applies to **other** elliptic targets, changed CRT models, higher-dimensional targets, or adaptive families.

## Reproducible independently authored checks

Python searches irreducible binary polynomials by demanding inverses for every nonzero residue and enumerates *all points* of E₂ over F_(2^f), f=1,…,8. Exact counts: 5,5,5,25,25,65,145,225, including infinity. JavaScript independently calculates these counts from the exact Frobenius trace recurrence, and separately multiplies the matrix modulo ell, checking det(F^f−I) against group orders for every tested f. Python independently performs that Frobenius matrix gate. Every smaller f is rejected before recording f_some.

The Python normal/−O and Node normal/−−no-opt programs produced identical 13-line canonical JSONL, SHA256 921ad2167c532288f3ee2b1bad2c42fc8d9073aac1be875f649ed765f91fb006. These are two **execution checks**, not signed PCC science certification.

Reproduce from repo root:

    D=research/cross_characteristic_global_torsion_20261112/ct76
    python3 -B "$D/ct76_binary_field.py" >/tmp/ct76-py.jsonl
    python3 -O -B "$D/ct76_binary_field.py" >/tmp/ct76-pyopt.jsonl
    node "$D/ct76_binary_independent.js" >/tmp/ct76-js.jsonl
    node --no-opt "$D/ct76_binary_independent.js" >/tmp/ct76-jsopt.jsonl
    cmp /tmp/ct76-py.jsonl /tmp/ct76-pyopt.jsonl
    cmp /tmp/ct76-py.jsonl /tmp/ct76-js.jsonl
    cmp /tmp/ct76-py.jsonl /tmp/ct76-jsopt.jsonl
    echo '921ad2167c532288f3ee2b1bad2c42fc8d9073aac1be875f649ed765f91fb006  /tmp/ct76-py.jsonl' | sha256sum -c

## Prior art, concurrent work, and limits

- CT55, research/cross_characteristic_global_torsion_20261112/ct55/CT55_UNIVERSAL_DIVISION_FIELD.md, already implements the same classical Frobenius technique for its **different** CRT-lifted curve reducing to y²+y=x³. It does not give the actual binary extension degrees for the unchanged CT74–75 rational model. The side-by-side table is a model-dependence warning, not an algorithmic speedup.
- CT75 documents independently checked logarithm-free **source-local** odd-prime torsion lifting, not the global point representation, a compatible target residue, or a complete solver.
- Primary references: Stanford elliptic curve notes https://crypto.stanford.edu/pbc/notes/elliptic/weier.html ; Snowden's elliptic curve local torsion lecture https://websites.umich.edu/asnowden/teaching/2013/679/L08.html . Root PR #3877 and stack PRs #4012/#4015 were reviewed. Cryptanalysis PR #1523 concerns distinct genus-three work.
- Shared research-KB MCP, its checkout wrapper and Conductor are unavailable in this tool runtime; repository search plus primary sources used as incomplete fallback. Independent PCC remains [pcc:unverified].

**Unfinished:** an actual input-uniform global recognition of *both* encoded source points, one compatible prime over 2, target point evaluation, and full cold solving-cost comparison against matched rho/index calculus. This result supplies a necessary field-of-definition gate for only one fixed global model. No cost advantage or paid compute claim.


## Split-versus-inert structural refinement (same binary model)

The fixed Frobenius polynomial X²+2X+2 has discriminant −4. Therefore, for **every odd prime ell ≡ 3 mod 4**, this polynomial is *irreducible* mod ell and its eigenvalues α and α^ell lie in F_(ell²)\\F_ell. If a nonzero ell-torsion point is F_(2^f)-rational, then α^f=1; its conjugate is also 1 and the semisimple Frobenius satisfies F^f=I. Thus **f_some(ell)=f_full(ell) for all ell≡3 mod4**, not only the three table entries (3,7,11).

For **ell ≡ 1 mod 4**, choose i²=−1 mod ell. Frobenius splits with eigenvalues −1+i and −1−i, so

    f_some = min(ord(−1+i), ord(−1−i)),
    f_full = lcm(ord(−1+i), ord(−1−i)).

For ell=5 these orders are 1 and 4; for ell=13 they are 6 and 12. Both independently authored auditors now check the split-order identities and the inert equality for the listed fixture primes while retaining the exact same 13-line output digest. This is classical finite-field splitting theory specialized to the fixed curve; it is not an obstruction to a different binary model. A faithful Lean statement has not been compiled because no Lean toolchain is present in the runtime.
