def median: sort | .[length/2|floor];
def signature: [.k,.expected_k,.target_u,.target_v,.batches,.admitted,.pure_rows,.rank,.hash];
map(select(startswith("CELL "))|ltrimstr("CELL ")|fromjson) as $raw |
($raw|group_by([.g,.p,.seed])|map(
 . as $all |
 (map(select(.arm|startswith("point-")))) as $point |
 (map(select((.arm|startswith("point-"))|not))) as $controls |
 ($point|map(select(.arm=="point-lazy"))|.[0]) as $candidate |
 ($point|map(select(.arm=="point-lazy-aa"))|.[0]) as $aa |
 ($point|map(select(.arm=="point-dense"))|.[0]) as $dense |
 ($controls|map(select(.correct and .k!=null))|min_by(.total_ns)) as $best |
 {g:.[0].g,p:.[0].p,seed:.[0].seed,
  complete_population:($all|length==18),
  exact:(($point|length)==4 and ($point|map(signature)|unique|length)==1 and ($all|all(.correct)) and ($all|map([.target_u,.target_v,.expected_k])|unique|length)==1),
  recovered:($candidate.k!=null),best_arm:$best.arm,best_ns:$best.total_ns,
  point_ns:$candidate.total_ns,dense_ns:$dense.total_ns,
  ratio:(if $candidate.k==null then null else $best.total_ns/$candidate.total_ns end),
  aa_spread:([$candidate.total_ns,$aa.total_ns]|max/min),
  dense_over_lazy:($dense.total_ns/$candidate.total_ns),
  batches:$candidate.batches,rank:$candidate.rank,anchors:$candidate.anchors,
  setup_fraction:($candidate.setup_ns/$candidate.total_ns),
  sieve_fraction:($candidate.relation_ns/$candidate.total_ns),
  forest_fraction:($candidate.forest_ns/$candidate.total_ns),
  solve_fraction:($candidate.solve_ns/$candidate.total_ns)})) as $targets |
{standing:"L0 producer observations; same-target cold costs; caps retained; no novelty or confidence claim",
 source:"cd441f8d6601881ec42dfcf0a51ea3363d5e7dab",
 cells:($raw|length),targets:($targets|length),
 E8_exact:($targets|length==18 and all(.complete_population and .exact)),
 recovered:($targets|map(select(.recovered))|length),
 capped:($targets|map(select(.recovered|not))),
 fixtures:($targets|group_by([.g,.p])|map({g:.[0].g,p:.[0].p,
  recovered:(map(select(.recovered))|length),
  primary_median:(if all(.recovered) then map(.ratio)|median else null end),
  dense_over_lazy: (map(.dense_over_lazy)|median),
  aa_max:(map(.aa_spread)|max),
  setup_fraction:(map(.setup_fraction)|median),sieve_fraction:(map(.sieve_fraction)|median),
  forest_fraction:(map(.forest_fraction)|median),solve_fraction:(map(.solve_fraction)|median)})),
 cases:$targets} |
 .E9_cost=(.E8_exact and .recovered==18 and (.fixtures|all(.primary_median>=10)))
