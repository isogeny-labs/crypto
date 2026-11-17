// Tooling: property test of Theorem K (README section 2) and reproduction of the source
// study's 11 exact translate counts.  Every figure is an exact integer count.
//   E_trans(M) = sum_{s>=1} #{ y in Z^n : y + s*nu in R for every exponent nu of f }
// for M = lattice points of a convex polytope R in the orthant;  c_i(M) = sum_{mu in M} mu_i;
// Theorem K:  W_i * E(M) <= c_i(M)  with  W_i = (max - min over supp f of the exponent of x_i).
import { writeFileSync } from "node:fs";

const S3_3 = [[2,0,2],[1,1,2],[0,2,2],[2,1,1],[1,0,1],[1,2,1],[0,1,1],[0,0,1],[2,2,0],[1,1,0],[0,0,0],[1,0,0],[0,1,0]];
const S3_2 = [[2,0],[1,1],[0,2],[2,1],[1,0],[1,2],[0,1],[0,0],[2,2]];
const LIN2 = [[0,0],[1,0],[0,1]];
const UNI2 = [[0],[1],[2]];
const ext = (supp, i) => Math.max(...supp.map((v) => v[i])) - Math.min(...supp.map((v) => v[i]));

// R = {x >= 0 : A x <= b};  returns { points:Set of keys, c:[..], count }
function lattice(n, A, b, box) {
  const pts = [], key = (v) => v.join(",");
  const rec = (i, v) => {
    if (i === n) { if (A.every((a, j) => a.reduce((s, ai, k) => s + ai * v[k], 0) <= b[j] + 1e-9)) pts.push(v.slice()); return; }
    for (let x = 0; x <= box[i]; x++) { v[i] = x; rec(i + 1, v); }
  };
  rec(0, new Array(n).fill(0));
  return { pts, inR: (v) => A.every((a, j) => a.reduce((s, ai, k) => s + ai * v[k], 0) <= b[j] + 1e-9) };
}
function Etrans(n, supp, R, box) {
  let E = 0;
  for (let s = 1; s <= 60; s++) {
    let cnt = 0;
    const rec = (i, y) => {
      if (i === n) { if (supp.every((nu) => R.inR(nu.map((c, k) => y[k] + s * c)))) cnt++; return; }
      for (let x = 0; x <= box[i]; x++) { y[i] = x; rec(i + 1, y); }
    };
    rec(0, new Array(n).fill(0));
    if (cnt === 0) break;
    E += cnt;
  }
  return E;
}
const cvec = (n, pts) => Array.from({ length: n }, (_, i) => pts.reduce((s, p) => s + p[i], 0));

// ---- 1. the source study's exact cases (box R = [0,L]^n): E_trans must equal its E exact
const srcCases = [
  ["linear", LIN2, 2, 4, 25], ["linear", LIN2, 3, 6, 77], ["univariate-d2", UNI2, 3, 6, 9],
  ["s3/2", S3_2, 1, 4, 9], ["s3/2", S3_2, 2, 4, 10], ["s3/2", S3_2, 2, 6, 34], ["s3/2", S3_2, 3, 6, 35], ["s3/2", S3_2, 3, 8, 83],
  ["s3/3", S3_3, 1, 3, 8], ["s3/3", S3_3, 2, 4, 28], ["s3/3", S3_3, 2, 5, 72],
];
const exactRows = [];
for (const [name, supp, t, L, Eexact] of srcCases) {
  const n = supp[0].length;
  const A = Array.from({ length: n }, (_, i) => Array.from({ length: n }, (_, k) => (k === i ? 1 : 0)));
  const R = lattice(n, A, new Array(n).fill(L), new Array(n).fill(L));
  // the source's E is the sum over s = 1..t of the layer dimensions
  let E = 0;
  for (let s = 1; s <= t; s++) {
    let cnt = 0;
    const rec = (i, y) => {
      if (i === n) { if (supp.every((nu) => R.inR(nu.map((c, k) => y[k] + s * c)))) cnt++; return; }
      for (let x = 0; x <= L; x++) { y[i] = x; rec(i + 1, y); }
    };
    rec(0, new Array(n).fill(0)); E += cnt;
  }
  const c = cvec(n, R.pts);
  const rho = c.map((ci, i) => (ext(supp, i) * E) / ci);
  exactRows.push({ name, t, L, E, Eexact, match: E === Eexact, rho: rho.map((r) => +r.toFixed(4)) });
}

// ---- 2. property test: random down-closed polytopes, S3 in 3 unknowns and 2 unknowns
let seed = 20261029;
const rnd = () => { seed = (seed * 1664525 + 1013904223) >>> 0; return seed / 2 ** 32; };
const rows = [];
let worst = 0, violations = 0;
for (const [name, supp, n] of [["s3/3", S3_3, 3], ["s3/2", S3_2, 2]]) {
  for (let trial = 0; trial < (n === 3 ? 160 : 400); trial++) {
    const nf = 2 + Math.floor(rnd() * 5);
    // down-closed: facets a.x <= 1 with positive normals, scaled by a size; extents follow from a
    const A = [], b = [];
    const scale = (n === 3 ? 4 + rnd() * 9 : 6 + rnd() * 30);
    const skew = Array.from({ length: n }, () => Math.exp((rnd() - 0.5) * 3.2));
    for (let j = 0; j < nf; j++) { const a = Array.from({ length: n }, (_, i) => (0.3 + rnd()) / (skew[i] * scale)); A.push(a); b.push(1); }
    // every axis must be cut: add the axis facets with random lengths
    for (let i = 0; i < n; i++) { const a = new Array(n).fill(0); a[i] = 1 / (scale * skew[i] * (0.5 + rnd())); A.push(a); b.push(1); }
    const box = Array.from({ length: n }, (_, i) => Math.ceil(1 / Math.min(...A.map((a) => (a[i] > 0 ? a[i] : 1e9)).map((x) => x)) ));
    if (box.some((x) => x > 70)) continue;
    const R = lattice(n, A, b, box);
    if (R.pts.length < 4) continue;
    const E = Etrans(n, supp, R, box);
    const c = cvec(n, R.pts);
    const rho = c.map((ci, i) => (ci > 0 ? (ext(supp, i) * E) / ci : 0));
    const rstar = (Math.min(...c.map((ci, i) => ci)) > 0) ? (E * ext(supp, c.indexOf(Math.min(...c)))) / Math.min(...c) : 0;
    const maxrho = Math.max(...rho);
    if (maxrho > worst) worst = maxrho;
    if (rho.some((r) => r > 1 + 1e-12)) violations++;
    rows.push({ name, pts: R.pts.length, E, c, rstar: +rstar.toFixed(5), maxrho: +maxrho.toFixed(5), aspect: Math.min(...box) / Math.max(...box) });
  }
}

// ---- 3. slabs [0,a] x [0,L]^(n-1): the ratio approaches 1/W from below
const slabs = [];
for (const [name, supp, n] of [["s3/3", S3_3, 3], ["s3/2", S3_2, 2]]) {
  for (const a of [2, 4, 8]) {
    for (const L of n === 3 ? [8, 16, 24, 32, 40] : [8, 16, 32, 64, 128, 256]) {
      const A = Array.from({ length: n }, (_, i) => Array.from({ length: n }, (_, k) => (k === i ? 1 : 0)));
      const b = [a, ...new Array(n - 1).fill(L)];
      const box = [a, ...new Array(n - 1).fill(L)];
      const R = lattice(n, A, b, box);
      const E = Etrans(n, supp, R, box);
      const c = cvec(n, R.pts);
      slabs.push({ name, a, L, E, c1: c[0], rho1: +((ext(supp, 0) * E) / c[0]).toFixed(5) });
    }
  }
}
writeFileSync("translate_test.json", JSON.stringify({ exactRows, rows, slabs, worst, violations }, null, 1));
console.log("source exact cases matching:", exactRows.filter((r) => r.match).length, "/", exactRows.length);
console.log("random polytopes:", rows.length, " theorem violations:", violations, " max W_i*E/c_i =", worst.toFixed(5));
console.log("slab best rho1:", Math.max(...slabs.map((s) => s.rho1)));
