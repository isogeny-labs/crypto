// Tooling: independent check of the review finding on PR #2900 (exact integer counts).
//   R = { (i,j) >= 0 : 2i + j <= 6 }; f = S3 in (e1, e2).
//   generic z:  supp = full simplex of degree 2 (6 monomials);  z = 0: T = (e2-a)^2 - 4b*e1, supp = {1, e2, e2^2, e1}.
const Fgen = [[2,0],[0,1],[1,1],[1,0],[0,0],[0,2]];
const Fz0  = [[0,0],[0,1],[0,2],[1,0]];
const inR = (i, j, L) => 2 * i + j <= L;
const pts = []; for (let i = 0; i <= 6; i++) for (let j = 0; j <= 6; j++) if (inR(i, j, 6)) pts.push([i, j]);
const c = [0, 1].map((k) => pts.reduce((s, p) => s + p[k], 0));
const Etr = (F, t) => { let E = 0; const per = [];
  for (let s = 1; s <= t; s++) { let n = 0; for (let y1 = 0; y1 <= 6; y1++) for (let y2 = 0; y2 <= 6; y2++) if (F.every((v) => inR(y1 + s * v[0], y2 + s * v[1], 6))) n++; per.push(n); E += n; } return [E, per]; };
const ext = (F, k) => Math.max(...F.map((v) => v[k])) - Math.min(...F.map((v) => v[k]));
console.log("|M| =", pts.length, " c =", c);
for (const [name, F] of [["generic z", Fgen], ["z = 0", Fz0]]) {
  const [E, per] = Etr(F, 3);
  const W = [ext(F, 0), ext(F, 1)];
  const reach = E / (c[0] + 2 * c[1]);
  const K = [W[0] * E <= c[0], W[1] * E <= c[1]];
  const ceil = 1 / (W[0] * 1 + W[1] * 2);
  console.log(`${name}: extents W = (${W}), layers |A_s| = ${per}, E = ${E}, determinant reach delta_B < E/(c1+2*c2) = ${E}/${c[0] + 2 * c[1]} = ${reach.toFixed(5)}; Theorem K holds: ${K}; its ceiling for these extents: 1/${1 / ceil} = ${ceil.toFixed(5)}`);
}
console.log("7/41 =", (7 / 41).toFixed(5), " 1/6 =", (1 / 6).toFixed(5), " 1/5 = 0.2");
// how close does the z = 0 polynomial get to its own ceiling 1/5?  scan simplices {i/alpha + j/beta <= 1}
const reachScan = (F, alpha, beta) => {
  const P = []; for (let i = 0; i <= alpha; i++) for (let j = 0; j <= beta; j++) if (i / alpha + j / beta <= 1 + 1e-12) P.push([i, j]);
  const has = new Set(P.map((p) => p.join(",")));
  let E = 0;
  for (let s = 1; s < 4000; s++) { let n = 0; for (const [y1, y2] of P) if (F.every((v) => has.has(`${y1 + s * v[0]},${y2 + s * v[1]}`))) n++; if (!n) break; E += n; }
  const cc = [P.reduce((s, p) => s + p[0], 0), P.reduce((s, p) => s + p[1], 0)];
  return [E / (cc[0] + 2 * cc[1]), P.length];
};
let best = [0, 0, 0, 0];
for (const alpha of [3, 6, 10, 20, 40, 80]) for (const beta of [6, 12, 24, 48, 96, 192, 400]) {
  const [r, n] = reachScan(Fz0, alpha, beta); if (r > best[0]) best = [r, alpha, beta, n];
}
console.log(`z = 0, simplex scan (convex R = {i/alpha + j/beta <= 1}): best reach ${best[0].toFixed(5)} at alpha=${best[1]}, beta=${best[2]}, |M|=${best[3]}  (ceiling 0.2)`);
let bestG = [0]; for (const alpha of [3, 6, 10, 20, 40, 80]) for (const beta of [6, 12, 24, 48, 96, 192, 400]) { const [r, n] = reachScan(Fgen, alpha, beta); if (r > bestG[0]) bestG = [r, alpha, beta, n]; }
console.log(`generic z, same scan: best reach ${bestG[0].toFixed(5)} at alpha=${bestG[1]}, beta=${bestG[2]}, |M|=${bestG[3]}  (ceiling 1/6 = 0.16667)`);
