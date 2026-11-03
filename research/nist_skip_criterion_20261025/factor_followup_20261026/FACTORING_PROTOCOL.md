# Remaining P-521 n+1 cofactor: native follow-on methods

The three other requested targets have passed independent native prime and
product checks. The remaining composite is the 125-digit, 414-bit residual
in `runs/refine-001/p521-1136-plus.residuals.txt`. It is not treated as prime.
The user confirmed the scope is completing factorizations and updating the
curve comparison, without adding a DLP experiment.

The next native backend is YAFU, source commit
`c7d90283972820f99b3b4920bfc1eff05c1f6342` from
<https://github.com/bbuhrow/yafu>. Build: GCC, GMP/ECM from the existing
native installation, `ECM=1 OMP=1 USE_AVX512IFMA=1`. The current host
advertises AVX-512 IFMA. No other host compatibility or speed claim is made.

Two distinct jobs use fresh working directories and immutable raw logs:

- Vector ECM: 1,024 requested curves, B1=11,000,000, two threads, seed
  20,261,026, 1,800-second watchdog, saved stage-one residues.
- Native GNFS: YAFU's NFS driver and bundled GGNFS sievers, two threads,
  seed 20,261,027, first 3,600-second watchdog, retained polynomial,
  relation and other progress files. No Python factoring driver runs.

The machine's CPU quota is four cores; these jobs together use at most four
configured threads. Each has a successful S3 checkpoint before launch and
another after its attempt. A timeout is a recorded incomplete attempt and
does not imply that the composite cannot be factored. Preserve progress for
continuation rather than silently restarting the search.

No factor is admitted based on a backend's printed prime label. Refine the
proposed factorization, check its exact product, then use the independent
PARI primality/product verifier on the full original target. Stop redundant
workers once that complete proof has succeeded, retaining their stop reasons.
The full-cohort `n+1` feature family stays unavailable until every row has
a proved complete factorization. The historical selection question stays
separate from integer-factor completion.

Launch amendment: the first two YAFU invocations exited zero without
evaluating the target because non-terminal stdin selects batch mode. They
remain recorded as failed admissions, with their original launcher source.
The corrected invocations use `ecm(@,1024)` or `nfs(@)` with the validated
integer file explicitly redirected to stdin. Algorithm bounds and seeds
are unchanged. Execution markers establish input admission only; factor
completion still requires the independent primality/product verifier.

Comparison snapshot amendment, 26 October: `summarize_factors.cpp` reads all
21 exact rows and the admitted complete records, recomputes every target,
rechecks all prime factors and their products, and emits descriptive trait
tables and hexadecimal factorizations. The single unresolved row stays
`NA`; this table does not supply an incomplete feature to the rule search.
The unchanged search already replayed 1,118 features on all 21 rows with
complete `n-1`, twist and discriminant families, leaving `n+1` disabled.

ECM continuation amendment, 26 October: attempt008 reached a last printed
counter of704 before its 1,800-second watchdog (exit124), with no
factor printed. That is not completion of all1,024 requested curves.
A fresh attempt011 requests320 curves, B1=11,000,000, B2=11,000,000,000,
two threads, seed20,261,028, a3,600-second watchdog and saved B1 residues.
It uses a separate seed and larger stage-two bound, preserves attempt008
unchanged and is not represented as a replay of its unfinished curves.
Only actual completed work is counted. Native launcher parameters and
source/protocol snapshots are retained before this new attempt starts.

Accounting correction: the launch snapshot's phrase "704 completed curves"
was too strong. The upstream vector loop prints a batch counter before
and after processing it; the final704 line includes ETA and is a start
marker. The preceding688 end marker is present. Retain the interrupted
batch as partial rather than counting704 completed curves. Also, the
upstream progress text hard-codes "B2=100*B1". The explicit B2 flag in
attempt011 flows through `ecm_obj.B2` to `STAGE2_MAX`; its factor.log records
the11,000,000,000 setting. Preserve both the raw display and this source
audit; do not infer a numeric bound from that hard-coded progress label.

GNFS progress continuation amendment, 26 October: if attempt009 closes
under its watchdog before a job file is finalized, `run_nfs_resume.sh` can
copy its immutable candidate blocks into a fresh native master file with
the validated input header. The original thread file stays unchanged.
YAFU's own `-R -np 1001,1002` path parses the existing candidates, resumes
one leading-coefficient range (the backend overrides start to its recovered
last coefficient) and writes its selected native `nfs.job`. This first
phase has a900-second watchdog. Only exit0 plus a nonempty job file permits
the native `nfs(@) -R` continuation, two threads, seed20,261,029 and a
10,800-second watchdog. All polynomial, sieving, filtering and later
progress remains in the new directory and is checkpointed. No Python NFS
driver runs. Prior work is preserved separately; backend reset timers are
not a measurement of total campaign cost. A selected polynomial, relations
or a successful backend exit is not complete factorization admission.

Polynomial admission amendment: a native C++/PARI checker validates every
complete degree-five candidate's common modular root before building the
resumable master file. It rejects a wrong complete polynomial, duplicate
coefficients or an incomplete interior record; an interrupted trailing
record is explicitly counted and retained only in the immutable source.
Positive, wrong-root and trailing-partial controls use the public synthetic
integer15 and `x^5-1` with root1, before the named residual is admitted.
Build and admission failures retain their statuses and are checkpointed.

Launcher correction: after attempt009's native timeout and checkpoint,
its outer shell reported an unterminated-quote error because the launcher
file had been edited during its wait. The backend's exit124 and candidate
files survive unchanged; this is separately retained as an outer-launcher
failure. Attempt011's launcher will not be edited while it runs. The GNFS
continuation executes an immutable temporary script copy with explicit
`NIST_FACTOR_BASE`, so later source edits cannot change its open script.

Resource-continuation amendment: after ECM011 closes, verify the actual
NFS012 PID/cwd/executable and send one graceful SIGINT to save its progress.
Preserve its resulting nonzero status as a requested stop, not a factor
finding or a watchdog event. Archive and verify that closed progress before
starting a fresh `run_nfs_continue.sh` attempt014 with four threads, seed
20,261,030 and a14,400-second watchdog. Copy its exact job, factor base,
relations and completed-q ranges; require the selected job to match a
separately checked common-root snapshot. All original progress stays.

Attempt014 asks the existing native driver to try filtering after1,000,000
relations and raise that threshold by50percent if no usable matrix results.
This is an adaptive internal factoring parameter, not a reduced completion
criterion. Sieving must continue on insufficient relations; no prime or
factorization is admitted without the unchanged independent full-target
proof. No speed improvement or completion-time prediction is claimed.

Observed stop correction: the master ignored the first SIGINT while its
worker threads waited inside `system()`. The pinned native source documents
this behavior. A single SIGINT to a separately verified own GGNFS child
made that child return2 (shell status512), set NFS_ABORT and close the
driver with status1. The final log records2,313,299 merged relations.
Both signal records are retained. No second SIGINT was sent to the master.
The closed backend progress archive is stored and fully read back before
attempt014 starts; its launcher PID is recorded before the launch checkpoint.

Two checkpoint packaging attempts observed active relation files changing
during tar reads; one returned nonzero. No scientific result uses those
in-flight tar copies. The closed, immutable backend archive and its member
hash verification are the durable resume record for the new continuation.
