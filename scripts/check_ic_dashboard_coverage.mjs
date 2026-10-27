#!/usr/bin/env node
// Check joins between the committed registry, leaderboard, browser and timeline.
// This is an index integrity check; it does not promote a measurement or reprice it.
import {createHash} from 'node:crypto';
import {existsSync, readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {dirname, resolve} from 'node:path';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const read = path => JSON.parse(readFileSync(resolve(root, path), 'utf8'));
const digest = path => createHash('sha256').update(readFileSync(resolve(root, path))).digest('hex');
const errors = [];
const requireThat = (condition, message) => { if (!condition) errors.push(message); };
const unique = (rows, label) => {
  const slugs = rows.map(row => row.slug);
  requireThat(slugs.every(slug => typeof slug === 'string' && slug.startsWith('icv1-')),
    `${label}: every curve needs an ICV1 slug`);
  requireThat(new Set(slugs).size === slugs.length, `${label}: duplicate ICV1 slugs`);
  return new Set(slugs);
};
const sameSet = (actual, expected, label) => {
  const missing = [...expected].filter(slug => !actual.has(slug));
  const extra = [...actual].filter(slug => !expected.has(slug));
  requireThat(!missing.length && !extra.length,
    `${label}: ${missing.length} registry curves missing, ${extra.length} unknown; ` +
    `examples: ${missing.slice(0, 3).join(', ') || 'none'}`);
};

const registry = read('docs/curves/registry.json');
const leaderboard = read('docs/ic/leaderboard.json');
const browser = read('docs/browser/data.json');
const timeline = read('docs/ic/progress-timeline.json');
const page = readFileSync(resolve(root, 'docs/ic-leaderboard.html'), 'utf8');
const registryCurves = unique(registry.curves, 'registry');
const rosterCurves = unique(leaderboard.roster, 'leaderboard roster');
const browserCurves = unique(browser.curves, 'lab browser');
sameSet(rosterCurves, registryCurves, 'leaderboard roster');
sameSet(browserCurves, registryCurves, 'lab browser');
const assessedCovers = browser.curves.filter(row => row.hyperelliptic_cover != null).length;
requireThat(browser.counts?.curves === browserCurves.size &&
  browser.counts?.cover_assessed === assessedCovers &&
  browser.counts?.cover_unassessed === browserCurves.size - assessedCovers,
  'lab browser cover coverage counts do not match the curve records');

// The canonical leaderboard grows past generic text-scan caps. Its IC1
// identities must remain searchable even when a scan of large files is skipped.
const ic1 = /IC1N\d+C[a-z][a-z0-9]*fb\d+PDP\d+[a-z0-9]+RC[a-z0-9]+LA[a-z0-9]+TD[a-z0-9]+ISO\d+h[a-f0-9]{12}/g;
const namedCandidates = new Set(readFileSync(resolve(root, 'docs/ic/leaderboard.json'), 'utf8').match(ic1) ?? []);
const browserCandidates = new Set(browser.candidates.map(row => row.candidate_id));
const missingCandidates = [...namedCandidates].filter(id => !browserCandidates.has(id));
requireThat(!missingCandidates.length,
  `lab browser omits ${missingCandidates.length} IC1 identities from the leaderboard; ` +
  `example: ${missingCandidates[0] ?? 'none'}`);

for (const path of ['docs/curves/registry.json', 'docs/curves/covers.json',
  'docs/curves/cover-links.yaml', 'docs/ic/leaderboard.json']) {
  requireThat(browser.sources?.[path] === digest(path),
    `lab browser source digest is stale for ${path}`);
}

const priced = new Set();
for (const row of leaderboard.board) {
  const label = `${row.slug} (${row.regime})`;
  priced.add(row.slug);
  requireThat(registryCurves.has(row.slug), `${label}: absent from registry`);
  requireThat(browserCurves.has(row.slug), `${label}: absent from lab browser`);
  requireThat(page.includes(row.slug), `${label}: absent from leaderboard page`);
  requireThat(typeof row.source === 'string' && row.source.length > 0,
    `${label}: missing frozen source`);
  if (typeof row.source === 'string') {
    requireThat(!row.source.startsWith('/') && !row.source.includes('..') &&
      existsSync(resolve(root, row.source)), `${label}: frozen source unavailable`);
  }
  requireThat(Number.isFinite(row.best?.ratio_rho) && row.best.ratio_rho > 0,
    `${label}: missing positive IC/rho ratio`);
  requireThat(Number.isInteger(row.factor_base?.points) && row.factor_base.points > 0,
    `${label}: missing measured factor-base size`);
  requireThat(typeof row.factor_base?.recipe === 'string' && row.factor_base.recipe.length > 0,
    `${label}: missing factor-base recipe`);
  requireThat(typeof row.factor_base?.link_status === 'string',
    `${label}: missing factor-base identity status`);
  if (row.factor_base?.link_status === 'reconciled') {
    requireThat(Array.isArray(row.factor_base.refs) && row.factor_base.refs.length > 0,
      `${label}: reconciled factor base has no identity reference`);
  }
  requireThat(typeof row.reference === 'string' && row.reference.length > 0,
    `${label}: missing matched-rho description`);
  requireThat(row.targets != null, `${label}: missing target count or scope`);
  requireThat(row.best?.phases_s && Object.keys(row.best.phases_s).length > 0,
    `${label}: missing phase split`);
  if (row.operation_unit?.includes('GAE')) {
    requireThat(Number.isFinite(row.best?.log2_total_gae) &&
      Number.isFinite(row.reference_log2_total_gae),
      `${label}: counted-operation row needs both IC and rho 2^x totals`);
  }
  requireThat(typeof row.validation?.status === 'string',
    `${label}: missing validation status`);
}
requireThat(priced.size > 0, 'leaderboard has no priced curves');

let timelinePoints = 0;
let latest = '';
for (const series of timeline.series) {
  for (const point of series.points) {
    timelinePoints++;
    const label = `${series.id} / ${point.label}`;
    requireThat(/^\d{4}-\d{2}-\d{2}$/.test(point.date ?? ''), `${label}: missing merge date`);
    requireThat(typeof point.source === 'string' && point.source.length > 0,
      `${label}: missing source`);
    requireThat(typeof point.anchor === 'string' && point.anchor.length > 0,
      `${label}: missing dashboard anchor`);
    requireThat(typeof point.reference === 'string' && point.reference.length > 0,
      `${label}: missing reference quality`);
    requireThat(typeof point.class === 'string' && point.class.length > 0,
      `${label}: missing progress class`);
    requireThat(Number.isFinite(point.value) && point.value > 0,
      `${label}: missing positive plotted value`);
    if (point.date > latest) latest = point.date;
  }
}
requireThat(timelinePoints > 0, 'progress timeline has no points');
requireThat(timeline.updated >= latest, 'timeline updated date precedes its latest point');

const unreconciled = leaderboard.board.filter(row =>
  row.factor_base?.link_status !== 'reconciled').length;
console.log(`IC dashboard: ${registryCurves.size} registered, ${rosterCurves.size} rostered, ` +
  `${browserCurves.size} searchable, ${priced.size} priced curves ` +
  `(${leaderboard.board.length} regime rows), ${unreconciled} rows without reconciled FB1, ` +
  `${browser.counts?.cover_unassessed} cover-unassessed curves, ${timelinePoints} timeline points`);
for (const error of errors) console.error(`coverage: ${error}`);
if (errors.length) process.exitCode = 1;
