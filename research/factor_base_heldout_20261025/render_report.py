#!/usr/bin/env python3
"""Layout of native analyzer output into a reproducible LaTeX report."""
import json
from pathlib import Path

root = Path(__file__).parent
selection = json.loads((root / "results/selection.json").read_text())
evaluation = json.loads((root / "results/evaluation-summary.json").read_text())
validation = json.loads((root / "results/validator-report.json").read_text())
assert validation["ok"]

def cell(value):
    return "null" if value is None else f"{value:.6f}"

selected = selection["selected_policy"].replace("_", r"\_")
all_row = next(row for row in evaluation["summary"] if row["scope"] == "all")
train_rows = "\n".join(
    f"{row['policy']} & {row['verified']}/840 & {cell(row['representative_ratio'])} \\\\"
    for row in selection["candidate_scores"]
)
eval_rows = "\n".join(
    f"{bits} & {next(row for row in evaluation['summary'] if row['scope']==f'bits-{bits}')['verified']}/60 & "
    f"{cell(next(row for row in evaluation['summary'] if row['scope']==f'bits-{bits}')['representative']['candidate_over_rho'])} & "
    f"{next(row for row in evaluation['summary'] if row['scope']==f'bits-{bits}')['wins']}/"
    f"{next(row for row in evaluation['summary'] if row['scope']==f'bits-{bits}')['losses']} \\\\"
    for bits in (16, 20, 24, 28, 32, 36, 40)
)
tex = rf"""\documentclass[11pt]{{article}}
\usepackage[margin=0.75in]{{geometry}}
\usepackage{{graphicx}}
\usepackage{{array}}
\begin{{document}}
\title{{N12 model-held-out factor-base tuning}}
\author{{Public known-answer genus-2 study}}
\date{{2026-10-25 UTC}}
\maketitle
\noindent Protocol: \texttt{{factor-base-heldout/20261025/v1}}. Training contains 14 curve models and 840 targets per policy. Evaluation contains seven disjoint curve models and 420 newly derived targets. The model split and seven candidate policies were frozen before per-model tuning. Training selection uses summed candidate GAE divided by summed matched source-negation-rho GAE at $S/M=0.5$, $I/M=10$ and logical-operation price 0.1 source-addition equivalents.\par
\medskip
\noindent The frozen choice is \texttt{{{selected}}}; its training ratio is {cell(selection['selected_ratio'])}. The held-out evaluation ratio is {cell(all_row['representative']['candidate_over_rho'])}, with {all_row['verified']} verified candidate and {evaluation['rho_verified']} verified rho answers out of 420. The evaluation aggregate has {all_row['wins']} winning and {all_row['losses']} losing price cells among 36. A null ratio means incomplete candidate or rho coverage, not a zero cost.\par
\medskip
\begin{{center}}
\begin{{tabular}}{{lrr}}
Training policy & Verified & Candidate/rho \\ \hline
{train_rows}
\end{{tabular}}
\end{{center}}
\begin{{center}}
\begin{{tabular}}{{rrrr}}
Size (bits) & Verified & Candidate/rho & Wins/losses \\ \hline
{eval_rows}
\end{{tabular}}
\end{{center}}
\begin{{figure}}[h]
\centering\includegraphics[width=\linewidth]{{factor-base-heldout.png}}
\caption{{Native-analyzer representative-cell ratios for the seven training policies (840 targets each). The line at one is equal cost. The frozen evaluation ratio is printed in the graph footer.}}
\end{{figure}}
\clearpage
\begin{{figure}}[h]
\centering\includegraphics[width=\linewidth]{{evaluation-by-size.png}}
\caption{{Held-out representative-cell ratio on each unseen 60-target curve model, one per size. All other prices are retained in the CSV.}}
\end{{figure}}
\section*{{Evidence and limits}}
The raw records are in \texttt{{results/raw/}}; five reused training arms are frozen by the earlier replication artifact manifest. The standardized target files are \texttt{{evaluation-targets.jsonl}} and \texttt{{evaluation-targets.csv}}. All 36 prices for the aggregate, each model and each size are in \texttt{{evaluation-price-grid.csv}}; the matching training grid is \texttt{{training-price-grid.csv}}. The native validator independently recomputed {validation['training_source_scalar_replays']} training, {validation['candidate_source_scalar_replays']} evaluation candidate and {validation['rho_source_scalar_replays']} evaluation rho source-curve scalar products, plus {validation['training_price_cells_recomputed']} training and {validation['evaluation_price_cells_recomputed']} evaluation price cells.\par
\medskip
Every cost is a cold single-target operation count including setup and source verification. The reference is source negation rho in the same binary and process. The shared host earned no isolation tier; elapsed wall time is diagnostic and carries no speed claim. One deterministic target seed per target supplies no sampling interval. These observations are for the 21 fixed public Scholten models and this implementation; no leaderboard row is proposed.\par
\end{{document}}
"""
(root / "REPORT.tex").write_text(tex)
