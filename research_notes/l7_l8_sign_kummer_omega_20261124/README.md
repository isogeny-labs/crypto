# L7/L8 — sign/Kummer audit and Ω(p) closure (2026-11-24)

**Leads:** L7 and L8 in `crypto-autoresearcher/docs/open-leads-20261123.md`,
from G32-003 and G32-004 in `research_goals_20261121_ic_tranche2_batch.md`.

## L7 — sign/Kummer bookkeeping audit

**Finding:** at the proposal level, G30-013 (cyclic-cover
label-conditioned smoothness) and G30-015 (split-Jacobian projected
smoothness) are unexecuted, and G31-003's arm is a counting design. None of the
three declares a `sign_convention`, and every stated density in those notes is
the full x-coordinate form

```
|B|^m / (m! · #G).
```

That formula silently assumes the signs work out. The executed Semaev/F4
engines in this repo (`examples/ic_semaev_gb.rs`, `src/cryptanalysis/f4_fp.rs`)
also operate on x-tuples; their density statements do not charge a sign search.
So the audit does **not** find an engine that already charges the factor. The
kill condition ("every engine already charges it") is not met; the result is the
contract field plus a per-engine correction.

| engine / lead | status | sign convention | density charge | corrected factor |
|---|---|---|---|---|
| G30-013 cyclic-cover labels | proposal, unexecuted | absent | full x-density | `2^m` if signs are searched; `1` if one representative per ±-orbit |
| G30-015 split-Jacobian | proposal, unexecuted | absent | full x-density | same |
| G31-003 large-prime arm | counting design | absent | full x-density | same |
| `ic_semaev_gb.rs` (S4 + membership) | executed | x-tuples, no declared field | full x-density | `2^m` to recover point tuples |
| `f4_fp.rs` Semaev cells | executed | x-tuples, no declared field | no explicit density claim | `2^m` if converted to point tuples |

**Recommended contract field:**

```yaml
sign_convention: full | kummer | one_rep
# density_multiplier:
#   full:     1        (x-tuple density; a point-tuple solver pays 2^m)
#   kummer:   1        (relation in E/±; the double-cover consistency condition
#                       replaces the 2^m search)
#   one_rep:  2^-m     (one representative per ±-orbit, relative to full points)
```

The multiplier must be stated next to every density or relation-count number.
This is metrology, not a new attack.

**What this does not establish:** it does not audit unread proposal text beyond
the three named leads, and it does not claim any engine is wrong — only that no
engine currently records which convention it uses.

## L8 — any full-sumset method costs Ω(p)

**Lemma.** Let `G` be cyclic of order `N = Θ(p)` and let `m ≥ 1`. A method that
outputs the complete m-fold sumset, or the full decomposition-count table for
all targets, must write `Ω(N) = Ω(p)` entries in the dense representation.
Each entry carries at least one bit of information in the worst case (target
present/absent, or a count), so the output alone is `Ω(p)` bits and the method
is `Ω(p)` time in the word model. A cyclic convolution computes the same table
in `O(N log N)`, so with FFT the cost is `Θ(p log p)`; the lower bound is
information-theoretic, not a model artifact.

**Consequence.** Since rho solves the same DLP in `O(√N) = O(p^{1/2})` group
operations, "build the entire relation table on a GPU" is dominated by rho by a
factor `~p^{1/2}/log p`, unconditionally, for every base and every `m`, whenever
the full dense table is the output. Sparse/structured convolution that computes
only the needed slice, or the case where the m-fold sumset has small support, is
**not** covered; that would be a genuine algorithmic result, not a loophole in
this bound.

**What this does not establish:** no lower bound for partial-target or
sparse-output algorithms; no statement about memory-only or streaming variants
that never materialize the table.

## Files

This note only; no code or measurement is claimed.
