# Exact endomorphism action on the eight twist kernels

Filed before v11 data. Continue XP-20261027-twist-endo-kernel-symmetry with
the unchanged public E:y²=x³+x+15/F827, order823, G=(2,5),
ICV1 icv1-fp10-t5-192cb216. Twist T:Y²=X³+4X+120 has order833.
Inputs are the complete v8 seven-torsion basis/points/eight kernels and
v10 certified degree17 endomorphism alpha=(pi+1)/7. All input bytes are pinned.

Literature reviewed before implementation: repository RESEARCH_ISOGENY_LOOP_GLV.md
(including native-round erratum), atlas LEAD-STR-glv-gls-isogeny-loop-scalar-mult;
primary Andrew Sutherland MIT18.783 Fall2025 Lecture5 Theorems5.11/5.15 and
Remarks5.16–17, Lecture13 Theorems13.6/13.8 and proofs. Both primary PDFs
accessed successfully on2026-10-28:
https://math.mit.edu/classes/18.783/2025/LectureNotes5.pdf and
https://math.mit.edu/classes/18.783/2025/LectureNotes13.pdf.
Galois-stable kernels give rational isogenies; endomorphism action on torsion
is standard theory. No novelty or new degree1 automorphism claim.

Use v8 basis T1=(0,51), T2=(6,130); labels(i,j) mean i*T1+j*T2 overF7,
index7*i+j. Compare all49 points with the archived set. On every point compute
the actual conjugated map alphaT(X,Y)=(2*R(X/2),Y*S(X/2)); native evaluation
embeds into E(F827²), applies frozen Vélu and inverse-scale7, then unembeds.
Recover the two matrix columns by exhaustive49-point labeling, without source
group scalar logs. Verify all49 action images and all2401 addition pairs.

Compute matrix trace/determinant, characteristic polynomial, exact matrix
order, every point orbit, alpha^8 action and projective permutation. Lines are
T1+c*T2 for c0..6 followed by T2, exactly as v8. Match all eight image-kernel
root sets directly to the archived three-root sets. Record cycles, not just
orders. The filed predictions are point order48, one48-cycle on nonzero points
and one8-cycle on lines. A prediction mismatch is a valid recorded outcome,
not a reason to choose a new basis/map or omit records.

Embedded controls use the same matrix/order/orbit path: identity has point
and projective orders1; scalar2I has point order3/projective1; the companion
matrix C=[[0,4],[1,1]] has its finite behavior checked independently. Corrupt
one actual matrix coefficient and swap two proposed line images; both must
be detected. Require all generic bookkeeping and controls before admission.

Independent GP reconstructs degree17 normalized Vélu with ellisogeny from
the frozen kernel polynomial; uses the rational conjugated twist map and
elladd/ellmul to recover its own matrix, all49 labels and images, all2401
homomorphism checks, complete point/line cycles and all8 root-set matches.
It compares exact matrix/rows/cycles with native records. Strict receipt
admission requires process0, matching script/output hashes, sole final marker
and empty stderr (or the exact declared stack-size warning). GP exit0 alone
never admits a run. Preserve failed/capped attempts before any amendment.

One native producer and one GP execution, each300s/4GiB/CPU0/one thread on
the existing managed L0 host. No timing result or statistical inference.
The prior EKS CodeBuild/ECR access/image gap and standing authorized local
fallback persist; no new paid resource. Development tests use only synthetic
matrix controls, not the actual p827 action. Freeze source/binaries/protocol,
commit and successful Setec checkpoint gate precede the two scientific runs.
No retry or output overwrite; fail closed on existing paths. Cold runtime is
unmeasured; two300s hard caps bound the exact workload, not a speed estimate.

Formalize feasible finite companion-module identities and exact orbit claims
in Lean using kernel-checked arithmetic, with fresh transitive axiom audit.
The concrete curve/torsion-basis/map-to-matrix bridge remains separate. A
matrix order on T[7] is distinct from scalar353's order274 on E(F827).
Induced quotient-to-quotient maps and complete charged relation-solving costs
are follow-up requirements; no IC advantage or NIST applicability is inferred.
