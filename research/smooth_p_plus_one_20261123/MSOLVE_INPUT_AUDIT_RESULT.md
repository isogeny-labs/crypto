# Explicit zero terms change the separating form

2026-11-25 · `msolve_input_audit_v1` · Native offline diagnostic · PR #5142

In the isolated upstream msolve 0.7.5 parser, a non-leading `+0*x` term is
read as `+1*x`. Consequently, the recorded form
`1*A+126*t0+0*a1+0*a0` is parsed as \(A-t_0+a_1+a_0\), although its
literal mathematical value is \(A-t_0\). This is a concrete incompatibility
between the current serializer and that parser. Seven synthetic parser
controls and the saved-record comparison support this result; both claims
are producer-local and unsigned. [pcc:unverified]

Both retained F127 outputs satisfy the effective parsed form at all 13
previously audited assignments. The eight original equations parse unchanged:
the older output satisfies them at all 13 assignments, and the newer output
at none. This accounts for the older form residuals at the source-excerpt
scope, while the newer equation failure remains unexplained.
**PCC-20261125-1a862d** pins the retained-input result. [pcc:unverified]

| Output | Literal form valid | Effective form valid | Eight equations valid | Effective whole system valid |
|:--|--:|--:|--:|--:|
| Older | 1/13 | 13/13 | 13/13 | 13/13 |
| Newer | 0/13 | 13/13 | 0/13 | 0/13 |

![Recorded counts from this diagnostic and the prior saved-RUR audit. Each output has 13 saved assignments; this is a finite census, with no sampling interval or timing measurement.](msolve-input-residuals.pdf){width=97%}

## Mechanism and limits

The primary source is
[iofiles.c lines657–711 at v0.7.5](https://github.com/algebraic-solving/msolve/blob/1d0d10971467d83985b2f1197e5fe7e4ed4be24f/src/msolve/iofiles.c#L657).
The first coefficient has an explicit leading-zero branch. Subsequent terms
whose `strtol` result is zero are assigned +1 or -1 according to sign,
including an explicitly written zero. Thus the intended fixed-form family
and effective parsed family differ at form 0. Omitting zero terms is the
concrete serialization correction; it changes actual solver input and needs
fresh qualification before any corrected-policy result is claimed.

At saved parameter 10, the older assignment has
\((t_0,a_1,a_0,A)=(113,91,12,10)\). Its literal form residual is 24;
the effective residual is 0. The newer assignment
\((12,62,67,10)\) has literal residual 125 and effective residual 0, but
its original-equation residuals remain nonzero. All 26 original literal
residual vectors reconcile with the prior audit. [pcc:unverified]

\newpage

## Reproducible evidence and next integration

The fixture is \(E/\mathbb F_{127}:y^2=x^3+x+7\), order 109, depth 4,
target \((41,95)\), unchanged cofactor comparator. This component reads
saved coordinates; it finds no roots or relations and calls no solver.

```text
source:   ae710f5660a91320c4e974d637af21e68bbaa3f4
identity: 5567b3ffdf3b7ca11e6f67545c90553421650276
```

The [protocol](MSOLVE_INPUT_AUDIT.md) and
[reproduction guide](MSOLVE_INPUT_AUDIT_REPRODUCE.md) identify exact commands.
The separate before-call identity pins 14 files, all unchanged afterward.
Raw parsed terms and every residual are in
`evidence/msolve_input_audit_v1/audit.jsonl`; the manifest binds source,
controls, output, report and figure.

| Requirement | Observed status |
|:--|:--|
| Source/control qualification before retained inputs | Both native builds and seven parser, three literal-rejection and two residual controls pass; PCC-20261125-f4dbc1, unsigned |
| Preserve inputs and prior results | All 14 identity pins unchanged; old RURs and prior residuals retained |
| Check both saved records | Two planned, attempted and completed; all 26 assignments reconcile; no retry |
| Explain zero-term behavior | Reproduced with exact attributed upstream function bodies and bounded harness |
| Explain newer equation failure | Open; eight equations parse unchanged but none of its assignments satisfy them |
| Integrate corrected serializer and semantic acceptance | Not attempted here; separate qualification next |

The once-run wrapper used one worker, 1 GiB address space and 30 seconds plus
five seconds grace on the shared Linux development host recorded in
`postcall_identity.json`. It exited 0 with empty stderr. Invalid residuals
are descriptive output, so exit 0 does not mean the newer output is valid.
The local stream increased from 711 to 713 records, with no external delivery.

The C harness supplies only fields used by four byte-exact attributed
upstream functions; source ranges, Git blob identities and GPLv2 COPYING
are retained in `native/diagnostics/`. It is not linked into the production
driver. The installed package reports Debian 0.7.5-1, but this separate
compilation is not byte-identification of its parser. No installed-binary
rebuild, general completeness certificate or independent signed verdict is
provided. The two compiler signedness warnings are retained. The frozen
protocol retains one prose-lint finding on a negated certification sentence; the
published protocol and the excerpt's extra EOF newline remain unchanged.

The wrapper source writes the recorded input, removes the preceding output
and passes one-thread solver arguments. This source inspection found no
stale-output reuse; it is not an independent capture of historical input
files. Exact source references and unavailable KB searches are in the
protocol. The [previous residual audit](RUR_RECORD_AUDIT_RESULT.md) remains
unchanged, including its failed prediction that the old literal form held.

Next, canonicalize every fixed form by omitting zero terms, then qualify a
semantic gate that evaluates the complete decoded parametrization against
the intended equations and form. Degree equality alone missed the newer
failure. Both changes need shared accounting and output qualification before
another cost comparison. Existing study plots, scoreboard and progress
timeline retain their historical values; this diagnostic makes no performance
comparison.

PR publication remains draft while required object-store upload and byte
replay for eleven earlier raw files are unfinished. Canonical experiment
export access is unavailable; `development_flow.json` is explicitly local
metadata. The Cloudflare receiver URL remains unconfigured; 713 records are
queued locally.
