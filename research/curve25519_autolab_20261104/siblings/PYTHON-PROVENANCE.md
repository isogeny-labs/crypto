# Python provenance of the sibling-points stages, and the native replacements

AGENTS.md "Implementation language: no Python" forbids Python for research result generation. The
independent validator of Stage 4 flagged the helpers below (erratum E7). Following the rule:
- Their committed outputs stay as evidence and are not deleted or rewritten. They are labelled
  here with the tool that produced them.
- The execution path is replaced by native `ecbench` commands. Every later stage (4b on) uses
  only these.

| helper (stages) | what it produced | native replacement (commit `4e2ea6c1be`) | check |
|---|---|---|---|
| `stats_bitops.py` (1, 2, 3, 4) | paired bitop/v1 ratios, bootstrap 95% CI, `bitops-*.json` | `ecbench compare --unit bitop/v1`: pairs each run's recorded `work.bitops_per_sqrt_r` by (workload, round); delta-method and cluster-bootstrap intervals; unit in `comparison_id` | Stage 4 j0b7-40 sib8-p512-aut3 / tw2cj-lazy: Python 0.348 [0.327, 0.370], native 0.348 [0.328, 0.369] |
| `scaling_fit.py` (3, 4) | slope of ln(ratio) on ln r, workload-cluster bootstrap, `scaling.json` | `ecbench trend --compare <w = 0 comparisons>` (fit `free_r`), which now accepts prime-field `icv1-fp` slugs (degree 1) | Stage 4 sib8-p512-aut3 / rho-cubic: Python +0.012 [-0.020, +0.044]; native +0.003 [-0.035, +0.042]. Different estimators (per-curve WLS against workload bootstrap), same reading: the interval contains 0 |
| `collect.py` (1, 2, 3, 4) | `results.json`, a collation of `ecbench compare` outputs | a shell script that only calls `ecbench compare --save` (Stage 4b `compare.sh`); the saved comparison files are the results | no computation of its own |
| `plot.py` (1, 2, 3, 4) | SVG figures from the JSON above | kept: figure rendering from tool outputs, with no statistic of its own (research-visuals skill) | n/a |

Every number cited from Stages 1–4 that came from `stats_bitops.py` or `scaling_fit.py` is a Python
reading, made before this replacement. Stage 4's are also covered by the independent validator's own
recomputation (`stage4-aut3/validator-report.json`). New claims use the native commands.
