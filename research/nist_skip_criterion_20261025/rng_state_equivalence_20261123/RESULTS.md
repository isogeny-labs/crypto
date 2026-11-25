# Seven-call mechanism narrowed; no curve skip rule

23November2026. Native exact source-model checks cover all42first-seven states
from the six already-published recovered seeds. Every state meets a sufficient
bound for first-advance acceptance for every positive draw bound<=2^26, and
both arithmetic routes agree. This is the unsigned development assertion
[pcc:PCC-20261123-f316df], standing unverified. No original FreeLIP executable,
primality test, point count, new curve or full base-point replay ran.

| Published control | Recovered seed | First seven state checks |
| --- | --- | --- |
| prime192v1 / P192 | 20000 | all pass |
| prime239v1 | 20420 | all pass |
| prime239v2 | 20507 | all pass |
| prime239v3 | 20509 | all pass |
| prime192v2 | 20512 | all pass |
| prime192v3 | 20517 | all pass |

The native checker, raw states.tsv, statuses and byte manifest retain the exact
arithmetic. Two routes in one producer are self-checks, not an independent
verifier or signed execution proof. The historical source bytes fetched for
inspection match upstream Git bloba86ce8ae5be49e0f35999a2c87f52e1718ebdb81;
SOURCE.json records size/hash and URL. No probability estimate or timing is used.

## Exact implication

Write M=2^107−1, g=3^121 mod M. zrandom(b) advances S to gS mod M until
S<M−(M mod b); its returned S mod b does not replace saved S. For b<=2^26,
M−(M mod b)>=M−2^26+1. The42checked states are below this lower bound.
Induction therefore equates the post-seven saved states for the published
discard recipe and any seven calls with bounds in the stated range.

Under correct arithmetic and a prime multi-limb input, zprobprime(n,7) takes
seven random Montgomery trials after its deterministic base2 test. This gives
a concrete sufficient explanation of the seven-discard state for all six
published controls, conditional on that caller sequence. It does not identify
the historical caller, check the original C arithmetic/ABI or recompute G.
Actual base-point output would additionally require the same subsequent draw,
limb-packing, reduction, on-curve search and sign rules. No per-curve offset was
searched or fitted. A second randomized prime test after initialization would
consume additional draws and is a falsifier of the simple caller sequence.

## What this rules out as an explanation of skipped labels

1. These first-seven draw bounds do not create per-curve variable RNG
   advancement for the six tested seeds. Rejection sampling in this warmup
   supplies no distinguishing branch in the source model.
2. FreeLIP's107bit saved state is separate from the48bit POSIX lrand48 state
   that generates curve seeds. FreeLIP consumption alone does not advance
   lrand48. Explaining eligible seed positions as unvisited requires an
   actual caller link, such as consuming lrand48 directly; it cannot be
   inferred from seven FreeLIP draws.
3. If E(Fp) has prime order n, Lagrange's theorem gives order n for every
   nonidentity point. An ordinary valid-point/generator-order check cannot
   distinguish curves already meeting that assumption. A failure would need
   different computed inputs, an added predicate or an implementation fault.

The older21order FreeLIP replay already accepts every published and omitted
order. This new state check does not change or repeat it. Historical omitted
base points remain unknown, so an assigned canonical point is not substituted
as their historical G. The b/coordinate identities from the curve equation
remain identities, not evidence of a selected scalar relation.

## Focused successor

The seven-call line is now a constrained point-generation mechanism, rather
than a candidate reject-all-omissions rule. The selection investigation should
target the caller's candidate visitation and computed-order/acceptance path.
A proposed caller model must prospectively specify both RNG states, every
state-consuming call and the order supplied to validation, retain all original
published/omitted controls, and predict the actual selected indices without
individually chosen offsets or exception lists. Original caller/decision logs
remain missing. The60000baseline is calibration, not a substitute for that
historical prediction. No criterion explaining all skips is established.

## Failures and reproduction scope

Initial build failed Werror because its boundary self-comparison was a
tautology. Exact source/script/compiler/error/status are retained under
evidence/attempt-001/. Replaced that expression with a call through the actual
acceptance predicate; no arithmetic/input/acceptance bound changed. Final
C++17 Wall/Wextra/Werror build and10second bounded arithmetic check exit0.
Initial claim creation rejected unsupported proof kind source before writing
a claim; a statement record with no verification-capable proof was then used.
The local Conductor command is absent127; docs/CONDUCTOR.md warning fallback
applies. KB is unavailable; existing reports and primary sources were read.
No scientific checkpoint, paid execution, AWS identity or infrastructure
change is claimed. Existing CI access problems are not used as a research
conclusion and no gate is bypassed.
