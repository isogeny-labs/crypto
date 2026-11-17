# Root library validation: failed first attempt

Run t1958-root-library-20261109 failed before compiler invocation. The worker commands array is empty. It waited10.091595156001858 seconds for an ownership permit that never arrived. Conservative campaign accounting charges1,800 active worker-seconds. No library test result or compilation outcome exists for this attempt.

## Recorded failure sequence

The launcher submitted the systemd worker, then a checkpoint tried to read supervisor/unit.stderr. Systemd had created unit.stdout and unit.stderr as root-owned0600 files. The unprivileged flush_logs operation raised PermissionError. The raw initial systemctl query already contained invocation4342ad37f3dd4cbbadf95ff27001cf69, but the exception interrupted normal ownership registration and permit issuance.

The supervisor receipt remains status=failed, phase=terminal-unsealed, seal_status=failed. Its cleanup path recorded cannot stop unit without exact owned InvocationID, followed by cannot seal before owned cleanup. Those original failures are preserved unchanged. The worker recorded supervisor ownership permit not received, retained commands=[], and exited1 after its bounded permit wait.

The later root-terminal.json joins the raw initial query invocation with the native worker and terminal unit. Its terminal readback records MainPID=0, Result=exit-code, ExecMainStatus=1, ActiveState=failed, same invocation, and absent cgroup. This later custody reconciliation is a separate record; it does not turn the earlier unsealed supervisor attempt into a success.

## Costs and controls

Worker elapsed10.091595156001858 seconds, supervisor elapsed0.4207382969907485 seconds and caller elapsed0.537700398999732 seconds measure different process intervals and must not be added as a reconstructed campaign runtime. Conservative charge is1,800 seconds regardless of the early failure. Final CPU time and peak RSS are not recorded.

Native worker cgroup controls show2 CPU quota,17,179,869,184 memory bytes, zero swap and512 tasks. The launch argv retains1,779-second systemd RuntimeMaxSec,5-second stop grace and isolation settings. Exact command and source/publication binding remain in launch-receipt.json and release.json. This archive did not change file ownership or modes, retry, compile, or run tests.

## Evidence and next step

The11 native/control files are archived with original ownership/mode metadata. Root-owned logs were read through sudo tar; no chmod/chown was applied. Archive SHA256 bdf179062b22a11b89f9d94ce20e077f5b98b9a81e4e0fc3508398a62b954385,4,110 bytes, lives in the content-addressed artifact store. A separate GET matched its entire SHA. .artifacts.json and raw-manifest.json provide exact replay custody.

The producer is preparing a separately reviewed bounded successor that addresses log ownership. This failed attempt, its1,800-second debit and its unsealed supervisor state remain immutable. The graph shows where this attempt stopped; a timing comparison would not be meaningful for these different process intervals.
