# Exploratory post-run fixed-work counterfactual, not a registered experiment.
def med: sort|if length%2==0 then (.[length/2-1]+.[length/2])/2 else .[length/2|floor] end;
[.[]|select(.stage=="measure")]|group_by([.g,.p,.seed])|map(
 ([.[].observation|select(.arm!="rational-linear" and .arm!="rational-linear-aa")]|min_by(.total_ns))as $best|
 ([.[].observation|select(.arm=="compact")][0])as $c|
 {g:.[0].g,p:.[0].p,seed:.[0].seed,conditional_ratio:($best.total_ns/($c.total_ns-$c.relation_ns)),remaining_ns:($c.total_ns-$c.relation_ns)}
)|{assumption:"Replace compact harvesting at zero time, holding every other observed cost fixed; per-case subtraction before aggregation; exploratory counterfactual, not an observed speedup or general impossibility theorem",cases:.,max:(map(.conditional_ratio)|max),fixtures:(group_by([.g,.p])|map({g:.[0].g,p:.[0].p,median:(map(.conditional_ratio)|med),max:(map(.conditional_ratio)|max)}))}
