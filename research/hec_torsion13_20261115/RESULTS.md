# Explicit genus-3 maximal isotropic 3-kernels

The construction and certificate replay pass. On the existing genus3p127family0
curve, both previously abstract rational3kernels now have explicit reduced
Mumford coordinates over F_(127^13). Each contains27distinct3torsion classes;
their729cross sums are distinct and exhaust J[3]. Both kernels are Frobenius
stable and annihilated by the respective certified irreducible cubic. This is
a constructive arithmetic prerequisite, not an isogeny or10xcost result.

## Terminal evidence

- Field: F127[z]/(z^13+z+10), first admitted c in the frozen1..126family.
  Prime-degree Rabin irreducibility criterion passes. q is
  2235879388560037062539773567, with q=3mod4.
-64nonzero field controls pass inverse(xgcd versus exponentiation), Frobenius13,
  field laws and square-root verification.128lifted scalar-pair controls match
  existing prime-field Cantor arithmetic including identity/opposites/doubling.
- Two extension-order computations agree exactly: Newton power sums and
  det(I−companion(chi)^13)using fraction-free Bareiss with exact divisions.
  N=11177511291979626673464884111002741535461902356678686148134635206505957838143504882.
  Its3valuation is6; h=N/729is
  15332662951961079113120554336080578237944996374044836965891131970515717199099458.
  The public prime target order1028893dividesN.
- First registered sample (attempt0)has[N]R=0and[h]Rnonzero of order3.
  No3-power descent was needed; both projector images are nonzero. No retry,
  cap, selected replacement, unknown DLPscalar or point-count rerun.
- Every one of the54emitted kernel records is reduced/valid, killed by3,
  in its primary kernel, and Frobenius-stable. Closure and intersection pass.
  All729cross sums are distinct, valid and killed by3.
- Replay mode accepts the stored coordinates without rerunning point sampling.
  Four controls reject nonmonic input, duplicate, wrong-primary assignment,
  and embedded publicD1substituted for3torsion. Raw receipts in REPLAY.txt.

## Isotropy certificate and mathematical scope

Let f=T³+2T+2andg=T³+T²+2overF3. The pinned Weil polynomial shows these are
distinct irreducible dual factors of Frobenius. On J[3], Frobenius preserves
the3-Weil pairing because127=1mod3. There is no nonzero equivariant map from
the fprimary module to its dual (which has polynomialg), so the Weil pairing
restricts trivially to it. The same holds for g. Each actual subgroup has
27elements, hence dimension3and is maximal isotropic in the6dimensional
3torsion. This is an exact primary-module isotropy certificate; no numerical
Miller/Weil-pairing evaluation or independent validation signature is asserted.

The projectors E0=1+2pi+2pi²+pi⁴andE1=pi+pi²+2pi⁴were applied only after
3torsion was verified. They are not projections of arbitrary DLPpoints to
3torsion. Frobenius period13makes each nonzero kernel split into two13point
orbits, exchanged by negation. A single representative with its negative and
Frobenius action therefore describes each kernel; all27points are retained
for transparent replay. This compression is not reported as a time gain.

No quotient curve, Jacobian divisor map, base-field/Serre descent result or
complete destination solve was computed. The full10xgenus2/3/4objective
remains active, including separate genus2and4work. Any future performance
comparison must charge field search, order/cofactor work, kernel sampling,
projection, quotient construction, descent, target transport and verification.

## Source, replay and limitations

Protocol8a67a890db precedes scientific source0f197ceb2879and checkpoint
20261115T134431Z-0f197ceb2879. Source/dependency/fixture/binary hashes pinned.
Cargo.lock fixes num-bigint0.4.8and transitive versions; offline build succeeds.
Existing native prime-field library is read-only; new fixed-degree extension
and Cantor implementation are standalone. No scientific Python/cloud workers,
performance benchmark, prior closed record/default/leaderboard mutation.

```sh
export PATH=/Users/adamburan/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH
export CARGO_TARGET_DIR=/Users/adamburan/pq-work/hec-torsion13-target
sh research/hec_torsion13_20261115/build.sh
# The original generation is terminal: replay existing evidence first.
$CARGO_TARGET_DIR/release/torsion13 --verify \
  research/hec_torsion13_20261115/CERTIFICATE.txt \
  research/hec_kernel_admission_20261115/fixtures/select-g3-p127-f0.out
```

Generation command is the same binary followed by the fixture path alone;
it has a600secondowned watchdog, sixteen-point cap and fixed public seed.
Replay shares field/Cantor arithmetic with generation and is not independent
controlled replay or an external implementation. Standalone scientific gate
is not automatically executed by existing native CI. Reports/diagram visually
inspected before publication. No measured speedup inferred from process walls.

Errors: initial offline build had unused-import/method warnings, retained in
BUILD-FIRST.stderr. Removed those before source freeze; final build has no
warnings. Scientific construction/replay have empty stderr, no mismatch/cap.

Next gate: construct both corresponding quotients with explicit same-field
curve/divisor maps, and establish the complete-cost case against compact.
The source's2torsion branch field has degree12while these3kernels need degree13;
a naive combined-field implementation would use degree156. Orbit/etale-algebra
methods may avoid materializing it; do not infer such an optimization or a
whole-solve gain without a concrete algorithm and charged setup.

Report authoring: corrected a Typst literal-underscore parse error before the
successful one-page render; visually inspected the resulting PDF and SVG.
