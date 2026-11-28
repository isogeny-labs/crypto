[inputs|select(startswith("CASE "))|capture("g=(?<g>[234]) p=(?<p>[0-9]+) family=(?<family>[01]) ell=(?<ell>[0-9]+) candidates=(?<candidates>[0-9]+) status=(?<status>[a-z0-9_]+) abstract_kernels=(?<kernels>[0-9]+|null)")|
 .g|=tonumber|.p|=tonumber|.family|=tonumber|.ell|=tonumber|.candidates|=tonumber|
 .kernels=(if .status|startswith("unknown") then null else .kernels|tonumber end)] |
 {cases:.,curves:length,all_middle_unique:all(.candidates==1),
 extension_elements:(map(if .g==3 then .p*.p elif .g==4 then .p*.p+.p*.p*.p else 0 end)|add),
 by_genus:(group_by(.g)|map({g:.[0].g,curves:length,obstructed:(map(select(.status=="obstructed"))|length),unknown:(map(select(.status|startswith("unknown")))|length),abstract_regular:(map(select(.status=="abstract_regular_kernels"))|length)}))}
