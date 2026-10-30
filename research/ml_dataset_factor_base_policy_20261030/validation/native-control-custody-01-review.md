# Parent scoped review before isolated compilation

The helper is confined to an exclusive copied cell and constructs edits from
nine fixed recipes. Original outer hashes and any worker registry are checked
before mutation. JSON replacements and rewritten custody documents are prepared
and capped before writes. Registry updates precede outer hash calculation, result
changes propagate identically to manifest.outcome, and removed files are removed
at both levels. Post-write checks and an unchanged-original comparison are
mandatory; partial I/O failure is not a successful negative.

Five authored unit tests cover all nine recipes using the actual library storage
verifier, plus unchanged originals/unrelated files, one-point/malformed-digest
prerequisites, stale outer hashes, duplicated registry with valid outer hash, and
same/nested/symlink sources. Fixtures intentionally test custody, not arithmetic.

No static blocker found to compilation. Tests have not run. Parent review is not
independent review. The module will be included only under cfg(test) for the next
controller example validation. Actual16-slot selection, commands, diagnostic
classification, global copy/storage admission and workflow ledger remain pending.
When wired for execution, bind its source in the acceptance manifest and implement
the root-run side of slot10; the current helper explicitly reports that obligation.
Do not label this helper as a completed CLI control workflow.
