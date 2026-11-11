# The scan's stages on main, on the EPYC class rule v9 ran on

**What this is.** A stage diagnostic, declared 2026-11-11 before any run.
It measures where a scanned summand's time goes on today's `main` with
R04's instrument, on the node class rule v9 ran on: Iso Lab EKS
`c8a.2xlarge` Spot, AMD EPYC 9R45. It is a record, not a round's
measurement. It prices stages, not cold time, and decides nothing by
itself.

**Why.** The scoping of three IC speed-up levers (slope keys, one shared
inverse for each ± pair, a split-block presence filter) priced each lever
from the stage table measured on v8 on an Emerald Rapids host
([`../scan-stages-v8-20261017/`](../scan-stages-v8-20261017/README.md):
about 16–17.6 ns a summand). Rule v9 ran on EPYC, where its rows imply a
scan about twice as fast. Its stage split has never been measured. Each
lever's kill rule depends on that split; lever 3, for example, dies if the
admitted stage is under 1.0 ns a summand at 2^44.3 and 2^44.5.

## Instrument

- **Probes:** R04's probes, ported by hand onto `main` at `0a1969d089`
  (the v8 patch no longer applies). They sit at the same stage boundaries:
  subtract (the fused kernel), key, filter (including the hashing), and
  admitted (R12's run prefetch, lookups and finishing).
- **Build:** `ic` built with `--features scan-probes`. Default builds compile
  none of it. The hash is in [`binaries.sha256`](binaries.sha256).
- **Rows and order:** `M1`'s two rows at `k0n53`, `k1n59` and `k0n61` (the
  suite's, `suite/v1`), four rounds, as v8's
  [`probe.sh`](../scan-stages-v8-20261017/probe.sh) ran them
  ([`job.sh`](job.sh)).
- **Isolation:** `isolated_bench` on CPU 2 with `RAYON_NUM_THREADS=1`.
- **Read-out:** stages are summarised by v8's [`stages.sh`](stages.sh),
  unchanged.

## Route

One `isolab.eks.command-request/v1` Job ([`make-request.sh`](make-request.sh),
[`build-layer.sh`](build-layer.sh)):
- **Node:** `cpu-l2-spot`, profile `l2`, `c8a.2xlarge`, 4 CPUs and 12 GiB.
- **Deadline:** 3 hours. Results sync to S3 every 60 seconds.
- **No L2 admission:** command Jobs have none. The stage figures are a
  diagnostic, as on v8.

## Budget

Ceiling USD 3: `c8a.2xlarge` Spot for at most 3 hours, plus the launcher.
The owner authorized this run on 2026-11-11 ("All", answering whether to
run the EPYC stage probe while the monthly AWS budget is over its limit).
