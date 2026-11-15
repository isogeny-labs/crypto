# sed 's/^COMPACT //' compact-run/paired.txt | jq -s -f summarize_compact.jq
group_by([.g,.p]) | map({
  g:.[0].g, p:.[0].p, correct:all(.correct),
  arms:(group_by(.arm) | map({
    arm:.[0].arm,n:length,
    median_ns:(map(.total_ns)|sort|.[2]),
    min_ns:(map(.total_ns)|min),max_ns:(map(.total_ns)|max),
    relation_ns:(map(.relation_ns//0)|sort|.[2])
  }))
}) | map(. as $r | . + {
  vs_orbit:(($r.arms|map(select(.arm=="orbit"))|.[0].median_ns) /
            ($r.arms|map(select(.arm=="compact"))|.[0].median_ns)),
  vs_table:(($r.arms|map(select(.arm=="table"))|.[0].median_ns) /
            ($r.arms|map(select(.arm=="compact"))|.[0].median_ns))
})
