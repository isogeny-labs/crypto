def median: sort | .[length/2|floor];
def candidate: .arm == "zero" or .arm == "positive" or .arm == "zero-aa" or .arm == "zero-gate";
def signature: [.k,.expected_k,.target_u,.target_v,.trials,.smooth,.hash,.status];
map(select(startswith("CELL "))|ltrimstr("CELL ")|fromjson) as $raw |
($raw|group_by([.g,.p,.seed])|map(
 . as $all |
 (map(select(candidate))) as $points |
 (map(select(candidate|not))) as $controls |
 ($points|map(select(.arm=="zero"))|.[0]) as $zero |
 ($points|map(select(.arm=="zero-aa"))|.[0]) as $aa |
 ($points|map(select(.arm=="positive"))|.[0]) as $positive |
 ($controls|map(select(.correct and .k!=null))|min_by(.total_ns)) as $best |
 {g:.[0].g,p:.[0].p,seed:.[0].seed,
  complete_population:($all|length==18),
  exact:(($points|length)==4 and ($points|map(signature)|unique|length)==1 and ($all|all(.correct)) and ($all|map([.target_u,.target_v,.expected_k])|unique|length)==1),
  recovered:($zero.k!=null),best_arm:$best.arm,best_ns:$best.total_ns,
  zero_ns:$zero.total_ns,positive_ns:$positive.total_ns,
  ratio:(if $zero.k==null or $best==null then null else $best.total_ns/$zero.total_ns end),
  aa_spread:([$zero.total_ns,$aa.total_ns]|max/min),
  positive_over_zero:($positive.total_ns/$zero.total_ns),
  trials:$zero.trials,smooth:$zero.smooth,
  setup_fraction:($zero.setup_ns/$zero.total_ns),
  walk_fraction:($zero.walk_ns/$zero.total_ns),
  oracle_fraction:($zero.oracle_ns/$zero.total_ns),
  rows_fraction:($zero.row_ns/$zero.total_ns)})) as $targets |
{standing:"L0 producer observations; same-target cold costs; caps retained; no novelty or confidence claim",
 source:"a500c14f9a2052317499038b0b3d515e0e3d8aa8",
 cells:($raw|length),targets:($targets|length),
 Z1_exact:($targets|length==6 and all(.complete_population and .exact)),
 recovered:($targets|map(select(.recovered))|length),
 capped:($targets|map(select(.recovered|not))),
 fixtures:($targets|group_by([.g,.p])|map({g:.[0].g,p:.[0].p,
  recovered:(map(select(.recovered))|length),
  primary_median:(if all(.recovered and .ratio!=null) then map(.ratio)|median else null end),
  positive_over_zero:(map(.positive_over_zero)|median),
  aa_max:(map(.aa_spread)|max),
  setup_fraction:(map(.setup_fraction)|median),walk_fraction:(map(.walk_fraction)|median),
  oracle_fraction:(map(.oracle_fraction)|median),rows_fraction:(map(.rows_fraction)|median)})),
 cases:$targets} |
 .Z2_cost=(.Z1_exact and .recovered==6 and (.fixtures|all(.primary_median!=null and .primary_median>=10)))
