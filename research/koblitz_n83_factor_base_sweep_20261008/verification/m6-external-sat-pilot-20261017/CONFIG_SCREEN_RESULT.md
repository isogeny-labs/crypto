# Four-arm external SAT configuration screen: all censored

The [frozen two-by-two panel](CONFIG_SCREEN_PROTOCOL.md) used the same Gray
`all_x` planted-witness XOR-DIMACS file as the preceding native and
CryptoMiniSat controls. The [independent audit](audit_config_screen.py) passed
all four raw command, hash, status, cap and resource checks; its
[sanitized receipt](config-screen-audit.json) binds the eight raw stdout/stderr
files by tree SHA-256
`d4ffaae8dc23154ffa6469c65ba479377d8122bc62807d6950cc032ea22f4580`.
The input SHA-256 remained
`c62fe682879435e8f1ee30a80ac7830f2aef81dd4f600d10a004ac45a4f914b8`.

| Branching | Gaussian limits | Disposition | Wall, local L0 | Peak RSS, local L0 |
| --- | --- | --- | ---: | ---: |
| Default | Default | `INDETERMINATE`, exit 15 | 58.664 s | 320.8 MB |
| Default | Wide | Outer timeout, killed after 150 s | 150.006 s | unavailable |
| VSIDS | Default | `INDETERMINATE`, exit 15 | 126.918 s | 95.5 MB |
| VSIDS | Wide | Outer timeout, killed after 150 s | 150.007 s | unavailable |

The two timed-out wide-Gaussian calls emitted no status or macOS time-wrapper
resource line. Their RSS and conflict counts are **unknown**, and a process
check found no surviving worker after cleanup. The default arm reproduces the
prior one-million-conflict censored result; the VSIDS arm also has no model.
The four full 150-second outer envelopes are conservatively charged at **600
seconds**, separate from validation cost and the earlier model-pin control.

These settings did not produce a positive solver-access arm and supply no
natural relation, useful rank, linear-algebra or individual-log measurement.
They do not rank factor bases or total index-calculus runtime;
`selected_best_total_runtime` remains null.
