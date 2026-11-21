#set page(paper:"a4",margin:17mm)
#set text(font:"Libertinus Serif",size:10pt)
#set par(justify:false)
#set document(title:"Primitive rational relations and complete single-target integration")
= Primitive rational relations
*Normalization, paired rank counts and single-target integration · 15 November 2026*

Removing vertical common factors recovers useful lower-degree relations from
signed rational functions previously rejected. It reduces required candidate
visits in 11 of 36 rank-screen arms, with a largest count ratio of 1.40549.
The complete normalized solver also recovers all 12 frozen single-target
scalars with full group checks. Runtime performance remains unmeasured.

#image("PRIMITIVE-RANK.svg",width:100%)

*What the paired count comparison covers.* Same curves, anchor sets, tuple
streams, seeds and stop rules as the preceding rank screen. Default rejection
reproduces the original JSONL byte-for-byte. Every anchor rank is independently
rechecked as $op("rank")(R)-op("rank")(R_("off"))$. Both modes retain 23 rank attainments,
12 family exhaustions and one tuple cap. Improved arms stop at earlier stream
prefixes, so aggregate admissions are not a paired yield comparison.

*Most visible improvement.* At genus 4, $p=127$, H16,d3 visits change from
8759 to 6232. This is a candidate-count reduction; all generation, normalization,
orientation, setup, target handling and recombination costs still have to be paid.

*Sources and reproduction.* Frozen protocols: #raw("PRIMITIVE-PROTOCOL.md")
and #raw("RATIONAL-SOLVE-PROTOCOL.md"). Raw counts and all 36 paired arms:
#raw("PRIMITIVE-RANK.jsonl"), #raw("PRIMITIVE-SUMMARY.json"). Native replay:
#raw("sh primitive_gate.sh /tmp/hec-primitive-replay").

#pagebreak()
= Correct divisor accounting and target cost
Write $h=gcd(a,b)$, $a'=a/h$, $b'=b/h$. Then
$a^2-f b^2=h^2 (a'^2-f b'^2)$. Only anchors with $h(x_i)!=0$ remain guaranteed
zeros of the reduced function. Remove every canceled anchor condition and
recompute the residual from the primitive norm. A zero of the reduced function
at a removed anchor is recovered through that residual. Never assume rational
interpolation survives a simultaneous numerator/denominator zero.

If $t=deg h$ and $k$ anchors are removed, the new residual has degree at most
$g-2t+k <= g-t$, since distinct anchors give $k<=t$. Orient split roots with
$y=a'/b'$, merge signed multiplicities, and check the resulting principal
relation in the Jacobian. This is established rational-function reduction
with explicit anchor bookkeeping, not a novelty claim.

*Algebra gate.* All 9408 generated curve cases pass normalization checks;
1370 primitive split cases pass the complete group identity, including 262
previously rejected common-factor cases. Synthetic controls cover branch
anchors, repeated factors at and away from anchors, constant primitive
functions and zero pairs. The first synthetic run used the wrong curve sign;
exact division rejected it. Both the failed output and correction amendment
are retained. The corrected seven-test gate passes.

*Identity receipt.* #raw("IDC1hf2c6f973357f42f7") certifies the scalar norm
factorization over the repository certificate field at 32 derived points,
plus 16 further producer recheck points. It does not prove the divisor
bookkeeping. The formal-proof pointer is null; the record is producer-issued.

*Complete solver.* The integration includes cold tables/catalogue, descent of
both supplied divisors, target-seeded anchors, complete Walsh batches including
unused tails, primitive normalization, root orientation, deduplication,
coefficient-aware forest, incremental scalar solve and $k D_1=D_2$ verification.
All 12 cases pass every admitted raw/projected group check. The known scalar
appears only in the test harness, never as solver input.

*Effective anchors matter.* At g3,p61, one requested-H9 case needs H10 after
descent. At g4,p31, requested H10 becomes H14 and H13. Requested H28 is clamped
to all 16 available factor-base places. These sizes are explicitly reported.
The first-H untargeted rank screen alone cannot price a single-target solve.

*Next gate.* Freeze and run an identical-target, fully cold timing comparison
of this complete solver. Diagnostic test clocks are not performance evidence.
Do not multiply the count ratio by earlier timing ratios. Existing timing
graphs are unchanged because no compatible runtime observation was added.

*Primary source.* D'Andrea, Krick, Szanto, Proposition 2.1 and Theorems 2.2/2.3:
#link("https://arxiv.org/html/1211.6895v3")[arXiv:1211.6895]. KB source IDs and
revision are in the protocol. All scientific execution and report generation
use native Rust, jq and Typst. Study log: #raw("PRIMITIVE-SESSION.md").
