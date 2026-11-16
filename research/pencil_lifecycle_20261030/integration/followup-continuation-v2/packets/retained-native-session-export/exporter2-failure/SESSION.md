# T1499 exporter2 failure

## What was done
One authorized compile/control sequence failed during compilation; raw receipt and logs retained. No tests or scientific runs.

## Decisions made
Preserve the failure and original v2 source. A separate fixture-only successor is proposed, with no automatic retry.

## Errors
E0624 private identity_view; this was missed during prelaunch source review.

## Open threads
Independent successor recipe review and root release, then exact twelve-control acceptance if compilation succeeds.

Publication: both official artifact objects freshly downloaded and hash/size verified; all 38 archive members checked. PDF visually inspected. Raw compiler stderr and TeX logs retain exact bytes as base64 and inside the archive.
