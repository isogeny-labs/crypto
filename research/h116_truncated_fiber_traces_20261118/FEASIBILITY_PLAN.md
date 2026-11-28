# H116 structural feasibility plan

The frozen `10^8` maximum-extension-size rule produced no cohort file: the
selector halted because only two of five required distinct subgroup orders
exist. No candidate moment, scalar predicate or timing has been computed.

Before inspecting which curves survive a relaxed cap, selector v3 will evaluate
this fixed ladder in order:

```text
100000000
10000000000
1000000000000
100000000000000
```

If one or more rungs yields five curves, the protocol amendment must use the
smallest such rung. Every other frozen condition and the exact selection order
remain unchanged. If no rung yields five, this feasibility plan fails and a
new structural rule must be frozen before further selection.

The feasibility output may contain only public curve parameters, group orders,
torsion degrees, extension sizes and selector work counters. It has no path for
fiber moments, scalar labels, predicates or timing. The v3 source and binary
hashes must be committed and pushed before `--feasibility` is invoked.
