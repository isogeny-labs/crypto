# Follow-up proposal validation, 2026-10-22

This is document/metadata validation. No E18–E29 scientific experiment ran.
[Machine receipt](checks/VALIDATION.json), [artifact hashes](ARTIFACTS.json).

- Twelve designs and their twelve blocked, unfunded cards agree; all proposal dependencies are acyclic. Static validation checks 71 local links and the frozen fixture hash.
- Native backlog validation passes for all fifteen materialized cards with the merged-commit check against origin/main.
- The required root library invocation passed 4,470 tests, failed nine local socket tests with sandbox PermissionDenied, and retained 119 existing ignored tests. The socket-enabled replay of the affected module passed all 44 tests, including every one of those nine. Across the two invocations, all 4,479 nonignored tests have a passing record. The initial full invocation remains a recorded failure; no source was changed and no clean single-invocation full pass is claimed.
- Both JavaScript sources pass syntax checks; render.sh passes bash syntax checking. Staged source/docs pass the ordinary whitespace check. Raw test stdout is checked with blank-at-EOF disabled so its original trailing empty lines are preserved verbatim. The final twelve-page PDF was inspected page by page; no overfull boxes, missing glyphs or unresolved references remain in its final typeset pass.
- Frozen protocols, original raw runs, delivered result PDFs, catalog/browser, benchmark and leaderboard data are unchanged.

Raw check output is retained under [checks/](checks/). The proposal regeneration
script refuses to overwrite a promoted, protocol-pinned, runnable or funded card.
To validate the delivered files, run the static validator without its write flag:

~~~sh
node research/twist_routes_1_3_20261021/proposals-20261022/validate.cjs
target/release/backlog validate --all --merged-into origin/main
~~~

The previous PR head already had failing curve-cover and SEA Lean checks and
merge conflicts. Its validation hold remains, and Copilot was asked to merge
the base without rebasing, force-pushing or rewriting frozen evidence. Detailed
job-log downloads were unavailable through the environment network policy;
GitHub metadata/annotations and the existing hold were inspected. New-head CI
must be checked after pushing; these local checks do not establish remote CI
success or authorize merging through a failure.
