# Frozen fresh-trial identity-pattern panel, primary N83

The [machine protocol](protocol.json) names 16 exact formulas: trials 1 and 2,
identity masks 0 and 1, hash and Gray residual pools, and 5+1 and 4+2
splits. It keeps the same public primary `a=0` Koblitz GF(2^83) target and
base archive as the earlier Linux first-pattern panel. Each trial/mask
stratum runs all four factor-base/split candidates on one Linux ARM host,
with adjacent hash 5+1 A/A repeats and a final hash 5+1 drift control.
The four strata may run on separate hosts, so cross-stratum wall comparisons
are not paired. A 300-second solver cap on each of 24 processes bounds the
aggregate solver allocation at roughly two hours, plus build, import, and
artifact overhead. A cap or `INDETERMINATE` result is censored.

The worker source is pinned to commit `83393dd87ef` and must be built from
that exact tree. The historical trial-zero all-affine XCNFs continue to use
their original preflight binary for byte-exact export-metadata replay: the
new worker has a different source digest. The new
`export-cross-xcnf-pattern` / `verify-cross-xcnf-pattern` commands take
explicit `TRIAL IDENTITY_MASK` arguments after the summand counts and use
metadata schema `n83.cross-stream-pattern-xcnf/v2`. The group target is
computed from the deterministic coefficients for that trial, and the
identity mask must be less than `2^(summands-2)`. The verifier rebuilds the
formula and metadata from the pinned archives, checks a complete external
assignment against original CNF/XOR/S3 equations, then lifts and checks
the original group equation. One group-checked candidate is a partial
relation; duplicate repeats are not separate yield observations.

The 16 SHA-256 digests and formula shape counts were calculated before the
hosted solve from local copies of the three archives. The base and pool
archive bytes matched the [eleven-object S3 manifest](../linux-xor-complete-inputs-20261025/manifest.json).
An independent fresh Linux GET must replay all eleven objects again, and
each formula must be rebuilt and byte-checked against this protocol before
solver timing. The source-pinned worker must reject an incomplete synthetic
SAT assignment at this same gate. Exact solver ZIP, package, binary digest,
flags, caps, host class and arm order are also pinned in the protocol.

For an individual export, use the source-pinned worker with the same manifest
and public corpus as the earlier panel. For example, hash 4+2, trial 1,
mask 1 is:

```sh
"$WORKER" export-cross-xcnf-pattern "$BASE_HASH" "$MANIFEST" "$BASE_OBJECT" \
  "$HASH_POOL" "$MANIFEST" "$HASH_OBJECT" "$CORPUS" 4 2 1 1 "$EMPTY_EXPORT_DIR"
```

The formula SHA-256 must be
`5804c664b9c584d256e08ba23ea6b514eff07c2e9fb96b5af589c5479be6803c`.
The same trial/mask Gray 4+2 formula SHA-256 is
`82bc902a1f1d30054e99ab829e75cb33455aa954b85ae0c1503ab99b6ce44fca`.

The panel measures only bounded access to a complete SAT model for these
particular natural equations and patterns. Even a verified partial would
need larger independent trial samples, double-large-prime closure, useful
rank, sparse linear algebra, unseen-target descent, all-stage one-target
cold accounting, and same-target rho before a total-runtime factor-base
choice. The finite sweep design covers many more base and solver tuples;
this panel executes only its named slice. `selected_best_total_runtime`
remains `null` until those gates are actually met.
