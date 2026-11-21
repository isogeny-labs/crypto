# Shifted reciprocal charts on balanced divisor classes

Prospective bounded correctness unit12, frozen before new adapter code or
native calls. It extends the actual balanced divisor component needed before
composing the CT49 Richelot maps. The chart10 producer and old full-class
Richelot suites will not be rerun. No result is recorded yet. [pcc:unverified]

## Primary sources and reuse

Read before selecting this unit: Consumer `experiments/chained-transfer-reference/src/balanced.rs`
(full existing arithmetic and its untranslated `Reciprocal`), `BALANCED.md`,
the full native `nonsplit_map.rs`, and Root `BalancedDivisors.lean`,
`BalancedChart.lean`, `KernelChart.lean`, `IndependentMapChart.lean` and
`RichelotAlgebra.lean`. Existing core Lean identities clear the quadratic
support/ordinate denominators and balanced counts; they do not construct fields,
projective curves or Picard groups. No new Lean declaration is planned.
[pcc:unverified]

Galbraith, *Mathematics of Public Key Cryptography*, corrected
[ch.8](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch8.pdf),
Theorem8.3.8/Corollary8.3.10/Exercise8.3.11, and
[ch.10](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch10.pdf),
Definition10.4.9, Lemmas10.4.10/14 and Theorem10.4.19 were reread in full relevant
sections. Cached PDF SHA values are9580b1eda23b976137861405ac545559f75051dadcc0db657d564f98ba9ff8a4
and dd5b456fb8a9a12a5dcb781c0bf5ec55422a39f397f4379cf71581263142a43a.
Use the established balanced arithmetic and induced Picard map; do not rederive
field/group laws. KB tools are unavailable, prior wrapper127 and locked atlas
remain; these exact primary files are the fallback. No novelty/absence claim.
[pcc:unverified]

## Fixed instances and question

`fixtures-12.csv` freezes the twelve monic output coefficient vectors already
recorded by audit11/f88c0c, copied as metadata without new arithmetic. Add one
separate auxiliary curve `y²=x⁶+1/F101` solely for rational Weierstrass support.
For each of the twelve models, Rust chooses the least nonzero a and least
positive b with b²=H(a)≠0. For the auxiliary use fixed a=2,b=41. A nonzero
shift is required: the prior untranslated reciprocal implementation alone
does not execute this adapter. [pcc:unverified]

Construct the monic output `F(w)=w⁶ H(a+1/w)/b²` and its inverse. Can an actual
balanced class map transport finite and closed quadratic support, both signed
infinities and pole multiplicities0/1/2, agree with pointwise transport, and
preserve the selected addition relations?

## Representation and planned adapter

Reuse the existing balanced composition/reduction method bodies and unchanged
Consumer `FpPoly`/`Fp2Ctx` sources with pinned versions of num-bigint/num-traits.
Copy the whole primary sources and record every extraction/import adjustment.
Keep this a standalone crate; no edits to old Consumer/Root sources or tests.

The class `(U,V,n)` is
`D_fin+n∞+ +(2−m−n)∞−−K`, `K=∞++∞−`, `m=deg U≤2`.
Forward finite support is `monic(w^m U(a+1/w))`; its ordinate is
`w³ V(a+1/w)/b` modulo that support. Lost roots at x=a go to infinity with
ratio V(a)/b. Old infinities go to `(0,±1/b)`. The inverse uses t=x−a,
`monic(t^m U(1/t))`, ordinate `b t³ V(1/t)`; lost support at w=0 has infinity
ratio bV(0), and old infinities go to `(a,±b)`. Retain multiplicities and all
old n/infinity weights. The two images of K form a hyperelliptic fibre, linearly
equivalent to the new K. State the resulting Picard argument separately from
finite executable checks and from any machine proof. [pcc:unverified]

## Frozen witnesses and coverage

Each ordinary model must produce exactly17 labeled witnesses:

1. Identity, δ=∞+−∞− and−δ (three finite-degree0 patterns).
2. `(a,±b)` each with either remaining infinity sign (four degree1 witnesses).
3. The least rational nonramified Q away from x=a, each infinity sign (two).
4. `(a,+b)+Q` and `(a,−b)+Q` (two degree2/pole multiplicity1).
5. Tangent doubles2(a,±b), using the actual derivative for V (two, multiplicity2).
6. Tangent double2Q and a distinct-abscissa pair Q+R (two, multiplicity0),
   with R the least nonramified rational point away from a and Q's abscissa.
7. One conjugate quadratic orbit with nonzero ordinate and its hyperelliptic
   opposite (two). Choose the least nonbase x with a square nonzero H(x) in
   the checked F101² backend, using its complete square table; do not choose
   an arbitrary trace input.

The auxiliary adds `(10,0)` and `(91,0)` each infinity sign and their pair,
so22 witnesses. These auxiliary ramified cases do not supply untested
degree-three Weierstrass support on the CT49 models.

For every witness, compare the coefficient adapter with an oracle that maps
its two geometric points individually, reconstructs closed conjugate support
and descends coefficients, or adds rational point classes with the explicit
K correction. The oracle shares the existing balanced group backend; it is a
different chart evaluation, not an independently authored trusted verifier.
Check forward/inverse roundtrips and the opposite-b chart equals target group
negation on every witness. For all ordered witness pairs in each model,
check map(D+E)=map(D)+map(E) and inverse recovery of the sum. Require all six
degree/infinity shapes, both infinity signs, and pole losses0/1/2 in each
direction. Planned totals are226 witnesses and3952 ordered pairs.

## Falsification and controls

All positive checks must pass. For every one of the13 models, three adapter
mutants must produce a wrong class or a structured mathematical-domain rejection
on at least one fixed witness: omit the positive lost-pole δ correction,
swap the two old-infinity images, or omit the x=a translation in the forward
support/ordinate transform. Record the complete offending witness and outcome.
Do not count build/I/O/timeouts/panics as a semantic rejection. Invalid b=0,
off-curve parameter and singular/noncanonical curve inputs must be rejected.
Both b signs remain legitimate maps: their relation to group negation is a
positive check, not a false invalid-sign control. Richelot dual orientation
still requires the actual correspondence's [2] test. [pcc:unverified]

## Execution and custody

Ordinary local bounded correctness unit: Rust1.99.1, locked/offline, release,
one compiler worker/codegen unit,120s per compilation/native phase and one
test worker. Reuse cached registry dependencies without network fetching.
Publish exact complete source/lock/input hashes in a normal immutable tag
before compilation/calls. Compile without running first, copy/hash a separate
chmod555 native qualifier before calls, then run the declared tests, strict
no-deps Clippy and format. Preserve full source/compiler/cache/qualifier, every
input, every witness/pair/control trace, output/exit and before/after guards as
one whole archive; retain all failures before any correction. No limit raises.

This is not the old full12317/10404-class suite, a full Richelot map, original
LP population, DLP solve or performance experiment. No new Lean/library/compiler
proof, paid computation, runtime admission, signed PCC verdict or matched-cost
promotion. The paid STOP, locked vault, original store/CI/merge gates and
canonical ledger/trust obstruction remain. [pcc:unverified]

## Subsequent discriminating step

After the class adapter, transform the actual Richelot quadratic factors and
correspondence through the same coordinate change, retaining their determinant,
normalization and infinity fibres; compare dual composition[2]. Continue the
nontrivial chain/prime-field descent queue without repeating settled cases.
