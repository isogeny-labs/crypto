# Amendment 4: repair the cold-panel resume header

Frozen before the first held-out cold target arm in this cohort. The prior
v4 store at `/Volumes/SSD990-2/pkm-z2xz8-holdout-cold-rebuild-ledger-20261128`
has only a header and one busy-host check; its receipt SHA-256 is
`4f19ddd5e5fedcaa9f46270ced32e1c386b079de04522c92d495bcc59267b043`.
Preserve it unchanged.

The v4 runner serialized its `arms` tuple as a JSON array, then compared the
loaded array directly with the in-memory tuple on resume. It therefore
refused the old store before launching a target, even though every other
header field matched. This amendment permits only that representation fix:
write the arms as a list in the header and retain the original six-arm order.
Pin this amendment's SHA-256 in the new header and validate it at startup.

Use the same source commit, executable, build receipt, six curves, 96 target
scalars and relation seeds, source-independent map preflight, interval base,
oracle settings, caps, and quiet-host rules as Amendments 1–3. Start a new
append-only store at
`/Volumes/SSD990-2/pkm-z2xz8-holdout-cold-resume-v4-20261128`.
The old store contributes no timing or target outcome. Failed, censored, and
unavailable arms in the new store remain in its 576-case denominator.

Run the new runner at least twice against the same store before relying on
target measurements, so its header can be checked on actual resume. This
repair changes no candidate selection or promotion criterion. All positive
native work still needs a measured price before complete cold-cost claims.
