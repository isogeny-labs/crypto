# Protocol: binary genus-3 image transfer and solver measurement

Frozen before the recorded transfer and measurement runs on 2026-10-31.
Conductor coordination was attempted before editing, but this checkout has no
`.conductor/` metadata and no `conductor` executable; the required check exited
127. This protocol does not authorize a production-target logarithm campaign.

## Requested scope and acceptance criteria

| Requirement | Acceptance evidence |
|---|---|
| Execute the pinned `m=83` pullbacks | Native receipt for the exact registered curve model, field basis, prime-order generator `P`, and public known-answer `Q=[37]P` |
| Certify image order | `i(P) != 0`, `[r]i(P)=0`, cited independent primality/order receipt for `r`, and `gcd(3,r)=1` |
| Certify pull/push composition | Typed certificates for `P`, `Q`, infinity, and the exceptional two-torsion fiber, each checking `phi_* phi^*=[3]` in its supported domain |
| Certify scalar recovery | `[37]i(P)=i(Q)`, a solver that receives neither `37` nor another planted scalar, recovered scalar replayed as `[k]P=Q`, and the degree inverse `3^-1 mod r` recorded |
| Implement a named destination solver adapter | Deterministic `jacobian_image.rho3` and `jacobian_image.bsgs` methods operating on the actual Mumford divisors returned by `OrdinaryBinaryCover::pullback_point_class` |
| Run the three-arm cold comparison | One sealed native `ecbench` session with source reference, destination generic control, and destination candidate on identical one-target workloads at four registered sizes |
| Attempt the exact `m=83` family gate | Run the complete transfer certificate and a bounded known-answer adapter check; do not represent a bounded interval solve as a full-order `m=83` DLP result |

## Frozen mathematical objects

The family is

```text
E_a: y^2 + x*y = x^3 + a*x^2 + 1
C_a: v^2 + u^2*v = u^7 + a*u^4 + u
phi(u,v) = (u^3, u*v + 1),       degree(phi)=3.
```

The primary exact gate uses `a=0` and the registered model
`icv1-f2m83-tm6151469093347-debefd74` over the polynomial basis
`z^83+z^45+z^2+z+1`. The exact representation is the existing public fixture:

```text
r = 2417851639230796216685689, h = 4
P = (0x477f77103dfad59850800, 0x2fa5e737d542c4e4fd5c3)
Q = (0x590a9c0709b34b0bcd3d4, 0x6aa49dc52796827db931a) = [37]P.
```

The existing prime receipt and group-order fixture are inputs, not regenerated
claims. The run must bind their repository paths and revisions.

The four measured toy sizes are the same `a=0`, `b=1`, degree-three cover
family and the repository's registered prime-order subgroup representation:

| Field degree | Curve slug | Prime subgroup order |
|---:|---|---:|
| 7 | `icv1-f2m7-t13-616700dd` | 29 |
| 9 | `icv1-f2m9-t5-81e744be` | 127 |
| 13 | `icv1-f2m13-t181-515ee569` | 2003 |
| 19 | `icv1-f2m19-t797-b6cf2467` | 130873 |

Every order is prime and coprime to three. Changing the curve, modulus,
generator, target derivation, target count, seeds, or accounting after seeing
the results is inadmissible.

## Hypotheses and falsification conditions

### H1: the executable pullback preserves the pinned subgroup

Prediction: for every frozen curve, `i=phi^*` maps `P` and each target into
valid reduced Mumford classes, `i(P)` has exact order `r`, and the typed
composition witness gives `phi_*i(R)=[3]R`. The exact `m=83` receipt must also
cover infinity and the exceptional rational two-torsion fiber.

H1 is refuted for a tested instance by an invalid divisor, zero generator
image, nonzero `[r]i(P)`, failed `[k]i(P)=i([k]P)` replay, failed composition,
or mismatched field/model identity. An infrastructure error is inconclusive.

### H2: native destination solvers reach the transported image

`jacobian_image.rho3` is the destination generic control: a deterministic,
seeded three-partition Pollard rho over the order-`r` cyclic image.
`jacobian_image.bsgs` is the named destination candidate: cold textbook BSGS
over the same Mumford image. Both construct the cover, pull back `P` and `Q`,
solve without receiving the planted scalar, and return `k` for independent
source verification.

H2 requires all recorded toy targets to verify and deterministic audit replay
to reproduce their answers and counters. Any wrong answer invalidates the
corresponding comparison. Exhaustion remains a recorded failure. The exact
`m=83` adapter gate uses the public `k=37` fixture with an interval bound of
64; it establishes adapter reachability only, not generic full-order cost.

### H3: measure destination and net cost without presuming an advantage

The three primary arms are:

1. `source_reference`: `rho.signed_frobenius_strong` on `E_0`;
2. `destination_generic_control`: `jacobian_image.rho3` including cover
   construction and both pullbacks;
3. `destination_candidate`: `jacobian_image.bsgs` including the same route.

An A/A repeat of the source reference is included for wall-noise assessment.
The spec uses eight independent one-target workloads per curve, five measured
rounds, one warmup, fixed derived seeds, and L2 isolation requested. Every run
is a fresh process, so setup is cold and target count is one.

The primary cryptographic unit remains `ecbench.gae/v1`. Jacobian additions,
Cantor polynomial work, table operations, and any operation without a pinned
source-equivalent conversion must be retained as unpriced counters. Therefore
an operation-count net advantage is supported only if the implementation
produces a common deterministic unit with no unpriced work. Cold wall time is
reported only at the isolation level earned and remains a practicality axis.
No zero price may be substituted for missing work.

## Boundary, success, and stopping rules

- Generic source boundary: the per-workload rho floor recorded by `ecbench`;
  source reference: the strong signed-Frobenius single-target rho arm.
- Destination question: compare candidate and destination control on identical
  image targets. Net question: compare the complete candidate and source
  reference.
- A measured net-advantage verdict requires every pair correct, no unpriced
  work, common units, replay of every deterministic run, and a totals-ratio
  95% interval entirely above one for source cost divided by candidate cost.
- Four sizes permit a descriptive exponent fit only after per-size rows are
  inspected. The exact `m=83` gate is not inferred from that fit.
- Stop and retain the run on wrong answers, audit mismatch, resource refusal,
  timeout, or exhaustion. Do not drop failed targets or replace seeds.
- Full-order BSGS/rho at `m=83` is outside this bounded public-fixture run. The
  adapter must refuse or avoid that allocation; only the bound-64 known-answer
  gate is attempted.

## Required artifacts

- Native implementation, unit/integration tests, exact commands, host capsule,
  spec, sealed session, replay audit, saved comparisons, and compact receipts.
- A v3 transfer assessment with separate transfer, solver applicability,
  modeled advantage, and measured advantage verdicts.
- A source-linked run report and results report, editable quantitative graph,
  SVG, 150-dpi PNG, and visually inspected PDFs.
- Copies of performance visuals under `benchmarks/20261031-n83-genus3-image/`
  and the required benchmark index row.
- Any missing common-unit or full-order solver obligation is named in
  `problems/` with a checkable exit condition; recording it does not satisfy it.

No ledger, frontier, curve-security, deployed-target, novelty, or asymptotic
sub-rho promotion is authorized by this protocol.
