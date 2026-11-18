# Descriptive terminal correctness counts only, no time estimates.
[inputs | select(startswith("TARGET ")) |
 capture("g=(?<g>[0-9]+) p=(?<p>[0-9]+) seed=(?<seed>[0-9]+) h=(?<h>[0-9]+) admitted=(?<admitted>[0-9]+) duplicates=(?<duplicates>[0-9]+) pure=(?<pure>[0-9]+) rank=(?<rank>[0-9]+) k=(?<k>[^ ]+) unions=(?<unions>[0-9]+) pins=(?<pins>[0-9]+) dense_bytes=(?<dense_bytes>[0-9]+) lazy_bytes=(?<lazy_bytes>[0-9]+)") |
 .recovered=(.k!="None") | .k=(if .k=="None" then null else (.k|capture("Some\\((?<v>[0-9]+)\\)").v|tonumber) end) |
 .g|=tonumber|.p|=tonumber|.seed|=tonumber|.h|=tonumber|
 .admitted|=tonumber|.duplicates|=tonumber|.pure|=tonumber|.rank|=tonumber|
 .unions|=tonumber|.pins|=tonumber|.dense_bytes|=tonumber|.lazy_bytes|=tonumber] |
{standing:"Producer correctness gate, fixed 128 batches per target; no timing or speed claim",
 targets:length,recovered:(map(select(.recovered))|length),
 capped:(map(select(.recovered|not))),admitted:(map(.admitted)|add),
 pure:(map(.pure)|add),cases:.}
