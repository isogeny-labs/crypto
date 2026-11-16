# Derives reach.csv from genus4.jsonl:
#   awk -f tables.awk genus4.jsonl > reach.csv
# Rows: family, quantity (L1, Lm1, or P_ord3 for n | P(lambda) with ord lambda = 3), prime, models.
function jlist(s, key, a,    i, body){
  i = index(s, "\"" key "\":["); if(!i) return 0
  body = substr(s, i + length(key) + 4); body = substr(body, 1, index(body, "]") - 1); gsub(/ /, "", body)
  return split(body, a, ",") }
function field(s, key,    i, v){
  i = index(s, "\"" key "\":"); if(!i) return ""
  v = substr(s, i + length(key) + 3); sub(/[,}].*/, "", v); gsub(/"/, "", v); return v }
BEGIN{ print "family,quantity,prime,models" }
/"kind":"screen_summary"/{
  F = field($0, "family")
  n = jlist($0, "odd_primes_L1", a); for(i = 1; i <= n; i++) print F ",L1," a[i] ","
  n = jlist($0, "odd_primes_Lm1", a); for(i = 1; i <= n; i++) print F ",Lm1," a[i] "," }
/"kind":"candidate"/{
  print field($0, "family") ",P_ord" field($0, "ord_lambda") "," field($0, "n") "," field($0, "models") }
