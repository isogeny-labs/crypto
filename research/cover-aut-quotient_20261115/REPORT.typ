#set page(margin: 1.8cm)

#set text(size: 9.5pt)

= Automorphism quotient of the cover factor base

All numbers are \[pcc:unverified\]; local development check.

#image("graph.svg", width: 100%)



#table(columns: 6, inset: 4pt, [p], [cells], [geo-mean S], [S/negrho w=0], [w=0.1], [w=1],
[1301], [3], [0.7371], [0.791], [0.822], [0.838],
[1451], [3], [0.6148], [0.660], [0.686], [0.699],
[1601], [3], [0.5111], [0.548], [0.570], [0.581],
)
#table(columns: 4, inset: 4pt, [p], [S/rho w=0 \[95%\]], [w=0.1], [w=1],
[1301], [0.794 \[0.719, 0.883\]], [0.826 \[0.747, 0.919\]], [0.842 \[0.763, 0.936\]],
[1451], [0.663 \[0.605, 0.729\]], [0.689 \[0.630, 0.758\]], [0.702 \[0.642, 0.774\]],
[1601], [0.551 \[0.504, 0.606\]], [0.573 \[0.524, 0.630\]], [0.584 \[0.535, 0.642\]],
)

All numbers are [pcc:unverified]; this is a local development check at small parameters, not a lab benchmark.

No improvement is claimed. Repo HEAD 3c24520f7cdc; bin `src/bin/cover\_aut\_quotient.rs`, helpers in `src/`.



== Question



The lead proposed quotienting the Nagao factor base of the genus-3 cover H/F\_(p^2) of E(F\_(p^6)) by an

automorphism group G of order n, to divide unknowns by |G|, relations by |G| and linear algebra by |G|^2,

assuming (besides the hyperelliptic involution iota) an automorphism of order 3 or 6 coming from

Gal(F\_(p^6)/F\_p). Is there such a group on the instances the Nagao arm solves, and what does quotienting give?



== Method



1. Census. For each instance `generate\_spec(p, seed)` (p in 503, 1009, 1301, 1451, 1601, seeds 1 to 3) take

   h = F\*N (monic septic, so infinity is a branch point) and search exhaustively every Moebius map over F\_q

   (q = p^2) that sends the branch set of y^2 = h onto that of y^2 = h (linear case) or of y^2 = h^tau

   (semilinear case, tau the p-Frobenius). Affine candidates are all q-1 values of s (the translation is forced by the x^6

   coefficient); pole candidates x -> u + k/(x-v) are enumerated for the rational roots u, v and all k.

   Completeness argument: a map usable on the factor base must be defined over F\_q, so it permutes the F\_q-rational

   branch points, so it is affine or of pole type (PLAN.md). kappa (the scalar in H(mu x)(cx+d)^8 = kappa h(x)) is recorded

   with its squareness. iota acts on Phi(G), Phi(Q) as lambda = l-1: verified by neg(Phi G) = [l-1] Phi G.

2. Quotient layer. `orbit.rs` builds orbits of the signed factor base under generators (permutation, scalar lambda),

   with log P\_s = coef[s] log rep, and rewrites a relation through it. Unit-tested on a synthetic group of order 6

   (iota and a cube root of unity): unknowns divide by exactly 6 and the hidden logs are recovered.

3. Paired Nagao arm. `paired` is the library's relation collection (same residual stream, F4 with degree 24, factor-base

   line search with 65536 lines, JV\_FB\_LINE config of research/jv\_cover\_fbline\_20261102) with two row builders

   fed by the same decompositions: A, the library's rows (iota already folded in: one unknown per class, coefficient 1 or l-1);

   Q, rows built through the orbit layer from G = {1, iota}. LA is Wiedemann on the square core, as in the library.

   S is total F\_p multiplications / (c\_e sqrt l) (group-addition units). S/negation-rho divides by the rescore

   protocol's pooled reference 0.9322 (U1), and priced at w = 0, 0.1, 1 per native operation:

   (S + w\*native/sqrt l) / (0.9322 + w\*15.81).



== Results against the preregistered predictions



\*\*P0 (structure): confirmed.\*\* 15 instances, all with the identity as the only linear map and no semilinear map

(`all\_trivial = True`), deg h = 7 on all. iota acts as lambda = l-1 on Phi(G) and Phi(Q) on all. Controls:

planted h = x^7 + 5x at p = 1301 returned 12 maps, not the 6 preregistered:

the reduced automorphism group of y^2 = x(x^6 + c) is dihedral of order 12 (6 affine plus 6 pole-type x -> c'/x), so

the detector found the whole group and my "exactly 6" expectation was wrong; 4 of the 12 preserve P^1(F\_p).

A random septic: identity only. Random alpha at p = 101: 4000 alpha, 0 with a nontrivial linear

automorphism (Wilson 95% upper bound about 0.1%); 3 had a semilinear map (small-field coincidences at q = 10201, none at the 15 instances).

Kummer control (alpha^3 in F\_q, p = 103 = 1 mod 3, 300 alpha): all 300 have a cyclic group of order 3 (x -> omega x), square kappa,

preserving the F\_p line: the detector sees the effect when it exists, and a G of order 6 does exist on that subclass.

That subclass is not an instance class: alpha^3 in F\_q makes E the base change of a curve over F\_q, so \#E(F\_(q^3)) is

divisible by \#E'(F\_q) > 4 for q large and is never 4 times a prime (numerically checked: a2^2/a4 = (1+omega)^2/omega, so j(E) lies in F\_p, on 300 of 300 Kummer alpha at p = 103; a j in F\_p curve over F\_(p^6) is a twist of a base change of a curve over F\_p, hence its order is divisible by an F\_p curve order of size about p, not 4 times a prime).

So the premise "an automorphism of order n from the field extension" fails for the weak class used by the Nagao arm:

sigma sends E to E^sigma, a different curve. G = {1, iota}, |G| = 2.



\*\*P1 (unknowns): confirmed, trivially.\*\* Q rows equal A rows on every cell (`True`), identical relation digest, same unknowns

(B+1 classes, core 715 at p = 1301 s1). The orbit layer maps 2B signed ids to B orbits (reduction 2.000), which is exactly what the

repo already does. The lead's "unknowns shrink by |G| for |G| >= 6" is untestable on this class.



\*\*P2/P3 (relation phase, overall S): the quotient arm equals the baseline; the arm-N prediction is refuted.\*\*

S(Q)/S(A) = 1.000 on every cell, relation-phase ratio 1.000 (identical rows). The preregistered unquotiented arm N (signed

unknowns, no iota information) could not be measured as planned: at p = 503 it ran 45 min with no result and was stopped

(wall-clock cut, see Limitations); the p = 251 re-run never determined d, through 922 relations. Dumping the first 420 relations

at p = 251 s1 and doing exact linear algebra over F\_l shows why: the system has rank 306 of 307

with d the free column, and the kernel has constant symmetric part and an antisymmetric part tied to the class logs. With the

B free identification rows x\_(i,+) + x\_(i,-) = 0 added, d is determined at 160 relations

(grid step 10) against 154 for arm A. So the iota quotient is not a 2x saving of relations; it is a necessary relation,

free to add, and an unquotiented system with it needs the same B+1 decomposition relations and only a larger LA dimension

(2B+1 against B+1, a factor 4 in a dense or Wiedemann n^2 cost, but LA is under 0.1 percent of S here: S\_la 0.00058 against S 0.729).

This also corrects the lead's accounting: relations needed scale with the number of orbit unknowns, so an extra automorphism group

of order n would cut the relations needed by about n relative to the iota-only baseline, and nothing else; it is a model for a class that has no instances.



\*\*P2 arm-N rank analysis on three instances.\*\* The p = 251 analysis was extended from seed 1 to seeds 1, 2, 3 (first 420 relations of each, exact linear algebra over F\_l, grid step 10; `rank\_n\_multi.py`):





On all three instances plain N has rank 2B (one less than its 2B+1 unknowns, d free, never determined), and N plus the free iota rows determines d at

exactly the same prefix as the orbit-quotient system. So the earlier n = 1 finding holds on n = 3: iota is a necessary free relation, not a 2x saving. (Arm N as a timed run was still not measured.)



\*\*P4 (distance to rho).\*\* Cells completed: p=1301 s1, p=1301 s2, p=1301 s3, p=1451 s1, p=1451 s2, p=1451 s3, p=1601 s1, p=1601 s2, p=1601 s3 (seed 3 added in this revision). Geometric mean over seeds of S/negation-rho, reference = the rescore's pooled 0.9322 / 15.81:





At the largest prime measured (1601) the point estimate is below 1 at all three prices (0.548, 0.570, 0.581): P4 holds on the point estimate.

My prediction that 1301 and 1451 would be above 1 is refuted: I took the rescore's cover records (1.405, 1.160, 0.870) to be this

configuration; they are the plain model (results/inputs/cover/dlp\_p1301\_s1.json has no fb\_line, base 634, S 1.494), and the factor-base-line config used here (base 728 at the same instance, S 0.729) is the 1.71x one of jv\_cover\_fbline\_20261102, so

S/rho is lower. Cover native operations are `f4\_native\_ops`, a lower bound (F\_p additions and memory not emitted), so an "below" verdict could flip under

complete counts. These are measured distances, not an improvement claim of this lead: the quotient changed nothing.



Matched rho on the same instances (added in this revision, `rho\_matched.py`, `rho\_matched.json`). The rescore's 12 negation-map rho runs per instance (`research/jv\_cover\_negrho\_rescore\_20261101/results/new/rhostrong\_p\*\_s\*\_run\*.json`, same E(F\_p) instances: l and c\_add\_e asserted equal to this study's records in all 9 cells) give per-instance and pooled references, priced at w = 0, 0.1, 1 with the rho native count (canon\_cmp + hash + dp\_check + scalar\_ops + table\_ops + cycle\_cmp) per sqrt l. Pooled reference: 108 runs, 0.9278 steps/sqrt l, native 15.73. Bootstrap (20000) over rho runs and over the 3 instances; with 3 instances the cover-side spread is under-covered:





Per instance against that instance's own 12 rho runs (interval over those 12 runs only; w = 0 and w = 1 shown):





The per-instance intervals are wide (12 rho runs, rho run-to-run sd about half its mean) and three of nine include 1 at w = 0 or w = 1; the pooled reference uses 108 runs and is the primary because rho expected cost does not depend on the instance (homogeneity across p was not rejected in the rescore, permutation p = 0.314) and the per-instance reference means span 0.80 to 1.14 purely by sampling noise. A/A check: the odd/even split of each instance's rho runs gives ratios 0.64 to 1.20 (stored per cell), which is the noise floor of a 6-run half and is why per-instance references are not used as the headline. Distance remaining to the matched rho: none at the point estimate at p = 1301, 1451, 1601 under the pooled reference, with upper interval ends below 1 at all three prices (0.94 at p = 1301, w = 1); under the per-instance references the interval at p = 1301 s1 and s2 reaches above 1. This is a property of the factor-base-line configuration already in the repo, not of the quotient. It is not validated: the cover native count is a lower bound (F\_p additions, pair selection and memory traffic are not emitted by the library, so complete pricing at 0, 0.1, 1 is not possible without changing it), there is no replay certificate or validator report, and the pooled reference mixes the 9 instances.



== Verifier disagreement



The skeptic's major issue says the rho reference "was not re-run on these fbline-model instances". Partly wrong: the rho runs of the rescore are on exactly these E(F\_p) instances (generate\_spec(p, seed), l and c\_add\_e equal in all 9 cells, asserted in `rho\_matched.py`); the cover configuration (plain model versus factor-base line) changes only the cover arm, never the group or rho's cost, so re-running rho would only add more samples of the same distribution. What was right and is fixed: there was no per-instance matching, no interval and no seed 3. Those are added above, and the headline numbers moved only because seed 3 was added (e.g. p = 1301 w = 0: 0.809 with 2 seeds to 0.791 with 3), not because of a disagreement. What is not fixable here: complete native-operation pricing (the library does not emit cover F\_p additions) and a replay certificate or validator report (forbidden here: no benchrec, isolab or ecbench). The summary phrase "remaining distance is none at the largest prime" is kept only as a measured point estimate of the baseline config.



\*\*P5 (replay): confirmed.\*\* Arm A at p = 1301 s1 reproduces the recorded line\_p1301\_s1 record of research/jv\_cover\_fbline\_20261102:

relation digest 4198168094706900228, 729 relations, 275600 residuals, 715 unknowns, 13297571 LA ops, S 0.72877507 against 0.72877516.



== Limitations



\* All three seeds ran at p = 1301, 1451, 1601 (p=1301 s1, p=1301 s2, p=1301 s3, p=1451 s1, p=1451 s2, p=1451 s3, p=1601 s1, p=1601 s2, p=1601 s3). The arm-N timing runs at p = 503 and 251 were stopped; arm N was not measured as a timed run, only by exact rank analysis on 3 instances (p = 251, seeds 1 to 3, 420 relations each). The cover side of every ratio is one deterministic run per instance, so intervals over only 3 instances under-cover.

\* G was only searched for automorphisms of the genus-3 curve over F\_q usable on an F\_p line; automorphisms over extensions of F\_q are irrelevant to the F\_p-line factor base.

  The census is exhaustive over the candidate maps in the PLAN argument, whose completeness rests on F\_q-rationality of a map preserving P^1(F\_p) and commuting with the q-Frobenius.

\* No LA-phase model was fitted; LA is under 0.1 percent of S in this config.

\* Nothing here enters the leaderboard, benchmarks or a claim; no replay certificate or validator report was produced for these cells; a matched-rho reference with a bootstrap band is given above (section 5a band is the w = 0, 0.1, 1 columns), with the cover native count a lower bound.

\* GHS binary covers (secondary target) were not touched.



== Repro



`cd /home/user/crypto && CARGO\_TARGET\_DIR=/home/user/crypto/target cargo build --release --bin cover\_aut\_quotient -j2 && research/cover-aut-quotient\_20261115/run.sh && python3 research/cover-aut-quotient\_20261115/analyze.py && python3 research/cover-aut-quotient\_20261115/build\_report.py`

(census commands: `cover\_aut\_quotient census-inst --sizes 503,1009,1301,1451,1601 --seeds 3`, `census-random --p 101 --n 4000`, `census-random --p 103 --n 300 --kummer`, `control`;

rank analysis: `python3 rank\_n\_arm.py`, `DUMP\_N\_ROWS=results/naive\_p251\_s\<k>\_first420\_rows.json target/release/cover\_aut\_quotient paired --p 251 --seed \<k> --naive --out /tmp/x.json` then `python3 rank\_n\_multi.py`; matched rho: `python3 rho\_matched.py`; seed 3 cells: `cover\_aut\_quotient paired --p \<p> --seed 3 --out results/paired\_p\<p>\_s3.json`). Unit tests: `cargo test --release --bin cover\_aut\_quotient`.



== Sources read



src/cryptanalysis/jv\_cover.rs; src/cryptanalysis/gaudry\_cubic.rs (square\_core, wiedemann\_u64); research/jv\_cover\_negrho\_rescore\_20261101/{README.md,fit.json};

research/jv\_cover\_fbline\_20261102/results/line\_p1301\_s1.json; research/jv\_cover\_negrho\_rescore\_20261101/results/new/rhostrong\_p\*\_s\*\_run\*.json; research/jv\_nagao\_hilbert\_20261112/{README.md,results/walls.txt}; research/jv\_sieve\_prospective\_20261023/PROTOCOL.md;

examples/jv\_endo\_check.rs; papers/ecdlp-index-calculus-treatise/survey/transfer.md (section 17 heading only).


