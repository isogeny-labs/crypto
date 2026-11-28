#set page(paper:"a4",margin:20mm)
#set text(size:10.5pt)
= Targeted multiplicity profiles: density and exact row identity
Genus 2, 3 and 4 · 15 November 2026

== All bounded partition families screened
The mixed-multiplicity result left a specific question: can a targeted partition improve useful relation density? Enumerate all integer partitions at M=g+1 and M=g+4 on the same three nonbranch anchor panels. This gives 63 families and 112 admission policies. No favorable partition is chosen before the search finishes.

Every constructed pair passes Gaussian versus Hermite-CRT / Euclidean primitive reconstruction and norm checks. Every unique admitted split row passes its Jacobian identity. Independent final full / off ranks agree in all policies. All caps and noncompletions remain in SCREEN.txt.

#table(columns:(0.5fr,1.2fr,0.4fr,0.8fr,1fr,0.9fr),[Genus],[Best repeated pattern],[LP],[First rank],[Best distinct first],[10x density test],
[2],[4+1+1],[2],[634],[701],[Fails],
[3],[2+2+1+1+1],[3],[1412],[1417],[Fails],
[4],[2+2+2+2],[4],[1334],[1501],[Fails])

These are the most favorable training selections among 57 repeated families and six distinct controls. They are not independently validated improvements. Genus-3/4 two-LP families do not complete and cannot supply an infinite-speed denominator. The tenfold gate assumes equal construction cost solely as an optimistic density diagnostic; actual kernels and pole levels differ.

== Stronger exact relation identity
For nonbranch inputs, reduce the ORIGINAL weighted anchor sum by Cantor. The signed principal row obtained from its negative residual equals the primitive interpolation row, even when common factors invalidate some prescribed conditions.

If D is original input, D' is surviving input and L=D-D', the common polynomial's zero divisor is L+conjugate(L)+a vertical remainder. The original residual is conjugate(L)+primitive residual+that vertical remainder. Remove vertical pairs. Integral signed rows agree, including branch outputs. Nonrational factors cannot cancel against rational anchors, so smoothness decisions also agree.

Original and primitive residual divisors can differ; full signed-row equality is the correct invariant. Input branch multiplicities are outside this lemma. Proof and its limits are in ROW-EQUIVALENCE.md.

#table(columns:(1fr,1fr),[Separate replay check],[Result],[Original-weight Cantor comparisons],[3694 pass],[Exact split integral rows],[622 pass],[Nonsplit decisions],[3072 pass],[External common branch factor fixture],[Pass],[Original screen output],[Byte-identical replay])

== Decision
The sampled targeted families do not supply a tenfold density case. The exact identity gives a simpler constructor using weighted group reduction instead of Gaussian reconstruction, with the same rows. Its arithmetic cost remains to be evaluated against complete existing controls. Established divisor algebra is not a novelty or end-to-end speed claim; the tenfold objective remains open.

#pagebreak()
= Verification scope, selected families and custody

== What was actually searched
Reused curves / nonbranch anchors:

- Genus 2: p1009, c12, ell241259, H8; 14 families / 14 policies.
- Genus 3: p503, c26, ell29977319, H9; 20 families / 40 policies.
- Genus 4: p127, c15, ell134781029, H10; 29 families / 58 policies.

Fixed per-family caps: 256 profiles, 2048 constructed functions, 50000 draw attempts. Sampling is uniform within a fixed multiplicity partition before uniqueness rejection. Every support sign class fixes the first support positive. Common factors / degree drops are counted before canonical primitive function deduplication, followed by separate row deduplication. No elapsed-time measurement is reported.

#table(columns:(0.6fr,1.3fr,1fr,1fr,1fr),[Genus],[Training-selected pattern],[Constructed total],[Unique functions],[Full-stream admissions],
[2],[4+1+1],[672],[665],[328],
[3],[2+2+1+1+1],[2048],[2036],[305],
[4],[2+2+2+2],[1680],[1680],[83])

Totals include overcollection after first rank completion and are not first-completion cost estimates. Selection is deliberately favorable to the repeated-profile hypothesis; any positive result would need a frozen pattern and disjoint validation. First-completion counts come from incremental elimination, while independent calculations verify final ranks.

== Original-weight equivalence verification
The first 64 functions in each family are fixed for the independent Cantor checks, including nonsplit residuals and common-factor cases. A separate directed genus-2 curve over F101 uses A=x^3+x^2+x+60 and B=(x^2-1)(x^2-4)(x^2-9), with f=A^2-B. The common factor x lies at a branch point outside the six nonbranch inputs. The primitive pair A,1, constant residual, and exact signed row all pass.

No branch output columns are dropped, no modular scaling is used for the row comparison, and no expected scalar enters it. Shared field / curve primitives remain a verification dependency. The original screen reproduces byte-for-byte after filtering only the new verifier lines.

== Custody and prior art
Partition source / binary freeze 9dabb35afe93, S3 checkpoint 20261116T053157Z. Row-proof amendment f14b327342b2, checkpoint 20261116T054136Z. Verifier freeze 873fee89826a, checkpoint 20261116T054326Z. Both executions run once, exit zero / empty stderr. No scientific or build failure; raw caps and warnings retained.

D'Andrea / Krick / Szanto provides classical osculatory interpolation prior art. Bernatska / Kopeliovich 2020 explicitly relates interpolation, arbitrary divisor reduction and addition; its complex / non-special formulation is not a finite-field implementation certificate. Cantor's original AMS PDF returned 403. Local divisor proof, native group code and exact gates support this application. No mathematical novelty, timing gain, PR merge or goal completion is asserted.
