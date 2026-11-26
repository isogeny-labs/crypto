# Matched norm-one-torus factor-base comparison

This directory holds the frozen protocol and, after the public-precommit run,
the retained inputs and outputs for the Rust comparison of the production
Prime31 Dickson factor base with the equally sized ordinary small-abscissa
factor base.

The experiment uses a streamed two-sum side: pair complements are generated in
fixed blocks and discarded after the three-sum-table probe. See
[`PROTOCOL.md`](PROTOCOL.md) for the pre-outcome decision rule and scope.
