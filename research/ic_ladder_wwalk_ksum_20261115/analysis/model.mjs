// Derived (not measured) k-sum cost model of research/prime_ksum_k5_20261028/PROTOCOL.md, used for the
// grid centre A* and for the memory / runtime estimate:  total(A) = M + c*A*r/(2M),  M = 2^(k-1) C(A,k),  c = 1.
//   node analysis/model.mjs
const R = { 36: 68719022539, 40: 1099512551701, 44: 17592192476281, 48: 281474991897067 };
const choose = (a, k) => { let c = 1; for (let i = 0; i < k; i++) c = (c * (a - i)) / (i + 1); return c; };
const M = (a, k) => 2 ** (k - 1) * choose(a, k);
const S = (a, k, r) => (M(a, k) + (a * r) / (2 * M(a, k))) / Math.sqrt(r);
const out = {};
for (const [b, r] of Object.entries(R)) for (const k of [5, 6, 7, 8]) {
  let best = k + 1;
  for (let a = k + 1; a < 400; a++) if (S(a, k, r) < S(best, k, r)) best = a;
  (out[b] ??= {})[k] = { A_star: best, entries: Math.round(M(best, k)), S_model: +S(best, k, r).toFixed(2) };
}
console.log(JSON.stringify(out, null, 1));
