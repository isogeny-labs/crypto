#set page(margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#set heading(numbering: "1.")
#show table: set text(size: 8pt)

= W1: canonical Pic#super[0] arithmetic on smooth plane quartics

Run record 2026-10-18, branch `claude/quartic-pic-canonical-20261018`, frozen protocol commit `3fad80fa2`.
Synthetic instances only; not compared with any external implementation (no Magma, Sage, PARI or msolve
on the host), so no fastest-implementation claim, and no key-recovery, sub-rho or deployed-curve claim.
Every number below is *measured* by native Rust unless marked derived. Raw records: `results/`.

== Question and answer

Can $"Pic"^0(C)(FF_q)$ of a smooth plane quartic be given arithmetic with class-unique keys using only $FF_q$ linear
algebra, so that the degree-6 key collisions of `quartic_ic.rs` disappear, and what does one operation cost as $q$ grows?
Answer (measured): yes. Keys are the echelon matrix of the conics through the unique effective degree-3 divisor of
$x + D_infinity$ (typical classes) or the point $P$ of the class $K - P$ (exceptional classes). All validation
tests pass (V1 to V5, below). A generic addition costs 3523 multiplications and 44 inversions for every
$q$ from $2^12$ to $2^31$ (3503 at $q = 251$), about 4343 multiplication-equivalents at $q = 2^31 - 1$.

#figure(image("reports/add_pipeline.svg", width: 100%),
  caption: [The addition pipeline (diagram of the algorithm, `reports/diagram.py`).])

== Validation (exact pass/fail)

#table(columns: (auto, 1fr),
  [V1], [21 instances (nine GHS quartics, three random each at q = 5, 7, 11, 13): enumerated degree-3 divisors equal the zeta coefficient; distinct keys = \#Jac from the L-polynomial (148, 356, 1348, 1324 on GHS; 1410, 1892, 3173 at q = 13); exceptional keys = \#C(F_q).],
  [V2], [300 random triples per instance (associativity, commutativity, inverse, representative independence, auxiliary-cubic independence), \#Jac $dot$ x = 0 for 100 x, full Cayley tables at q = 5 (87 and 148 classes, 658 503 and 3 241 792 triples), 14 650 exceptional pairs, 6 141 agreements with an independent second reduction path.],
  [V3], [key equality iff equal divisors or collinear with the same residual point, on every pair of enumerated divisors (23 016 same-residual pairs).],
  [V4], [GHS transfers at q = 5, 7, 11 (table below): homomorphism, order l, scalar relation hold on every pair; no unexplained canonical collision.],
  [V5], [smoothness certificate accepts all instances, rejects node, cusp, two conics, quadruple line.],
)

#table(columns: 12, align: center,
  [q],[l],[usable n],[deg-6 keys],[old deg-3 keys],[canonical keys],[pairs],[hom. fail],[l x != 0],[x(n) != n x(1)],[unexplained],[old collisions resolved],
  [5],[37],[18],[18],[17],[18],[66],[0],[0],[0],[0],[1],
  [7],[89],[38],[38],[32],[38],[236],[0],[0],[0],[0],[7],
  [11],[331],[164],[164],[144],[164],[6536],[0],[0],[0],[0],[20],
)
V4 uses the split instance `quartic_ic.rs` builds at each q (records 1, 4, 8 of the GHS run); the other six GHS
quartics are covered by V1 to V3, V5. Skipped by the dumper: 6, 6, 0 transfers (non-reduced degree-6 divisors or cubic space dimension not 4).

== Operation cost (M1, M2)

#figure(image("reports/m1_cost_vs_q.svg", width: 100%),
  caption: [Median and interquartile range over 200 operations per point, scaling quartics from `results/m1_summary.jsonl`. (a) $FF_q$ multiplications; (b) with inversions priced at the measured inv/mul time ratio (7.8 at q = 251, 18.6 at q = 2^31 - 1); (c) wall time on two pinned cores of a host with load average above 80.])

#table(columns: (auto, auto, auto, auto, auto, auto),
  [operation], [mul], [inv], [wall at 2^8 ($mu$s)], [wall at 2^31 ($mu$s)], [priced at 2^31],
  [add, generic], [3523], [44], [32.7], [37.6], [4343],
  [add, exceptional operand], [6663], [68], [58.1], [67.2], [7930],
  [double], [5625], [54], [47.8], [53.0], [6631],
  [negate], [5402], [35], [40.1], [43.0], [6054],
  [key encoding], [36], [3], [0.7], [0.8], [92],
  [scalar mul, 64-bit], [467 111], [4810], [4190], [4373], [556 748],
)
Fits against $log_2 q$ (7 points, 95% interval): generic add multiplications $0.56 [-0.25, 1.37]$ per bit
(flat), priced $18.5 [9.5, 27.4]$ per bit, wall $207 [136, 278]$ ns per bit. Of 1400 generic adds none lifted a
generating space or took an exceptional branch. Over the small-$q$ tests 927 of 132 994 operations lifted, 16 945 had $P_0$ in
the support, 21 126 ended exceptionally.

#figure(image("reports/m1_scalar_key.svg", width: 85%),
  caption: [Scalar multiplication by a random 64-bit scalar and key encoding, same records.])

== Limits and deviations

The constants are those of this dense-linear-algebra implementation; they are counts, not bounds. V4 covers three of the
nine GHS quartics (limited by reusing `quartic_ic.rs` unmodified). The base shift of the degree-6 transfer is inferred from the
data and then verified on every pair. M1 and the tests ran through a stand-alone crate including the module; the pinned
repository `cargo test --lib` result is in the PR. Details: README.md.
