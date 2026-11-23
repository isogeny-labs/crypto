# Nearby CRT coefficient pairs across all 22 local bridges

For each of the 22 locally bridged source pairs, write its first CRT triple as \((b_1,b_2,b_3)\) and set

\[
k_1=b_1+77u,\qquad k_2=b_2+77v,\qquad k_3=b_3+77t,
\quad u,v\in\{-2,-1,0,1,2\},\quad t\in\mathbb Z.
\]

All **550** frozen \((\text{pair},u,v)\) cells have an exact degree-24 necessary numerator of \(x(6Q(t))-x(P)\) with no root at a recorded prime from 13 through 199. Thus every integer third-coefficient shift in these fixed first-two-coefficient classes excludes \(P+6Q=\mathcal O\) on a nonsingular rational specialization. All 550 centered curves retain the order-four relation at 7 and order-eight relation at 11; the centered rational relation flag is false in every cell.

| Stage | Exact result |
| --- | ---: |
| Source pairs / \((u,v)\) cells per pair | 22 / 25 |
| All-integer \(t\) certificates | 550 |
| Generic numerator / denominator degrees | 24 / 22 in every cell |
| Unresolved, generic-identity, or producer-failure cells | 0 |
| Independent symbolic numerator and denominator replays | 550 |
| Lean rootlessness / no-integer-root declarations | 550 / 550 |

The [protocol](protocol.json) fixed the source digests, window, unbounded third shift, prime cap 997, local controls, and failure statuses as `4ba1e737801`. Checked Sage finished the [run](result.json) under its 600-second cap, retaining all [550 rows](rows/), event records, runtime receipt, and empty stderr. The prior 25-cell pair-31 packet is an exact overlap, as are the 22 central rows of the immediately preceding shift packet.

The [independent audit](audit.py) used a separately implemented SymPy point-addition chain in four local processes. It rederived every complete numerator and denominator coefficient list, checked each first-rootless-prime certificate, and verified all 47 overlap rows. Its [result](audit-result.json) is `PASS`; the [progress record](audit-events.jsonl) retains all 550 verified cells, and stderr is empty.

The pinned [Lean module](formal/PairGrid.lean) proves `rootless000` through `rootless549` for the exact stored integer polynomials, followed by `no_integer_root000` through `no_integer_root549` for every integer \(t\). Its generator was frozen before the build; the audit binds all 550 Lean coefficient lists to the stored rows. The actual [build output](formal/build-output.txt) contains 1,100 successful `#print axioms` lines, showing only `propext`, `Classical.choice`, and `Quot.sound`, with empty stderr.

| Result | Lean declaration or exact receipt | Status |
| --- | --- | --- |
| Each stored numerator is rootless at its recorded prime | `XedniBridge6PairGrid.rootless000`–`rootless549` | Lean checked |
| Each stored numerator has no integer root | `no_integer_root000`–`no_integer_root549` | Lean checked |
| 550 curve-to-polynomial identities | Checked-Sage rows and independent SymPy audit | Independently replayed |
| Specialization from the group relation to a polynomial root | Written x-coordinate implication | Elliptic-section binding remains open in Lean |

The source set and two first-coefficient shifts are bounded by this packet. The next search can widen those two shifts, test different first CRT representatives, or derive a structural criterion for when any local coefficient-six bridge can lift to a rational relation. Each direction needs a direct rational replay and its selection cost recorded.
