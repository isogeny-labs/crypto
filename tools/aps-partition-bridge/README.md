# APS production-linked partition correctness bridge

This package links the existing `crypto_lib` APS sampler, equation builder,
recovery code and certified F4 trace API. It does not copy or replace those
algorithms. The separate `aps-partition-lab` supplies ordinary-residue affine
arithmetic, candidate colorings and cheap specialization features.

The separate [quartic-cell extension](QUARTIC.md) integrates `Expanded4`
with a complete disjoint label-profile cover. The tuple-only valuation
sections below still describe the original binary and its frozen reports.
The [encoding comparison](ENCODINGS.md) matches Expanded4 and Split4 on that
same cell plan, retaining all auxiliary witnesses and failed arms.
The [four parallel leads](FOUR-LEADS.md) add auxiliary constraints, fresh label
laws, finite-domain remainders and per-cell pairing controls.

The purpose is structural correctness, not timing, throughput or a speed
ranking. No profiler is launched, and trace diagnostics are not complete
costs. The whole ecbench registry/record pipeline is outside this bridge.

## Fixed questions and design

* Do cold, shared-cache and signature-bucketed production F4 calls return the
  same complete represented root set as a separate affine group oracle?
* Do actual production point labels remain valid after recoloring, and does
  recovery return a scalar exactly when the represented set has an informative
  signed relation?
* Does the numerical certification path retain correctness as coefficients
  change, including shapes that do not support the cached candidate?

The public fixture is y^2=x^3+59x+6 over F101, subgroup order 107,
G=(78,4), Q=37G. Both sampler streams use that one target; they are not two
independent DLP targets. Only the fixture constructor uses the known scalar;
the sampler and solver receive G and Q. There is no search for a favorable
seed. Seeds20261115 and20261116 each draw the fixed3,3,2 labeled profile,
with a1000-draw cap. Eight deterministic colorings are fixed before calls.

Each nonempty coloring is split by the third coordinate. The actual APS
builder creates the partitioned3 equations, including its singleton third
domain. Root enumeration uses the native exact-domain visitor. The feature
bucket is the support/degree signature of specialized f3, computed without
solving it. Each sampler stream and route starts with fresh pools; each pool has
at most8 entries and100000 retained words. A root-level key hit still uses
the existing numerical input reductions and Buchberger certification.

The per-cell cooperative budget is2000ms and the degree bound24. These are
correctness resource limits, not observations of runtime. Empty colorings,
input errors, incomplete calls and roots excluded by the capped family remain
explicit. A report does not turn incomplete enumeration into an empty answer.
Every family cell is actually searched; duplicate roots are deduplicated after
solving, and root incidents remain counted. This bridge does not claim to
eliminate repeated polynomial work through first-owner coverage bookkeeping.
The caller must impose an additional process limit;420seconds is the proposed
fixed limit for this bounded panel. There are no automatic favorable retries.

Production guaranteed weights for odd arity are all one before canonical
signs. Their informative-label reasoning differs from the earlier toy
power-of-two fixture. Do not transfer a zero-B frequency between these label
laws or infer an even-arity guarantee from this triple-only integration.

## Controls and boundaries

### Label-valuation partition candidate

The same two fixed sampler seeds also construct even-arity2,2,2,2 pools
through the production sampler. A separate tuple-level experiment groups
their known B labels by power-of-two divisibility. Center each coefficient
modulo the subgroup order, discard zero coefficients for the minimum
valuation, require that minimum to occur an odd number of times, and require
the sum of absolute coefficients to be smaller than the subgroup order.

The proposed sufficient-condition argument factors out the smallest power
of two: the remaining signed sum is odd, hence a nonzero integer; the strict
size bound prevents a nonzero multiple of the modulus. The bound is essential.
The negative control1,2,2,2 modulo7 would otherwise admit the all-positive
sum7. Complete small-modulus sign enumeration tests this predicate, and
every tuple in each actual pool receives a separate full sign check.

Prediction before the fixed calls: two samples of each absolute weight1/2/4/8
give70 unordered quadruples,48 admitted by this sufficient condition and22
uncertain. Uncertain does not mean uninformative; every uncertain tuple stays
in the report. At arity four, exact sign enumeration is also a cheap, stronger
baseline. This candidate is a label-profile stratum, not a claim that F4 or
quartic relation construction has been integrated for these pools.

### Existing-path negative controls

The reference traverses coordinate products and checks independent affine
signed sums. It also counts informative and zero-B incidents; the production
recovery result must agree with that classification and pass a separate
scalar multiplication check. A corrupted actual sampler label is refused.

A fixed27-member coefficient grid over F5 compares actual certified trace
visits with literal equation evaluation. Raw trace fingerprint damage is
injected through the public recording/replay API and must be refused. That
raw-trace control is distinct from injecting a damaged private cache entry:
the latter is not exposed by the public API and is explicitly marked untested.
Natural cache rejections and fallbacks, if observed, remain diagnostic data.
An expired visitor must not be presented as a complete empty root set.

The package has a separate workspace/lockfile, seeded from the root lockfile.
Only the root library is linked; the root binaries are not experiment arms.
The release profile retains256 codegen units for the existing root build
policy. This is a build choice for correctness checks, not the lab performance
profile. No existing library assertion, feature guard or solver identity is
weakened to make the bridge compile.

```sh
cargo test --release --locked --manifest-path tools/aps-partition-bridge/Cargo.toml -- --test-threads=1
cargo clippy --locked --manifest-path tools/aps-partition-bridge/Cargo.toml --all-targets -- -D warnings
cargo fmt --check --manifest-path tools/aps-partition-bridge/Cargo.toml
```

For a source-pinned structural call, build the release executable, separately
copy and hash a read-only qualifier, create distinct exclusive build/pre-call
manifests, then use a new output path:

```sh
QUALIFIER structural --source-revision HEX40 --source-manifest-sha256 HEX64 --out NEW_REPORT.json
```

Exit0 requires all fixed calls to be complete and equal to their reference;
exit2 retains a report with failed/incomplete correctness obligations.
Exit1 is a CLI/I/O failure. User-supplied pins and unsigned custody files are
not execution attestations. Register native statements before public result
citation. Performance still requires the prospective checkpoint, complete
accounting/replay/export, frozen protocol and run admission. Ledger, raw-log,
private-history and host-capsule publication restrictions remain unchanged.
