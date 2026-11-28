# Target-seeded anchor relations and signed-gain forest (2026-11-14)

The goal remains a mathematical >=10x complete-cost improvement for genus2+. No current result
meets it. Principal-row pilot: all330 solves correct but unrestricted rows lose at every size.
Table pipeline: all660 solves correct and220 exact stream/pivot gates pass; best-native total
gain1.15..1.97x. These observations motivate eliminating generic group additions and a large
linear-algebra core together. This protocol precedes the new implementation and timing.

## Construction and mathematical boundary

Retain the exact principal identity in hec_relation_math_20261114/DERIVATION.md. First descend
D1 and D2 into full-base place sums; every place appearing in either decomposition becomes an
anchor. Fill to H distinct anchors using the seed, where H is max(target support, g+1,
ceil(2 p^(1/3))) for g2, ceil(p^(2/3)) for g3, ceil(p^(3/4)) for g4, clamped to the catalogue.
Choose g+1 distinct anchors and interpolate h of degree<=g. Split Q=(f-h^2)/A of degree g with
the exact tabulated oracle. Keep only relations with at most2 distinct off-anchor places.

Each relation has <=2g+1 terms, including at most two off-anchor logarithms. Weighted union-find
represents L_u=alpha_u L_root+beta_u dot H. For an incoming cu Lu+cv Lv+h dot H=0, substitute
both endpoint expressions. Different roots are joined after dividing by a nonzero coefficient.
For a cycle with combined root coefficient zero, h is a pure anchor relation. A nonzero cycle
coefficient instead determines its root in terms of anchors; attach it to the distinguished
zero vertex. This is coefficient-aware elimination, not unsigned cycle counting.

All target rows are on anchors, so only the anchor logarithms and k remain in final elimination.
The number of possible interpolants is at most2^g binomial(H,g+1), after quotienting global
negation. Distinct anchor tuples can still yield the same interpolant; deduplicate normalized
principal rows too. Finite supply and rank failure are real failure modes, not hidden retries.

For g2, a heuristic random signed graph suggests a cycle threshold around half the off-anchor
catalogue size; that is a cost hypothesis only. The geometric outputs need not be independent
random edges. At g3/4 the requirement of <=2 off-anchor places can erase any relation gain.
The construction is related to geometric index calculus and large-prime forests, already cited
in the two preceding studies. No novelty is established by this combination.

## Correctness gates

Check every accepted principal row by exact Cantor group addition. Obtain an ell-torsion
projection certificate for the full catalogue by finding h prime to ell, within conservative
integer Hasse bounds, such that (h ell) kills every catalogue place. Check hF_j is killed by ell.
Then check every emitted pure-anchor row on projected places. This gate-only computation is
counted and logged separately, not charged selectively to one timed arm.

Check weighted-forest identities against generic elimination on planted modular systems,
including balanced/unbalanced cycles, signed coefficients and singleton partials. Descents,
normalization and final k D1=D2 verification remain mandatory. Certify ell^2 exceeds Hasse upper
bound before normalizing logs. No knowledge of planted k enters the candidate algorithm.

## Pilot and caps

Same eleven instances, five seeds20260916..20260920. Compare to strongest observed cold native
AND table configurations from hec_table_oracle_20261114/run/pipeline_summary.json, with matched
input and target. No precomputation is shared between solves. Include lookup construction,
catalogue, inverse cache, descents, row dedup, every forest step and solve retry in total time.
All arms must verify, and count rank/capped failures explicitly. Cap1million tuple draws,
16 solve attempts and10000 steps per descent. H growth is not allowed in this frozen pilot.
The pilot is exploratory; >=10x still requires frozen arms, held-out curves, scaling and replay.

Relevant sources already checked: GTTD three-author primary draft (2005), section3.1, signed
cycles; Magma handbook on principal-divisor/geometric IC; existing table-oracle Cardano/Ferrari
proof and gates. No new family or special-subfield assumption is used.

## Signed-orbit amendment (after first pilot, before new implementation/timing)

First anchor pilot:55 gate and55 timed answers all check, no caps,15890 principal identities and
1638 projected anchor identities. Total gain against prior table baseline mostly<=1.2x. Therefore
retain that failure; optimize the measured interpolation and forest costs, not just the oracle.

For a fixed anchor tuple let A=product(x-x_i), w_i=y_i/A'(x_i), h_i=w_i A/(x-x_i).
For i!=j, h_i h_j/A=w_i w_j product_{k!=i,j}(x-x_k), a polynomial of degree<=g-1.
If h_sigma=sum sigma_i h_i, Q_sigma=(f-h_sigma^2)/A, changing only sign j gives

    Q_new = Q_old +4 sum_{i!=j} sigma_i sigma_j (h_i h_j/A),
    h_new = h_old -2 sigma_j h_j.

Proof: expand the difference of squares; the h_j^2 terms cancel because sigma_j^2=1.
Thus all2^g sign classes (first sign fixed, global negation identified) share interpolation and
one norm division. Gray-code order changes one sign per step. This is an exact algebraic reuse
identity, not a claim of new mathematics or of tenfold complete speedup.

Precompute inverse anchor differences with one batch inversion, O(H^2) field operations/words.
For g2 enumerate/shuffle unordered triples without replacement (O(H^3)=O(p) at the frozen H);
for g3/4 sample distinct tuples, consuming all sign classes in each. Do not normalize/deduplicate
raw rows: repeated group relations yield zero/duplicate anchor equations and do not create rank.
Deduplicate emitted anchor rows. Record this change in metadata. Finite family limits remain.

Use a flat dense potential array for the signed forest, with reusable buffers and path
compression. Its O(mH) storage is explicit, capped at16million u64 words. Coefficients remain
mod ell, including non-unit cycle gains; +/-1 uses addition/subtraction. The small anchor solve
uses incremental dense echelon rows, checking target determination as rows arrive; no matrix
rebuild or arbitrary oversampling is charged away. This replaces the16-rebuild cap; the1million
candidate cap and10000-step descent cap remain.

Catalogue evaluation uses finite differences (2g+1 additions per x) and the square-root table;
polynomial evaluations for the initial difference table are charged. A new interleaved baseline
must receive the same catalogue option in a subsequent confirmation, so catalogue-only savings
cannot be attributed uniquely to the signed-orbit mechanism.

Gates: every Gray-code residual/h must equal full signed interpolation and norm division on
random tuples. Same principal/projected checks as before, arbitrary-coefficient forest fixture,
full target verification. Timing remains exploratory against all strongest existing native and
table controls, with all cold setup and memory reported. >=10x and novelty remain unestablished.

### Integration pilot amendment, before timing

The implemented variant includes Gray reuse, dense forest, incremental echelon, one batch
inversion and shuffled genus2 triples without replacement. Genus3/4 sample distinct unsigned
tuples. Full-p inverse cache and raw-row dedup are absent in this variant. Pure rows are deduped.
Retain the original Horner catalogue in BOTH arms: finite-difference evaluation is deferred.
The reported storage includes tables, anchor inverse differences, triple capacity and dense
forest arrays; it is not peak RSS and excludes Rust allocator overhead and variable row sets.

orbit_bench.rs first runs all55 gate cases, then110 cold solves interleaving candidate and the
strongest previous table configuration, frozen per row in the harness before this run. Candidate
must finish every case and satisfy every group identity before timing. Five seeds, L0 local
host, same inputs and known-answer targets as prior pilot. Report every size; no best-seed
selection, no pooled genus ratio. Earlier native controls remain descriptive cross-run evidence,
not a substitute for this paired table control. This pilot cannot establish novelty or the
held-out/scaling requirement of the10x goal. Explicitly retain losses and capped outcomes.
