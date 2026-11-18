# Larger full-recovery follow-up

Freeze this addition before starting its measurements. The primary protocol
completed all 12 curves, all 24 planned recoveries and 11,370 independent
dictionary checks. Its deterministic replay matched records.jsonl exactly.

Run full known-answer recoveries on the four already registered larger
fixtures: p in {499,1009}, construction seeds {20261019,936451}, weights
{1,2,4}. Keep 128 matched PDP queries and the c=1 A/A arm. Set
`--dlp-max-p 1009`, retaining the primary limits of 10,000 residuals and
5,000,000 pair tests per recovery. This requests 12 additional recoveries.
Retain every budget exhaustion. Do not change the stopping rule in response
to results. The primary study is not retroactively redefined.

Use the same independent signed-pair dictionary, group relation checks,
secret-free solver and external PARI fixture checks. Publish the source
commit, all raw records and a separate report. This follow-up is exploratory
after seeing the primary run, not a new independent holdout or an
end-to-end performance comparison. All missing-cost and rho obligations
from PROTOCOL.md remain open.
