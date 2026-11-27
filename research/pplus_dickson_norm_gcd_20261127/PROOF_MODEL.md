# Public proof model and seed statement

## What is proved

This experiment produces a deterministic, publicly replayable certificate for
each fixed query. The candidate publishes the dense quotient-algebra
polynomials `A`, `B`, and `N`, the split-gcd polynomial, every norm value on the
complete known Dickson fibre, all exceptional-chart decisions, and a canonical
signed witness or an explicit miss.

A separately written Rust verifier does not import the candidate. It rebuilds
the curve, Dickson roots, signed leaves, complete unordered pair support, and
sealed support digest. It then evaluates the homogenized Dickson recurrence
pointwise at every root, checks every published polynomial evaluation and
split-gcd root, directly enumerates pair truth, and replays every witness. A
verified miss therefore includes a nonzero value at every excluded root and is
not inferred from timing, a digest, or the candidate's control flow.

The operation counters remain source-instrumented measurements. The verifier
attests the mathematical transcript, while the source manifest binds the code
that emitted the counters. It does not independently count machine
instructions or attest the host.

## Why this is not zero knowledge

All inputs, targets, leaves, witnesses, and intermediate coefficients in this
study are intentionally public. There is no secret witness whose disclosure
must be prevented, so a zero-knowledge proof would not strengthen the stated
experimental claim. This repository therefore makes **no zero-knowledge
claim**. SHA-256 provides source and evidence integrity only; it is neither a
zero-knowledge proof nor remote-execution attestation.

A future zero-knowledge artifact would be a different protocol: it would need
a precise private-witness statement, commitments, a field/circuit encoding of
the exceptional charts and split-gcd checks, a named proof system and security
assumptions, setup provenance, and verifier test vectors. None of those may be
retroactively inferred from this experiment.

## Seeds and deterministic choices

There is no pseudorandom seed (`seed=none`). The protocol fixes:

- the registered curve and generator;
- Dickson depths 1 through 6;
- the smaller-square-root-first branch convention;
- signed-leaf and canonical witness ordering; and
- at every depth, `planted_pair`, `[1]G` through `[8]G`, `first_leaf`, and
  infinity in that order.

The candidate and verifier both emit `seed=none`. Reproduction requires only
the source-manifest-bound code and public protocol; there is no hidden seed,
sample selection, or post-hoc target replacement.

## Custody boundary

Before the sole production run, the runner requires a clean named branch whose
local head is exactly the same branch head on `origin`, verifies every active
source-manifest SHA-256 before and after the release build, and refuses an
existing evidence directory. Successful evidence is moved into place only
after candidate, verifier, analyzer, schema, and evidence-hash checks pass.

This is reproducible pushed-Git custody. It is not S3 custody, because the
owner stated that S3 is unavailable, and it is not a trusted-execution claim.
