# Staged resultant witness diagnostic

Date: 2026-11-25. Protocol: `PROTOCOL.md` and
`AMENDMENT_RESULTANT_20261125.md`. Solver source:
`fcb65e6be0b6f1c92a89e10c7a87b3ef97fe4269`,
`resultant_witness.py`. These local diagnostics are outside the frozen
independent-curve, cold-process, 2-GiB qualifying panel. Other jobs shared
the host, so wall and process CPU are both reported without treating their
ratio as a solver speedup.

## Construction and direct checks

The map is the `p-1` Kummer coset `x^B=g^B`, `B=2^t`, `g=1`. The witness
solver uses the `m=3` Semaev chain `S3(x1,x2,u), S3(u,x3,x(R))`. It computes
three staged resultants, factors only the successive gcds, lifts point
signs, and checks `P1+P2+P3=R`. The intermediate identity-point case is
handled explicitly.

The selected high-degree map is the composition of `t` degree-2 maps
`z -> z^2`, ending in `1-z` (or the corresponding coset terminal). That is
the PKM paper's `p-1` construction. This diagnostic solver substitutes the
composition into the single high-degree equation before elimination; it
does **not** measure the paper's proposed advantage from retaining the
degree-2 auxiliary equations in a low-degree polynomial system.

For `E/F_97: y^2=x^3+2x+3` at `t=3`, Sage independently enumerated the nine
base points with `x^8=1`:

| Target | Solver status | Solver ordered witnesses | Sage ordered witnesses | Exceptional branch points |
| --- | --- | ---: | ---: | ---: |
| `(92,16)` | complete | 18 | 18 | 0 |
| `(1,43)` | complete | 33 | 33 | 9 |

The latter target tests the missing `P1+P2=O, P3=R` branch. The Sage
control used its independent elliptic-curve group law and enumerated every
ordered triple in the nine-point base. These count agreements do not yet
verify the exact set of triples on independent curves.

## Measured stage and censoring

On target `(92,16)`, the `t=3` diagnostic completed with 3 resultants,
31 gcds, 31 factorizations, 16 candidate coordinate triples, and 18
verified signed point triples. It took 5.070 s wall, 3.889 s process CPU,
and 65.45 MiB peak process RSS. The first resultant `H` had degree 16 and
16 terms. The expanded `J` had bidegree `(32,32)` and 1,069 terms; the
final `K` had degree 250 and 242 terms; the first gcd had degree 4.

At `t=4` on the same curve and target, the source-pinned diagnostic reached
the 60 s wall cap during the second resultant, after `H` completed. It
spent 60.010 s wall and 26.501 s process CPU, with 71.56 MiB peak process
RSS. `H` had degree 32 and 32 terms. One resultant completed and two were
attempted; the witness set is unresolved. This row is censored, not a
zero-yield query. The timer and CPU counters start after Python import,
input validation and primitive-root selection, and no external 2-GiB cap
was imposed.

Commands:

```sh
python3 research/pkm_nonenum_decomp_20261125/resultant_witness.py --height 3 --max-seconds 60
python3 research/pkm_nonenum_decomp_20261125/resultant_witness.py --height 4 --max-seconds 60
sage -c 'F=GF(97); E=EllipticCurve(F,[2,3]); R=E(F(92),F(16)); V=[x for x in F if x^8==1]; P=[q for x in V for q in E.lift_x(x,all=True)]; print(len(V),len(P),sum(1 for a in P for b in P for c in P if a+b+c==R))'
```

## Falsifier and next method gate

The expanded `J` has generic bidegree proportional to `B` in each of two
variables and therefore quadratically many coefficients; the `t=3` row is
nearly dense (1,069 of 1,089 possible monomials). This representation has
an output-size floor of order `B^2` on such instances before witness
extraction. The `t=4` cap is the protocol's resource-censoring falsifier for
this implementation. It gives no basis to extrapolate this backend to s59,
P-192, or a NIST field. The same argument does not rule out a lazy circuit,
quotient-algebra, or other solver that avoids materializing `J`; any such
solver needs its own frozen cost model and witness checks.

For a conditional dense-output budget, `(4B+1)^2` coefficients at `t=16`
would need about 512 GiB of raw 59-bit coefficient payload on the s59-minus
`p-1` Kummer map, before polynomial metadata and arithmetic workspace.
At `t=76` on P-224's available `p-1` map, the corresponding raw payload is
about `2^160.8` bytes using 28 bytes per field element. These are
representation-size extrapolations, not measurements or lower bounds for
all compact solvers. The current implementation is rejected by its `t=4`
cap regardless of those wide extrapolations. The s59-plus, P-192, P-256,
and P-521 favorable sides use a separate `p+1` norm-one/Dickson map;
this solver has no corresponding cost row for them. P-384's available
`p+1` height fails the independent relation-density gate in
`WIDE_M3_GATE.md`.

The independent-curve/full-rank/target-descent/cold same-target rho panel
remains open for a replacement solver. The `p+1` norm-one/Dickson map is a
separate construction and requires a separate witness implementation.

Sources checked (KB): `source:13c233fff93bab742d135ed6c12bb7b6`
(`crypto:research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md`) for
the previously measured tower systems, and `source:362976dae6e255e22be8c13a8c7a2e2f`
(`crypto-autoresearcher:knowledge/literature/KN-LIT-10b0fa.md`) for the
primary-paper construction review. These are pointers to prior analysis;
the algebraic size claim above comes from this implementation's polynomial
degrees and the reported term count.
Primary source: Petit, Kosters and Messeng, *Algebraic approaches for the
Elliptic Curve Discrete Logarithm Problem over prime fields*, PKC 2016,
§3.1 and §3.3, https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf.
