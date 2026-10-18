# L2 persistence and the tail layout on the campaign sigma-table walk

The [RTX PRO 6000 sigma-table sweep](../../AUTOSWEEP.md) (section 6.1) found
`PACKED_L2_PERSIST=1 PACKED_TAIL_LAYOUT=1` 6.55% faster than the
sigma-table build, in five alternating pairs with 300/300 replay and
identical DPs, at the automatic worker count (96,256 threads x 16 slots). The
sweep has no A/A panel, so [`gpujob-headline.sh`](gpujob-headline.sh) runs the
headline protocol of `benchmarks/sigma-fused/HEADLINE-PROTOCOL.md` on it:
device arithmetic suites, client integration, replay and sorted-corpus
identity, four warmups, five A/A pairs, five alternating A/B pairs of 64
launches, the frozen native summarizer. The control is
`gpu-rtx-pro6000-sigma-table`, and each sample must print the L2-persist,
tail-layout and CLMAD self-addend values its arm was built with.

## At the campaign's 385,024 workers: do not promote

[`results-385k/`](results-385k/). Both arms time 385,024 threads x 16 slots,
the campaign shape.

| | control | candidate |
|---|---:|---:|
| A/B median, B complete updates/s | 15.788 | 15.833 |
| paired ratios | | 1.0032, 1.0032, 1.0036, 1.0013, 1.0026 |
| median ratio | | **1.0032** |
| A/A maximum drift | 0.16% | |

Every pair favours the candidate, but by less than the protocol's promotion
bar, and the summarizer's verdict is `DO_NOT_PROMOTE`. Correctness passed in
full: all seven device arithmetic suites, integration, 300/300 re-walks with
zero drops, identical sorted 1,709,477-record corpora.

The reason is in the persisting window the engine reports
([`results-385k/l2-window.txt`](results-385k/l2-window.txt)): the 80 MiB cap
covers 83,886,080 of 104,726,528 field bytes (80%) at 96,256 workers, but
only 83,886,080 of 418,906,112 (20%) at 385,024. The sweep's gain depends on
the state fitting the window, and the campaign's worker count is four times
too large for that.

## At the automatic 96,256 workers: promote

[`results-96k/`](results-96k/). Source: commit `3606a4bf5` plus this
directory's per-arm worker option and `modal_job.py`'s app-name option,
committed together with these receipts. Same protocol and binaries; the control
still times 385,024 threads, the campaign shape, and the candidate times
96,256 (`CANDIDATE_BENCH_THREADS=96256`, the engine's automatic count on
188 SMs). Replay and corpus identity run both arms at 96,256, so the walk is
compared like for like.

| | control, 385,024 workers | candidate, 96,256 workers |
|---|---:|---:|
| A/B median, B complete updates/s | 16.029 | **17.088** |
| paired ratios | | 1.0673, 1.0661, 1.0661, 1.0652, 1.0659 |
| median ratio | | **1.0661** |
| A/A maximum drift | 0.52% | |

Every pair favours the candidate, the minimum ratio is 1.0652 against 0.52%
A/A drift, and the summarizer's verdict is **`PROMOTE_ENGINEERING`**.
Correctness passed in full as above, with the identical 1,709,477-record
corpus. The persisting window covers 83,886,080 of 104,726,528 field bytes
at this population ([`results-96k/l2-window.txt`](results-96k/l2-window.txt)).

What this does and does not establish. It is a device-local rate for the same
walk and the same per-update work, measured with the repository's promotion
protocol. It compares two worker populations, so it bundles the knobs with a
4x smaller walk population. A campaign moved to it would run 1.54 M walks
instead of 6.16 M; the distinguished-point tail and checkpoint shape change
with that, and existing run IDs keep their 385,024-worker shape when a pass
resumes. Whether and when to move the campaign is the campaign owner's
decision. `make gpu-rtx-pro6000-sigma-table-l2` builds the promoted binary;
run it at automatic workers (`--threads 0` or no `--threads`).
