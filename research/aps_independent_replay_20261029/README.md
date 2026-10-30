# Independent replay of bounded APS transcripts

Bounded witness replay verified, 2026-10-29. This follows the merged classical
archive (PR #2797) and the four-lead APS implementation (PR #2803).
It addresses the independent-replay gap in
problems/aps-single-relation-complete-cost.md. It does not complete that
problem's accounting, export, parameter-ladder or controlled-panel obligations.

## Scope and acceptance

The input is the existing aps.single-relation.correctness/v1 JSON from
examples/aps_single_relation_check.rs, for its three fixed public fixtures and
twelve arms. Preserve both unsuccessful attempts and successful witnesses.
The replayer uses separate ordinary-residue affine curve arithmetic, with
no producer curve, solver, recovery or Montgomery implementation calls.

Required checks:

- Bind the fixture, full curve model, ICV1 identity, generator and target.
- Independently check group order, every labelled point uG+vQ, unique
  coordinates and partition membership.
- Check reported coordinate sets by signed point sums, including grouped
  four-point memberships and the label certificate for orbit pruning.
- Check recovered A, B, nonzero B, d=-A/B and dG=Q.
- Keep early stop, timeout, exhaustion and unverified completeness distinct.
- Reject corrupt labels, points, models, coordinates, signs, coefficients,
  answers, duplicate JSON/map keys and contradictory outcome flags.
- Replay native producer fixtures and preserve deterministic input hashes and
  reports. No timing experiment or historical Python path is executed.

Fourteen actual native producer transcripts replay successfully across all
twelve arms and the three CLI fixtures. All nine replay/corruption tests pass
both in the standalone package and as a repository example. These are the
same nine tests in two build contexts, not eighteen distinct tests.
No performance result, solver novelty, general theorem, leaderboard entry,
or change to either index-calculus track's baseline is claimed.

## Run the verifier

The standalone package links only serde, serde_json, clap and sha2. It does
not depend on crypto_lib. Its Cargo.lock pins its separate build.

    cargo run --locked --manifest-path research/aps_independent_replay_20261029/native/Cargo.toml -- research/aps_independent_replay_20261029/fixtures/line3.json
    cargo test --locked --manifest-path research/aps_independent_replay_20261029/native/Cargo.toml
    cargo test --release --locked --example aps_single_relation_replay

The CLI writes a replay receipt to stdout or exits nonzero on inconsistency.
An accepted no-witness transcript reports zero verified recoveries. Its
producer_status remains a producer report; even a reported complete search
has enumeration_completeness_verified=false. Empty or omitted solver output
does not prove absence of a relation. Reported counters and RNG seeds are
parsed but not certified.

Fixtures are in [fixtures/](fixtures/), deterministic replay outputs in
[validation/replays/](validation/replays/), and development checks in
[validation/summary.json](validation/summary.json). The
[generation script](validation/generate-fixtures.sh) accepts a producer binary
and a new output directory and refuses to overwrite existing evidence.
All use the existing seed 20261026 and bounded CLI parameters.

Initial generation completed nine cases, then the guaranteed-label orbit
case hit the producer's short/overlapping-orbit guard before JSON emission;
four later cases were not attempted. The
[failure record](validation/failures.json) preserves it. The three final
orbit cases use the same random-label positive fixture as the producer's
existing orbit unit tests, with the same seed. The first nine generated files
were preserved and the remaining five cases generated separately. This is a
curated correctness corpus, not an unbiased yield or performance population.

## Reports and unchanged measurements

[Report PDF](reports/report.pdf), [editable source](reports/report.typ),
[replay diagram](reports/replay.svg) and [diagram source](reports/replay.dot)
record the contract and actual checks. The diagram explains verification;
there is no timing population or cost comparison to plot.

The canonical IC leaderboard, index-calculus scoreboard, progress timeline
and performance-gains sources were checked. This verifier adds no admitted
measurement or curve route, so their values and rendered copies are unchanged.
No benchmark row, VALIDATION.json, experiment export or panel completion is
claimed. The focused [problem record](../../problems/aps-single-relation-complete-cost.md)
retains the accounting, export, ladder, checkpoint and protocol obligations.

## Sources checked before implementation

- Amadori–Pintore–Sala, revised author manuscript, sections 3 and 3.1:
  https://iris.unitn.it/retrieve/e3835194-60e8-72ef-e053-3705fe0ad821/Amadori%2C%20Pintore%2C%20Sala%20-%20On%20DLP%20for%20prime-field%20EC%20%28rev%29.pdf
  Labelled points give a signed zero sum, followed by one scalar congruence.
  The browser extractor returned 403; direct public HTTPS retrieval succeeded,
  and the relevant full text was read. The third-party PDF is not redistributed.
- Milne, Elliptic Curves, second edition, I section 3, pages 25–28:
  https://www.jmilne.org/math/Books/EC2.pdf
  Group law, inverse and infinity cases specify the independent affine checker.
- research/aps_followup_20261028/README.md and its proof/validation maps:
  bounded correctness exists; a separate EC replayer remains an explicit gap.
- Sibling ECDLP atlas: LEAD-ICP-algebraic-membership-factor-base and
  LEAD-ICE-rootfind-pairtable-oracle. This work changes neither their
  obstructions nor performance dispositions. The old sibling URL redirects
  to isogeny-labs/crypto-autoresearcher.

## Limits

Only the producer CLI's fixed tiny fixtures and bounded transcript schema
are in scope. A signed point-sum checker certifies the supplied witnesses,
not that the solver listed every root, sampled fairly, obeyed its deadline
or charged its work completely. Producer counters remain diagnostic.
No S3 checkpoint upload or performance job is attempted. Earlier rejected
shared-log and checkpoint uploads are not retried.
