// Stage 1b: recompute S per arm, I(w), the 9 pooled ratios, per-replicate ratios, A/A delta, bootstrap intervals
// and the registered L1 rule, from the loaded records (see load.mjs).
import { BITS, REPS, WEIGHTS, EXPECTED } from "./load.mjs";
import { splitmix32, quantile, mean, tQuantile } from "./lib.mjs";

const B = 4000, SEED = 20261002;                  // resamples and seed named by PROTOCOL.md section 3
export const BOOT = { resamples: B, seed: SEED };
const wkey = (w) => String(w);

// per curve: arm -> arrays aligned on the same pair index (session-major)
export function buildCurve(C, b) {
  const sqrtR = C.curves[b].sqrtR;
  const sessions = REPS.map((rep) => C.sessions[`P${b}-r${rep}`]);
  const armNames = sessions[0].spec.arms.map((a) => a.name);
  const pairs = [];                              // {sess, wid, round, tidx}
  const idx = {};                                // arm -> array of record indices per pair
  const data = {};                               // arm -> {gmc, un, extra, bitops}
  for (const a of armNames) data[a] = { gmc: [], un: [], extra: [], bitops: [], G: [] };
  sessions.forEach((S, si) => {
    const byKey = {};
    S.recs.forEach((r, i) => { if (r.warmup || r.outcome.status !== "verified") return; (byKey[r.workload.workload_id + "|" + r.round] ||= {})[r.arm] = i; });
    const keys = Object.keys(byKey).sort();
    for (const k of keys) {
      const cell = byKey[k];
      if (Object.keys(cell).length !== armNames.length) throw new Error("incomplete cell " + k);
      const [wid, round] = k.split("|");
      pairs.push({ sess: si, wid, round: +round, tidx: S.recs[cell[armNames[0]]].workload.target_index });
      for (const a of armNames) {
        const i = cell[a], r = S.recs[i], d = S.dec[i];
        if (d.missing.length) throw new Error("missing unpriced count " + r.record_id);
        data[a].gmc.push(d.G - d.C); data[a].un.push(d.U + d.nC); data[a].G.push(d.G);
        let extra = 0;
        for (const p of r.phases) for (const [n, v] of Object.entries(p.work_native || {})) if (!n.startsWith("counted:") && n !== "table_insert_attempts" && typeof v === "number") extra += v;
        data[a].extra.push(extra);
        data[a].bitops.push(r.work && r.work.bitops !== undefined ? Number(BigInt(r.work.bitops)) : null);
      }
    }
  });
  return { b, sqrtR, armNames, pairs, data, n: pairs.length, perSession: REPS.map((_, si) => pairs.filter((p) => p.sess === si).length) };
}
const costAt = (d, w, k) => d.gmc[k] + w * d.un[k];

export function sumRange(d, w, lo, hi) { let s = 0; for (let k = lo; k < hi; k++) s += costAt(d, w, k); return s; }
const bounds = (cv) => { const o = [0]; for (const n of cv.perSession) o.push(o[o.length - 1] + n); return o; };

// ratio of sums of arm `a` over arm `bb` on pair set [lo,hi) at weight w
export function ratioOver(cv, a, bb, w, lo = 0, hi = cv.n) { return sumRange(cv.data[a], w, lo, hi) / sumRange(cv.data[bb], w, lo, hi); }

export function meanS(cv, arm, w, lo = 0, hi = cv.n) { return sumRange(cv.data[arm], w, lo, hi) / (hi - lo) / cv.sqrtR; }

// 95% intervals for R = sum(num)/sum(den) over pair sets; strata = sessions inside the scope
function intervals(cv, num, den, w, scopeSessions, seedOff) {
  const bd = bounds(cv);
  const nArr = new Float64Array(cv.n), dArr = new Float64Array(cv.n);
  for (let k = 0; k < cv.n; k++) { nArr[k] = costAt(cv.data[num], w, k); dArr[k] = costAt(cv.data[den], w, k); }
  const strata = scopeSessions.map((si) => [bd[si], bd[si + 1]]);
  const R = (() => { let n = 0, d = 0; for (const [lo, hi] of strata) for (let k = lo; k < hi; k++) { n += nArr[k]; d += dArr[k]; } return n / d; })();
  // (1) pair bootstrap: resample (workload, round) pairs with replacement within each session
  const rng = splitmix32(SEED + seedOff);
  const vals = new Float64Array(B);
  for (let t = 0; t < B; t++) {
    let n = 0, d = 0;
    for (const [lo, hi] of strata) { const m = hi - lo; for (let j = 0; j < m; j++) { const k = lo + Math.floor(rng() * m); n += nArr[k]; d += dArr[k]; } }
    vals[t] = n / d;
  }
  const s1 = Array.from(vals).sort((x, y) => x - y);
  // (2) cluster bootstrap: resample workloads (each with its 3 rounds) within each session
  const rng2 = splitmix32(SEED + seedOff + 7919);
  const clusters = strata.map(([lo, hi]) => { const m = new Map(); for (let k = lo; k < hi; k++) { const wid = cv.pairs[k].wid; (m.get(wid) || m.set(wid, []).get(wid)).push(k); } return [...m.values()]; });
  const cn = clusters.map((cs) => cs.map((ks) => [ks.reduce((s, k) => s + nArr[k], 0), ks.reduce((s, k) => s + dArr[k], 0)]));
  for (let t = 0; t < B; t++) {
    let n = 0, d = 0;
    for (const cs of cn) { const m = cs.length; for (let j = 0; j < m; j++) { const c = cs[Math.floor(rng2() * m)]; n += c[0]; d += c[1]; } }
    vals[t] = n / d;
  }
  const s2 = Array.from(vals).sort((x, y) => x - y);
  // (3) linearised (delta-method) interval of the ratio of totals, workloads as clusters, Student t (the interval ecbench reports as primary)
  const all = cn.flat(); const K = all.length;
  const meanA = mean(all.map((c) => c[1]));
  const z = all.map((c) => (c[0] - R * c[1]) / meanA);
  const mz = mean(z); const sd = Math.sqrt(z.reduce((s, x) => s + (x - mz) ** 2, 0) / (K - 1));
  const se = sd / Math.sqrt(K), tq = tQuantile(0.975, K - 1);
  return {
    ratio: R, n_pairs: strata.reduce((s, [lo, hi]) => s + hi - lo, 0), clusters: K,
    pair_bootstrap: { lo: quantile(s1, 0.025), hi: quantile(s1, 0.975) },
    cluster_bootstrap: { lo: quantile(s2, 0.025), hi: quantile(s2, 0.975) },
    delta_method: { lo: R * Math.exp(-tq * se / R), hi: R * Math.exp(tq * se / R), se, t: tq },
  };
}

export function analyzeCurve(C, b) {
  const cv = buildCurve(C, b);
  const incs = cv.armNames.filter((a) => a.startsWith("inc-"));
  const out = { bits: b, pairs: cv.n, pairs_per_session: cv.perSession, arms: cv.armNames, incumbents: incs, sqrt_r: cv.sqrtR };
  // mean S per arm per weight (pooled and per replicate)
  const bd = bounds(cv);
  out.mean_S = {}; out.mean_S_by_replicate = {};
  for (const a of cv.armNames) {
    out.mean_S[a] = Object.fromEntries(WEIGHTS.map((w) => [wkey(w), meanS(cv, a, w)]));
    out.mean_S_by_replicate[a] = Object.fromEntries(WEIGHTS.map((w) => [wkey(w), REPS.map((_, si) => meanS(cv, a, w, bd[si], bd[si + 1]))]));
  }
  // recorded cost.s mean (pre-band, for reference only)
  out.recorded_cost_s_mean = {};
  for (const a of cv.armNames) out.recorded_cost_s_mean[a] = mean(cv.data[a].G) / cv.sqrtR;
  out.exact_sums = Object.fromEntries(cv.armNames.map((a) => [a, { charged_sum: cv.data[a].gmc.reduce((x, y) => x + y, 0), native_sum: cv.data[a].un.reduce((x, y) => x + y, 0) }]));
  out.weights = {};
  for (const w of WEIGHTS) {
    const ranked = incs.map((a) => [a, out.mean_S[a][wkey(w)]]).sort((x, y) => x[1] - y[1]);
    const I = ranked[0][0];
    const W = { incumbent: I, incumbent_ranking: ranked.map(([a, s]) => ({ arm: a, mean_S: s })), margin_second_over_best: ranked[1][1] / ranked[0][1] };
    const scopes = [["pooled", [0, 1, 2], 0], ...REPS.map((rep, si) => [`r${rep}`, [si], 100 * (si + 1)])];
    W.ksum5_over_incumbent = {};
    for (const [name, ss, off] of scopes) W.ksum5_over_incumbent[name] = intervals(cv, "ksum5", I, w, ss, off + Math.round(w * 10) * 1000 + b);
    // per-replicate argmin (would a replicate-alone incumbent differ?)
    W.per_replicate_argmin = REPS.map((rep, si) => incs.map((a) => [a, out.mean_S_by_replicate[a][wkey(w)][si]]).sort((x, y) => x[1] - y[1])[0][0]);
    // every incumbent, every scope: the largest pair-bootstrap upper end (covers the per-replicate-argmin reading of "each replicate alone")
    W.upper_each_incumbent_each_scope = {};
    for (const [name, ss, off] of scopes) { W.upper_each_incumbent_each_scope[name] = {}; for (const a of incs) { const iv = intervals(cv, "ksum5", a, w, ss, off + 500 + incs.indexOf(a) * 37 + Math.round(w * 10) * 1000 + b); W.upper_each_incumbent_each_scope[name][a] = { ratio: iv.ratio, upper_pair_bootstrap: iv.pair_bootstrap.hi }; } }
    W.per_replicate_argmin_reading = REPS.map((rep, si) => ({ replicate: `r${rep}`, incumbent: W.per_replicate_argmin[si], ratio: W.upper_each_incumbent_each_scope[`r${rep}`][W.per_replicate_argmin[si]].ratio, upper_pair_bootstrap: W.upper_each_incumbent_each_scope[`r${rep}`][W.per_replicate_argmin[si]].upper_pair_bootstrap }));
    // leave-one-replicate-out selection: choose I on the other two replicates, evaluate on the held-out one
    W.leave_one_replicate_out = REPS.map((rep, si) => {
      const other = REPS.map((_, j) => j).filter((j) => j !== si);
      const sel = incs.map((a) => { let s = 0, n = 0; for (const j of other) { s += sumRange(cv.data[a], w, bd[j], bd[j + 1]); n += bd[j + 1] - bd[j]; } return [a, s / n]; }).sort((x, y) => x[1] - y[1])[0][0];
      return { held_out: `r${rep}`, selected: sel, ratio_on_held_out: ratioOver(cv, "ksum5", sel, w, bd[si], bd[si + 1]) };
    });
    // against every incumbent, pooled
    W.ksum5_over_each_incumbent_pooled = Object.fromEntries(incs.map((a) => [a, ratioOver(cv, "ksum5", a, w)]));
    W.ksum4_over_incumbent_pooled = ratioOver(cv, "ksum4", I, w);
    // distance remaining and context
    W.ksum5_over_rho_neg = ratioOver(cv, "ksum5", "rho-neg", w);
    W.incumbent_over_rho_neg = ratioOver(cv, I, "rho-neg", w);
    W.ksum5_over_bsgs_neg = ratioOver(cv, "ksum5", "bsgs-neg", w);
    W.rho_neg_aa_over_rho_neg = ratioOver(cv, "rho-neg-aa", "rho-neg", w);
    W.pairs_ksum5_cheaper_than_incumbent = (() => { let c = 0; for (let k = 0; k < cv.n; k++) if (costAt(cv.data.ksum5, w, k) < costAt(cv.data[I], w, k)) c++; return c; })();
    // order of [rho-neg, ksum5, incumbent] by mean S
    W.order_by_mean_S = ["rho-neg", "ksum5", I].map((a) => [a, out.mean_S[a][wkey(w)]]).sort((x, y) => x[1] - y[1]).map((x) => x[0]);
    out.weights[wkey(w)] = W;
  }
  // order across the band
  out.order_across_band = {
    rho_vs_ksum5: WEIGHTS.map((w) => (out.mean_S["rho-neg"][wkey(w)] < out.mean_S.ksum5[wkey(w)] ? "rho-neg<ksum5" : "ksum5<rho-neg")),
    ksum5_vs_incumbent: WEIGHTS.map((w) => (out.mean_S.ksum5[wkey(w)] < out.mean_S[out.weights[wkey(w)].incumbent][wkey(w)] ? "ksum5<I" : "I<ksum5")),
  };
  // A/A noise floor: arms inc-tb, rho-neg, ksum5; max/min over 3 replicates of the arm's mean S
  out.aa_noise = {};
  for (const w of WEIGHTS) {
    const per = {};
    for (const a of ["inc-tb", "rho-neg", "ksum5"]) { const v = out.mean_S_by_replicate[a][wkey(w)]; per[a] = { replicate_means: v, max_over_min: Math.max(...v) / Math.min(...v) }; }
    out.aa_noise[wkey(w)] = { per_arm: per, delta: Math.max(...Object.values(per).map((x) => x.max_over_min)) };
  }
  // within-session A/A: rho-neg vs rho-neg-aa identical counts?
  out.rho_aa_within_session = REPS.map((rep, si) => {
    const S = C.sessions[`P${b}-r${rep}`]; const m = new Map(); let identical = 0, differ = 0;
    S.recs.forEach((r, i) => { if (r.arm === "rho-neg") m.set(r.workload.workload_id + "|" + r.round, i); });
    S.recs.forEach((r, i) => { if (r.arm !== "rho-neg-aa") return; const j = m.get(r.workload.workload_id + "|" + r.round); const a = S.recs[j];
      const same = a.cost.total_gae === r.cost.total_gae && JSON.stringify(a.counters) === JSON.stringify(r.counters) && a.outcome.recovered === r.outcome.recovered;
      same ? identical++ : differ++; });
    return { session: `r${rep}`, identical_records: identical, differing_records: differ };
  });
  // sensitivity: bitop/v1 unit (complete work accounting) and an all-work_native stress price
  const hasBitops = cv.armNames.every((a) => cv.data[a].bitops.every((x) => x !== null));
  out.sensitivity = { bitops_present_all_arms: hasBitops };
  if (hasBitops) {
    const sum = (a, lo = 0, hi = cv.n) => { let s = 0; for (let k = lo; k < hi; k++) s += cv.data[a].bitops[k]; return s; };
    out.sensitivity.bitop_v1_ksum5_over_each_incumbent_pooled = Object.fromEntries(incs.map((a) => [a, sum("ksum5") / sum(a)]));
    out.sensitivity.bitop_v1_ksum5_over_each_incumbent_by_replicate = Object.fromEntries(incs.map((a) => [a, REPS.map((_, si) => sum("ksum5", bd[si], bd[si + 1]) / sum(a, bd[si], bd[si + 1]))]));
    out.sensitivity.bitop_v1_ksum5_over_rho_neg_pooled = sum("ksum5") / sum("rho-neg");
    out.sensitivity.bitop_v1_incumbent_over_rho_neg_pooled = Object.fromEntries(incs.map((a) => [a, sum(a) / sum("rho-neg")]));
  }
  { const sumG = (a, lo = 0, hi = cv.n) => { let t = 0; for (let k = lo; k < hi; k++) t += cv.data[a].G[k]; return t; };
    out.sensitivity.recorded_total_gae_ksum5_over_each_incumbent_pooled = Object.fromEntries(incs.map((a) => [a, sumG("ksum5") / sumG(a)]));
    out.sensitivity.recorded_total_gae_ksum5_over_each_incumbent_by_replicate = Object.fromEntries(incs.map((a) => [a, REPS.map((_, si) => sumG("ksum5", bd[si], bd[si + 1]) / sumG(a, bd[si], bd[si + 1]))]));
    out.sensitivity.recorded_total_gae_ksum5_over_rho_neg_pooled = sumG("ksum5") / sumG("rho-neg"); }
  out.sensitivity.stress_price_all_work_native = {};
  for (const w of [0.1, 1]) {
    const cost = (a, lo = 0, hi = cv.n) => { let s = 0; for (let k = lo; k < hi; k++) s += cv.data[a].gmc[k] + w * (cv.data[a].un[k] + cv.data[a].extra[k]); return s; };
    out.sensitivity.stress_price_all_work_native[wkey(w)] = {
      ksum5_over_each_incumbent_pooled: Object.fromEntries(incs.map((a) => [a, cost("ksum5") / cost(a)])),
      ksum5_over_rho_neg: cost("ksum5") / cost("rho-neg"),
      mean_uncharged_work_native_ops_per_run: Object.fromEntries(cv.armNames.map((a) => [a, mean(cv.data[a].extra)])),
    };
  }
  return { cv, out };
}

// registered L1 rule per row (PROTOCOL.md section 3), from the analysis of one curve
export function l1Rule(out) {
  const ws = WEIGHTS.map(wkey);
  const pairUpper = (scope, w) => out.weights[w].ksum5_over_incumbent[scope].pair_bootstrap.hi;
  const scopes = ["pooled", "r1", "r2", "r3"];
  const rule = { definition: "upper end of 95% interval of R_w < 1 at w=0,0.1,1, pooled and in each replicate alone; and 1 - max_w upper(R_w, pooled) > delta - 1 (delta at w=0)", by_method: {} };
  for (const method of ["pair_bootstrap", "cluster_bootstrap", "delta_method"]) {
    const up = (scope, w) => out.weights[w].ksum5_over_incumbent[scope][method].hi;
    const cells = {};
    for (const s of scopes) cells[s] = Object.fromEntries(ws.map((w) => [w, up(s, w)]));
    const all_below_1 = scopes.every((s) => ws.every((w) => up(s, w) < 1));
    const maxUpper = Math.max(...ws.map((w) => up("pooled", w)));
    const delta0 = out.aa_noise["0"].delta;
    rule.by_method[method] = { upper_ends: cells, all_upper_below_1: all_below_1, max_pooled_upper: maxUpper, delta_w0: delta0, one_minus_max_upper: 1 - maxUpper, delta_minus_1: delta0 - 1, exceeds_aa_noise: 1 - maxUpper > delta0 - 1,
      exceeds_aa_noise_with_delta_at_each_w: Object.fromEntries(ws.map((w) => [w, 1 - maxUpper > out.aa_noise[w].delta - 1])), holds: all_below_1 && (1 - maxUpper > delta0 - 1) };
  }
  rule.holds = rule.by_method.pair_bootstrap.holds;
  return rule;
}
