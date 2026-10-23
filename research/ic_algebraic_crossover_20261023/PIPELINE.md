# Native full-pipeline adapter

`structured-s4-base-v1:dimension=L,family=geometric,seed=S` builds the exact
seeded geometric subspace with the standard binary point lift and negation
column fold. Families prefix, geomtraceu and random are also accepted; all
parameters are mandatory. Dimensions above ten are refused by this pilot.

`structured-s4-oracle-v1:m=3,form=symmetric,degree=2,preclose=1,node_budget=20000,max_solutions=4096,max_cols=30000`
uses the actual product spaces, quadratic symmetric S4 relaxation, optional
full fixed-degree closure, inherited F4, repeated-root cubic division, and
the framework's counted sign lift. `form=direct,preclose=0` provides the
direct S4 control. The library's effective degree is at least the input
degree, as explicitly frozen in the protocol. Extraction above 21 variables
is refused. Solver environment overrides are refused to preserve identity.

The plug-ins are in `src/bin/ecbench/structured_s4{,_math}.rs`, registered
alongside existing plug-ins. Existing method behavior and library source
are unchanged. The ordinary `ecbench run`, `verify`, `table`, `resources`
and `band` commands apply. Positive controls recover repeated-root group
witnesses for every family and both formulations, with actual additions
on GroupOps; missing parameters are rejected.

**Accounting scope.** Lifts are charged group additions. Fixed closure,
readback and hybrid word operations are counted in the existing inherited-
F4 unit, unpriced in S. Input term counts, algebraic roots, split tests,
non-splitting roots, refutations and exhausted queries are retained. All
full-pipeline phases remain present. Symbolic-construction and split field
arithmetic are not completely counted in native units, so these sessions
cannot support a full-price win. Elapsed phases at L0 remain diagnostic.

The common band at 0, 0.1 and 1 GAE is a sensitivity scenario for recorded
native work, not a calibration of the unmeasured construction and splitting.
Neither a low S nor a solver-stage word reduction discharges those costs.
