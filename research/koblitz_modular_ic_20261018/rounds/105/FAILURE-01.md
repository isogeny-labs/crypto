# R105 attempt 01: incomplete mixed lambda jets

The source-bound script exited zero and its endpoint checks passed, but
inspection of `result.json` showed only `G_y0_lambda_jets` and
`G_y1_lambda_jets`. The frozen protocol requires every rational G51
coefficient's t-degree-0..4 contribution, including mixed y terms.
The accepted result requires a full support table indexed by `(l,y)`.
Attempt 01 is retained raw and is not accepted as protocol-complete.

Amendment 02 adds the full mixed support, checks its degree-zero jet
against R101, and derives endpoint projections from that table.
