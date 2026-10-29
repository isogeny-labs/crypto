# Exact shifted source/twist overlap: pre-data protocol

The user asked to continue pursuing twist symmetries, generator/progression
connections and coordinate/factor-base improvements. The 880-cell predecessor
finds valid bases but no twist-specific cost reduction. This separate exact
diagnostic asks what controls the **full-coordinate-universe** supply. It does
not replace finite-prefix yield, decomposition or whole-IC cost experiments.

## Fixed inputs and outputs

Use all ten original public odd-prime-order fixtures from
execution-v3/decomposition-20261026/PLAN.json, with their exact registered source
and twist identities. For each p, enumerate every shift c=0,...,p-1 and every
u in Fp. No seed, target, shift, orbit generator or fixture selection by outcome.
The producer and independent verifier are two actual executions on the same
complete panel, not independently sampled processes per shift. Their counts
are not pooled. Final E28 target labels remain unopened.

For E:y²=f(u), f=u³+au+b, d nonsquare and no rational f-root, record
N_c=#{u: chi(f(u))=-1, chi(f(u+c))=1}, ordinary positive-positive overlap M_c,
correlation C_c=sum chi(f(u))chi(f(u+c)), and p-C_c. Producer uses Euler's
criterion and sign arrays. Independent verifier recomputes cubic values and
square-root multiplicities, directly counts y²=f(u)f(u+c)/d, checks exact
counts, source cardinality/primality/no rational two-torsion, polynomial gcds,
zero-shift controls and integer Weil inequality. All shifts and zero counts
are retained. Reject a deliberately plus-one false count at every shift.

For exactly c=1 and c=3 on each fixture, PARI computes the full genus-two
Frobenius polynomial, checks the independently counted Fp points, and tests
divisibility by source and twist elliptic Frobenius polynomials. Controls are
the known covers H:y²=x^6+a*x²+b -> E, (x,y)->(x²,y), on all ten fixtures:
the source factor must be present. P+1 must fail that factor test. These are
embedded correctness checks, not separate measured experimental arms. Native
integer polynomial division independently replays every factor flag. Require
exactly 20 overlap polynomials, ten positive covers and ten false-polynomial
rejections, actual exit/hash receipt and strict clean GP output.

## Classical interpretation and proof scope

Because chi only takes ±1 and translation permutes the field,
4*N_c = sum (1-chi(f(u)))(1+chi(f(u+c))) = p-C_c.
The cubic is irreducible because it has no rational root. For c nonzero and
p>3, f(x+c) has nonzero x² coefficient 3c; it is a distinct irreducible cubic.
The product is squarefree, so D_c:y²=f(x)f(x+c)/d is smooth genus two. Its
leading coefficient is nonsquare, hence it has no Fp points at infinity and
#D_c(Fp)=p-C_c=4*N_c. The classical Weil bound gives
(4*N_c-(p+1))² <=16*p. The full-universe mixed supply is near p/4, or near
half of the twist x-support at large p. This is not a bound on a short orbit
prefix or a decomposition algorithm's cost. c=0 is singular and is excluded
from genus-two/Weil claims.

A Lean proof checks the ring expansion and exact conditional finite
sign/permutation identity. Legendre-symbol, projective smooth-model and Weil
bridges are not formalized. An identity certificate supplements, not replaces,
those proofs. The point-count identity is a classical character-sum
interpretation; no novelty is claimed. A Frobenius factor test is not an
implemented subgroup map, destination solver or DLP speedup.

## Execution and admission

Pin the complete tested native source/binary, structured protocol and analysis
before panel data. Same authorized local fallback as execution-v4 applies:
CodeBuild monitoring/ECR access is denied and no custom EKS image is verified.
No new cloud resource, funding, IAM change or permission question. Push a full
Setec checkpoint and gate before launching. One thread, 4 GiB address space,
64 MiB/file; producer and independent verifier each have 600-second hard cap
plus one-second grace. PARI has 300 seconds, initial 64,000,000-byte stack and
unchanged 4 GiB maximum. No retries except separately recorded infrastructure
failures. No wall-time/cost claim, A/A or statistical power inference; the
runtime estimate is the prospective watchdog envelope, explicitly unmeasured.

Require exact independent equality and clean native/PARI checks. Preserve all
failures, scripts, argv/env/input/output hashes, actual exit/signal and caps.
Unified export records the two actual complete-panel producer/verifier rows;
the later GP stage verifies the fixed polynomial subset. Embedded controls remain
explicit records, not fabricated standalone executions. Publish deterministic
raw archive/manifests to the user-confirmed Setec artifact store, report,
editable graphs and inspected PDF; update the same regular PR. No validated
benchmark or leaderboard promotion. Stop at this complete finite panel/cap;
do not claim mathematical exhaustion or unscheduled background continuation.
