---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# An actual 160-bit point-order certificate

Lean proves primality and 160-bit bounds for the explicit modulus p below,
nonsingularity of the corrected curve, exact prime order p of an actual
Mathlib point G, and the fixed known-answer relation Q=dG. The proof uses
35 Lucas nodes and 448 candidate point nodes. Supplied coordinates and slopes
are independently checked by the kernel and connected to actual point addition.
[pcc:unverified]

$$E:\ y^2=x^3+ax+b,\qquad 2^{159}\le p<2^{160},\qquad
\operatorname{ord}(G)=p,\quad Q=[d]G.$$

![Mathematical witnesses, actual point proofs and explicit scope. [pcc:unverified]](proof.pdf){width=98%}

The literal curve from the uncorrected author preprint has a different
j-invariant from the stated construction. The author's published erratum
already corrects the missing leading digit and replaces its points. Lean
reproduces that known discrepancy and checks j=-32768 on the corrected curve;
no new error or refutation of the corrected paper is claimed.
[pcc:unverified]

\newpage

# Fixed parameters, proof map and next obligation

```text
p = 730750818665451459112596905638433048232067471723
a = 425706413842211054102700238164133538302169176474
b = 203362936548826936673264444982866339953265530166
G = (1, 310536468939899693718962354338996655381367569020)
Q = (259479713158273757050367190534858072594223227627,
     39414799304662693276020416800594933604368475811)
d = 210393287966660756596132643172172640405085536179
```

G and Q are fresh known-answer points on the corrected curve, not the
example points printed in either paper version. The actual equality pG=O
and G≠O give exact order p because p is prime. This component does not prove
the whole curve point count is p, execute Smart's attack, prove its p-adic
bridge, or measure security cost. A subsequent concrete attack certificate
must be bound to this actual point relation. [pcc:unverified]

| Module | Checked obligation |
|:--|:--|
| AnomalousPrime | Recursive Lucas primality certificates and bit bounds |
| CurveData | Actual discriminants, j-invariants and known preprint control |
| AffineCertificates | Actual secant/tangent group addition from checked slopes |
| PointTrace | Every numerical point/edge and the scalar chains for pG and dG |

Main binding: `AnomalousPointTraceAudit.large_order_audit_certificate`.
Full gate: 59 public theorem declarations, zero failing and zero trusted-native
exceptions; private generated certificates are compiled and checked transitively.
Lean 4.35.0 / Mathlib 56a00585; only propext, Classical.choice, Quot.sound.
No holes, custom axioms, native decision exceptions, linter waivers or added
elaboration option. Failed sources/logs are retained losslessly; fresh
lean4checker is unavailable. [pcc:unverified]

Source commit `c10e04d5` precedes packet `PCC-20261110-90623b`. Actual local
verification and signed standing are recorded separately. Proof map and
provenance: `../../README.md`, `../../point-sources.json`,
`../../point-verification.json`. C++/GMP/FLINT produces untrusted proof inputs;
the source writer performs no curve arithmetic. No timing or population
inference, native attack run or paid compute in this component.
[pcc:unverified]

Sources read: original Smart report HPL-97-128 (1997); original author preprint
and two-page erratum for Leprévost et al., IPL 93(5), 225–230 (2005), DOI
10.1016/j.ipl.2004.11.008. Original example and both erratum pages were visually
inspected. NIST SP 800-186 explicitly excludes anomalous curves and requires
large embedding degree; its screened recommendation is not refuted.
KB/Conductor unavailable; primary/repository/Mathlib fallback used. The generic
Smart proof and matched SafeCurves benchmark remain open. [pcc:unverified]
