# Capped Macaulay provenance solver: prospective correctness qualification

Owner goal remains active: complete classical subexponential prime-field ECDLP
algorithm, with complete end-to-end functionality and a different direction from
tensors. This component targets the missing source-ideal certificate interface.
It does not assume or establish a useful growing-family success probability.
All producer-local claims have unverified standing. [pcc:unverified]

Freeze before implementation and solver calls. Reuse the canonical native
polynomial parser/type and the existing grevlex/quotient primitives. No Boolean
mask arithmetic is valid here. Prior narrow source checks: sparse_macaulay.rs
and disk_macaulay.cpp are Boolean-only; GBRL Buchberger has no input-row
certificate interface. Do not claim a new mathematical elimination method.
KB remains unavailable; no novelty or exhaustive review claim.

Producer design: enumerate ALL input monomial multiples of total degree<=D.
Forward sparse Gaussian row reduction retains field-scalar weights over those
original rows. A candidate basis uses divisibility-minimal leading pivots.
A separately compiled symbolic verifier must check each candidate polynomial's
input-row identity, every original generator's zero remainder, and every
S-polynomial's zero remainder. Only after that verifier succeeds may the
msolve-compatible producer return exit0. Partial rowspace or failed verification
is UNKNOWN, never UNSAT or a completed certificate. Preserve candidate files.

Implementation bounds for this development instrument: prime characteristic
as admitted by the existing parser (p<2^31), degree cap1..6, <=20 variables,
<=20000 input multiples, <=2000000 live polynomial/provenance entries,
<=1000000 row-reduction steps, <=128 candidate basis polynomials, <=1MiB
serialized certificate. Stop on any exceeded bound. A prospective runner uses
one native process,20s CPU/wall and2GiB address cap per solver case, including
its verifier; exact failures/exits stay recorded. No cap is raised adaptively.

Frozen first panel, each case once in fresh write-once qualification-001:

- F5 grid x^2=x,y^2=y at D3: source-ideal basis, quotient4, exactly4 points.
- F5 nonreduced x^2=y^2=0 at D3: source-ideal basis, quotient4, one point.
- F3 x^2+1 at D3: source-ideal basis, quotient2, no rational points; do not
  call this ideal unit or an algebraic-closure refutation.
- F5 x,x-1 at D1: valid unit-ideal row identity, quotient0.
- F5 x at D2, and F5 x,y at D2: valid finite ideals used by negative checks.
- Negative certificates: wrong row coefficient; forged unit for ideal(x);
  basis(x) for input(x,y) with otherwise valid row identity; the two original
  generators x^2-y,xy-1 claimed as a complete GB with valid input identities.
  The verifier must reject all four. Never weaken its checks to accept them.
- Three distinct fixed degree caps D4,D5,D6 on the same preserved natural
  positive norm input m3,t3,c1,R=(0,6). Public input is exactly the parent's
  core001 model call37, with its original sourceb5e2b31d42, not a planted
  coefficient witness. Preserve every completed/unknown attempt; no extra
  target, cap or retry. Compare any successful basis's complete rational
  assignment set with the retained parent printed basis, checking original
  equations. Stop on disagreement, keep failure as unknown.

The parent population already established one rational point model for this
natural target. That reference is only an outer software control; no scalar,
assignment or decomposition witness enters producer/verifier. This panel is
exact finite development computation, not random-target yield, timing, speedup
or a scientific growth experiment. Incidental clocks do not become measurements.
No cloud/paying job or cold end-to-end claim is authorized by this first panel.
Connecting the new backend to cold IC requires its own amendment and actual
replay; the parent end-to-end path remains separately qualified.

Capture source commit and all producer/verifier/backend hashes BEFORE calls.
Retain every input, output, candidate, certificate, stdout/stderr, status and cap.
If certificate serialization exceeds the declared cap, return unknown before
writing that oversized certificate and record the reason/counts; do not emit
and then delete records. Preserve partial files already emitted. Whole-panel
archives follow existing artifact policy; do not split/shard to evade64KiB.
Any oversized evidence needs the authorized store, not silent omission.

Formal scope: source membership and Buchberger's criterion are standard
polynomial ideal facts. A faithful Lean instance map/declaration search is
required before any new Lean edit. Native verification is not a signed external
PCC verdict or proof of the program. No generic law should be copied from
Mathlib/crypto-lean into a study. Do not raise Lean limits or assume the endpoint.

Cost question: with v=O(mt), D=O(m+t) and balanced parameters, the monomial
count binom(v+D,D) is exp(O(sqrt(n)log(n))). Capping all phases by a polynomial
in that count yields a conditional per-attempt envelope. Proving a sufficient
verified relation yield and implementing arbitrary large-prime arithmetic remain
separate obligations. An unknown from this strategy/cap is not a lower bound
on the ideal's certificate degree, and does not refute every possible solver.
[pcc:unverified]

Pre-execution certificate format clarification: the proof serializes the source
multiples actually referenced by the selected candidate basis, with their IDs
remapped compactly. The producer still generates/reduces the entire prescribed
Macaulay rowspace and logs its total counts. Unused internal rows are not part
of the successful basis-membership proof object. No raw file is truncated or
removed, and this compact proof does not certify a negative rowspace/rank claim.

## Amendment A1 — keep original generators in the candidate

First panel: all6 small ideals pass and all4 adversarial certificates are
rejected for their declared reasons. The natural D4/5/6 candidates all fail
original-generator inclusion, although their source row identities pass.
Dropping nonminimal-leading pivots before completion can generate a proper
subideal; the checker correctly refuses it. Preserve qualification-001.

Repair candidate construction by retaining every nonzero original generator
in addition to the minimal-leading pivot subset (omit exact duplicate
polynomials only). Each added original has its trivial source-row certificate.
Keep the exact same provenance, original-inclusion and Buchberger checks and
all caps. This preserves I=J even if the candidate is still not a complete GB.
It does not assert GB completion or a degree bound.

Before new calls, freeze exactly one repeat of the same6+4+3 panel in a fresh
qualification-002, with new source/binary hashes captured first. Preserve the
old binaries/source commit38c0d23859 and raw panel. No degree, support, time,
memory or verifier criterion is loosened, and no further retry is authorized
by A1. The repeat is justified by this actual candidate-construction change;
completion may still remain unknown. No timing comparison is admitted.

## Amendment A2 — diagnostic logic and saved-certificate replay

A1 still returns unknown on all3 norm caps. The candidate now literally
contains every original generator, so the old message `original generator
not included` is too strong: nonzero division remainder under an incomplete
basis does not prove ideal non-membership. The initial checker criterion was
sufficient for acceptance, but its failure label/interpretation was inaccurate.
Preserve both panels and correct that interpretation explicitly.

Change only verifier check order/diagnostics: source identities, then all
S-polynomial reductions (completion), then original-generator reductions.
The acceptance conjunction stays identical; use the message `original
generator does not reduce to zero`. Freeze exactly one checker-only replay
of the saved002 six positive/four negative/three norm candidates, with fresh
hashes/logs and unchanged20s/2GiB bounds. Do not regenerate source rows or
rerun the producer. Expected positives pass; the deliberately missing original
is rejected after its candidate GB check; incomplete norm candidates remain
unknown/rejected at completion. No new degree or resource experiment.

A1's phrase about a possible proper subideal was a general possibility, not
an established property of these norm candidates. Source membership cannot be
inferred negatively from a nonzero normal form before GB completion is known.
The first panels' raw error messages are historical diagnostics, not proofs
of that stronger statement or a certificate-degree lower bound.
