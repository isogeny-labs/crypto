# Actual 160-bit point order and a checked source correction

Lean proves that the prime below has 160 bits, and that an actual Mathlib
point on the corrected curve has exact order p. It also proves the fixed
known-answer relation Q=dG. The arithmetic uses 35 Lucas primality nodes and
448 candidate point nodes; every accepted coordinate, slope and addition is
checked by the Lean kernel. [pcc:unverified]

```text
p = 730750818665451459112596905638433048232067471723
a = 425706413842211054102700238164133538302169176474
b = 203362936548826936673264444982866339953265530166
G = (1, 310536468939899693718962354338996655381367569020)
Q = (259479713158273757050367190534858072594223227627,
     39414799304662693276020416800594933604368475811)
d = 210393287966660756596132643172172640405085536179
```

Here E is the actual nonsingular curve y²=x³+ax+b over ZMod p. G and Q are
fresh known-answer points on the corrected curve, not the points printed in
either version of the paper. Exact prime order follows from pG=O and G≠O.
This component does not prove that the entire curve point count is p, execute
Smart's attack, formalize its p-adic bridge, or establish a security cost.
Those remain separate obligations for original item 11. [pcc:unverified]

## The source discrepancy is already corrected

The original author preprint's Example 1 omits the leading digit of a.
Lean checks its actual discriminant and j-invariant and directly disproves
j=-32768 for that literal curve. It checks j=-32768 for the corrected curve.
The author's [erratum](https://www.monnerat.info/publications/anomalous_erratum.pdf)
explicitly fixes the coefficient and replaces the example points. Both erratum
pages and the original example page were read and visually inspected.
This reproduces a known correction; it is not a new error claim or a
refutation of the corrected construction. [pcc:unverified]

Untrusted GMP computations also proposed counterexamples to the old example's
point-order and scalar assertions. Those particular proposals have not been
proved in Lean and are not promoted here. The formal source discrepancy is
the j-invariant statement only. [pcc:unverified]

## Proof map

| File / declaration | Exact obligation |
|:--|:--|
| `AnomalousPrime.p_prime` | Actual 160-bit modulus is prime, using 35 explicit Lucas certificates |
| `AnomalousPrime.p_bit_bounds` | 2^159≤p<2^160 |
| `CurveData.printed_discriminant_ne_zero` / `formula_discriminant_ne_zero` | Both literal models are nonsingular |
| `CurveData.printed_j` / `printed_j_not_claimed` / `formula_j` | Actual Mathlib invariants reproduce the known preprint/correction distinction |
| `AffineCertificates.secant_add` / `tangent_double` | Supplied slopes are checked by multiplication and connected to actual Mathlib point addition |
| `PointTrace` private certificates | Every candidate point is on the curve; every edge and scalar relation is proved |
| `PointTrace.G_order` | Actual G has exact prime order p |
| `PointTrace.known_answer_relation` | Actual Q=dG |
| `PointTrace.large_order_audit_certificate` | Combined prime, bit bounds, nonsingularity, exact order, known answer and source-invariant control |

Names use the namespaces ending in `Audit` shown in the source. Main binding:
`AnomalousPointTraceAudit.large_order_audit_certificate`. The complete gate
passes 59 public theorem declarations, zero failing and zero trusted-native
exceptions. Private generated certificates are checked during compilation and
transitively through the final public theorems. The count is not a claim of
59 new research results. Lean 4.35.0 / Mathlib 56a00585 are pinned.
Only propext, Classical.choice and Quot.sound are used. [pcc:unverified]

The C++/GMP/FLINT constructors produce untrusted mathematical witnesses. The
JavaScript writer formats strings as proof obligations and performs no curve
arithmetic. Reproducing the Lean proof needs only the committed literal source,
not trust in a constructor or its execution. This is theorem construction,
not a timing, population or native attack experiment. No paid compute.
Failed attempts are preserved losslessly in the construction archive.
[pcc:unverified]

Source commit `c10e04d5` precedes theorem packet `PCC-20261110-90623b`.
Actual local verification and signed standing are recorded separately in
`point-verification.json`; `point-sources.json` remains the frozen preissue
plan. Fresh lean4checker is unavailable. See the
[inspected PDF report](reports/point-20261110/report.pdf). [pcc:unverified]

## Sources and reproduction

Read the original [author preprint](https://www.monnerat.info/publications/anomalous.pdf),
its [erratum](https://www.monnerat.info/publications/anomalous_erratum.pdf),
Smart's [original report](https://shiftleft.com/mirrors/www.hpl.hp.com/techreports/97/HPL-97-128.pdf),
and NIST SP 800-186 section 3.1.2 / Appendix C.1. The corrected paper is
Leprévost–Monnerat–Varrette–Vaudenay, Information Processing Letters 93(5),
225–230 (2005), DOI 10.1016/j.ipl.2004.11.008. Hashes and exact read scopes
are in the source plan. NIST explicitly excludes anomalous curves and requires
large embedding degree; its screened recommendation is not refuted.
KB/Conductor unavailable; primary/repository/Mathlib fallback used, no novelty
claim. [pcc:unverified]

```sh
cd research/anomalous_curve_audit_20261110/lean
lake build
cd ../../..
tools/lean/proof-gate.sh research/anomalous_curve_audit_20261110/lean
```

The next component must bind a concrete finite-precision Smart computation to
this actual point relation and use a separate protocol frozen before native
attack output. The matched SafeCurves benchmark remains open.
[pcc:unverified]
