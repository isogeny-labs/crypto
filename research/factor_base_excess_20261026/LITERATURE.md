# Sources checked before selecting the experiment

1. [Lead atlas: linearization excess](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/knowledge/leads/ecdlp/ic-engine/LEAD-ICE-linearization-excess-fb-prior.yaml).
   Read the complete current record. It names the missing transfer test on
   m=3/4 descent cells; it is an open lead, not evidence that transfer works.
2. [Historical PDP-degree study](https://github.com/isogeny-labs/cryptanalysis/blob/main/experiments/pdp-degree-heuristics/README.md)
   and [product-profile source](https://github.com/isogeny-labs/cryptanalysis/blob/main/experiments/pdp-degree-heuristics/factor_base.py).
   Read the summary, definitions, product-profile algorithm, subgroup/column
   policy, fitted-rule table, applicability limits and next steps. The source
   is Python; it was read as prior work and **not executed**. The empirical
   m=2 fit (656 bases, reported MAE 0.059/held-out-field 0.0823) is historical
   and limited to that solver/formulation/size range. It is not imported into
   the native output. The same source already records geometric progressions,
   trace-constrained variants, point-projection rank caveats and symmetric
   m=3 experiments. No claim that these mechanisms are new is made.
3. Bachoc, Serra and Zémor, *An analogue of Vosper's theorem for extension
   fields*, Math. Proc. Cambridge Philos. Soc. 163 (2017), 423–452,
   [DOI 10.1017/S0305004117000044](https://doi.org/10.1017/S0305004117000044),
   [primary preprint 1501.00602v2](https://arxiv.org/pdf/1501.00602v2).
   Downloaded the actual PDF; read pages 1–3, specifically Theorems 1 and 3
   and the finite-prime-extension qualifications. PDF SHA256:
   `2db1298da4f9c42e365d2cbb94746ddac8d8970a9d7e5a5fc20e24d38d5982cc`.
   This justifies treating minimal product growth as known structure. It
   proves no solver-complexity or degree-of-regularity bound.
4. [KN-FIND-007](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/knowledge/findings/KN-FIND-007.md).
   Read the full double-counting statement and its coverage/rank exclusions.
   It counts all target elements of a finite group; subgroup filtering and
   unequal point inventories must not be discarded in applying it.
5. [Native degree-specialization results](../dreg_closure_specialise_20261017/RESULTS.md).
   Read the complete result and failed-null interpretation: fixing an
   intermediate coordinate worked as often as a summand coordinate in the
   stated panel. Therefore neither excess nor a guessed-variable heuristic
   is assumed causal or sufficient in advance.
6. Current atlas records for cofactor/torsion quotients, generic table
   oracles, free self-collision relations, factor-base subspace choice and
   Frobenius-orbit bases were read before selecting the next step. They
   retain scoped negative evidence and distinguish constant engineering
   from a new algebraic oracle. The nonlinear Frobenius-stable-base lead
   remains separate; this change does not claim to execute or close it.
7. [Previous native three-lead result](https://github.com/isogeny-labs/crypto/blob/0f47fe33e297c7af6189f4207d7116036cfeccf8/research/factor_base_oracle_leads_20261019/README.md).
   Read its exact successful-control counts, incomplete pricing and fresh
   target/rank results. Its old executable-bound runs are not rerun or
   transferred to the new source.

Search scope: the current repository's named results, the sibling lead atlas
and primary-source theorem relevant to product growth. This is not an
exhaustive literature or novelty review. Magma/msolve/Singular matched
solver comparisons remain unperformed. No external timing is quoted as a
matched reference. No Lean proof or identity certificate is newly claimed;
the structural definition/algorithm implements an existing construction.
