[inputs|select(startswith("CASE label=fixture-"))|
 capture("label=fixture-p(?<p>[0-9]+)-f(?<family>[01]) g=(?<g>[234]) cycles=\\[(?<cycles>[^]]+)\\] all=(?<full>[0-9]+) paired=(?<paired>[0-9]+) fixed_dim=(?<fixed>[0-9]+)")|
 .p|=tonumber|.family|=tonumber|.g|=tonumber|.full|=tonumber|.paired|=tonumber|.fixed|=tonumber|.cycles|=(split(", ")|map(tonumber))] |
 {cases:.,fixtures:length,by_genus:(group_by(.g)|map({g:.[0].g,curves:length,full_eligible:(map(select(.full>0))|length),pair_eligible:(map(select(.paired>0))|length),full_kernels:(map(.full)|add),pair_kernels:(map(.paired)|add)}))}
