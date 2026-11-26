# Proposed report-evidence skill addition

Target: [report-evidence SKILL.md](https://github.com/isogeny-labs/workspace/blob/main/.agents/skills/report-evidence/SKILL.md),
under **Run and preserve evidence**, after the first evidence-retention bullet.

Proposed text for the maintained skill:

> Before binding a build identity or publishing a checksum manifest, explicitly
> stage nested dependency lockfiles and every named input, including files
> hidden by repository ignore rules. Compare each subject with its staged
> blob and final committed blob; a passing local hash check or clean working
> tree does not establish that the commit contains the reproduction inputs.

This applies newly adopted lesson `L-84b5496efed1` (two confirmations). The new
confirmation `T-d63418369992` records the exact Cargo.lock correction from
c6b870aa038488048ac691c9107ad74b0ba2ed40 to
1ab4844db046a65dabec9ce962c070ac18709e6a. The lockfile was used by the passing
native invocations and added unchanged before creating their identity/claim.
This PR proposes the text; it does not claim to have changed the workspace
repository's maintained skill.
