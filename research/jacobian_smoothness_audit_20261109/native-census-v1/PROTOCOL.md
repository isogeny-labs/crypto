# Calibration preregistration before curve data

This phase checks the two exact known-answer fixtures before collecting
the full F3/F5monic-quintic census. The machine-readable record is
[protocol-calibration.json](protocol-calibration.json). It is a draft at
the initial source commit and must be frozen and committed before calls.
No calibration or population data has run at this writing. [pcc:unverified]

Run the F3positive detector first, then the F5all-smooth negative detector,
one execution per arm and no warmups. These are the same Rust binary and
C++checker, with the full actual-pair/addition/group/subgroup paths.
Pre-data answers are8pairs/7smooth for F3and16pairs/16smooth for F5,
from complete Lean polynomial-pair proofs. F3must detect an unequal proper
nontrivial cyclic proportion; F5must detect none. Identity is included.
These are exact finite controls, not a statistical null or typicalness test.
[pcc:unverified]

Every C++pair/addition/group/subgroup check must agree. Analysis regenerates
the checked TSVfrom the raw JSON and requires exact byte equality. A failure,
timeout, missing footer, wrong count or detector failure invalidates the
execution and stops the remaining calibration cells. Preserve all partial
artifacts and failed rows; any source/design correction requires a new,
dated version before affected reruns. [pcc:unverified]

The planning allowance is60seconds per small fixture,120seconds estimated
total; hard cap120seconds per execution and240seconds across both. These
are pre-data scheduling estimates, not measured timings. Calibration
elapsed intervals will inform the separate full-census protocol. There is
no scientific timing claim or statistical power calculation for complete
known-answer checks. Existing local capacity and local checkpoints are
authorized; paid capacity is forbidden by the user's budget. [pcc:unverified]

Record actual compiler/binary/source identities, seed, arm/instance/replicate,
host facts, exit status and all native counters. The seeds are metadata-only;
enumeration is deterministic. Raw files are write-once in fresh directories.
Export every execution under experiment.export/v1 and run the canonical
check-export with file verification. No separate-runner review or separate
machine is claimed. [pcc:unverified]

## Prospective full census, not yet an active frozen protocol

After valid calibration, freeze a different protocol for four calls:
F3repeat0,F5repeat0,F3repeat1,F5repeat1, all monic degree-five polynomials
in each field. Model code is the low-coefficient-first radix-p integer;
singular models remain explicit rows. Expected squarefree model counts
162/2500are pre-data combinatorial controls. Every reduced representation,
ordered addition, group-axiom triple and distinct cyclic subgroup is checked.
The weighting is by model, not by geometric isomorphism class. Repeats check
determinism, not variance. [pcc:unverified]

Keep proper order-two, characteristic-order, odd-prime noncharacteristic
and composite strata explicit. Compare exact cross-products; never round
fractions. Stoll's source-known F3curve x⁵+1is the first valid model in the
full encoded F3population and must show the predicted order5all-smooth
cycle of(X,1), with ambient10pairs/8smooth. This prediction is recorded
before any native curve invocation and is not a novelty claim. A faithful
Lean extension is still required before reporting it as a new formal result.
[pcc:unverified]

Full geometry, a scaling family, qualified typical/large-prime asymptotics,
and the original all-p<500elliptic/binary/ternary and other fourteen-item
requirements remain in the main audit map; these genus-two fixtures do not
replace them. [pcc:unverified]
