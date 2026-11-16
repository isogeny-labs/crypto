import json, sys
IMG = sys.argv[1]
HS = "b01eeb6285af002d4808ed4b0476bdeb57f5831e41b72920092c2bfc2affbd4e"
def inst(extra=None):
    f = {"entry": "r2mllc_entry.sh SIZE... (runs python3 mllc_harness.py --sizes SIZE --out ladder_mSIZE.json)",
         "threads_env": "OMP_NUM_THREADS=OPENBLAS_NUM_THREADS=MKL_NUM_THREADS=2, PYTHONHASHSEED=0",
         "versions": "python 3.13.18, numpy 2.5.3, scipy 1.18.1, scikit-learn 1.9.1, joblib 1.6.0, threadpoolctl 3.8.0",
         "protocol_dict": "PROTOCOL in mllc_harness.py (sizes 12,16,20,24; 3 curves; null_retrains 20/10; alpha_family 0.01; gate_delta 0.10)"}
    if extra: f.update(extra)
    return {"repo": "crypto", "commit": None, "build": [], "binary_sha256": None, "flags": f,
            "env": {"OMP_NUM_THREADS": "2", "PYTHONHASHSEED": "0"}, "threads": 2, "image_digest": IMG.split("@")[1]}
def arm(i, role, desc, extra=None): return {"id": i, "role": role, "description": desc, "instrument": inst(extra)}
def pred(i, metric, direction, eff, seen, status, role, scope):
    return {"id": i, "metric": metric, "direction": direction, "minimum_effect": eff, "role": role,
            "provenance": {"written_at": "2026-11-07", "first_recorded_in": "research/ml_learnability_control_20261107/PROTOCOL.md section 8",
                           "data_seen": seen, "sample_status": status, "decisive_scope": scope}}
DEV = [{"path": "research/ml_learnability_control_20261107/dev/dev_m12.json",
        "measured": "development check at m=12 only (3 curves, null R=5): curve ~chance, PC-A, PC-B and PC-D accuracies; nothing at m>=16"}]
comp = lambda i, arms, d, design, thr=None, reason=None: {"id": i, "arms": arms, "differs_in": d, "design": design, **({"threshold": thr} if thr else {}), **({"reason": reason} if reason else {})}
P = {
 "schema": "experiment.protocol/v1", "protocol_id": "MLLC-LEARNABILITY-CONTROL-20261107", "version": 1, "created": "2026-11-07",
 "repository": "crypto", "study_dir": "research/ml_learnability_control_20261107",
 "authors": ["Claude Sonnet 5.5 (PROTOCOL.md draft 2026-11-07; protocol.json machine record 2026-11-08, round 2); frozen by the main session"],
 "links": {"narrative": "research/ml_learnability_control_20261107/PROTOCOL.md", "lead": "knowledge/leads/ecdlp/exotic/LEAD-EXO-ml-learned-models.yaml (crypto-autoresearcher)",
           "memo": "docs/reports/leads-round2-20261104/new-leads-memo.md item D4", "instrument_source": "research/ml_learnability_control_20261107/mllc_harness.py"},
 "question": "Can a generic learner (logistic regression, gradient-boosted trees, MLP) predict the lsb or msb of the discrete logarithm k from the bit encoding of the coordinates of Q = kP on prime-order curves of order N ~ 2^m, m in {12,16,20,24}, better than chance, and does the instrument have the power to see a leak of a stated kind (planted bit; homomorphic leak of graded multiplicative height)?",
 "claims": [
  {"id": "C1", "text": "For each (size, model, label) cell, the real-curve arm is classified SIGNAL, NULL or UNDERPOWERED by the mechanical rule in mllc_harness.decide(); a NULL is scoped by MDE(size) and by the PC-B height ladder.",
   "class": "comparative", "decided_by": ["C-curve-vs-chance", "C-curve-vs-relabel"],
   "if_supported": "SIGNAL: independent re-draw of curves before any claim; candidate statistic for LEAD-EXO-coordinate-statistic-leakage.",
   "if_not_supported": "NULL at all models and labels for a size closes the lead at that scale only if PC-B detects c=1 (PROTOCOL.md section 7)."},
  {"id": "C2", "text": "The instrument detects an explicit planted bit of advantage 0.10 on the curve feature layout (gate G) per size and model; where it does not, the cell is UNDERPOWERED and no NULL is claimed.",
   "class": "comparative", "decided_by": ["C-pca-vs-chance"], "if_supported": "NULL calls at that cell are admissible.", "if_not_supported": "Cell is UNDERPOWERED."},
  {"id": "C3", "text": "HD4: a planted homomorphic leak label = lsb(c^-1 u), u = c k mod N, is learnable only for small multiplicative height of c; random c reads chance at m >= 16 (heuristic, from the round-2 memo).",
   "class": "heuristic_estimate", "decided_by": [], "if_supported": "PC-B is a valid control as designed.", "if_not_supported": "Random c learnable at m >= 16: the memo's falsifier holds and the PC-B scope sentence is weaker."},
  {"id": "C4", "text": "No statement about attacks, speedups, rho, or learnability at group fractions above the sampled 4% (m = 24) or of memoriser-type leaks is made.", "class": "out_of_scope", "decided_by": []}],
 "hypotheses": [
  {"id": "H-mllc", "statement": "Held-out accuracy of every model class on the curve arm is 1/2 at every size (H0); PC-A and PC-B(c=1) are detected (control validity); random-c PC-B reads chance at m >= 16 (HD4).",
   "mechanism": "Bit encodings of (x, y) of kP carry no generic low-height linear/boolean structure in k; PC-B random c has multiplicative height ~sqrt(N), which trees and MLPs cannot fit once N >> 2^12.",
   "predictions": [
    pred("P1", "heldout_accuracy", "higher", "PC-A(delta=0.10) detected (pooled p < alpha_cell) by all models at m >= 16", [], "out_of_sample", "decisive", "m = 16, 20, 24"),
    pred("P1b", "heldout_accuracy", "higher", "PC-A detected only at delta=0.20 at m = 12", DEV, "in_sample", "calibration", "m = 12 (development data seen)"),
    pred("P2", "heldout_accuracy", "higher", "PC-B c=1 detected by all models at every size; c=2 detected; c=3 partly", DEV, "partially_in_sample", "calibration", "all sizes; m=12 seen in development"),
    pred("P3", "heldout_accuracy", "equal_within", "PC-B 5/3 and random c within chance (alpha_cell) at m >= 16; falsified if random c reads >= 0.9 at m = 16 (stated as the memo's falsifier)", [], "out_of_sample", "decisive", "m = 16, 20, 24"),
    pred("P4", "heldout_accuracy", "equal_within", "PC-D (Legendre in F_p^* bit encoding) within chance for all models", DEV, "partially_in_sample", "calibration", "boundary report, not a gate"),
    pred("P5", "heldout_accuracy", "equal_within", "curve arm within chance for all models, sizes and labels (prior ~97%)", DEV, "partially_in_sample", "decisive", "m = 16, 20, 24 (m=12 development seen)")]}],
 "arms": [
  arm("curve", "candidate", "Real prime-order curve points: bits of x, bits of y (2m features) of Q = kP; labels lsb(k) and msb(k); 3 independent curves per size, curve seeds 20261107+1000m+i."),
  arm("aa-curve-0", "control_aa", "Curve 0 of a size versus curve 1 (aa-curve-1): independent draws that differ only in the curve seed; pooled decisions use all three."),
  arm("aa-curve-1", "control_aa", "Curve 1 of a size (see aa-curve-0)."),
  arm("pc-a", "control_positive", "Curve features plus one planted feature equal to lsb(k) with probability 1/2+delta, delta in {0.02,0.05,0.10,0.20}; same code path and feature layout as the curve arm. delta=0.10 is the gate."),
  arm("pc-b", "control_positive", "Z/N affine encoding: bits of u = c k mod N for c in {1,2,3,5/3,rand0,rand1}; the HD4 height ladder. c=1 must be detected (identity encoding)."),
  arm("pc-d", "candidate", "F_p^* safe-prime group, h = g^k, label k mod 2 = Legendre(h); predicted blind in the bit encoding; reported as a boundary, not a gate."),
  arm("null-relabel", "control_negative", "The curve arm's own encodings with labels randomly permuted (train and test jointly): R=20 retrains (10 for mlp) per (curve, model, label); checks pipeline bias (NULL_BIAS flag).")],
 "controls": {
  "aa": {"present": True, "comparison": "C-aa-curves", "not_applicable_reason": None},
  "positive": {"present": True, "arm": "pc-a", "expected_detection": "PC-A delta=0.10 detected (pooled one-sided binomial p < alpha_cell = 4.2e-4) by each model at m >= 16; PC-B c=1 detected by all models at all sizes. A model that fails the gate yields UNDERPOWERED for that size, not NULL.", "not_applicable_reason": None},
  "negative": {"present": True, "arm": "null-relabel",
   "null_population": {"description": "Curve encodings with random label permutation: accuracy must be 1/2; mean null accuracy over retrains deviating from 1/2 by more than 3 binomial sd flags NULL_BIAS and voids the cell's NULL.",
                       "generator": "numpy default_rng(curve seed).permutation of the labels, R retrains with seeds seed+r+1", "n": 20, "why_n": "R=20 (10 for mlp for time); the primary test is the exact binomial, the retrain null only checks bias.",
                       "matched_on": ["curve", "model", "features", "split", "image digest"]}, "not_applicable_reason": None}},
 "variable_ledger": {"crosses_map": False,
  "independent": [{"name": "size_m", "levels": [12, 16, 20, 24]}, {"name": "model", "levels": ["logreg", "hgb", "mlp"]}, {"name": "label", "levels": ["lsb", "msb"]}, {"name": "arm", "levels": ["curve", "pc-a", "pc-b", "pc-d", "null-relabel"]}, {"name": "chance_reference", "levels": [0.5]}, {"name": "label_truth", "levels": ["true", "permuted"]}, {"name": "planted_delta", "levels": [0.02, 0.05, 0.10, 0.20]}, {"name": "height_c", "levels": ["1", "2", "3", "5/3", "rand0", "rand1"]}, {"name": "curve_seed", "levels": ["20261107+1000m+i"]}],
  "factors": [
   {"name": "software", "treatment": "fixed", "value": "one digest-pinned image; python 3.13.18, numpy 2.5.3, scipy 1.18.1, scikit-learn 1.9.1", "how": "versions.txt in the result set; the image digest is in every request."},
   {"name": "host", "treatment": "measured", "value": "any EKS Spot x86 node; CPU model recorded in host.txt", "how": "Accuracy counts are not clock metrics; MLP/HGB threading can change last digits, recorded as a deviation if reruns differ."},
   {"name": "threads", "treatment": "fixed", "value": "OMP/OPENBLAS/MKL threads = 2 per size process", "how": "set by r2mllc_entry.sh and the harness."},
   {"name": "seeds", "treatment": "fixed", "value": "curve seed 20261107+1000m+i, all other seeds derived from it", "how": "harness frozen rule; no seed chosen after a look."},
   {"name": "order", "treatment": "fixed", "value": "arm and model order fixed in code; every cell independent", "how": "no shared state between cells; cell seeds are derived, not sequential."},
   {"name": "job_split", "treatment": "fixed", "value": "sizes run as separate processes (one request per size or all in one request)", "how": "each size is deterministic given the seed; a preempted size reruns whole."},
   {"name": "curve", "treatment": "randomized", "value": "3 independent curves per size", "how": "seeded curve search; the within-size replicates are the A/A."}],
  "comparisons": [
   comp("C-curve-vs-chance", ["curve"], ["chance_reference"], "absolute", "pooled one-sided exact binomial p < alpha_cell = 0.01/24 = 4.2e-4 and >= 2 of 3 curves with p < 0.05 and accuracy above own null max."),
   comp("C-curve-vs-relabel", ["curve", "null-relabel"], ["label_truth"], "one_factor"),
   comp("C-pca-vs-chance", ["pc-a"], ["planted_delta"], "absolute", "delta=0.10 pooled one-sided p < alpha_cell (gate G)."),
   comp("C-pcb-ladder", ["pc-b"], ["height_c"], "absolute", "reported per c as detected / not detected; h* = largest height detected."),
   comp("C-aa-curves", ["aa-curve-0", "aa-curve-1"], ["curve_seed"], "aa", "per-curve accuracies of a size agree within binomial noise; a pooled SIGNAL needs >= 2 of 3 curves.")]},
 "randomization": {"arm_order": "fixed", "arm_order_reason": "Cells are independent deterministic computations with derived seeds and no clock metric; order cannot change a count.",
  "seed_derivation": {"method": "harness_drawn", "seeds": [], "nonce": None, "note": "curve seed = master_seed 20261107 + 1000*m + curve index; rng and model seeds derived from it in mllc_harness.run_size; recorded in the output JSON protocol dict."},
  "instance_sampling": {"rule": "k drawn without replacement from [1, N-1] (all of it at m <= 16, 7e5 sampled at m >= 20); curves drawn by seed; independent of arm.", "independent_of_arm": True},
  "blinding": {"analysis_labels_masked": False, "how": "The decision rule is mechanical (decide()), no analyst choice; the analysis is the harness's own decisions table recomputed from the JSON."}},
 "sample_size": {"instances": 3, "replicates": 1, "warmups": 0,
  "power": {"method": "exact", "alpha": 0.01, "tests_in_family": 24, "correction": "bonferroni", "passes": True,
            "note": "Family alpha 0.01 split over 4 sizes x 3 models x 2 labels = 24 cells (alpha_cell 4.2e-4). MDE at 90% power (normal approx.): per curve 0.081/0.020/0.0052/0.0052, pooled over 3 curves 0.047/0.012/0.0030/0.0030 for m = 12/16/20/24. Blind below ~0.05-0.08 at m = 12 by design."},
  "preflight_runtime": {"fixed_seconds": None, "per_sample_seconds": None, "threads": 2, "estimated_total_seconds": 7200,
                        "measured_on": "[derived] from one 1e5-sample timing probe on a loaded dev host (logreg 0.3 s, hgb 0.8 s, mlp 11 s per fit): about 2 h for each of m = 20 and 24, minutes for 12 and 16; not a protocol measurement."}},
 "metrics": {
  "primary": [{"id": "heldout_accuracy", "name": "Held-out accuracy", "unit": "fraction", "kind": "rate", "definition": "correct predictions over n_test on the held-out split of one (curve, arm, model, label); pooled over curves by summing counts.",
               "source": "ladder_m*.json sizes[m].curves[i].arms[arm][model] fields correct, n, acc", "aggregation": "sum", "resolution": "1/n_test", "uncertainty": "clopper_pearson_ci95"}],
  "secondary": [{"id": "p_one_sided", "name": "One-sided exact binomial p", "unit": "probability", "kind": "derived", "definition": "binomtest(correct, n, 0.5, greater).", "formula": "P(X >= correct), X ~ Binomial(n, 0.5)", "source": "same fields, p_one_sided", "aggregation": "per_row", "resolution": None, "uncertainty": "none_exact"},
                {"id": "null_mean_acc", "name": "Relabelling-null mean accuracy", "unit": "fraction", "kind": "rate", "definition": "mean held-out accuracy over R random-label retrains.", "source": "null_mean, null_max, null_accs", "aggregation": "mean", "resolution": None, "uncertainty": "stdev"}]},
 "measurement": {"isolation_required": "none", "warmups": 0, "order": "sequential", "clock": "none", "counters": [],
  "record": ["Image digest, python/numpy/scipy/sklearn versions (versions.txt)", "sha256 of PROTOCOL.md, mllc_ecgen.py, mllc_harness.py, mllc_job.sh (source.sha256)", "CPU model, nproc, memory, kernel (host.txt)", "Exit code per size, SHA256SUMS of every uploaded file"],
  "timed_primary_justification": None},
 "execution": {"backend": "isolab-k8s", "queue": "isolab.eks.command-request/v1 via launch-remote-command (EKS Spot, standard isolation)", "measurement_class": "training",
  "placement": {"requests": "eks/requests/*.json; cpu 4 per size job (2 BLAS threads in use), memory 16Gi", "local": "development checks only, labelled not evidence"},
  "timeout_seconds": 14400, "checkpoint": {"policy": "none", "interval_seconds": None, "grace_seconds": None, "restore_validation": "none", "after_restore": {"wall_time": "invalid", "counted_units": "valid"}},
  "retries": {"infrastructure_only": True, "max_attempts": 3}},
 "stopping_rules": [{"rule": "Fixed N: all 4 sizes x 3 curves x all arms and models; a preempted or lost size job is rerun whole with the same seeds. No optional stopping.", "kind": "fixed_n", "preregistered": True}],
 "invalidation_rules": ["Rows from a different image digest, library versions, harness sha256 or seeds than the frozen ones are inadmissible.",
                        "A cell flagged NULL_BIAS cannot support a NULL (reported UNDERPOWERED).", "A rerun that differs in more than the last digits from the first run is reported as a deviation."],
 "decision_rule": {"text": "Per (size, model, label) mllc_harness.decide(): SIGNAL if pooled curve p < alpha_cell and >= 2 of 3 curves have p < 0.05 and accuracy above their null max; NULL if not SIGNAL and gate G (pooled PC-A delta=0.10 p < alpha_cell) holds, scoped by MDE and the PC-B ladder; UNDERPOWERED if not SIGNAL and not G, or NULL_BIAS. The lead is closed at a scale only if NULL holds for all three models and both labels there and PC-B detects c=1. Mixed outcomes are reported cell by cell; no aggregation hides a SIGNAL. No gap to rho is reported: no solver runs and this is not a speedup claim.",
  "comparison_ids": ["C-curve-vs-chance", "C-curve-vs-relabel", "C-pca-vs-chance"], "threshold": "alpha_cell = 0.01/24", "statistic": "pooled exact one-sided binomial test of held-out accuracy against 1/2",
  "interval": "Wilson 95% CI per cell", "multiple_comparison_correction": "bonferroni", "effect_size_measure": "accuracy minus 1/2, with MDE(size) stated", "verdict_vocabulary": ["SIGNAL", "NULL", "UNDERPOWERED"]},
 "analysis_plan": {"script": {"path": "research/ml_learnability_control_20261107/mllc_harness.py", "sha256": HS}, "inputs": ["ladder_m12.json", "ladder_m16.json", "ladder_m20.json", "ladder_m24.json"],
  "outputs": ["decisions and ladder tables inside ladder_m*.json (computed by the harness at the end of each size)"], "blinded": False, "frozen_before_data": True},
 "reporting": {"export_schema": "experiment.export/v1", "review_schema": "experiment.review/v1", "graphs": ["accuracy by arm and model per size with Wilson CI and MDE; PC-B ladder by height"], "report_dir": "reports"},
 "inadmissible": ["Changing sizes, seeds, models, hyperparameters, alpha or the gate after seeing results", "Reading a 2^12 PC-B result as evidence about 2^16 to 2^24", "Reporting a NULL without its MDE and PC-B scope, or a NULL from a cell that failed the gate",
                  "Any statement about attacks, speedups or rho from this study", "Treating development checks (dev/) as evidence"],
 "literature": [{"ref": "docs/reports/leads-round2-20261104/new-leads-memo.md item D4", "provenance": "internal", "claim": "Heuristic HD4: planted homomorphic leak unlearnable unless multiplicative height is small.", "verified_by": "Read by the draft's author 2026-11-07; not re-read here."}],
 "amendments": [],
 "freeze": {"status": "draft", "protocol_sha256": None, "commit": None, "frozen_at": None, "filed_before_affected_data": True}}
json.dump(P, open(sys.argv[2], "w"), indent=2); print("written")
