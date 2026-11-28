# The fixed10xcount target requires a joint construction

[pcc:unverified]. Freeze597fa1ad5c3; input hashes passed; deterministic post-analysis exit0. All12phase sums reconcile actual integrated full16vector REDC. All32changed-phase subsets enumerated independently per workload and per budget, including120single-phase-free inequalities. No new performance experiment, producer rerun or random selection.

Fixed original reference total B is unchanged. Current phase costs use the already observed short verification. A candidate that changes phases C and retains the others has cost at least U=sum_unchanged cost_i. Necessary condition for10x is10U<=B, even if all changed phases become free. The exact integer budget is floor(B/10); no floating rounding in decisions. This is not a sufficient constructive result.

## Fixed original target, not a moving threshold

|p|source multiple|case|fixed original B|current integrated total|10xinteger budget|phases individually above that budget|
|---|---|---|---|---|---|---|
|101|1|0|5280|2652|528|scan,verify|
|101|1|1|4324|1750|432|scan,verify|
|101|17|0|6098|4554|609|source,scan,verify|
|101|17|1|5240|3072|524|source,scan,verify|
|251|1|0|7022|4664|702|prepare,scan,verify|
|251|1|1|7780|4570|778|prepare,scan|
|251|17|0|8770|6434|877|prepare,scan,verify|
|251|17|1|9558|6202|955|prepare,scan,verify|
|601|1|0|13940|10110|1394|prepare,scan|
|601|1|1|14360|9768|1436|prepare,scan|
|601|17|0|15258|12310|1525|prepare,scan,verify|
|601|17|1|15660|11904|1566|prepare,scan,verify|

Across all12workloads, the unique inclusion-minimal COMMON phase set whose removal could meet the fixed original target is:

    prepare + source + scan + verify

This means a uniform construction leaving ANY ONE of those four phase types unchanged everywhere fails at least one workload. It does NOT mean every individual workload must change all four: CONTROL.txt lists every per-workload minimal set, some of which have only two phases. With all four hypothetically free, anchor work fits the fixed budget, but real replacement phases must still share the remaining budget. No implemented method is supplied by setting costs tozero.

There are zero passing single-phase-free inequalities across both budget sets: even making one phase completely free after the existing verification change cannot meet10x on any workload. Thus another isolated verifier/index/scan/LA-component improvement is not by itself a full-cost10xcase here. Base-field linear phase is0because subgroup arithmetic is counted separately, not because linear algebra is free or irrelevant to runtime.

For context only, a SEPARATE stronger-current budget uses B=current integrated total. Its unique minimal common phase set includes allfive phases, adding anchor. This is not substituted for the original target and not used to redefine goal success.

## Consequence for research selection

Existing lazy-cyclotomic-cold already removed fullfactorbase/character stage and failed its complete basefield case; streaming/restricted pencil gates already tried earlier stopping/shortened scans. Do not repeat them under new names or infer10xfrom a component percentage. The next prototype needs an explicit joint public-input construction and cost case spanning the required setup/source/producer/verification work, with target binding and useful rank retained. A genuinely different geometric attack is also admissible, but needs its own complete-cost/transport certificate.

This finite counter constraint applies to retained code and exact phase boundaries. It is not a universal lowerbound on index calculus, other sources, larger fields, or algorithms that reorganize phases. Original sourcecontrol is not claimed globally fastest; beating the stronger available comparator suite remains a distinct assessment. No mixed-modulus ratio, walltime or mathematical-breakthrough claim. Independent review pending; fullgenus2+objective remains active/unachieved.
