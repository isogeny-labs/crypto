---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Diem's size hypothesis and an actual small-field control

Lean checks the arithmetic last step of Diem2011 Proposition4.11:
for natural n,q,N, the following implication holds. The geometric
estimate is an explicit premise. [pcc:unverified]

$$n\ge2,\quad 2^{7n}\le q,\quad
N\ge q+1-n2^{n+2}(\sqrt q+1)
\quad\Longrightarrow\quad q+1\le2N.$$

It also constructs an actual nonsingular F4 curve with one rational point
and an actual point/chart factor base of cardinality0. This refutes the
unrestricted strengthening; the source's size/admissibility hypotheses
remain explicit. [pcc:unverified]

![Checked arithmetic and actual model, with distinct scopes. [pcc:unverified]](proof.pdf){width=96%}

\newpage

# What is proved, and what remains qualified

For n≥2, $n\le2^{n-1}$ gives $(n2^{n+4})^2\le2^{4n+6}\le2^{7n}\le q$.
With $s=\sqrt q$ and $a=n2^{n+2}$, we have $s\ge4a$ and $s\ge1$.
Thus $a(s+1)\le2as\le q/2$, giving the conditional half-size bound.
An arithmetic-only control n2/q4/N0 directly disproves inference from
the geometric estimate alone after omitting the size hypothesis.
That N0 is not asserted to be an actual curve count. [pcc:unverified]

The actual model uses $F=\mathbf F_2[t]/(t^2+t+1)$ with $\alpha=[t]$.
Lean checks irreducibility, degree2 and cardinality4. The curve
$y^2+y=x^3+\alpha$ has discriminant1. Fourth powers equal the element,
so $y^2+y$ and $x^3$ are fixed by squaring; the curve equation would
force $\alpha^2=\alpha$, a contradiction. Only O remains: actual point
count1. The homography $\varphi=\alpha+1/x$ has a checked inverse and
sends O to alpha outside the F2 projective chart, so the actual factor
base is empty. Its half-size inequality $3\le2\cdot0$ is false.
[pcc:unverified]

Main binding: `DiemSmallFieldAudit.diem_domain_audit_certificate`.
Proof map: `../../DIEM_DOMAIN.md`. Exact hypotheses, source hashes,
gate and actual packet standing are recorded separately in
`../../diem-domain-sources.json` and the verification record.
Whole census gate535passes, zero failures/zero trusted-native exceptions.
Local source commit `6e8973a3` precedes packet `PCC-20261110-dda065`;
actual local verdict and signed standing are recorded separately.
Lean4.35.0/Mathlib56a00585; standard axioms propext, Classical.choice,
Quot.sound only. No hole, custom axiom, native-decision exception, elaboration
option or linter waiver. Failed attempts are retained; fresh lean4checker
is unavailable. [pcc:unverified]

Original Diem2011 Condition2.7 and Props4.9–4.11 were read; Prop4.11
and proof visually checked on PDFpage22/printed95. DOI
10.1112/S0010437X10005075, PDF SHA25674902b58…934a58a.
Reviewed Mathlib quotient-field/power-basis/cardinality, characteristic
transport, real sqrt and actual elliptic point constructors/coordinates.
KB/Conductor unavailable; bounded primary/repository/Mathlib fallback,
no novelty or blanket source promotion. [pcc:unverified]

The source assumes n≥2 and $\log_2q\ge7n$, as well as Condition2.7.
The actual control has q2/n2 and violates that size condition. Scheme
cover degree/ramification, admissibility identification and the geometric
Hasse–Weil/genus estimate are not formalized here. The qualified source
proposition is not refuted. No native experiment, timing or paid compute.
Signed standing remains unverified pending a trusted signed verdict.
[pcc:unverified]
