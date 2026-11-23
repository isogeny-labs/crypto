# Aggregate a (possibly multi-part, combined) callgrind file: exclusive and
# "inclusive" (self + direct call costs, first-level, recursion may double count) per function.
function nm(s,   id, rest) {
  if (match(s, /^\([0-9]+\)/)) { id = substr(s, 2, RLENGTH-2); rest = substr(s, RLENGTH+2);
    if (rest != "") names[id] = rest; return id } return s }
/^fn=/ { cur = nm(substr($0,4)); aftercall=0; next }
/^cfn=/ { nm(substr($0,5)); next }
/^calls=/ { aftercall=1; next }
/^[0-9+*-]/ { v = $2+0; if (aftercall) { inc[cur]+=v; aftercall=0 } else { ex[cur]+=v; inc_self[cur]+=v; tot+=v } next }
END {
  for (f in ex) { n=(f in names)?names[f]:f; printf "E %d %s\n", ex[f], n }
  for (f in inc) { n=(f in names)?names[f]:f; printf "I %d %s\n", inc[f]+ex[f], n }
  printf "T %d\n", tot
}
