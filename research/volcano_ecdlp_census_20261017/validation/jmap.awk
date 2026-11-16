# j-invariant of y^2 = x^3 + a x + b over F_p, exact in double arithmetic
# for p < 2^26 (every product of two residues stays below 2^53).
# Mode "census": input volcano,id,level,a,b,p,j_recorded -> checks j and
# prints volcano,j,id,level. Mode "slug": input spec,slug,a,b,p -> prints
# spec,slug,volcano,j.
function mm(x, y, p) { return (x * y) % p }
function pw(b, e, p,   r) { r = 1; b = b % p; while (e > 0) { if (e % 2 == 1) r = mm(r, b, p); b = mm(b, b, p); e = (e - e % 2) / 2 } return r }
function jinv(a, b, p,   a3, num, den) {
  a3 = mm(mm(a, a, p), a, p)
  num = mm(6912 % p, a3, p)                       # 1728 * 4 a^3
  den = (mm(4, a3, p) + mm(27, mm(b, b, p), p)) % p
  if (den == 0) return -1
  return mm(num, pw(den, p - 2, p), p)
}
BEGIN { FS = OFS = "\t" }
mode == "census" { j = jinv($4, $5, $6); if (j != $7) { bad++; print "JMISMATCH", $0 > "/dev/stderr" } printf "%s\t%.0f\t%s\t%s\n", $1, j, $2, $3 }
mode == "slug" { split($1, s, "-"); printf "%s\t%s\t%s\t%.0f\n", $1, $2, s[1], jinv($3, $4, $5) }
END { if (mode == "census") print "census j mismatches: " bad + 0 > "/dev/stderr" }
