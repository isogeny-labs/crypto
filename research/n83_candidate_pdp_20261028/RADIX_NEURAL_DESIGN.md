# Neural lead probe for the prime m=2 radix census

## Question and dataset

Can a small neural regressor trained on the retained prime m=2 radix census
rank alternative encodings below the established degree-42 baseline? The
input is the complete 3,010-row census under
`s3://setec-astronomy-artifacts/runs/n83-radix-census-20261030-r2/raw/`, bound
to raw manifest SHA-256
`6dfb197cd95ee385c72f8874c3acddd9661ab2000515503449c1d261f8019dad`.
It retains 35 exact first-vanishing degrees and 2,975 certified lower bounds.
Every row encodes the same prime candidate domain and point base. This is not
a dataset of alternative factor-base point sets.

## Target and feature contract

Predict the original-top first graded vanishing degree. Use the exact integer
degree when present and preserve each other row only as the one-sided
constraint `prediction >= certified_lower_bound`. Never replace a lower bound
with a point estimate or train it as an exact degree. The censoring is
deterministic and depends on the presentation size; this is not the stochastic
runtime-censoring setting in Eggensperger et al., *Neural Model-based
Optimization with Right-Censored Observations* (2020 preprint),
https://arxiv.org/abs/2009.13828. In particular, do not use its Tobit
likelihood or claim calibrated uncertainty. The present model uses a fixed
one-sided squared hinge solely to respect the known algebraic inequality.

Features are fixed, label-free summaries of the radix partition:
part count, socle degree, largest part, smallest part, count of unit parts,
and count of parts at least five. Do not include exact/bounded status, Hilbert
coefficients, target degrees, or any factor-base witness. Normalize using
training-fold inputs only.

## Model and evaluation

One deterministic 6-4-1 fully connected network with tanh hidden units and a
linear scalar output. Use full-batch Adam, 160 epochs, learning rate 0.01,
fixed initialization seeds, and no hyperparameter selection. Average exact
degree squared error within its group; average one-sided squared lower-bound
violation within the censored group; give those two group losses equal weight.

Use a fixed five-fold split of exact rows: SHA-256 of the fixed byte tag `n83-radix-neural-fold-v1` concatenated with
the JSON serialization of radix parts; interpret the first eight digest bytes
big-endian modulo five. In each fold, withhold the
exact labels in that fold, train on the other exact rows and all censored
rows, and report held-out exact MAE beside an intercept-only comparator
optimized under the same grouped objective. This small, related set supports
only an exploratory within-census estimate, not generalization to new curves,
families, m values, or factor-base point sets.

Then fit one model on all 3,010 rows and score the same census. For every
candidate with a model prediction below 42, join its immutable case record and
report whether an existing exact degree or certified lower bound refutes it.
These are model-flag counts, not new mathematical results. No iterative model
guided expensive evaluation is performed: the presentation family is already
exhausted by the exact/bounded census.

## Reproduction and boundaries

Native Rust implementation: `src/bin/radix_neural_probe.rs`. Inputs are the
3010 immutable `raw/census/cases/NNNN.json` objects, plus the raw manifest.
Outputs are the trained weights, per-row predictions, fold predictions and
metrics. The held-out labels, lower-bound checks, input manifest, binary hash,
protocol hash and command line must all be retained. Do not report elapsed
time as an algorithm comparison. No FFD, LFD, solver-cost or factor-base
improvement follows from this probe.
