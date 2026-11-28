The [general independent-label proof](WEIGHTED_GENERAL.md) now verifies
actual in-degree mean 1 and variance 1−Σpᵢ² for every finite abelian group,
every distinct finite increment support, and every normalized nonnegative
label law. Lean also disproves dropping distinctness, even for nonzero
increments. Whole walk gate: 70 declarations, zero failures or trusted-native
exceptions. Collision-time and mixing asymptotics remain separate.
[pcc:unverified]

The new [unequal-weight PMF proof](WEIGHTED.md) verifies actual in-degree
mean1and variance1−(a²+(1−a)²)on the complete8hash Z3ensemble, for every
0≤a≤1; probabilities3/4and1/4give variance3/8. Expanded whole gate43passes,
0failures/0trusted-native. This checks a variance component, with the
collision-time penalty still open. [pcc:unverified]

# Actual walk boundaries and exact finite populations

Lean proves a fixed-support nonmixing family using actual PMF histories
and addition in Z/p. For the four distinct increments0,1,2,3, an arc of
at most3t+1residues has probability1 aftertsteps. When6t+2≤p, this violates
quarter-TV event mixing. The closed prime witness p101,t10 has event mass1
versus uniform31/101. Claim PCC-20261109-13b8fc passes its full local check
and remains unsigned. This is a known obstruction to dropping Hildebrand's
random-support averaging, not a refutation of its qualified theorem.
[pcc:unverified]

An actual Z/3rho graph with two distinct nonzero increments has normalized
in-degree mean1 and variance1/2. Lean enumerates the actual8-element
hash-function space, proves its cardinality and uses it as the denominator.
This corrects a literal reciprocal-variance sentence in Galbraith ch14
version2.0,p297, whose page was visually checked. Claim PCC-20261109-a34842
passes its full local check and remains unsigned. The collision-time
heuristic itself is not refuted. [pcc:unverified]

All four frozen native calls pass, with identical outputs in both repeats:

| Process | Complete population per repeat | Exact checks |
|---|---|---|
| iid mixing | 3930supports; primes5,7,11,13; sizes2,3,4,6where possible | Sparse recurrence=dense matrix, mass, TV contraction;0censored |
| fixed-hash rho | 80supports,82034maps; primes3,5,7; sizes2,3 | Visited array=Floyd, first-repeat bound, mass, in-degree moments |

Uniform labels refer to the ensemble; individual fixed hash maps can have
unequal empirical partition sizes. Both all-support and no-zero-support
strata were frozen before data. The equal-weight penalty is shown beside
the exact observed/mapping-baseline ratios. General nonuniform collision populations, asymptotic
constants and practical hash families remain open; the new finite weighted
PMF moment is recorded separately. [pcc:unverified]

The original walk gate audits21declarations,0failing,0trusted-native exceptions.
It includes actual uniform-history product probabilities, distinct
increments, an event-mass bound, exact p101uniform mass, iteration
pigeonhole, and the full-support hash-map equivalence with uniform
endofunctions. No custom axiom, sorry/admit or native_decide is used.
[pcc:unverified]

Protocol hash:
dfd90b02047a4dd8c59453d4f5f1099ecbf3cbd9667fb041218b14f9a6677452.
Native source committed in8c376a41754c03c8baa045e316ea2514f3363491;
protocol frozen/committed ind801653d085b0951356ffb3a2686349582347884
before affected calls. Development preflight is disclosed as in-sample;
all its source generations and output remain retained. No after-data
native repair or protocol change occurred. [pcc:unverified]

Flow:4planned/attempted/completed,0not-completed,0interrupted/resumed,
0excluded. Export validation against frozen protocol and file hashes passes.
One shared local host, one thread, approved local checkpoints, zero paid
spend. Preflight clocks budgeted the run and are not scientific metrics.
No statistical confidence, independent-host or signed validation claim.
[pcc:unverified]

See [Lean source](lean/WalkBoundary.lean),
[proof gate](evidence/proof-gate-normalized.log),
[protocol](protocol.json), [narrative](PROTOCOL.md), [export](export.json),
[provenance and primary hashes](provenance.json),
[native code](native/census.cpp), and
[two-page report](reports/run-20261109-walk/report.pdf).
The report plots recorded native means/ranges and exact finite ratios;
it makes no asymptotic fit. [pcc:unverified]

Reproduce the source with the recorded g++/GMP build command; run.sh checks
all frozen identities and refuses to overwrite outputs. Run the repository
Lean gate and canonical export checker on a fresh checkout, hydrating any
artifact-store files first using tools/artifacts. [pcc:unverified]

```sh
tools/lean/proof-gate.sh research/walk_audit_20261109/lean
python3 /workspace/crypto-autoresearcher/tools/experiment_protocol.py check-export \
  research/walk_audit_20261109/export.json \
  --protocol research/walk_audit_20261109/protocol.json --verify-files
```

This is item10progress. The entire fourteen-item acceptance map remains
[here](../theorem_census_20261109/README.md), including general averaged
mixing, weighted collision constants, the matched SafeCurves benchmark,
pairing/Jacobian/Frobenius and independent-host obligations. Source read
status and source corrections do not silently promote canonical corpus
nodes or signed claims. [pcc:unverified]
