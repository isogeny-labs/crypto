#set page(paper: "a4", margin: (x: 17mm, y: 16mm))
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false, leading: 0.55em)
#set heading(numbering: none)
= Collision certificates: construction and proof
*Implementation checkpoint, 25 October 2026. Actual case comparison pending.*

The existing mixed-component zero-fiber collector solves four restricted S4
polynomials in three elementary symmetric coordinates. This follow-up tests
whether a small exact certificate can prove that every solution repeats the
ordinary factor already peeled from the relation. Such a solution cannot pass
the existing distinct-factor decoder.

#image("figures/flow.svg", width: 100%)

== The sufficient rejection rule
Let $q_a = a^3 - a^2 e_1 + a e_2 - e_3$ and let the input equations be
$f_0, f_1, f_2, f_3$. The producer searches for polynomial multipliers with
$q_a^k = sum_(i=0)^3 h_i f_i$. At a common zero, $q_a^k=0$; in a field this
implies $q_a=0$. Substitution of the three elementary symmetric expressions
gives $(a-x_1)(a-x_2)(a-x_3)=0$, so some factor abscissa equals the peeled $a$.
This is a sufficient condition. A degree-capped failure leaves the system
requiring fallback.

The dense Macaulay reduction searches total degrees 4 through 8 and powers
1,2,3 in fixed order. A separate sparse expansion checks the coefficient
identity over the actual prime field. Both construction and checking are
charged, including unsuccessful degrees. Original roots and relation hits
are excluded from the producer's input pack and supplied to the later audit.

== Checked implementation controls
Gate v3: five Rust tests passed, zero failed, production build and Lean exit 0.
Controls include a linear certificate, a nonradical system requiring a square,
a distinct-root system where rejection must fail, a tampered coefficient,
duplicate/noncanonical terms, the Vieta identity over the toy field, and the
Phase 1 schema regression. These are implementation checks, not a measured
benefit on the 42 case-arms.

Native executable SHA-256:\
#text(size: 7pt)[4c49d77faa9b5ef3fd54459bd0ac0cf69ee077eef43ec84361753f7dbc3555f5]

#pagebreak()
= Scope of the proof and comparison
Lean 4.34.1, compiler commit
`5045d0056413266e57c625dcd7c365b10e377c52`, checks
`RepeatedFactor.cubic_identity`, `power_zero` and
`rejection_of_certificate`. The rejection theorem exposes an explicit
no-zero-divisor hypothesis and an evaluated certificate identity. The cubic
and rejection lemmas use `propext`, `Classical.choice`, `Quot.sound`;
`power_zero` has no axioms. No accepted declaration uses `sorryAx`.

The native parser, actual finite-field representation, polynomial evaluation
bridge, S4-to-group bridge and decoder implementation are not formalized by
these lemmas. The independent expansion and retained physical references check
those finite executions within their stated scope. This is a conditional
algebraic rejection proof, with executable integration obligations still open.

The universal cubic identity has standard certificate
`IDC1h10810f17e0f6c8f6`, with 64 derived and 64 extra evaluation points and a
hash pointer to the checked Lean source. A false identity obtained by adding
one is refused. That standard certificate's fixed Mersenne modulus does not
replace each instance's sparse expansion over its own small prime.

== Registered experiment
All seven original primes, all three presentations, both phi and doubling:
42 case-arms, each with a first execution and one deterministic repeat
(84 producer calls), 42 independent first-run audits and 42 parity checks.
The first two p37 seed-0 case-arms also serve as the actual implementation
gate and are retained in the full comparison. No extra pilot is repeated.
All canonical pairs are included, with their original polynomial inputs.

Each case has one worker, 8 GiB, a 1,200-second outer watchdog and a
1,180-second producer deadline. Separate reservations let the already running
85-configuration target-recovery panel continue. Actual launch receipts are
written after the reservation wait. Existing successful 189+21 and Phase 1
F4/point-census studies are reused, never rerun.

== Failures retained and price boundary
Gate v1: four Rust tests/build pass; the auxiliary Lean zero-exponent branch
fails. Gate v2: four Rust tests/build and corrected Lean pass. Preparation v1
rejects a valid completed Phase 1 record because the new parser expected a
pipeline status tag. The parser is corrected and covered by gate v3's fifth
test. None of these attempts runs a new retained-system F4 solve.

The production result remains pending at this checkpoint. The comparison will
report certified system counts, successful/failed degree work and verification
cost. Any replacement calculation combines actual certificate multiplications
with retained F4 row counts on uncertified systems; it is a partial stage model.
Other F4 work, polynomial construction, memory, hashing and I/O remain unpriced.
No new fallback execution, integrated target/rank result, admitted timing,
complete-cost gain or rho claim follows. Canonical IC/rho charts and the
benchmark index stay unchanged until their existing gates are met.

Sources: `collision_certificate/PROTOCOL.md`, `EXECUTION.md`,
`proof/PROOF_MAP.md`; raw native gates v1/v2/v3, preparation v1/v2 and
`collision-certificate-identity-v1`. Standard prior symmetrization work:
#link("https://link.springer.com/chapter/10.1007/978-3-642-55220-5_3")[Faugère et al., EUROCRYPT 2014] (publisher abstract read; full text unavailable).
