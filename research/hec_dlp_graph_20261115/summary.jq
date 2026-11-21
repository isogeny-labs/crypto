def median: sort|.[length/2|floor];
def fits($o;$p): ($o+5 >= $p/2 and $o <= 2*$p+5);
. as $lines |
map(select(startswith("SNAP "))|ltrimstr("SNAP ")|fromjson) as $s |
($s|map(select(.holdout))) as $held |
($s|map(select(.t==32768))) as $last |
{standing:"Real-curve count calibration; first2048 rates fixed; no timing or speed claim",
 terminal:($lines|map(select(startswith("TERMINAL ")))),
 snapshots:($s|length),holdouts:($held|length),
 M2_pass:($held|all(.M2)),
 violations:($held|map(select(.M2|not)|{g,p,seed,t,
  known_ok:fits(.known;.pred_known),rows_ok:fits(.root_rows;.pred_rows),depth_ok:fits(.depth;.pred_depth),
  known,pred_known,root_rows,pred_rows,depth,pred_depth})),
 full_recovered:($last|map(select(.full_solved_at>0))|length),
 rooted_recovered:($last|map(select(.root_solved_at>0))|length),
 geometric_nullity_monotone:($s|all(.full_rows>=.root_rows)),
 fixtures:($last|group_by([.g,.p])|map({g:.[0].g,p:.[0].p,h:.[0].h,l:.[0].l,
  full_recovered:(map(select(.full_solved_at>0))|length),root_recovered:(map(select(.root_solved_at>0))|length),
  root_rows:(map(.root_rows)|median),full_rows:(map(.full_rows)|median),
  root_rank:(map(.root_rank)|median),full_rank:(map(.full_rank)|median),
  root_known_fraction:(map(.known/.l)|median),
  root_pred_rows_ratio:(map(.root_rows/.pred_rows)|median),
  root_mean_weight:(map(if .root_rows>0 then .root_entries/.root_rows else 0 end)|median),
  full_mean_weight:(map(if .full_rows>0 then .full_entries/.full_rows else 0 end)|median),
  full_first_target:(map(.full_solved_at)|median),root_first_target:(map(.root_solved_at)|median)})),
 cases:$s}
