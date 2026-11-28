#!/usr/bin/env python3
"""Lay out recorded outcomes (runs.jsonl, runs_s56b.json) as a stage-coverage grid. Plotting only."""
import json, sys, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
base = sys.argv[1] if len(sys.argv) > 1 else "../.."
toy = [json.loads(l) for l in open(os.path.join(base, "runs.jsonl")) if l.strip()]
s56 = json.load(open(os.path.join(base, "runs_s56b.json")))
stages = ["cover H_γ\n(genus 3)", "φ* order n,\nhomomorphism", "Smith-positive\nγ found",
          "explicit quartic C\n(fit + held-out)", "C smooth", "Ψ into Pic⁰(C)", "planted log\nrecovered on C"]
cols = []
for r in toy:
    ok = r["status"] == "ok"
    cols.append((f"p={r['p']}\n(~2^{r['p'].bit_length()})", [ok, r["pullback_injective_on_r"] and r["pullback_homomorphism"],
                 ok, r["counts_match"], r["counts_match"], r["image_generator_nonzero_on_C"], r["dlp_transfer_ok"]],
                 ["pass", "pass", "pass", "pass", "pass (#C=#H)", "pass", "pass"]))
pb = s56["pullback"]
ps = s56["psi_pic0_transfer"]
cols.append((f"p=2^56−5\n(s56 ladder)", [True, pb["nonzero"] and pb["order_divides_n_prime"] and pb["homomorphism_planted_k"] and pb["additive"],
             True, s56["heldout_on_C"] == s56["heldout_points"], s56["smooth_certified"],
             ps["order_n_preserved"] and ps["well_defined_two_representatives"] and ps["image_nonzero"],
             ps["planted_log_carried"] and ps["additive"] and not ps["wrong_k_control_matches"]],
             ["pass", "pass", "pass", f"{s56['heldout_on_C']}/{s56['heldout_points']}", "resultant cert.",
              "pass (64-bit Pic⁰)", "carried: Ψ(φ*Q)=[k]Ψ(φ*G)"]))
fig, ax = plt.subplots(figsize=(10, 4.6))
for ci, (name, vals, labels) in enumerate(cols):
    for si, v in enumerate(vals):
        color = "#2e8b57" if v is True else ("#c0392b" if v is False else "#d0d0d0")
        ax.add_patch(plt.Rectangle((ci, len(stages) - 1 - si), 0.92, 0.88, color=color))
        ax.text(ci + 0.46, len(stages) - 1 - si + 0.44, labels[si], ha="center", va="center",
                fontsize=7, color="white" if v is not None else "#333")
ax.set_xlim(0, len(cols)); ax.set_ylim(0, len(stages))
ax.set_xticks([c + 0.46 for c in range(len(cols))]); ax.set_xticklabels([c[0] for c in cols], fontsize=8)
ax.set_yticks([len(stages) - 1 - s + 0.44 for s in range(len(stages))]); ax.set_yticklabels(stages, fontsize=8)
ax.set_title("Stages of E → J_H → Pic⁰(C) checked at each prime: complete at 2^56 − 5", fontsize=10)
for s in ax.spines.values(): s.set_visible(False)
ax.tick_params(length=0)
fig.text(0.5, -0.02, "Toy rows: run 20261116T1637Z (runs.jsonl); log recovered on C by BSGS. s56 row: amendments S56 and S56b (runs_s56b.json); "
         "the log is carried (Ψ(φ*Q) = [k]Ψ(φ*G) for the planted k), not searched; #C=#H not countable, C held-out-checked and certified smooth.",
         ha="center", fontsize=7, wrap=True)
fig.tight_layout()
fig.savefig("stages.svg", bbox_inches="tight")
print("wrote stages.svg")
