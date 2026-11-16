def med: sort|if length%2==0 then (.[length/2-1]+.[length/2])/2 else .[length/2|floor] end;
def stats: {n:length,min:min,median:med,max:max};
def signature: [.g,.p,.family,.seed,.ell,.expected_k,.k,.target_u,.target_v,.requested_h,.anchors,.m,.trials,.constructed,.tuples,.splits,.admitted,.duplicates,.pure_rows,.unions,.pins,.descent_steps,.common,.normalized_admitted,.status];
def baseline: .arm!="rational-linear" and .arm!="rational-linear-aa";
. as $all|[.[]|select(.stage=="gate")]as $gates|[.[]|select(.stage=="measure")]as $timed|
($timed|group_by([.g,.p,.family,.seed])|map(
 . as $rows|[.[].observation|select(.!=null)]as $obs|
 ($obs|map(select(.arm=="rational-linear"))|.[0])as $c|
 ($obs|map(select(.arm=="rational-linear-aa"))|.[0])as $aa|
 ($obs|map(select(.arm=="rational-generic"))|.[0])as $generic|
 ($obs|map(select(baseline)))as $base|
 (($c|signature)==($generic|signature) and $c!=null and $generic!=null)as $equal|
 (($c|signature)==($aa|signature) and $aa!=null and $c!=null)as $aa_equal|
 ((length==16)and all(.status=="exit_0") and ($obs|length)==16 and ($base|length)==14 and ($obs|all(.correct==true and .k==.expected_k and .total_ns>0)) and ($obs|map([.ell,.expected_k,.target_u,.target_v])|unique|length)==1 and $equal and $aa_equal)as $valid|
 ($base|min_by(.total_ns))as $best|
 {g:.[0].g,p:.[0].p,family:.[0].family,seed:.[0].seed,valid:$valid,counts_equal:$equal,aa_counts_equal:$aa_equal,
  best_baseline:$best.arm,baseline_ns:$best.total_ns,linear_ns:$c.total_ns,generic_ns:$generic.total_ns,
  requested_h:$c.requested_h,effective_h:$c.anchors,constructed:$c.constructed,consumed:$c.trials,
  primary_ratio:(if $valid then $best.total_ns/$c.total_ns else null end),
  generic_ratio:(if $equal and $c.correct and $generic.correct then $generic.total_ns/$c.total_ns else null end),
  aa_factor:(if $aa_equal and $aa.correct and $c.correct then ([$c.total_ns,$aa.total_ns]|max)/([$c.total_ns,$aa.total_ns]|min) else null end)}
))as $cases|
{schema:"hec.linear.cold.summary/v1",gates:{n:($gates|length),all_correct:($gates|length==18 and all(.status=="exit_0" and .observation.correct)),raw_checks:($gates|map(.observation.raw_checks//0)|add),pure_checks:($gates|map(.observation.pure_checks//0)|add)},
 timed:{n:($timed|length),statuses:($timed|group_by(.status)|map({status:.[0].status,n:length})),all_correct:($timed|length==288 and all(.status=="exit_0" and .observation.correct))},cases:$cases,
 fixtures:($cases|group_by([.g,.p,.family])|map({g:.[0].g,p:.[0].p,family:.[0].family,valid:all(.valid),primary_ratio:(map(.primary_ratio)|map(select(.!=null))|if length>0 then stats else null end),generic_ratio:(map(.generic_ratio)|map(select(.!=null))|if length>0 then stats else null end),aa_factor:(map(.aa_factor)|map(select(.!=null))|if length>0 then stats else null end)})),
 phases:([$timed[]|select(.arm=="rational-linear" and .observation.correct)|.observation]|group_by([.g,.p])|map({g:.[0].g,p:.[0].p,n:length,setup_fraction:(map(.setup_ns/.total_ns)|med),relation_fraction:(map(.relation_ns/.total_ns)|med),forest_fraction:(map(.forest_ns/.total_ns)|med),solve_fraction:(map(.solve_ns/.total_ns)|med)})),
 note:"Producer L0 observations, reused six dense curves and18new target seeds. Full cold same-target totals; minimum of14existing baselines. No statistical-confidence, exponent or novelty claim. Count/time losses and terminal statuses retained."
}
