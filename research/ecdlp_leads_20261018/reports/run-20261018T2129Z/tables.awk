# Derives the run's tables from its JSONL records.
#   awk -v T=hits  -f tables.awk genus2.jsonl genus3.jsonl quartic.jsonl > hits-by-n.csv
#   awk -v T=reach -f tables.awk genus2.jsonl genus3.jsonl quartic.jsonl > reach.csv
# Families: genus2 -> g2, genus3 -> g3h (hyperelliptic), quartic -> g3q (plane quartic).
function fam(f){ return (f ~ /genus2/) ? "g2" : (f ~ /genus3/) ? "g3h" : "g3q" }
# "key":{"a":1,"b":2} -> m[a]=1, m[b]=2 (returns count)
function jmap(s, key, m,    i, body, parts, n, kv, j){
  i = index(s, "\"" key "\":{"); if(!i) return 0
  body = substr(s, i + length(key) + 4); body = substr(body, 1, index(body, "}") - 1)
  n = split(body, parts, ","); for(j = 1; j <= n; j++){ split(parts[j], kv, ":"); gsub(/"/, "", kv[1]); m[kv[1]] = kv[2] + 0 }
  return n }
# "key":[1, 2, 3] -> a[1..n]
function jlist(s, key, a,    i, body){
  i = index(s, "\"" key "\":["); if(!i) return 0
  body = substr(s, i + length(key) + 4); body = substr(body, 1, index(body, "]") - 1); gsub(/ /, "", body)
  return split(body, a, ",") }
/"kind":"hit"/{
  F = fam(FILENAME); s = $0; delete seenc
  while(match(s, /\{"n":[0-9]+,"chi":\[[0-9, ]*\]/)){
    e = substr(s, RSTART, RLENGTH); s = substr(s, RSTART + RLENGTH)
    n = e; sub(/^\{"n":/, "", n); sub(/,.*/, "", n)
    entries[F "," n]++; hitn[F "," n] = 1
    if(!(e in seenc)){ seenc[e] = 1; chars[F "," n]++ } } }
/"kind":"summary"/{
  F = fam(FILENAME); fams[++nf] = F
  delete cpn; delete chn; jmap($0, "curves_per_n", cpn); jmap($0, "chars_per_n", chn)
  for(n in chn){ tested[F "," n] = chn[n]; curves[F "," n] = cpn[n]; ns[F, ++nn[F]] = n + 0 }
  np1[F] = jlist($0, "odd_primes_L1", tmp); for(i = 1; i <= np1[F]; i++) p1[F, i] = tmp[i]
  npm[F] = jlist($0, "odd_primes_Lm1", tmp); for(i = 1; i <= npm[F]; i++) pm[F, i] = tmp[i] }
END{
  if(T == "hits"){
    print "family,n,characters_tested,curves_with_n_dividing_J,chars_with_hit,hit_entries"
    for(f = 1; f <= nf; f++){ F = fams[f]
      for(i = 1; i <= nn[F]; i++) for(j = i; j > 1 && ns[F, j-1] > ns[F, j]; j--){ t = ns[F, j]; ns[F, j] = ns[F, j-1]; ns[F, j-1] = t }
      for(i = 1; i <= nn[F]; i++){ k = F "," ns[F, i]; printf "%s,%d,%d,%d,%d,%d\n", F, ns[F, i], tested[k], curves[k], chars[k], entries[k] } } }
  else {
    print "family,quantity,prime,hit"
    for(f = 1; f <= nf; f++){ F = fams[f]
      for(i = 1; i <= np1[F]; i++) printf "%s,L1,%d,%d\n", F, p1[F, i], (F "," p1[F, i]) in hitn
      for(i = 1; i <= npm[F]; i++) printf "%s,Lm1,%d,0\n", F, pm[F, i] } } }
