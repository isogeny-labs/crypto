def med: sort | if length%2==0 then (.[length/2-1]+.[length/2])/2 else .[length/2|floor] end;
def stats: {n:length,min:min,median:med,max:max};
def control: (.arm|startswith("n")) or ((.arm|startswith("t")) and .arm!="t1faa");
. as $all |
[$all[]|select(.stage=="select")] as $selection |
[$all[]|select(.stage=="gate")] as $gates |
[$all[]|select(.stage=="measure")] as $timed |
($timed|group_by([.g,.p,.family,.seed])|map(
  . as $rows | [.[].observation|select(.!=null)] as $obs |
  ($obs|map(select(.arm=="compact"))|.[0]) as $c |
  ($obs|map(select(.arm=="orbit"))|.[0]) as $o |
  ($obs|map(select(control))) as $controls |
  ($obs|map(select(.arm=="t1f"))|.[0]) as $aa1 |
  ($obs|map(select(.arm=="t1faa"))|.[0]) as $aa2 |
  ((($aa1|[.m,.trials,.relations,.solve_rounds,.k,.target_u,.target_v]) ==
    ($aa2|[.m,.trials,.relations,.solve_rounds,.k,.target_u,.target_v])) and $aa1!=null and $aa2!=null) as $aa_equal |
  (($rows|length)==15 and ($rows|all(.status=="exit_0")) and ($obs|length)==15 and
    ($obs|all(.correct==true and .k==.expected_k and .total_ns>0)) and
    ($obs|map([.expected_k,.target_u,.target_v])|unique|length)==1 and
    ($controls|length)==12 and $aa_equal) as $valid |
  ($controls|min_by(.total_ns)) as $fast |
  {g:.[0].g,p:.[0].p,family:.[0].family,seed:.[0].seed,valid:$valid,aa_counts_equal:$aa_equal,
   fastest_control:$fast.arm,compact_ns:$c.total_ns,orbit_ns:$o.total_ns,control_ns:$fast.total_ns,
   ratio:(if $valid then $fast.total_ns/$c.total_ns else null end),
   orbit_ratio:(if $valid then $fast.total_ns/$o.total_ns else null end),
   aa_factor:(if $aa_equal then ([$aa1.total_ns,$aa2.total_ns]|max)/([$aa1.total_ns,$aa2.total_ns]|min) else null end)}
)) as $cases |
{schema:"hec.heldout.summary/v1",source:"2404e34dca6529ff2b9267f16942974b0e777671",
 selection:{n:($selection|length),statuses:($selection|group_by(.status)|map({status:.[0].status,n:length}))},
 gates:{n:($gates|length),correct:($gates|all(.status=="exit_0" and .observation.correct)),
        raw_checks:($gates|map(.observation.raw_checks//0)|add),pure_checks:($gates|map(.observation.pure_checks//0)|add)},
 timed:{n:($timed|length),statuses:($timed|group_by(.status)|map({status:.[0].status,n:length})),
        all_correct:($timed|all(.status=="exit_0" and .observation.correct and .observation.k==.observation.expected_k))},
 cases:$cases,
 fixtures:($cases|group_by([.g,.p,.family])|map({g:.[0].g,p:.[0].p,family:.[0].family,valid:all(.valid),
    primary_ratio:(map(.ratio)|map(select(.!=null))|if length>0 then stats else null end),
    orbit_ratio:(map(.orbit_ratio)|map(select(.!=null))|if length>0 then stats else null end),
    aa_factor:(map(.aa_factor)|map(select(.!=null))|if length>0 then stats else null end),
    fastest_controls:(map(.fastest_control)|unique)})),
 arms:($timed|group_by([.g,.p,.family,.arm])|map({g:.[0].g,p:.[0].p,family:.[0].family,arm:.[0].arm,n:length,
    all_correct:all(.status=="exit_0" and .observation.correct),
    total_ns:(map(.observation.total_ns)|map(select(.!=null))|if length>0 then stats else null end)})),
 phases:([$timed[]|select(.arm=="compact" and .status=="exit_0")|.observation]|group_by([.g,.p])|map({g:.[0].g,p:.[0].p,n:length,
    relation_fraction:(map(.relation_ns/.total_ns)|med),forest_fraction:(map(.forest_ns/.total_ns)|med),
    solve_fraction:(map(.solve_ns/.total_ns)|med),setup_fraction:(map(.setup_ns/.total_ns)|med)})),
 note:"Producer self-checks [pcc:unverified]. Per-target fastest of12controls is conservative/exploratory. No pooling into a multi-target solve, no exponent fit, no novelty or10x conclusion. Raw receipts retain failures/unstarted cells."
}
