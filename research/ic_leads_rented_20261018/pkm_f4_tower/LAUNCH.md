# Launch card P3: native `f4_fp_tower` at m = 3, N = 21 (192 GB cell), if affordable

**Protocol:** [`research/pkm_lead1_m3n21_20261016/PROTOCOL.md`](../../pkm_lead1_m3n21_20261016/PROTOCOL.md), frozen 2026-10-16; nothing here changes it. Job: `pod/boot.sh pkm` (`run_pkm`, unchanged since the protocol: identity, `main 0`, `main 1` one after the other with `SHARE = 1`, confirmation, verify).
**Question and rule:** that protocol's §1 and §5 (D = 7 on both targets → plateau L = 6; D ≥ 8 or a productive degree-8 step → H0; a stop → inconclusive).

| | |
|---|---|
| host | RunPod `cpu3m-32-256` (32 vCPU, 256 GB ≥ the 192 GB the protocol requires for `SHARE = 1`) |
| price | **$1.76/h** |
| hours | protocol §4.5 (*derived*, not measured): 14–60 h per system on 32 threads, two systems in sequence → **28–120 h**, central about 60 h |
| expected cost | **$106** (range $49–$211); lifetime cap 125 h = $220 at most |

**Recommendation:** not at the current balance. Run it only after P2a: if msolve at `-t 16` finishes degree 7 or opens degree 8 at N = 21, this cell's question is answered more cheaply, and P3 becomes a cross-instrument check that can wait.

```sh
cd research/dreg_closure_m4l6_20261018/pod
python3 launch.py run --job pkm --name pkm-f4-tower-m3n21 --branch main \
  --flavors cpu3m --vcpu 32 --disk 100 --max-seconds 450000 --env SERVE_SECONDS=10800
# results: python3 launch.py fetch ID research/pkm_lead1_m3n21_20261016/runs --path pkm/runs/ ; then terminate ID
```

Tested: the identity stage ran on the authoring container on 2026-10-16 (protocol §7). The boot path is the one that ran the first m4l6 host.
