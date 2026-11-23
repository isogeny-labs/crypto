# Atlas, carry encoding and rank selection: prospective protocol

Status: **review_required; not launched**. This is a development instrument and
prospective experiment specification, not an admitted performance result.

## Requested work

Pursue (1) isogeny representatives with cheaper decomposition, (2) sparse
addition with explicit modular carries, and (3) useful-relation-based selection.
All three remain open until their respective comparisons and replay finish.

## Prior work and boundary

The source review is in README.md. These are compositions of existing leads,
not claims of new mechanisms. Do not duplicate or overwrite the earlier
Edwards E1Q systems, Curve25519 autolab rounds, or PFDR CDCL(T) lane.

The headline comparison is **constraint encoding of identical signed,
distinct-column, three-summand queries**. The finite set is explicit; this
instrument makes no claim to compact algebraic membership. A separate generic
enumerator supplies functional answers and rank diagnostics. Never count its
enumeration as a successful algebraic solve.

## Inputs and fixed arms

- Proposed miniature fields: p = 2029, 8173, 32749 (2^k-19, k = 11,13,15).
  Curves: y^2 = x^3 + (486662 mod p)x^2 + x. Count the whole group, take its
  largest prime ell, and reject a field if the odd subgroup is not simple.
  Do not replace a rejected field silently. These reductions share selected
  traits with the RFC curve, not a demonstrated complexity law.
- N = 8 sign-orbit columns per arm; 32 training and 32 holdout queries;
  seeds 13,29,47,71. Reject insufficient subgroup capacity. Targets are
  distinct nonzero multiples of the subgroup generator; all train and holdout
  targets occupy disjoint sign orbits (R and -R never straddle the panels).
  Shuffle representatives 1..(ell-1)/2, choose 64, and assign independent
  random signs with the frozen native RNG. Reject insufficient capacity.
  Scalars are retained for known-answer replay
  and never enter the emitted relation constraints.
- Arms: source small-x; same support transported through the rational
  2-isogeny; target small-x pulled back to the source; mixed source/target
  support; random equal-size support. Preserve column signs across transport.
- Never select a winner using holdout data. Selection uses training
  coefficient-rank/probe counts, ties retain the earlier fixed arm. This
  criterion is a **generic stage diagnostic**, not total computation.
- Emit direct modular and radix-4 schoolbook-carry constraints for every
  holdout query of every arm. No query may be removed after seeing an outcome.

## Native construction

For E(a,b): y^2=x^3+a*x^2+b*x, use
E'=E(-2a,a^2-4b), phi(x,y)=(x+a+b/x,y*(1-b/x^2)); map O and (0,0) to O.
The dual applies the corresponding quotient and rescales by (1/4,1/8).
On the odd subgroup, inverse transport is [2^-1] dual. These are standard
formulas, exercised by finite functional checks, not newly proved here
[pcc:unverified].

Each modular multiplication AB=C+pQ has canonical A,B,C,Q in [0,p).
In radix B0, the carry arm imposes, for each schoolbook column k,
carry_k + sum_i a_i*b_(k-i) - c_k - sum_i p_i*q_(k-i)
= B0*carry_(k+1), with zero endpoint carries and signed bounded internal
carries. This is an integer encoding, not Weil descent. Linear reductions
still use mod; this arm isolates multiplication carries rather than claiming
that every reduction is expressed as a carry circuit.

## Controls and outcomes

- A/A: duplicate the source arm with an identical solver configuration and
  fresh invocation. The current native instrument does not yet orchestrate
  solver invocations or timings; implement and freeze that runner first.
- Negative: random equal-size bases; impossible signed-distinct queries;
  same-support conservation. The latter must have exactly identical generic
  hits, probe counts and coefficient ranks after transport.
- Positive: known signed three-point sums and exact product encodings.
- Validate every SAT model by native on-curve, membership, distinct-column
  and full group-sum checks. Native fixture replay and small-prime run replay
  for canonical native-Z3 models exist; other solver formats remain open.
- UNSAT/unknown: for miniatures, compare with exhaustive native enumeration.
  Unknown, timeout or censored output is not UNSAT and not a failed hypothesis.
- Report all setup, selection, target generation, transport, solver, replay
  and matrix work separately; retain cold totals, memory and failures.
  Pair-probe counts are not GAE, S, wall-time bounds or full cost.
- A consistent encoding reduction advances idea 2 at the solver stage only.
  A held-out rank/probe gain advances idea 3 at the generic stage only.
  Idea 1 needs a matched solver-stage result on identical transported support.
  No gain is reported until the frozen matched comparison is complete.

## Execution route and limits

Use EKS cpu-spot for untimed diagnostic collection; any timing comparison
requires cpu-l2-spot admission, warmup, five rounds, matched A/A and the
applicable isolation checks. Four vCPUs, 4 GiB, 900 s per job, zero retries,
USD 5 total ceiling including image build, idle and failed work. Preserve
the original absolute deadline once submitted. Do not launch if budget
headroom is unknown or a prerequisite is missing.

Prospective inner command (one cell, not a submission receipt):

```
study --run 2029 8 32 13 /work/p2029-n8-seed13
```

Required before launch: passing checkpoint gate; published exact source,
protocol and input hashes; source-bound custom image containing this native
instrument and frozen solvers; end-to-end model replay integration; canonical machine
experiment.protocol/v1 and export adapters; recorded budget headroom;
authenticated command-job route. The current GitHub ecbench build route does
not contain this binary. Do not dispatch the stock validation fixture instead.

Until these dependencies are met, execution is blocked, not approved or
running. Local functional tests do not discharge the requested experiments.
