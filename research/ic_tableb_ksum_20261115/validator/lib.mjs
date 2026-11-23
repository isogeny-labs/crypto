// Independent validator library for ic_tableb_ksum_20261115.
// Written from the artifacts, AGENTS.md section 5a and the *documented* pricing formula of
// src/cryptanalysis/ecbench/band.rs (priced_gae(w) = G - C + w*(U + n_C)); it shares no code with
// ecbench, with the study's analysis/evaluate.mjs (not read), or with the producer's scripts.
// Node stdlib only (BigInt for curve arithmetic).
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";

export const sha256 = (buf) => createHash("sha256").update(buf).digest("hex");
export const sha256File = (p) => sha256(readFileSync(p));

// ---------------------------------------------------------------- canonical JSON (ecbench canonical.rs)
export function canonical(v) {
  if (v === null) return "null";
  if (typeof v === "boolean") return v ? "true" : "false";
  if (typeof v === "number") {
    if (!Number.isInteger(v)) throw new Error("non-integer in identity: " + v);
    return String(v);
  }
  if (typeof v === "string") return canonStr(v);
  if (Array.isArray(v)) return "[" + v.map(canonical).join(",") + "]";
  const keys = Object.keys(v).sort();
  return "{" + keys.map((k) => canonStr(k) + ":" + canonical(v[k])).join(",") + "}";
}
function canonStr(s) {
  let o = '"';
  for (const ch of s) {
    const c = ch.codePointAt(0);
    if (ch === '"') o += '\\"';
    else if (ch === "\\") o += "\\\\";
    else if (ch === "\n") o += "\\n";
    else if (ch === "\r") o += "\\r";
    else if (ch === "\t") o += "\\t";
    else if (c === 8) o += "\\b";
    else if (c === 12) o += "\\f";
    else if (c >= 0x20 && c <= 0x7e) o += ch;
    else if (c < 0x10000) o += "\\u" + c.toString(16).padStart(4, "0");
    else {
      const h = ((c - 0x10000) >> 10) + 0xd800, l = ((c - 0x10000) & 0x3ff) + 0xdc00;
      o += "\\u" + h.toString(16) + "\\u" + l.toString(16);
    }
  }
  return o + '"';
}

// ---------------------------------------------------------------- short Weierstrass arithmetic over F_p (affine, BigInt)
const mod = (a, p) => { const r = a % p; return r >= 0n ? r : r + p; };
function inv(a, p) {
  let [t, nt] = [0n, 1n], [r, nr] = [p, mod(a, p)];
  while (nr !== 0n) { const q = r / nr; [t, nt] = [nt, t - q * nt]; [r, nr] = [nr, r - q * nr]; }
  if (r !== 1n) throw new Error("not invertible");
  return mod(t, p);
}
export class Curve {
  constructor(p, a, b) { this.p = BigInt(p); this.a = mod(BigInt(a), this.p); this.b = mod(BigInt(b), this.p); }
  onCurve(P) { // null = point at infinity
    if (P === null) return true;
    const { p } = this;
    return mod(P[1] * P[1] - (P[0] * P[0] * P[0] + this.a * P[0] + this.b), p) === 0n && P[0] >= 0n && P[0] < p && P[1] >= 0n && P[1] < p;
  }
  neg(P) { return P === null ? null : [P[0], mod(-P[1], this.p)]; }
  add(P, Q) {
    const { p } = this;
    if (P === null) return Q;
    if (Q === null) return P;
    let lam;
    if (P[0] === Q[0]) {
      if (mod(P[1] + Q[1], p) === 0n) return null;
      lam = mod((3n * P[0] * P[0] + this.a) * inv(2n * P[1], p), p);
    } else lam = mod((Q[1] - P[1]) * inv(Q[0] - P[0], p), p);
    const x = mod(lam * lam - P[0] - Q[0], p);
    return [x, mod(lam * (P[0] - x) - P[1], p)];
  }
  mul(k, P) {
    k = BigInt(k);
    if (k < 0n) { k = -k; P = this.neg(P); }
    let R = null, Q = P;
    while (k > 0n) { if (k & 1n) R = this.add(R, Q); Q = this.add(Q, Q); k >>= 1n; }
    return R;
  }
}
export const eqPt = (P, Q) => (P === null || Q === null) ? P === Q : P[0] === Q[0] && P[1] === Q[1];

// ---------------------------------------------------------------- pricing (band.rs, as documented)
// Counters the calibration prices (native map of a phase), with the unit key that prices each.
export const PRICED_NATIVE = [
  ["sqrt_solves", "ns_per_sqrt"], ["as_solves", "ns_per_as_solve"], ["s4_pairs", "ns_per_s4_pair"],
  ["lookups", "ns_per_lookup"], ["row_ops", "ns_per_row_op"], ["legendre_symbols", "ns_per_legendre"],
  ["inversions", "ns_per_inversion"], ["frobenius_maps", "ns_per_frobenius"], ["canonicalisations", "ns_per_canon"],
  ["target_guard_probes", "ns_per_lookup"],
];
export const PINNABLE_NATIVE = [
  ["hashed_tensor_steps", "ns_per_hashed_tensor_step"], ["pkm_system_fp_mults_uncharged", "ns_per_fp_mul"],
  ["pkm_grid_fp_mults_uncharged", "ns_per_fp_mul"], ["pkm_shared_poly_fp_mults_uncharged", "ns_per_fp_mul"],
  ["pkm_trace_learn_spec_fp_mults_uncharged", "ns_per_fp_mul"], ["pkm_t0_base_map_fp_mults_uncharged", "ns_per_fp_mul"],
  ["pkm_base_check_fp_mults_uncharged", "ns_per_fp_mul"], ["pkm_t0_target_fp_mults_uncharged", "ns_per_fp_mul"],
];
export const NON_OPERATION = ["residual_table_bytes_uncharged", "factor_base_vec_bytes_floor_uncharged", "native_table_bytes_uncharged"];
export const TABLE_WORK = ["table_insert_attempts"];
export const SIBLING_WORK = ["sibling_second_points", "sibling_lift_products"];

// Decompose a record's cost into  G - C  (charged GAE with calibration-priced native work removed)
// and  U + n_C  (operations the band prices at w).  `cal` is the pinned ratio row of the curve.
export function decompose(r, cal) {
  const G = r.cost.total_gae;
  const out = { G, C: 0, nC: 0, U: 0, missing: [], unpricedNames: [], tableWork: {}, exempt: {} };
  if (G === null || G === undefined) { out.noCost = true; return out; }
  // calibrated native work
  const pinned = (r.detail && Array.isArray(r.detail.calibration_pinned_units)) ? r.detail.calibration_pinned_units : [];
  if (pinned.length) {
    for (const p of r.phases) {
      for (const [counter, unit] of [...PRICED_NATIVE, ...PINNABLE_NATIVE]) {
        const n = (p.native && p.native[counter]) || 0;
        if (n > 0 && pinned.includes(unit)) {
          const k = cal[unit];
          if (typeof k !== "number") throw new Error(`no ratio ${unit} for ${r.record_id}`);
          out.C += n * k; out.nC += n;
        }
      }
    }
  }
  // unpriced counters named by the record
  for (const name of r.cost.unpriced || []) {
    const base = name.endsWith("_uncharged") ? name.slice(0, -"_uncharged".length) : name;
    let v = null;
    if (r.counters && Object.prototype.hasOwnProperty.call(r.counters, name)) v = r.counters[name];
    else {
      let tot = 0, found = false;
      for (const p of r.phases) {
        const nat = p.native || {};
        const x = Object.prototype.hasOwnProperty.call(nat, name) ? nat[name] : (Object.prototype.hasOwnProperty.call(nat, base) ? nat[base] : undefined);
        if (x !== undefined) { tot += x; found = true; }
      }
      if (!found && base.startsWith("solver_") && r.solver) { v = r.solver.ops; }
      else v = found ? tot : null;
    }
    if (NON_OPERATION.includes(name)) { out.exempt[name] = v; continue; }
    out.unpricedNames.push(name);
    if (v === null) out.missing.push(name); else out.U += v;
  }
  // counted table work (charged only behind its `counted:<name>` marker)
  for (const name of [...TABLE_WORK, ...SIBLING_WORK]) {
    const marker = "counted:" + name;
    if (!r.phases.some((p) => p.work_native && Object.prototype.hasOwnProperty.call(p.work_native, marker))) continue;
    let tot = 0;
    for (const p of r.phases) if (p.work_native && p.work_native[name] !== undefined) tot += p.work_native[name];
    out.tableWork[name] = tot; out.U += tot;
  }
  return out;
}

// ---------------------------------------------------------------- statistics
export function splitmix32(seed) { // small deterministic PRNG; seed documented in the report
  let s = seed >>> 0;
  return () => {
    s = (s + 0x9e3779b9) >>> 0;
    let z = s;
    z = Math.imul(z ^ (z >>> 16), 0x85ebca6b);
    z = Math.imul(z ^ (z >>> 13), 0xc2b2ae35);
    z = (z ^ (z >>> 16)) >>> 0;
    return z / 4294967296;
  };
}
export const quantile = (sortedAsc, q) => { // linear interpolation (type 7)
  const h = (sortedAsc.length - 1) * q, lo = Math.floor(h), hi = Math.ceil(h);
  return sortedAsc[lo] + (h - lo) * (sortedAsc[hi] - sortedAsc[lo]);
};
export const mean = (a) => a.reduce((s, x) => s + x, 0) / a.length;

// Student t quantile by bisection on the CDF (regularised incomplete beta, Lentz continued fraction).
function lgamma(x) {
  const c = [76.18009172947146, -86.50532032941677, 24.01409824083091, -1.231739572450155, 0.1208650973866179e-2, -0.5395239384953e-5];
  let y = x, t = x + 5.5; t -= (x + 0.5) * Math.log(t);
  let s = 1.000000000190015; for (const cj of c) s += cj / ++y;
  return -t + Math.log(2.5066282746310005 * s / x);
}
function betacf(a, b, x) {
  const FPMIN = 1e-300; let qab = a + b, qap = a + 1, qam = a - 1, c = 1, d = 1 - qab * x / qap;
  if (Math.abs(d) < FPMIN) d = FPMIN; d = 1 / d; let h = d;
  for (let m = 1; m <= 500; m++) {
    const m2 = 2 * m; let aa = m * (b - m) * x / ((qam + m2) * (a + m2));
    d = 1 + aa * d; if (Math.abs(d) < FPMIN) d = FPMIN; c = 1 + aa / c; if (Math.abs(c) < FPMIN) c = FPMIN; d = 1 / d; h *= d * c;
    aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
    d = 1 + aa * d; if (Math.abs(d) < FPMIN) d = FPMIN; c = 1 + aa / c; if (Math.abs(c) < FPMIN) c = FPMIN; d = 1 / d;
    const del = d * c; h *= del; if (Math.abs(del - 1) < 1e-15) break;
  }
  return h;
}
function betai(a, b, x) {
  if (x <= 0) return 0; if (x >= 1) return 1;
  const bt = Math.exp(lgamma(a + b) - lgamma(a) - lgamma(b) + a * Math.log(x) + b * Math.log(1 - x));
  return x < (a + 1) / (a + b + 2) ? bt * betacf(a, b, x) / a : 1 - bt * betacf(b, a, 1 - x) / b;
}
export function tCdf(t, df) { const x = df / (df + t * t), ib = betai(df / 2, 0.5, x); return t > 0 ? 1 - 0.5 * ib : 0.5 * ib; }
export function tQuantile(p, df) { let lo = 0, hi = 50; for (let i = 0; i < 200; i++) { const m = (lo + hi) / 2; if (tCdf(m, df) < p) lo = m; else hi = m; } return (lo + hi) / 2; }
