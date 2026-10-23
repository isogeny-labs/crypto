#!/bin/bash
# Q4, second check: committed sessions that use frozen ids, replayed by the branch binary.
# Each line: session  replay-count (all = --replay-all).  A replay re-executes the run and compares counts with the record.
set -u
cd "$(git rev-parse --show-toplevel)"
O=research/ecbench_accounting_v2_20261023/audits/committed
mkdir -p $O
while read -r s n; do
  [ -z "$s" ] && continue
  tag=$(echo "$s" | sed 's#/sessions/#__#; s#/#_#g')
  if [ "$n" = all ]; then a="--replay-all"; else a="--replay $n"; fi
  ./target/release/ecbench verify --dir "$s" $a --allow-interrupted --exit-code --out "$O/$tag.audit.json" > "$O/$tag.verify.txt" 2>&1
  echo "$s $n exit=$? $(grep -E '^audit ' $O/$tag.verify.txt | head -1)"
done <<LIST
research/ecbench_calibration_20261002/sessions/followup-prime 24
research/ecbench_calibration_20261002/sessions/followup-koblitz 24
research/ecbench_numa_vm_20261002/sessions/auto all
research/ghs_curve_comparison_20261009/sessions/panel-01 24
research/ghs_curve_comparison_20261009/sessions/non-koblitz-01 24
research/ecbench_wide_gf2m_20261004/sessions/smoke-m83 all
research/ecbench_bounds_challenges_20261006/sessions/koblitz-toy-reference-e1 24
research/ecbench_rootfind_engine_20261012/sessions/claim-n13 24
research/ecbench_rootfind_engine_20261012/sessions/smoke-koblitz 24
research/ecbench_factor_bases_20261013/sessions/koblitz17 all
research/ecbench_n37_shared_rank_20261004/sessions/mac_arm64_l0_01 all
LIST
