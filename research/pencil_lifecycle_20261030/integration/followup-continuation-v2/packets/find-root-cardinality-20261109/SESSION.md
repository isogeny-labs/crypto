# 2026-11-09 — source-only root-cardinality construction

## What was done

Prepared source proving preservation of root labels and root predicates under existing Run; connected returned roots to those labels and defined root-only cardinality preservation. Added actual recursive Run and stale-child controls, plus exact source bindings.

## Decisions made

Reuse the existing rank/semantic theorem rather than duplicate its recursive machinery. Keep the literal word bridge, initialization, node growth and merged-component induction explicit. No source has been compiled.

## Open threads

Independent source review, pinned import closure and bounded prospective replay remain required. No runtime allowance is allocated here.

## Errors

No new compiler/error result. The source is prospective. USize controls explicitly retain the predecessor's64-bit platform assumption.
