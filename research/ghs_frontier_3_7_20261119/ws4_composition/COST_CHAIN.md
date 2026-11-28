# Composition cost chain

The interface is fixed throughout: one frozen known-answer target on the same
restricted GHS-weak curve instance, the same subgroup and success condition,
and complete setup/walk/verification accounting. The scalar is unavailable to
the solver and is used only by the independent correctness check; this wording
does not relabel the fixture as a public-target claim `[pcc:unverified]`.

```text
C0 = C(setup6, walk6, verify6, counted field/native sinks)              definition: strong-b8
   -> C1 = C(setup5, walk5, verify5, counted field/native sinks)         replace R17 cubic mul;
                                                                          per phase: -1 fq_mul and
                                                                          +4 fq_small_div per f3_mul
   -> C2 = C(setup5-table, walk5, verify5, counted field/native sinks)   replace R36 walker setup only;
                                                                          walk and verify unchanged
   =  C(strong-b8-mul5-walker-table)                                     definition: counted composition

C2 = C2'                                                                 restructure accounting sink only;
                                                                          mathematical transition unchanged
   -> C3 = C(setup5-table, walk5, verify5, no field/native mutations)    replace R64 accounting sink for wall;
   =  C(bare-b8-mul5-walker-table)                                       definition: wall composition
```

The R64 line is a wall-measurement implementation change, not a counted-cost
gain: its zero counters are explicitly non-authoritative. It is paired with a
counted execution of the same mechanism. R17 and R36 replacements are measured
again after composition; their standalone q=223 deltas are never summed.

The same chain is instantiated for the R17-only and R36-only candidates. A
reported gain must name the interval: counted complete cost for the `strong-*`
pair, native diagnostic parity for `fast-*`, or admitted wall time for the
`bare-*` pair. No line changes q, target, subgroup, target law, resource
envelope, or correctness rule.
