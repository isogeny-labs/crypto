---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Composite orders: checking an alleged exception

The prime-field modular embedding-order conclusion extends to every integer
$r>2$ dividing the actual curve group order, provided the source
supersingular trace condition and Hasse bound are retained. Primality of
$r$ is unnecessary in this arithmetic step. Hasse itself is an explicit
premise; no general geometric classification is proved here.
[pcc:unverified]

For prime $p\ge5$, the source condition gives $p\mid t$. The premise
$t^2\le4p$ forces $t=0$, so $\#E(\mathbf F_p)=p+1$. Consequently
$p=-1\pmod r$. Its square is 1; if it were 1 already, then $r\mid2$,
contradicting $r>2$. Thus its multiplicative order is 2, including composite
$r$. The previous $r=2$ counterexample remains a separate boundary.
[pcc:unverified]

![Checked arithmetic dependencies and actual composite positive control.](proof.pdf){width=100%}

The actual smooth source curve $E/\mathbf F_7:y^2=x^3+x$ has 8 points and
satisfies the source trace definition. The new actual point $P=(3,3)$ obeys
$8P=0$ and $4P\ne0$; Lean concludes $\operatorname{addOrderOf}(P)=8$.
The modular embedding order of 7 modulo 8 is 2. These are actual Mathlib points,
not an asserted cyclic operation table. [pcc:unverified]

Primary source: original [FST2010](https://doi.org/10.1007/s00145-009-9048-z),
p230 Definition 2.1/Remark 2.2; integer subgroup order is allowed there.
Lean source: ../../lean/CompositeTorsion.lean. No general cyclotomic-field
or full geometric-torsion equivalence, binary/ternary census, or
all-fourteen completion is inferred. No native population, timing or paid
compute was run. [pcc:unverified]

Whole census axiom gate:205 declarations,0 failing,0 trusted-native exceptions,
including generated declarations. Packet PCC-20261109-10789d binds the
universal conditional theorem and unconditional actual control; producer
standing remains unsigned. [pcc:unverified]
