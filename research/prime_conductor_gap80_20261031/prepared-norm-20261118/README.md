# Prepared native polynomial log norms

The original requirement remains an evaluated ordinary isogeny crossing a
certified prime endomorphism-conductor ratio of at least 80 bits, followed
by paired scalar/ECDLP measurements and IC comparison. This component is
bounded native construction engineering, not a measured crossing.

## Frozen implementation and acceptance

Add opt-in `--prepared-norm` to the existing supplied-case evaluator. Reuse
h's reversed-series inverse, trace weights and scalar inverses 2/3 once per
baby block. Borrow immutable h and consume the existing FLINT jet objects
directly. Keep the established log coefficients u, v-u²/2, w-uv+u³/3 and
trace functional, gcd collision checks and all complete-coverage/map gates.
Keep default output bytes and historical source/receipts unchanged.

The context owns reusable scratch polynomials and two persistent
size-dependent cache polynomials. It remains live during later jet builds;
no peak-memory reduction or elapsed speedup is assumed. Text conversions
are absent inside this typed path, but scalar/point/polynomial arithmetic,
full window storage and setup remain actual costs. The old textual path is
still the default and the differential control.

Require: native independent direct-root sums over three fields (including
both frozen large fields), changing jets on the same prepared object,
collision rejection followed by successful reuse, invalid modulus rejection;
old/new default fixture bytes; seven existing supplied-case schedules across
four fields, with affine and projective caps 7/64; quotient/images equal to
old backend and direct PARI; duplicate/new-option and existing input guards.
The extra 96-bit degree-5 input serializes existing checked point records;
it performs no new point search or arithmetic. Small isogeny degrees are
not conductor gaps. The unchanged default fixture includes tiny known-answer
recovery controls; no performance session or new ECDLP solver is introduced.

Use separate candidate, checker and preserved baseline executables. Freeze
source/input/binary/qualifier hashes before calls and retain every failure.
One correctness qualifier has a 300-second cumulative cap; no paid compute,
new parameter search or performance experiment. Worker packaging includes
and hashes both native headers; its image remains unpublished/unqualified.

Sources: prior native optimized.c and direct-root oracle, existing supplied
cases and 96-bit map packet, installed FLINT 3.1.3 headers and official
https://flintlib.org/doc/fmpz_mod_poly/index.html . The online reference is
newer 3.7-dev; installed API declarations govern this build. KB unavailable,
wrapper exit 127. No novelty or fastest-implementation claim is made.

Results will be recorded separately in RESULTS.md. The initial source/binary
manifests and immutable baseline stay beside the checks.
