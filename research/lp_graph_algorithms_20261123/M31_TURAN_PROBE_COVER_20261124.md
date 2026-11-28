# M31 universal three-part pair-probe cover

2026-11-24. [pcc:unverified] Written combinatorial derivation and frozen-record metadata check, not a native run. Scope notice #5122 comment 7146525567. Producer #5097, extension #5076 and native integration #5120 keep their ownership.

## Source and protocol

The existing generic exhaustive four-distinct-index producer is `research/prime31_graph_ecdlp_20261123/native/mitm_four.rs`, at #5097 head ddf4d20161b5eaa3fd4a367ba6cca7e0ec9a41f6, blob 8095178e98ee8b26c40bf5450f25c108f76eecde. The historical input is the immutable `research/prime31_graph_ecdlp_20261123/evidence/mitm-b677-control-01/forty-cap-full-cost.json` at d8efd1dcaaa8b5e6d285a4406e1cdae7718992e7, blob 0d53e2f787eafa7c422e369983d6b04ea275e859. Curve p=2147483647, a=847514846, b=2087444724, subgroup/order r=2147446831, h=1; G=(625037715,2100627251), Q=(976979610,1228979761); 509 small/168 LP support indices. Historical seed 0x7033315f6d340101 is unchanged; no new PRNG, new random trial, collection, scalar recovery or timing. The fixed rule i mod 3 was declared before the metadata census and not tuned to rows. One coordinator, zero workers; elapsed/CPU/RSS unavailable.

Sources checked (KB): unavailable (no KB tool/wrapper/local checkout). Read current AGENTS, ACTIVE, main and branch dated logs, previous #5122 dedup report, #5097 source and receipt, #5076/#5120 status, graph catalog/SOURCES; bounded repository/PR searches for Turan pair (no hits) and existing batch-complement/negation-folding work (#4618 and orbit_pair_table). Classical Turán theorem: Yufei Zhao, MIT 18.225, §1.2, Theorem 1.2.4 (https://ocw.mit.edu/courses/18-225-graph-theory-and-additive-combinatorics-fall-2023/mit18_225_f23_lec_full.pdf); text accessible, screenshot cache miss. The result applies the classical theorem, not a novelty claim. Conductor and private job inventory unavailable; bounded remote scope notice only.

## Proof and restricted optimum

Keep the full right-side index with every signed pair ID and every collision bucket. Partition the original B point indices into three balanced classes using i mod 3. Only visit a left pair if both indices are in the same class. Every four-distinct-index support has a same-class pair by pigeonhole. For every valid signed four-leaf decomposition, choose such a pair with its original signs as left, and its two other signed leaves as right. The right index necessarily contains the exact complementary point and pair ID. Conversely the existing group-law equality and disjointness checks still exclude nonrelations. Thus the complete signed output set is unchanged under the stated exhaustive source contract. Public a,b, row signs, final sorting, group replay, graph eligibility and all stopping boundaries remain unchanged. The optional canonical left choice is the lexicographically first eligible pair of the four sorted indices. This REPLACES, not combines with, the earlier left.j < right.i guard; that earlier guard need not select a same-class pair. Keep the HashSet initially for validation and compare exact ordered output. A future Vec is conditional on the separate uniqueness proof.

Optimality only within universal, fixed, sign-independent, support-only left-pair restrictions with the full right index and no missed four-supports: the selected edge graph must meet every 4-subset, so its complement is K4-free. Turán's theorem bounds the complement by the balanced complete tripartite graph. A union of three balanced cliques is therefore minimum-edge in this restricted model. This is NOT a lower bound on algebraic extraction, adaptive/sign-specific selection, restricted eligible-support families or general index calculus. Repeated-index/identity/incomplete table/early-stop variants need separate contracts.

## Frozen-data census

Read the pinned JSON and require four increasing distinct in-range indices with signs ±1 on every row. For each, enumerate the six position pairs and retain i mod 3 == j mod 3. Require nonempty and exactly one lexicographic minimum. Query counts 52,54,71,64,72,75,68,54,69,66,62. All 707 archived rows covered; 4242 pair checks, 1424 eligible splits, 707 canonical splits. Histogram of eligible splits per row: 1:302, 2:162, 3:220, 6:23. This verifies support coverage only, not a new point-law replay or a candidate execution. Reproduce with the following deterministic receipt inspection (no randomness):

```js
const freq = {};
let checks=0, eligible=0, canonical=0;
for (const row of receipt.relation_rows) {
  const ids=row.terms.map(t=>t[0]);
  if(ids.length!==4 || ids.some((x,i)=>!Number.isInteger(x)||x<0||x>=677||(i>0&&x<=ids[i-1])) || row.terms.some(t=>![1,-1].includes(t[1]))) throw Error("terms");
  const pairs=[];
  for(let i=0;i<4;i++) for(let j=i+1;j<4;j++){
    checks++; if(ids[i]%3===ids[j]%3)pairs.push([ids[i],ids[j]]);
  }
  if(!pairs.length)throw Error("coverage");
  const c=pairs[0];
  canonical+=pairs.filter(x=>x[0]===c[0]&&x[1]===c[1]).length;
  eligible+=pairs.length; freq[pairs.length]=(freq[pairs.length]||0)+1;
}
```

## Exact modeled work, not measured speed

Class sizes 226,226,225. Four signs per unordered pair. Existing left visits 4*C(677,2)=915304 per query. Proposed left visits 4*(2*C(226,2)+C(225,2))=304200. Over eleven frozen queries: 10068344 -> 3346200 complement additions, 6722144 fewer (66.7651%). Right table setup unchanged (915304 pair entries, 915116 distinct sum keys; do not discard collision IDs). Under the explicit assumption all other charged additions remain identical, 10987751 -> 4265607, about 2.576x fewer total counted group additions; not a measured wall or complete solver improvement. The source still enumerates O(B²) pairs, stores O(B²), and computes O(B²) complements per query; no exponent gain. Do not apply Amdahl timing estimates as measured results. Current generic control remains far behind same-target rho, and #5097's tuned negation-map baseline is not available.

## Next native acceptance gate (proposed, unexecuted)

Coordinate #5097 owner; preserve old source and use opt-in restricted left iteration (skip before ar.neg/ar.add), maintain full right index, replay the exact same public queries/fixture and compare full sorted signed rows and affine identities query by query; preserve exact 645/707 prefixes and eligibility, all rank/recovery checks, collision buckets and failure states. Test canonical ownership separately, with a complete exhaustive support test and mutation omitting a class edge. Record build/seed/corpus identities, source and output hashes, all costs including setup, lookups, bucket visits, selection, replay, memory and stopping. Freeze protocol and independent held-out seed panel before any new randomized trial; retain existing A/A and timing schedule. No new native execution, collector or CI dispatch authorized here; no paid compute/merge. Existing M31, experiment-seeds/v1, resource, publication/stop and impact instructions preserved. Native build/ownership and independent validation remain open.

Impact score: 4/10 — Proven exact universal coverage and restricted optimality, checked on all 707 frozen supports; exact modeled complement-work reduction at the dominant stage, but no native or complete-cold runtime evidence.