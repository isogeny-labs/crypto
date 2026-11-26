# Frozen offline input-parser diagnostic

2026-11-25. This follows [the saved-RUR residual audit](RUR_RECORD_AUDIT_RESULT.md).
No solver call, replacement parametrization or production change is planned.

## Source evidence and scope

Sources checked (KB): status and searches for "msolve zero coefficient linear
form parser parametrization" and "msolve RUR coordinates linear elimination
finite field bug" return wrapper exit127; no KB MCP is available. Read the
actual native transport/adapter, the installed msolve0.7.5 tutorial and Debian
0.7.5-1 changelog, plus upstream tag v0.7.5 at commit
`1d0d10971467d83985b2f1197e5fe7e4ed4be24f`. Read version-tagged
`src/msolve/iofiles.c`, `linear.c`, `msolve.c`, `main.c`, `src/neogb/io.c` and
`src/fglm/fglm_core.c`; exact file identities are in `parser_sources.json`.

The transport writes the exact recorded prefix plus comma/newline/form,
deletes an earlier output, and invokes `-f INPUT -o OUTPUT -t 1 -c 0`.
No source-level stale-output reuse was found. This inspection does not prove
the historical file bytes independently of the recorded caller.

In upstream `iofiles.c`, `get_coefficient_ff_and_term_from_line` treats a
non-leading term whose `strtol` result is zero as coefficient +1 or -1
(lines692–697). The first term has a separate explicit-zero case. Thus the
adapter's form0, `1*A+126*t0+0*a1+0*a0`, is expected to parse as
\(A-t_0+a_1+a_0\), while the recorded mathematical form is \(A-t_0\).
This predicts the older output's form failures. It does not explain the
newer output's eight original-equation failures.

## Frozen controls and offline comparison

Compile a standalone C harness around byte-exact excerpts of the upstream
variable-exponent, term-count, term-splitting and finite-field coefficient
functions. Retain upstream attribution/license and exact excerpt ranges.
The small harness supplies only the used structure fields and bounded
inputs; it calls no solver, Groebner basis or FGLM code. It is not a rebuild
or byte-identification of the installed Debian binary. Keep that distinction.

A Rust companion first runs seven synthetic parser controls: F127 form0,
zero-free form0, nonzero form1, F8191 form0, leading zero, signed inner zero
and a zero polynomial. Expected outputs retain the upstream behavior,
including its changed zeros. Three literal-parser rejection controls and
two finite residual controls also run. Keep all failures and qualify before
reading the retained scientific records through the new program.

After source/tests/build publication and a separately published executable
and input identity, run the two-record comparison once. Inputs are the
original `factorwise_extract_v1/f127_d4.jsonl`, original
`liftable_fibers_v1/f127_d4.jsonl`, and the unchanged
`rur_record_audit_v1/audit.jsonl`. Select the same F127 depth4 target(41,95)
cofactor record in each. Parse each of its eight equations and fixed form
with both literal native arithmetic and the isolated upstream C parser.
Evaluate the resulting terms on the 13 saved assignments, including all
cofactors and A. Reconcile every original residual with the prior audit.
Retain parsed terms and all effective residuals. No roots are searched.

Prediction: all eight original equations parse identically; the form changes
in both records. Both saved outputs should satisfy the effective form at all
13 assignments. The old output should satisfy all effective equations; the
new output should still fail them. Record any contrary result without retry.
Both inputs are attempted even if one errors; malformed input or a failed
prior-residual reconciliation makes the wrapper fail. Nonzero effective
residuals remain descriptive output.

Limit the once-run comparison to one worker,1GiB address space and30 seconds
plus five seconds termination grace. Preserve exact command, stdout, stderr,
exit, before/after identities, local stream capture, native source controls,
unsigned claims, report/figure/PDF and reproduction. The prior protocol,
raw outputs, missing panel suffix and production acceptance stay unchanged.
Any canonical serialization fix, semantic acceptance gate or new solver
panel requires its own qualification. No performance conclusion belongs to
this parser diagnostic.
