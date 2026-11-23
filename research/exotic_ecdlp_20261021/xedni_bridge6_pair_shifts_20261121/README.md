# All-integer third-coefficient shifts for 22 local bridges

For each of the 22 first CRT bridges from the frozen source-pair sweep, hold its exact point pair and CRT values \(k_1,k_2\) fixed, and set \(k_3(t)=k_3(0)+77t\) for every integer \(t\). All 22 primitive degree-24 numerators of \(x(6Q(t))-x(P)\) have a rootless residue polynomial at a recorded prime between 13 and 97. Consequently, every nonsingular rational specialization in these 22 fixed-\(k_1,k_2\) families excludes the coefficient-six relation \(P+6Q=\mathcal O\). The degree-22 denominators and all failed modular candidates preceding each certificate are retained.

| Frozen stage | Exact result |
| --- | ---: |
| First CRT local bridges tested | 22 |
| Generic numerator / denominator degrees | 24 / 22 in every cell |
| Rootless-prime certificates | 22 |
| Unresolved or producer-failure cells | 0 |
| Independent symbolic replays | 22 |
| Lean concrete rootlessness / no-integer-root declarations | 22 / 22 |

The [protocol](protocol.json) fixed the 22 source rows, coefficient law, prime cap 2003, statuses, and process cap before measurement as `6ad7fd23aa2`. The checked [Sage runtime receipt](sage-runtime-info.json), event log, every [row](rows/), aggregate [result](result.json), and empty run stderr are retained. The first certificates are at 13 (five cells), 17 (five), 19 (one), 23 (two), 29 (two), 31 (two), 37 (two), 53 (one), 71 (one), and 97 (one). No wider prime search was required.

The [independent SymPy audit](audit.py) reconstructs the Weierstrass coefficients and performs its own point doublings and additions for every pair. It matches all 22 complete numerator and denominator coefficient lists against the checked-Sage rows, verifies every earlier prime has a root and the recorded prime has none, and binds the exact numerator lists to Lean. Its [result](audit-result.json) is `PASS`; audit stderr is empty.

The pinned [Lean module](formal/PairShifts.lean) proves `rootless000` through `rootless021` for the exact stored integer polynomials, then `no_integer_root000` through `no_integer_root021` for every integer \(t\). The generator and independent audit bind its 22 lists to the recorded rows. The actual [build output](formal/build-output.txt) contains 44 successful `#print axioms` lines, with only standard Lean foundations and no `sorryAx`. `lean4checker --fresh` is unavailable on this host.

| Result | Lean declaration or receipt | Status |
| --- | --- | --- |
| Each exact numerator is rootless at its recorded prime | `XedniBridge6PairShifts.rootless000`–`rootless021` | Lean checked |
| Each exact numerator has no integer root | `no_integer_root000`–`no_integer_root021` | Lean checked |
| Curve-to-polynomial identities for 22 rational-function families | Sage rows and independent SymPy audit | Independently replayed |
| Specialization from \(P+6Q=\mathcal O\) to a root of the saved numerator | Written x-coordinate implication and generic rational-function construction | Elliptic-section binding remains open in Lean |

This result covers one CRT \(k_1,k_2\) choice per locally bridged source pair. The next test should vary those two coefficients across their full congruence classes, or construct a different source-pair and coefficient selection rule; either route must retain its search cost and validate any rational relation directly.
