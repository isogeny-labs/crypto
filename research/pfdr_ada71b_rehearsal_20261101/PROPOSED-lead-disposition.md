# Proposed update to LEAD-ICP-coincidence-harvest-structured-base (NOT applied)

Pending: (1) a supervised, claimed reproduction of EXP-PFDR-ada71b (the rehearsal is not an admitted run), (2) independent review per the spec's decision_impact and AGENTS.md rule 12,
(3) a committed Coordinator decision. The lead YAML is therefore unchanged.

Proposed text once those hold (scope exactly the six lines, h = 2, 16-32 bits):

    disposition: exhausted_at_scale          # for window and rank-2/3 label-GAP point bases only
    obstruction:
      experiment: EXP-PFDR-ada71b            # reading O-SUB-BE, controls C1,C3-C6 passed
      max_upper95_delta: 0.0144              # over six valid lines, against break-even delta* = 1/3 (23x below)
      delta_hat_range: [-0.0044, 0.0070]
      kappa_range: [0.916, 1.058]            # per rung; upper95 kappa at 32 bits <= 1.105
      scope: prime-order E/F_p, 16-32 bits, b = 2*round(2^(bits*theta)/2), theta 0.30/0.35, h = 2
      not_covered: [FAM-1 small_x/subgroup/dickson, log-GAP, bbc53e base, j=0/1728, >32 bits, h>2]
    note: >-
      No structural relation source on F_p was observed: window and label-GAP bases carry no detectable pair-sum
      coincidence excess (kappa ~ 1) at any rung. The frozen class bounds the elasticity of kappa, not the exponent of a small
      excess (an excess on the 1/3 exponent with eps(32) < ~0.1 would not be flagged); state it as "no detectable excess".
