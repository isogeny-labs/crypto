# Sum exclusive Ir per source file from a raw callgrind profile (inlined code
# is charged to the file its line belongs to). Handles name compression.
function nm(s,   id) {
  if (match(s, /^\([0-9]+\)/)) { id = substr(s, 2, RLENGTH - 2); rest = substr(s, RLENGTH + 2)
    if (rest != "") names[id] = rest; return names[id] }
  return s }
/^fl=/ { fl = nm(substr($0, 4)); cur = fl; next }
/^fi=/ || /^fe=/ { cur = nm(substr($0, 4)); next }
/^fn=/ { cur = fl; next }
/^calls=/ { skip = 1; next }
/^c(fi|fl|fn|ob)=/ { x = nm(substr($0, index($0, "=") + 1)); next }
/^(ob|events|positions|summary|totals|cmd|version|creator|pid|part|desc):?/ { next }
/^[+-]?[0-9]+ / || /^\* / {
  if (skip) { skip = 0; next }
  s[cur] += $2; next }
END { for (f in s) print s[f] "\t" f }
