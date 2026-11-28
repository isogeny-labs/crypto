# One-shot Dickson norm-gcd production runbook

The deterministic depth-1--6 production ladder is launched only with:

```sh
research/pplus_dickson_norm_gcd_20261127/run_census.sh
```

Run it from a clean checkout of the named study branch after the active source
manifest has been pushed. The wrapper requires local `HEAD` to equal the same
branch on `origin`, verifies all source hashes, builds locked release binaries,
rechecks source hashes, and then invokes in order:

1. `dickson-norm-gcd --max-depth 6`, with the production guard set;
2. the independently coded pointwise verifier; and
3. the preregistered analyzer.

The wrapper refuses to overwrite `evidence/`. It writes to a unique temporary
directory, retains a failed directory for diagnosis without retrying, and
atomically renames it only after all three programs, schema assertions, and
SHA-256 checks succeed. Standard error and elapsed nanoseconds are retained;
peak RSS is explicitly recorded as unavailable because the environment has no
approved measurement dependency.

There is no random seed. The curve, roots, branch convention, leaf ordering,
and all 66 queries are fixed by `PROTOCOL.md`. Candidate and verifier outputs
state `seed=none`. `PROOF_MODEL.md` explains the public replay guarantee and
why no zero-knowledge claim is made.

The active custody record is `SOURCE_MANIFEST.json`. Pushed Git is the
checkpoint transport because S3 is unavailable. Any pre-candidate wrapper
failure or scientific rerun requires a prospective, committed amendment; the
runner never silently replaces evidence.
