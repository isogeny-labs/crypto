# Native producer recomputation for an unsigned PCC receipt, not independent replay.
def med: sort | .[length/2|floor];
def control: (.[4]|startswith("n")) or ((.[4]|startswith("t")) and .[4]!="t1faa");
. as $raw |
(.timed|group_by(.[0:4])|map(. as $a |
  ($a|map(select(.[4]=="compact"))|.[0][8]) as $c |
  ($a|map(select(control))|min_by(.[8])|.[8]) as $b |
  {g:.[0][0],p:.[0][1],family:.[0][2],ratio:($b/$c)})) as $cases |
($cases|group_by([.g,.p,.family])|map(map(.ratio)|med)) as $medians |
{status:"pass",standing:"unsigned producer recomputation; independent signature absent",
 raw_data:[{path:"research/hec_anchor_relations_20261114/HELDOUT-PCC-RAW.json",sha256:$sha}],
 headline:[
  {name:"fixtures",claimed:$original[0].selection.n,recomputed:($medians|length)},
  {name:"gated_cases",claimed:$original[0].gates.n,recomputed:($raw.gates|length)},
  {name:"correct_timed_cells",claimed:$original[0].timed.n,recomputed:($raw.timed|map(select(.[5]=="exit_0" and .[6]==.[7] and .[9]))|length)},
  {name:"principal_checks",claimed:$original[0].gates.raw_checks,recomputed:($raw.gates|map(.[5])|add)},
  {name:"projected_checks",claimed:$original[0].gates.pure_checks,recomputed:($raw.gates|map(.[6])|add)},
  {name:"minimum_curve_median_ratio",claimed:($original[0].fixtures|map(.primary_ratio.median)|min),recomputed:($medians|min)},
  {name:"maximum_curve_median_ratio",claimed:($original[0].fixtures|map(.primary_ratio.median)|max),recomputed:($medians|max)},
  {name:"maximum_target_ratio",claimed:($original[0].cases|map(.ratio)|max),recomputed:($cases|map(.ratio)|max)}]
} | if all(.headline[];.claimed==.recomputed) then . else error("producer recomputation mismatch") end
