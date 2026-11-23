// Stage 1a: provenance, identity, registration, record integrity, certificates and pricing decomposition.
// Does NOT read results.json / results.md / summary.json / band/ / analysis/ (producer outputs).
import { readFileSync, existsSync, statSync } from "node:fs";
import { execFileSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { sha256, sha256File, canonical, Curve, eqPt, decompose } from "./lib.mjs";

export const HERE = path.dirname(fileURLToPath(import.meta.url));
export const STUDY = path.resolve(HERE, "..");
export const REPO = path.resolve(STUDY, "../..");
export const REL = "research/ic_tableb_ksum_20261115";
export const FROZEN = "6860852096c10585ea76085486d24df3ec758795";       // commit that lands sessions + results
export const REG_COMMIT = "8143aef076bcf7903477dde8852194cb9c927949";   // commit that registered the protocol
export const BITS = [20, 22, 24];
export const REPS = [1, 2, 3];
export const WEIGHTS = [0, 0.1, 1];
const git = (...a) => execFileSync("git", ["-C", REPO, ...a], { encoding: "utf8", maxBuffer: 1 << 28 });
const gitBuf = (...a) => execFileSync("git", ["-C", REPO, ...a], { maxBuffer: 1 << 28 });

// ---- expectations transcribed by hand from PROTOCOL.md section 2 (NOT from make_specs.mjs)
const EXPECT = {
  20: { name: "bench-20bit", slug: "icv1-fp20-t1293-3c2999ba", cal: "prime/bench-20bit", log2r: 20.0, tb: 51, extra: { "inc-rec": 45, "inc-sel": 64 }, a5: 9, a4: 10, maxt: "1410236" },
  22: { name: "generated-22bit-3290411", slug: "icv1-fp22-tm1267-a5acaf6d", cal: "prime/generated-22bit-3290411", log2r: 21.65, tb: 74, extra: { "inc-lo": 53, "inc-hi": 104 }, a5: 10, a4: 12, maxt: "4243777" },
  24: { name: "generated-24bit-10935329", slug: "icv1-fp24-t1577-77336155", cal: "prime/generated-24bit-10935329", log2r: 23.38, tb: 111, extra: { "inc-rec": 162, "inc-w0": 115 }, a5: 11, a4: 14, maxt: "4243777" },
};
export const EXPECTED = EXPECT;
const KMAX = "13528399621";

export function expectedArms(b) {
  const e = EXPECT[b], mitm = (n) => ({ id: "ic.pipeline", params: { factor_base: `prime-abscissa:size=${n}`, oracle: "mitm:negation_folded=1", max_trials: e.maxt } });
  const ks = (k, a) => ({ id: "ic.pipeline", params: { factor_base: `prime-abscissa:size=${a}`, oracle: `ksum:m=${k}`, max_trials: KMAX } });
  const arms = { "rho-neg": { id: "rho.negation" }, "rho-neg-aa": { id: "rho.negation" }, "bsgs-neg": { id: "bsgs.negation" }, "inc-tb": mitm(e.tb) };
  for (const [k, n] of Object.entries(e.extra)) arms[k] = mitm(n);
  arms["ksum4"] = ks(4, e.a4); arms["ksum5"] = ks(5, e.a5);
  return arms;
}

export function loadAll(log = () => {}) {
  const C = { errors: [] };                 // C.errors: reasons the run is invalid_measurement / identity failure
  const fail = (m) => { C.errors.push(m); };
  const prov = C.provenance = {};
  prov.frozen_commit = FROZEN;
  prov.head = git("rev-parse", "HEAD").trim();
  prov.head_time = git("log", "-1", "--format=%cI", "HEAD").trim();
  prov.frozen_time = git("log", "-1", "--format=%cI", FROZEN).trim();
  prov.registration_commit = REG_COMMIT;
  prov.registration_commit_time = git("log", "-1", "--format=%cI", REG_COMMIT).trim();
  prov.frozen_is_ancestor_of_head = (() => { try { git("merge-base", "--is-ancestor", FROZEN, "HEAD"); return true; } catch { return false; } })();
  prov.changed_study_paths_frozen_to_head = git("diff", "--name-only", FROZEN, "HEAD", "--", REL).split("\n").filter(Boolean);
  prov.git_status_porcelain_study = git("status", "--porcelain", "--", REL).split("\n").filter(Boolean);
  prov.git_status_porcelain_repo_count = git("status", "--porcelain").split("\n").filter(Boolean).length;
  prov.note = "records.jsonl are gitignored (artifact store); their bytes are bound by .artifacts.json (committed) and re-hashed here";

  // ---- byte-identity of every committed file consumed, worktree vs the frozen-commit blob
  const consumed = [`${REL}/PROTOCOL.md`, `${REL}/REGISTRATION.sha256`, `${REL}/registered-at.txt`, `${REL}/BINARY.sha256`, `${REL}/make_specs.mjs`, `${REL}/results.json`, `${REL}/replay-certificate.json`,
    "docs/ic/calibration.json", "docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json"];
  for (const b of BITS) for (const r of REPS) {
    const n = `P${b}-r${r}`;
    consumed.push(`${REL}/specs/${n}.json`, `${REL}/sessions/${n}/spec.json`, `${REL}/sessions/${n}/session.json`, `${REL}/sessions/${n}/.artifacts.json`);
  }
  const identical = {};
  for (const f of consumed) {
    const blob = sha256(gitBuf("show", `${FROZEN}:${f}`));
    identical[f] = { frozen_blob_sha256: blob, worktree_sha256: sha256File(path.join(REPO, f)) };
    if (identical[f].frozen_blob_sha256 !== identical[f].worktree_sha256) fail(`worktree differs from frozen blob: ${f}`);
  }
  prov.consumed_files = identical;
  for (const code of ["src/cryptanalysis/ecbench/band.rs"]) prov[`sha256_${code}_worktree`] = sha256File(path.join(REPO, code));

  // ---- registration, protocol and binary identity
  const reg = {};
  for (const line of readFileSync(path.join(STUDY, "REGISTRATION.sha256"), "utf8").split("\n")) {
    const m = line.match(/^([0-9a-f]{64})\s+(\S+)/); if (m) reg[m[2]] = m[1];
  }
  const binSha = readFileSync(path.join(STUDY, "BINARY.sha256"), "utf8").split("\n")[0].split(/\s+/)[0];
  C.binary_sha256_expected = binSha;
  if (binSha !== "7d46e5ee0778a741193c4af08274fa4861406182e479b06770f5dd8303c5c9cb") fail("BINARY.sha256 differs from the dispatch's binary hash");
  const regChecks = { protocol_hash_ok: sha256File(path.join(STUDY, "PROTOCOL.md")) === reg["PROTOCOL.md"], make_specs_hash_ok: sha256File(path.join(STUDY, "make_specs.mjs")) === reg["make_specs.mjs"] };
  if (!regChecks.protocol_hash_ok) fail("PROTOCOL.md hash differs from REGISTRATION.sha256");
  if (!regChecks.make_specs_hash_ok) fail("make_specs.mjs hash differs from REGISTRATION.sha256");
  // PROTOCOL.md / specs unchanged since the registration commit
  regChecks.protocol_and_specs_unchanged_since_registration_commit =
    git("diff", "--name-only", REG_COMMIT, FROZEN, "--", `${REL}/PROTOCOL.md`, `${REL}/REGISTRATION.sha256`, `${REL}/specs`, `${REL}/make_specs.mjs`, `${REL}/registered-at.txt`).trim() === "";
  if (!regChecks.protocol_and_specs_unchanged_since_registration_commit) fail("registered files changed after the registration commit");
  const regAt = readFileSync(path.join(STUDY, "registered-at.txt"), "utf8").trim();
  regChecks.registered_at_txt = regAt;
  regChecks.registered_at_ms = Date.parse(regAt);
  regChecks.registration_commit_ms = Date.parse(prov.registration_commit_time);
  C.registration = regChecks;

  // ---- curves from the Round-5 ledger (independent of make_specs)
  const ledger = JSON.parse(readFileSync(path.join(REPO, "docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json"), "utf8")).ledger.instances;
  const calAll = JSON.parse(readFileSync(path.join(REPO, "docs/ic/calibration.json"), "utf8")).instances;

  C.curves = {}; C.sessions = {};
  for (const b of BITS) {
    const e = EXPECT[b];
    const inst = ledger.find((x) => x.curve.name === e.name);
    if (!inst) { fail(`ledger has no instance ${e.name}`); continue; }
    const c = inst.curve;
    const ledgerSpec = { kind: "prime_explicit", p: c.p, a: c.a, b: c.b, group_order: c.group_order, r: inst.r, gx: c.generator.x, gy: c.generator.y };
    const E = new Curve(c.p, c.a, c.b);
    const G = [BigInt(c.generator.x), BigInt(c.generator.y)];
    const rr = BigInt(inst.r);
    const curveChecks = { name: e.name, generator_on_curve: E.onCurve(G), order_kills_generator: eqPt(E.mul(rr, G), null), group_order_eq_r_cofactor1: String(c.group_order) === String(inst.r), r: inst.r, sqrt_r: Math.sqrt(Number(inst.r)), log2_r: Math.log2(Number(inst.r)), calibration_row: e.cal, ledger_spec: ledgerSpec };
    if (!curveChecks.generator_on_curve || !curveChecks.order_kills_generator) fail(`${e.name}: generator not on curve or order wrong`);
    if (Math.abs(curveChecks.log2_r - e.log2r) > 0.01) fail(`${e.name}: log2 r ${curveChecks.log2_r} vs protocol ${e.log2r}`);
    C.curves[b] = { E, G, r: rr, sqrtR: Math.sqrt(Number(inst.r)), cal: calAll[e.cal], checks: curveChecks, ledgerSpec };
    if (!calAll[e.cal]) fail(`no calibration row ${e.cal}`);

    for (const rep of REPS) {
      const name = `P${b}-r${rep}`, dir = path.join(STUDY, "sessions", name);
      const S = { name, bits: b, rep, checks: {} };
      const ck = S.checks;
      // spec identity
      const specBytes = readFileSync(path.join(dir, "spec.json"));
      const regKey = `specs/${name}.json`;
      ck.spec_file_sha256 = sha256(specBytes);
      ck.spec_equals_specs_file_bytes = Buffer.compare(specBytes, readFileSync(path.join(STUDY, "specs", `${name}.json`))) === 0;
      ck.spec_sha256_matches_registration = ck.spec_file_sha256 === reg[regKey];
      const spec = JSON.parse(specBytes), session = JSON.parse(readFileSync(path.join(dir, "session.json"), "utf8"));
      S.spec = spec; S.session = session;
      // ecbench serialises an arm's method as {id, params} with params defaulting to {} (MethodSpec in methods.rs), label/description excluded
      const idView = { schema: spec.schema, workloads: spec.workloads, arms: spec.arms.map((a) => ({ name: a.name, role: a.role, method: { id: a.method.id, params: a.method.params || {} } })), measurement: spec.measurement };
      let idSha = null; try { idSha = sha256(Buffer.from(canonical(idView))); } catch (err) { idSha = "ERR " + err.message; }
      ck.identity_view_sha256_recomputed = idSha;
      ck.session_spec_sha256 = session.spec_sha256;
      ck.spec_id_prefix_ok = session.spec_id === "ECS1h" + session.spec_sha256.slice(0, 12);
      ck.identity_view_sha256_matches_session = idSha === session.spec_sha256;
      ck.binary_sha256_session = session.binary_sha256;
      ck.session_git_commit = session.git_commit; ck.session_git_dirty = session.git_dirty;
      // spec content vs PROTOCOL section 2
      const wl = spec.workloads;
      const issues = [];
      if (wl.curves.length !== 1 || canonical(wl.curves[0]) !== canonical(ledgerSpec)) issues.push("curve differs from ledger instance " + e.name);
      if (wl.targets_per_curve !== 48) issues.push("targets_per_curve != 48");
      if (wl.target_kind !== "public") issues.push("target_kind != public");
      const m = spec.measurement;
      if (m.rounds !== 3 || m.warmup !== 1 || m.order !== "alternate") issues.push("measurement rounds/warmup/order");
      const wantArms = expectedArms(b);
      const gotNames = spec.arms.map((a) => a.name);
      if (canonical([...gotNames].sort()) !== canonical(Object.keys(wantArms).sort())) issues.push("arm set " + gotNames.join(","));
      for (const a of spec.arms) {
        const w = wantArms[a.name]; if (!w) continue;
        if (canonical(a.method) !== canonical(w)) issues.push(`arm ${a.name}: ${canonical(a.method)} != ${canonical(w)}`);
      }
      ck.spec_vs_protocol_issues = issues;
      ck.target_seed = wl.target_seed; ck.measurement_seed = m.seed;
      for (const f of ["spec_equals_specs_file_bytes", "spec_sha256_matches_registration", "spec_id_prefix_ok", "identity_view_sha256_matches_session"]) if (!ck[f]) fail(`${name}: ${f} false`);
      if (issues.length) fail(`${name}: spec vs protocol: ${issues.join("; ")}`);
      if (session.binary_sha256 !== binSha) fail(`${name}: session.json binary_sha256 differs`);
      // session start time vs registration
      const t0 = session.preflight?.before?.unix_ms;
      ck.session_preflight_start_ms = t0; ck.session_preflight_start_utc = new Date(t0).toISOString();
      ck.started_after_registration = t0 > regChecks.registered_at_ms && t0 > regChecks.registration_commit_ms;
      if (!ck.started_after_registration) fail(`${name}: started before registration`);

      // ---- artifacts manifest and the records file
      const manifest = JSON.parse(readFileSync(path.join(dir, ".artifacts.json"), "utf8")).files["records.jsonl"];
      const rp = path.join(dir, "records.jsonl");
      const raw = readFileSync(rp);
      ck.records_sha256 = sha256(raw); ck.records_bytes = raw.length;
      ck.records_manifest_sha256 = manifest.sha256; ck.records_manifest_bytes = manifest.bytes;
      ck.records_hash_matches_manifest = ck.records_sha256 === manifest.sha256 && raw.length === manifest.bytes;
      if (!ck.records_hash_matches_manifest) fail(`${name}: records.jsonl does not hash to .artifacts.json`);
      S.records_path = rp; S.records_sha256 = ck.records_sha256;

      // ---- parse records
      const lines = raw.toString("utf8").split("\n").filter((l) => l.length);
      const recs = [];
      let autBad = 0, methodBad = 0, tseedBad = 0, sealBad = 0, binBad = 0, sessBad = 0, notVerified = 0, notMatch = 0, certBad = 0, onCurveBad = 0, warm = 0, sumBad = 0, slugBad = 0, specBad = 0, sBad = 0, noCost = 0, zeroCost = 0, planted = 0;
      for (const line of lines) {
        const r = JSON.parse(line);
        const field = `"record_id":"${r.record_id}"`;
        if (!line.includes(field) || !r.record_id.startsWith("ECR1h") || "ECR1h" + sha256(Buffer.from(line.replace(field, '"record_id":""'))).slice(0, 12) !== r.record_id) sealBad++;
        if (r.binary_sha256 !== binSha) binBad++;
        if (r.session_id !== session.session_id) sessBad++;
        if (r.warmup) warm++;
        if (r.outcome.status !== "verified") notVerified++;
        if (r.outcome.matches_target !== true) notMatch++;
        if (r.workload.curve.slug !== e.slug) slugBad++;
        if ((r.arm === "rho-neg" || r.arm === "rho-neg-aa" || r.arm === "bsgs-neg") && r.boundaries?.automorphisms_used !== 2) autBad++;
        if (r.workload.target_seed !== wl.target_seed || r.workload.target_law !== "hash_to_subgroup_v1") tseedBad++;
        { const sa = spec.arms.find((x) => x.name === r.arm); const rp2 = r.method.params || {};
          if (!sa || r.method.id !== sa.method.id || Object.entries(sa.method.params || {}).some(([k, v]) => String(rp2[k]) !== String(v))) methodBad++; }
        if (canonical(r.workload.curve_spec) !== canonical(ledgerSpec)) specBad++;
        if (r.outcome.matches_planted !== null) planted++;
        // independent certificate: [recovered] G == target, target on curve
        const T = [BigInt(r.workload.target[0]), BigInt(r.workload.target[1])];
        if (!E.onCurve(T)) onCurveBad++;
        const d = BigInt(r.outcome.recovered);
        if (!(d >= 0n && d < rr) || !eqPt(E.mul(d, G), T)) certBad++;
        // cost bookkeeping
        const G0 = r.cost.total_gae;
        if (G0 === null || G0 === undefined) noCost++; else if (!(G0 > 0)) zeroCost++;
        const ph = r.phases.reduce((s, p) => s + p.gae, 0);
        if (G0 != null && Math.abs(ph - G0) > 1e-9 * Math.max(1, Math.abs(G0))) sumBad++;
        if (G0 != null && Math.abs(r.cost.s - G0 / C.curves[b].sqrtR) > 1e-9 * Math.max(1, r.cost.s)) sBad++;
        recs.push(r);
      }
      ck.record_count = recs.length; ck.warmup_records = warm; ck.measured_records = recs.length - warm;
      ck.seal_failures = sealBad; ck.binary_sha_mismatches = binBad; ck.session_id_mismatches = sessBad;
      ck.not_verified = notVerified; ck.matches_target_not_true = notMatch; ck.certificate_failures = certBad;
      ck.target_off_curve = onCurveBad; ck.curve_slug_mismatches = slugBad; ck.curve_spec_mismatches = specBad;
      ck.negation_arms_not_using_2_automorphisms = autBad; ck.method_or_params_differ_from_spec = methodBad; ck.target_seed_differs_from_spec = tseedBad; ck.phase_sum_ne_total = sumBad; ck.cost_s_ne_total_over_sqrt_r = sBad; ck.no_cost = noCost; ck.nonpositive_cost = zeroCost; ck.matches_planted_non_null = planted;
      for (const [f, bad] of [["seal", sealBad], ["binary_sha", binBad], ["session_id", sessBad], ["unverified", notVerified], ["matches_target", notMatch], ["certificate", certBad], ["off_curve", onCurveBad], ["slug", slugBad], ["curve_spec", specBad], ["negation_automorphisms", autBad], ["method_vs_spec", methodBad], ["target_seed", tseedBad], ["no_cost", noCost], ["nonpositive_cost", zeroCost]]) if (bad) fail(`${name}: ${bad} record(s) fail ${f}`);
      S.recs = recs;
      // ---- structure: arms x targets x rounds
      const armNames = spec.arms.map((a) => a.name);
      const byArm = {};
      for (const r of recs) (byArm[r.arm] ||= []).push(r);
      ck.records_per_arm = Object.fromEntries(armNames.map((a) => [a, byArm[a]?.length || 0]));
      const want = 48 * (m.rounds + m.warmup);
      for (const a of armNames) if ((byArm[a]?.length || 0) !== want) fail(`${name}: arm ${a} has ${byArm[a]?.length} records, expected ${want}`);
      const measuredPerArm = Object.fromEntries(armNames.map((a) => [a, (byArm[a] || []).filter((r) => !r.warmup).length]));
      ck.measured_per_arm = measuredPerArm;
      // warmup flag == round 0
      ck.warmup_iff_round0 = recs.every((r) => r.warmup === (r.round === 0));
      if (!ck.warmup_iff_round0) fail(`${name}: warmup flag inconsistent with round 0`);
      // pairing validity: per (workload_id, round) every arm once; same target, same algorithm_seed
      const cell = new Map();
      let dupe = 0;
      for (const r of recs) {
        const k = r.workload.workload_id + "|" + r.round;
        const c = cell.get(k) || { arms: {}, seeds: new Set(), targets: new Set(), tidx: new Set() };
        if (c.arms[r.arm]) dupe++;
        c.arms[r.arm] = r; c.seeds.add(r.algorithm_seed); c.targets.add(r.workload.target.join(",")); c.tidx.add(r.workload.target_index);
        cell.set(k, c);
      }
      let incompl = 0, seedSplit = 0, targSplit = 0;
      for (const c of cell.values()) {
        if (Object.keys(c.arms).length !== armNames.length) incompl++;
        if (c.seeds.size !== 1) seedSplit++;
        if (c.targets.size !== 1 || c.tidx.size !== 1) targSplit++;
      }
      ck.cells = cell.size; ck.duplicate_arm_records = dupe; ck.cells_missing_an_arm = incompl;
      ck.cells_with_more_than_one_algorithm_seed = seedSplit; ck.cells_with_more_than_one_target = targSplit;
      if (dupe || incompl || targSplit) fail(`${name}: pairing structure (dupes ${dupe}, incomplete ${incompl}, target splits ${targSplit})`);
      S.cell = cell;
      // seeds: is the algorithm seed shared across arms, across rounds?
      const seedsByTargetRound = {};
      for (const [k, c] of cell) seedsByTargetRound[k] = [...c.seeds];
      ck.algorithm_seed_shared_across_arms = seedSplit === 0;
      const wids = new Set(recs.map((r) => r.workload.workload_id));
      ck.distinct_workloads = wids.size;
      S.wids = wids;
      // pricing decomposition (all records)
      const calRow = C.curves[b].cal;
      S.dec = recs.map((r) => decompose(r, calRow));
      { let ic = 0, nonint = 0, maxfrac = 0, missingCounts = 0, noMarker = 0;
        recs.forEach((r, i) => { const d = S.dec[i]; if (d.missing.length) missingCounts++;
          if (d.nC > 0 || r.method.id === "ic.pipeline") { ic++; const g = d.G - d.C, fr = Math.abs(g - Math.round(g)); maxfrac = Math.max(maxfrac, fr); if (fr > 1e-6) nonint++; if (d.tableWork.table_insert_attempts === undefined) noMarker++; } });
        ck.ic_records_incl_warmup = ic; ck.ic_records_with_nonintegral_G_minus_C = nonint; ck.ic_max_fractional_part_G_minus_C = maxfrac; ck.records_with_uncounted_unpriced_counter = missingCounts; ck.ic_records_without_table_insert_marker = noMarker;
        if (nonint) fail(`${name}: ${nonint} IC records with non-integral G - C (wrong calibration row?)`);
        if (missingCounts) fail(`${name}: ${missingCounts} records name an unpriced counter without a count`);
        if (noMarker) fail(`${name}: ${noMarker} IC records lack the counted:table_insert_attempts marker`); }
      C.sessions[name] = S;
      log(`${name}: ${recs.length} records, cert failures ${certBad}, seal failures ${sealBad}`);
    }
  }
  // cross-session: distinct workloads and targets within each curve; replicate seeds differ
  C.cross = {};
  for (const b of BITS) {
    const all = new Map(); let collisions = 0, targCollide = 0; const targs = new Set();
    for (const rep of REPS) {
      const S = C.sessions[`P${b}-r${rep}`];
      for (const w of S.wids) { if (all.has(w)) collisions++; all.set(w, rep); }
      for (const r of S.recs) if (!r.warmup && r.arm === "ksum5") { const t = r.workload.target.join(","); if (targs.has(t) && !S.__t?.has(t)) targCollide++; targs.add(t); }
    }
    const seeds = REPS.map((rep) => [C.sessions[`P${b}-r${rep}`].spec.workloads.target_seed, C.sessions[`P${b}-r${rep}`].spec.measurement.seed]);
    C.cross[b] = { distinct_workload_ids: all.size, workload_id_collisions_across_sessions: collisions, seeds, replicate_seeds_distinct: new Set(seeds.map((s) => s[0])).size === 3 && new Set(seeds.map((s) => s[1])).size === 3 };
    if (collisions) fail(`P${b}: workload_id collides across sessions`);
    if (!C.cross[b].replicate_seeds_distinct) fail(`P${b}: replicate seeds not distinct`);
  }
  return C;
}
