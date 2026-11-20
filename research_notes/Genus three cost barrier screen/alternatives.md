# Genus 2 and 4 successor screen

## Does a structural mechanism justify full 10× native IC?

### Takeaway
Reject admission of a new 10× lead from this screen. The strongest structural option, point sieving plus public challenge labels and sparse large-prime recombination, is already a repository cost proposal. Its useful-rank yield and complete-cost constants remain unbounded. An exponent or a relation-count improvement is insufficient. No experiments were run.

### Cited Findings
- The objective is complete IC cost on the same genus-2/3/4 inputs; the old PLAN already lists double-large-prime methods and their genus-dependent exponents. Its historical infrastructure gaps are not current-state assertions. — [PLAN](../../research/hec_ic_speedup_20261113/PLAN.md)
- Held-out curve median gains are 0.9433–3.1684; largest-size relation generation occupies 47.8–68.5% of total, dense forest reaches 31.7% at g2p65537, and dense anchor solve reaches 17.5% at g3p1009. These are local unvalidated observations. — [held-out handoff](../../research/hec_anchor_relations_20261114/HELDOUT-STATUS.md)
- Genus-2 zero-core preserves the registered stopping signatures but loses to Compact: fixture median best/zero ratios 0.245253 and 0.362194. Its fixed-work diagnostic, with setup and row handling both free, never exceeds 0.649240. — [zero-core results](../../research/hec_zero_core_20261115/RESULTS.md)
- Coldcost13 is one genus-3 pair, not genus-2/4 evidence: cold 1780.974708 ms versus Compact 0.797166 ms; even removing both maps leaves median 240.507208 ms. It rejects treating transfer/map improvements alone as sufficient on that case. — [coldcost13](../../research/hec_coldcost13_20261115/RESULTS.md)
- Double-large-prime IC has expected cost Õ(q^(2−2/g)) under the cyclic/known-class-group-structure assumptions; this is existing theory, not a new exponent. — [Diem, 2006, theorem and §§2–3](https://arxiv.org/pdf/math/0606607)

### Inferences
Let T0 be the fastest admitted native complete cost for each fixed input, R its relation-generation cost, and U all remaining cost. A relation-only replacement with speed factor s satisfies Tnew=R/s+U. A 10× gain requires U<T0/10 and s≥R/(T0/10−U). At observed relation fractions 0.478–0.685, making relations free gives only 1/(1−r)=1.92–3.17×, provided downstream work is unchanged. Conversely, freeing all other work leaves at most 1/r=1.46–2.09×. A successful successor must change both relation supply and downstream cost, or justify a different regime before measurement. These are conditional arithmetic bounds derived from the cited phase shares, not universal lower bounds.

### Gaps
There is no validated full-cost 10× bound for genus 2 or 4 in the sources read. This bounded search does not prove that such an algorithm is impossible.

## Which structural candidates survive novelty and cost screening?

### Takeaway
Zero new leads are recommended for experimentation. Two mechanisms were screened precisely enough to explain rejection; the first remains useful as an established-method baseline, contingent on a further mathematical cost certificate.

### Cited Findings
1. **Point sieve + challenge labels + sparse recombination: prior-art collision, cost case incomplete.** The primary paper identifies the hyperelliptic sieve as Sarkar–Singh, preceded by Joux–Vitse, and separates harvesting from unchanged LP/LA stages. Its approximately 3× observation concerns non-hyperelliptic relation search, not complete genus-2/4 IC. — [Improved sieving on algebraic curves, introduction and §2](https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf). The repository already derives the labelled residual construction, short buckets, yield heuristic and joint cost expression. — [JOINT-COST](../../research/hec_lazy_forest_20261115/JOINT-COST.md)
2. **Higher-parameter affine pencil: exact equivalence rejection.** For D=[u,v], t=v−u(λx+μ), and two interpolated places Pi,Pj, the residual monic((t²−f)/(u(x−xi)(x−xj))) represents −(D+Pi+Pj). This is ordinary composition/reduction, not extra independent relation supply; degree drops do not evade that identity. The repository retains its proof and finite native gate. — [affine-pencil results](../../research/hec_affine_pencil_20261115/RESULTS.md); primary reduction reference [Bernatska–Kopeliovich, §3.2](https://arxiv.org/html/1912.13277) (primary link supplied by repository; not independently fetched here).

### Inferences
**Candidate 1 equation and threshold.** Use D=[u,v]=aG+bQ with public labels, t=v−λu, Qλ=(t²−f)/u, degree g+1. On split residuals the principal identity gives a row supported on at most g+1 places plus the target column; keeping a separate unknown for each batch is unnecessary. These algebraic statements are reproduced from JOINT-COST, not a new construction.

With B oriented small places and q field elements, the repository's rare-occupancy heuristic predicts

    Yg ≈ B^(g−1) / (2 (g−1)! q^(g−2)) raw two-LP candidates/batch.
    Y2 ≈ B/2; Y4 ≈ B³/(12q²).
    Cpartial ≈ cscan B/Yg.

Let η be the retained useful fraction after duplicate/orientation/degeneracy rejection and Mstop the partials actually required for target determination. Then a necessary full-cost case is

    Tnew = Ccatalogue + Cjumps + (Mstop/(ηYg))*Cscan(B)
           + Ccandidate + Cdedup + Cforest(Mstop,W)
           + CLA(Z,rank,restarts) + Cdescent + Cverify ≤ T0/10.

W is total label-expression expansion work; Z is actual matrix nonzeros. Neither is controlled by calling the graph sparse. Assuming Mstop=Õ(q), η bounded away from zero, and sparse algebra Õ(B²), the classical balance B≈q^(1−1/g) recovers Õ(q^(2−2/g)): q for genus 2, q^(3/2) for genus 4. It supplies no constant 10× guarantee. For g4, reducing B harms supply quadratically per scanned place; for g2 the leading scan/partial quotient is approximately constant, so shrinking B alone supplies no relation-search gain. The unknown η, stopping/rank tail and W/Z costs prevent admission.

**Candidate 1 negative controls, if a later paper calculation passes.** Preserve same public pair and full cold setup; compare fastest existing Compact/Fast/Hybrid and the established point sieve with identical labels. Use dense/lazy prefix equality controls, A/A timing, full principal identities before subgroup projection, duplicate/sign/multiplicity and u(x)=0 rejection, low-degree branches, and unselected held-out curves across both genera. Charge public jumps, bucket construction/clearing, expanded cycles, rank failures, descent and verification. A raw-candidate or per-batch gain cannot pass the full objective.

**Candidate 2 cost/falsifier.** Tpair=Csetup+Npair(Cinterpolate+Creduce+Csplit)+Ccombine+CLA+Cverify. Compare against optimized two-add_place scheduling on the exact same pairs. Any claim of extra mathematical yield must exhibit a useful relation absent from the equivalent shifted-divisor stream after matching signs/multiplicities and counting duplicates. Changing pair scheduling may alter correlations, but no proved lower Mstop or complete-cost bound was found. Reject as a successor mechanism; batching is an engineering question.

### Gaps
The sieve's dependent signed bucket occupancies are not independent Poisson samples, and raw candidates are not independent rows. No η lower bound or genus-2/4 complete stopping-cost certificate was found. Full primary sieve §2 retrieval timed out on expansion after its introduction was fetched; the detailed equations above are explicitly repository derivations. GTTD ePrint retrieval was unsuccessful, so its implementation constants are not asserted. Harvesting/filtering is also established prior art, not a fresh mechanism; the publisher describes modest example improvements rather than a full 10× guarantee. — [Avanzi–Thériault, 2010](https://www.aimsciences.org/article/doi/10.3934/amc.2010.4.189)

## What was checked and what changes next?

### Takeaway
Continue mathematical screening only after an explicit complete-cost inequality with useful-rank supply; do not restart dense/zero-core tuning or the terminal held-out cohort on the basis of this note.

### Cited Findings
Sources checked (KB): revision `9141e386254340a9bdba3036f2bb706a`, last refresh `2026-11-14T21:44:48Z`, ready=true, lexical current, hybrid_current=false. Queries: `hyperelliptic genus relation recombination double large prime`; `hec anchor zero core coldcost13`; `hyperelliptic sieving harvesting relation genus`. Recent-source lookup for hec-followups returned no entries. Thus current local files, not KB absence, determine the November 15 novelty collisions.

Opened metadata/context for `source:a2b0d45868e0cf479f3ce1e23a626be5`, `crypto-autoresearcher:knowledge/literature/KN-LIT-92caf4.md`: it explicitly labels its original review abstract-level. Related-source lookup led to `source:37965b1557e457ad28a6ada00ccc8c27`, `KN-LIT-a45b7b.md` on small-degree plane models. Neither note supplies native constants. Primary theorem verification instead uses [Diem's original paper](https://arxiv.org/pdf/math/0606607).

Repository reads: PLAN; anchor PROTOCOL and HELDOUT-STATUS; zero-core RESULTS; coldcost13 RESULTS; lazy-forest JOINT-COST; affine-pencil RESULTS; pairic13 RESULTS; FOLLOWUPS; latest dated session log. The latter explicitly records parent delegation of three disjoint literature screens. — [session log](../../docs/session-logs/2026-11-15.md)

### Inferences
A new paper-and-pencil admission should establish a lower bound on useful independent rows per charged scan, an upper bound on expansion/rank cost, and cold target-aware Tnew≤T0/10 on a stated genus-2/4 regime. Prior-art equivalence must be resolved before implementation. No novelty or impossibility theorem is claimed.

### Gaps
Conductor conflict checks and an attach-to-T-6285 attempt report the parent's directory lease as overlapping this child file. The user's explicit delegation and parent log identify this disjoint output; only this requested note is written. No lease release, commit, PR, session-log edit, memory update, scientific execution, or other file edit is performed by this child.
