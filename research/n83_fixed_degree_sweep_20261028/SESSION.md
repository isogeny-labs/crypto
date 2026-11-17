# Session 2026-10-28

What was done: user explicitly requested the fixed-base regularity rerun. Read
prior runner/design/algebra, fixed base identities and primary S3 paper. Checked
live EKS jobs: no active duplicate. Scoped conductor check passed for this study.

Decisions: preserve96 attempts and 40s/3GiB per-process budgets, fix ell27, validate
every planted coordinate against stored point data. Use existing Spot host with
pinned Singular container because EKS shared-bucket failure remains unresolved.

Open threads: freeze source/protocol, execute, preserve all outcomes, publish.

Errors: shared docs/session-logs path reserved by T-671; this study-local log
avoids that scope. Docker requires existing sudo access; no permissions changed.
Singular manual web fetch failed; installed version controls remain available.

## Production completed 03:41:23 UTC

What was done: all 96 attempts executed from03:25:12 UTC with the exact fixed
base hashes. Four affine computations completed, 63 timed out and 29 hit memory
limits. No top-form degree completed. Terminal exit0 means the sweep driver
finished all attempts, not that every mathematical computation succeeded.
Five native tests, eight controls and the full4966-test root suite passed;
122 root tests ignored. Source05d5a84ac7f and frozen protocol committed/pushed.

Decisions: retain all failures and the separate affine degrees. Preserve the
prior balanced-size evidence. Archive raw data and update the existing PR.

Open threads: finish archive/export validation, report rendering and PR delivery.
Exact regularity values remain unresolved; the prior prime dimension argument
is representation-specific and is not a solver result.

Errors: no production infrastructure failure. Resource-limit outcomes are
recorded per attempt. Current main-branch CI sparse-checkout failure on the
previous PR head reports missing factor-base; no unrelated workflow edits here.

## Delivery validation

All541 raw-manifest hashes pass. The completed deterministic archive uses xz
preset0, one thread; an incomplete preset6 packaging attempt was stopped before
upload because compression was slow. No producer data or measurement changed.
The final archive was stored through the content-addressed shared-bucket tool.
The export checker reports zero findings. Both PDF pages were rendered and
visually inspected. Report rendering initially used the reserved Typst word
show as a function name; renamed the presentation helper and rendered successfully.

## Data publication and report delivery

### What was done

Data/evidence commit79d719fdba4 was pushed before the derived report. The
346484264-byte archive was downloaded in full and its SHA-256 matched; the
archived manifest matches the verified541-file manifest. The artifact remote
checker reports one hydrated archive and zero problems. The export passes file
verification with zero findings. Both final PDF pages were inspected.

### Decisions made

Retain raw Singular version output byte-for-byte, including its whitespace,
using a scoped Git whitespace attribute. Keep the regularity problem open.

### Open threads

Exact-head CI and review remain separate from the completed bounded rerun.
Finite regularity values are still unresolved under the recorded limits.

### Errors

No unresolved delivery or archive verification error. Earlier main CI failures
reported a missing factor-base package in the workflow's sparse checkout.
