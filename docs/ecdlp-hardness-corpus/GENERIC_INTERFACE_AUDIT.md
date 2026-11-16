# Concrete generic-interface audit

Date: 2026-11-01

Status: initial complete pass over the oracle classification; internal argument, not an externally reviewed theorem

## Question

Which repository methods called “generic decomposition” are actually covered by
Shoup's generic-group lower bound or the Corrigan-Gibbs--Kogan (CGK)
preprocessing bounds, and which use a coordinate predicate or an augmented oracle
that escapes those interfaces?

This audit separates three questions that are easy to conflate:

1. **Relation mechanism:** can the same returned relation be obtained from group
   operations, equality, public scalar multiplication, and a table of prior generic
   encodings?
2. **Setup mechanism:** can the factor base and all tables be generated through that
   same interface, or are points selected by coordinates, subspaces, polynomial
   equations, or a cheap endomorphism?
3. **Cost preservation:** does the simulation preserve the charged cost? A method can
   be generically simulable while losing a claimed speedup because a cheap coordinate
   map becomes many generic group operations.

The answer after the first pass is:

- Ordinary subtract, MITM, k-sum, and pair-enumeration oracles are standard generic
  table algorithms once their base points are supplied and all setup is charged.
- `rootfind-subtract`, `semaev_s3_roots`, and the GPU root finder return the same
  relations as group subtraction. Their online *relation mechanism* is generic, but
  their coordinate arithmetic changes implementation constants and their
  smallest-abscissa factor-base setup is representation-specific.
- Koblitz/GLV Frobenius-orbit methods require an augmented interface with a cheap
  known-scalar automorphism. They are not same-cost instances of Shoup's standard
  oracle merely because the map equals scalar multiplication on the subgroup.
- F4/F5/SAT, Weil-descent, PKM-tower, and small-root lattice oracles are genuinely
  coordinate-algebraic. Shoup and CGK do not exclude them.

## Interfaces used in this audit

### G0: standard generic group

The algorithm receives opaque random encodings and may request group composition,
inversion/negation, equality, and known scalar multiplication (implemented through
group operations). Its computation may store encodings and use ordinary data
structures keyed only by exact encoding/equality. It cannot inspect `x`, `y`, field
bits, trace, Hamming weight, polynomial membership, or a curve equation.

This is the interface to which the Shoup lower bound applies. The theorem bounds
the number of generic oracle queries; it does not price hashing, RAM traffic, field
instructions inside a concrete addition, or parallel hardware.

### G1: generic group with charged target-independent advice

G1 is G0 plus a target-independent setup/advice object whose size or construction
is charged exactly as required by the selected CGK theorem. Calling an object
“precomputed” is not enough: its bit length `S`, preprocessing work `P`, online
queries `T`, target distribution, and success `epsilon` must match the theorem.

### GA: known-scalar automorphism augmentation

GA adds an oracle `phi(P)` with a public scalar action `phi(P)=[lambda]P` on the
target subgroup. If `phi` is free or one native field map, GA is stronger than G0:
G0 can reproduce the point with scalar multiplication, but generally at
`Theta(log r)` group operations. A standard-G0 lower bound remains informative only
after this cost difference is handled. Replacing group size `r` by an orbit count is
not justified without an explicit GA lower-bound reduction and stabilizer analysis.

### C: coordinate-algebraic interface

C exposes the concrete representation: coordinate bits, finite-field arithmetic,
membership in algebraic sets, polynomial construction/solving, Weil descent,
pairings, or coordinate-generated points. Standard generic-group lower bounds do not
cover C.

## Verdict legend

- **G0:** directly a generic online and setup mechanism, subject to ordinary cost
  accounting.
- **G1:** generic only after the setup/advice object is included in the CGK units.
- **GA:** needs a cheap-automorphism augmented model for a same-cost claim.
- **C:** genuinely coordinate-algebraic.
- **Mixed:** different phases belong to different interfaces.

## Method matrix

| Method / oracle | Setup | Online decision | Exact equality check? | Verdict | Escaping predicate or remaining obligation |
|---|---|---|---:|---|---|
| `subtract`, m=2 | supplied base; no sum table | enumerate `P in F`, compute `R-P`, membership by point key | yes | G0 online; G1 if base is advice | Base construction may still be coordinate-selected |
| `mitm`, m=2/3 | table of charged pair sums | probe `R`, or enumerate `R-P` and probe | yes | G0/G1 | Count table construction, entries, lookup model, and memory |
| `ksum`, m=3/4/5 | table of every signed k-sum | one exact point-key probe | yes | G0/G1 | Exponential setup is advice/preprocessing, never free |
| `mitm-prime-batch` / `blocked` | same pair sums, built using batched coordinate arithmetic | identical MITM probe | yes | Mixed: G0 relation set, concrete implementation | Same relations are G0; native batch inversion changes the implementation cost, not query count |
| `rootfind-subtract`, m=2 | coordinate reps and base index | computes `x(R +/- F_i)` by batched inversion, then base-key probe | yes, with group re-add | Mixed: G0 relation set, C implementation/setup | Online candidates are exactly `R-F`; smallest-abscissa base and batched field arithmetic are representation-specific |
| `rootfind-subtract`, m=3 | above plus pair table | same candidates, pair-table probe | yes, with group re-add | Mixed: G0/G1 relation set, C implementation/setup | Same caveat plus charged pair-table advice |
| `semaev_s3_roots`, m=2 | coordinate base index | solve each specialized S3 quadratic and test roots | yes | Mixed: G0 relation set, C implementation | The roots are exactly group additions; the square-root route does not create a new algebraic search space |
| GPU root finder, m=2 | device copy of coordinate base/table | batched slopes and exact table probes | yes | Mixed | Parallel hardware and field arithmetic improve constants only; same generic relation search |
| `rootfind-mitm`, m=2/3 | ordinary pair table | probe target or enumerate `R +/- P_k` | yes | G0/G1 | Name “rootfind” does not change the group-law mechanism |
| `rootfind-pairs`, m=3 | no pair table | enumerate `R +/- P_i +/- P_j`, base membership | yes | G0 | Quadratic enumeration floor; no polynomial system is solved |
| binary PDP `engine=rootfind*` | binary pair table, sometimes negation-folded/windowed | group additions and exact table probe | yes | G0/G1 | Factor-base construction can be C or GA even though the pair search is generic |
| `pkm-semaev engine=exhaustive` | explicit PKM base | enumerate candidate base tuples | yes | Generic enumeration control over C-selected base | No solver escape; coordinate selection belongs to setup |
| `mitm-frobenius*` | Frobenius-folded pair table and orbit canonicalization tables | cheap Frobenius canonicalization plus exact probe | yes | GA/G1 | Need a GA theorem or charge generic simulation by scalar multiplication; account orbit stabilizers |
| `compact-orbit-scan` base | raw-`x` scan, point lift, cofactor projection, Frobenius closure | supplies columns to later oracle | yes for stored points | C + GA setup | Raw-abscissa ordering, lift, coordinate squaring, and cheap Frobenius are outside G0 |
| `koblitz-orbit` / invariant-subspace base | factors Frobenius module and enumerates coordinate subspace | supplies folded columns | yes for stored points | C + GA setup | Subspace membership and coordinate generation are representation-specific |
| `glv-orbit` / CM orbit bases | smallest-`x` seeds plus cheap endomorphism closure | later lookup/addition | yes for stored points | C + GA setup | Endomorphism is known-scalar only on the subgroup; same-cost generic simulation is unresolved |
| linear S3 Weil descent | algebraically defined binary subspace | solves descended linear system | lifted and checked | C | Coordinate-subspace equations are the mechanism, not a table search |
| `descent-algebraic` F4/F6/SAT | construct Weil-descended polynomial/Boolean system | polynomial-system solve and lift | yes after solve | C | Solving degree, field equations, rationality, and lift completeness |
| `pkm-semaev engine=f4` | prime Kummer/tower equations | F4 over structured tower ring | yes after solve | C | Genuine candidate escape from generic bounds; end-to-end scaling remains open |
| planned `pkm-tower` | structured tower ideal | algebraic solver | planned | C | Must prove/measure solving degree, matrix size, yield, rank, and descent |
| small-root lattice Semaev lead | coordinate window and polynomial lattice | lattice small-root recovery | planned | C | Window definition and root bound are representation-specific; no generic lower bound applies |

## Code evidence

### Plain generic mechanisms

`SubtractOracle` performs exactly one group addition per candidate and probes the
factor-base point key (`src/cryptanalysis/ic_framework/plugins.rs:2094-2133`).
`MitmOracle` builds its pair table through group additions, merges the build operations
into the ledger, and delegates the probe to `decompose_mitm`
(`plugins.rs:2137-2214`). `rootfind-mitm` and `rootfind-pairs` state in their module
contract that they decide targets by the group law, not an algebraic system, and charge
each point addition (`src/cryptanalysis/ic_framework/rootfind.rs:1-35`); the latter's
terminal test is `fb.index_of_key(g.key(&c))` (`rootfind.rs:355-389`).

These paths are G0 provided their input base is treated as public input. If the base or
sum table is curve-specific setup reused over targets, they move to G1 and its size/build
must be charged.

### Why S3 root finding at m=2 is not an algebraic escape

The prime root finder documents the identity explicitly: the roots of the specialized
quadratic are `x(R-F_i)` and `x(R+F_i)` and the search visits the same candidates in
the same order as subtraction (`plugins.rs:2402-2421`). The implementation reads
coordinates and performs one batched inversion (`plugins.rs:2577-2643`), then probes
an exact point key and re-adds a hit in the group (`plugins.rs:2647-2680`).

Consequently:

- the *relation set and online search space* have a G0 simulation;
- the batched coordinate formula can have a different wall-time constant from a black-
  box addition, so equality of generic query counts is not equality of implementations;
- a `prime-abscissa` base is selected as the smallest coordinates carrying points
  (`plugins.rs:81-135`), which is not a G0 construction.

This is why the verdict is Mixed rather than simply G0 or C.

### Frobenius and orbit folding

`compact-orbit-scan` selects points by an ordered raw-abscissa scan, lifts them through
the curve equation, projects by the cofactor, finds a minimum coordinate over Frobenius
images, verifies the eigenvalue, and closes the column under signed Frobenius
(`plugins.rs:955-1253`). `koblitz-orbit` constructs a Frobenius-invariant coordinate
subspace and folds its points by signed Frobenius orbits (`plugins.rs:868-938`).

On the order-`r` subgroup the implementation checks
`[lambda]P = Frobenius(P)` (`plugins.rs:1151-1160`). This proves that a G0 simulator
can reproduce the *point*, but not for the same price: coordinate Frobenius is a native
field map, while generic `[lambda]P` costs logarithmically many group operations.
Orbit-key canonicalization also inspects coordinates. The applicable model is therefore
GA, with C used during base generation.

## CGK preprocessing checklist

Before applying `S T^2 = soft-Omega(epsilon N)` or
`P T + T^2 = Omega(epsilon N)` to a repository method, record all of:

1. prime subgroup order `N` and target distribution;
2. success probability `epsilon`, including decomposition and final-rank failure;
3. advice size `S` in bits, including point encodings, indices, coefficients, hash-table
   overhead if the representation is part of the advice, and automorphism metadata;
4. setup work `P` in the theorem's operation unit, including factor-base generation,
   lifting/projection, pair/k-sum construction, canonicalization, and filtering;
5. online oracle queries `T`, including verification and failed probes where required;
6. whether coordinate predicates or a cheap automorphism oracle make the algorithm
   strictly stronger than the theorem's generic interface;
7. whether the advice is fixed before the target is sampled;
8. how multi-target reuse is modeled.

No current classification label by itself supplies those eight facts.

## Initial conclusions and next proof obligations

1. **Closed for ordinary tables:** subtract, MITM, k-sum, rootfind-MITM, and
   rootfind-pairs are generic table/enumeration mechanisms. Their opportunity is a
   time-memory-preprocessing tradeoff, not a coordinate-algebraic asymptotic escape.
2. **Closed at S3 relation level:** naming direct subtraction “Semaev root finding”
   does not place it outside the generic search class. Higher summation-polynomial
   system solving remains outside this conclusion.
3. **Open model bridge for automorphisms:** write GA formally and prove a lower bound
   or a cost-preserving reduction before substituting an effective orbit count into
   Shoup/CGK. This is corpus target `RT-AUTOMORPHISM-PREPROCESS`.
4. **Open setup/advice theorem:** determine whether coordinate-selected bases can be
   treated as arbitrary fixed advice under the exact CGK experiment, and price their
   encodings and construction consistently.
5. **Live algebraic attack surface:** F4/F5/SAT, PKM tower systems, Weil descent, and
   small-root lattice methods remain outside generic lower bounds. They must be tested
   by the end-to-end obligations in `RT-END-TO-END-IC`, not dismissed by Shoup.

## Sources and verification boundary

The theorem interfaces were checked against the primary PDFs of Shoup (1997) and
Corrigan-Gibbs--Kogan (2018), identified in `corpus.json`. The method classification
and source paths come from
`research/prime_ic_program_20261029/ruling/classification.json` and the live source
files cited above. This audit proves no new external lower bound. Its strongest claims
are source-level simulation/classification arguments and remain
`proved_internal_unreviewed` in the corpus.
