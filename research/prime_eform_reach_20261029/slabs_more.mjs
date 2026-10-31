// Tooling: slab ratios for S3 in two unknowns (Newt = [0,2]^2, so counts are closed form).
// E_trans = sum_{s>=1} (a-2s+1)_+ (L-2s+1)_+ ;  c_1 = (a+1)a/2 * (L+1).  rho_1 = 2 E / c_1.
import { readFileSync, writeFileSync } from "node:fs";
const out = [];
for (const a of [4, 8, 16, 32, 64, 128]) for (const L of [8, 32, 128, 512, 2048, 8192, 32768]) {
  if (L < a) continue;
  let E = 0;
  for (let s = 1; 2 * s <= a; s++) E += (a - 2 * s + 1) * (L - 2 * s + 1);
  const c1 = ((a + 1) * a / 2) * (L + 1);
  out.push({ a, L, E, c1, rho1: (2 * E) / c1 });
}
// cross-check the closed form against the brute-force count in translate_test.json
const d = JSON.parse(readFileSync("translate_test.json", "utf8"));
let bad = 0;
for (const s of d.slabs.filter((s) => s.name === "s3/2")) {
  let E = 0; for (let k = 1; 2 * k <= s.a; k++) E += (s.a - 2 * k + 1) * (s.L - 2 * k + 1);
  if (E !== s.E) bad++;
}
console.log("closed form vs brute force mismatches:", bad);
writeFileSync("slabs_more.json", JSON.stringify(out, null, 1));
console.log(out.filter((r) => r.a === 128).map((r) => `L${r.L}:${r.rho1.toFixed(4)}`).join(" "));
console.log("max rho1:", Math.max(...out.map((r) => r.rho1)).toFixed(6));
