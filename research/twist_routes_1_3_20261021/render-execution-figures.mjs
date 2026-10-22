// Layout of already recorded results only. Mathematical aggregation is Rust.
import fs from 'node:fs';
import path from 'node:path';
const study = path.resolve(process.argv[2]);
const reports = path.resolve(process.argv[3]);
const mapRun = process.argv[4] ?? 'isogeny-20261021T2320Z';
const maps = JSON.parse(fs.readFileSync(path.join(study, 'execution-v1/runs', mapRun, 'E14-isogenies.json')));
const fixtures = JSON.parse(fs.readFileSync(path.join(study, 'execution-v1/fixtures.json'))).fixtures;
const quote = s => JSON.stringify(s);
let dot = 'digraph Maps { rankdir=LR; graph [fontname="DejaVu Sans", label="Verified rational isogeny maps; no cost comparison or assigned volcano depth", labelloc=t, nodesep=0.5, ranksep=0.8]; node [shape=box, fontname="DejaVu Sans", fontsize=11, style=filled, fillcolor="#eef5fa"]; edge [fontname="DejaVu Sans", fontsize=10];\n';
for (const [i, record] of maps.filter(x => x.status === 'verified-explicit-Velu-and-rational-subgroup').entries()) {
  const fixture = fixtures.find(x => x.id === record.fixture);
  dot += `subgraph cluster_${i} { label=${quote(`Fp=${fixture.p}; subgroup order ${fixture.order}`)}; color="#c5d2dc"; s${i} [label=${quote(record.fixture)}]; d${i} [label=${quote(record.destination)}]; s${i}->d${i} [label=${quote(`degree ${record.ell}; verified`)}]; `;
  dot += record.algebraic_dual_return_map
    ? `d${i}->s${i} [label=${quote(`dual degree ${record.ell}; composition [${record.ell}]`)}];`
    : `d${i}->s${i} [style=dashed, label=${quote(`finite lookup return; ${record.finite_lookup_return_map_entries} entries`)}];`;
  dot += ' }\n';
}
dot += '}\n';
const isogeny = path.join(reports, mapRun);
fs.writeFileSync(path.join(isogeny, 'MAPS.dot'), dot);
const md = path.join(isogeny, 'REPORT.md');
let text = fs.readFileSync(md, 'utf8');
if (!text.includes('MAPS.pdf')) {
  text += '\n![Verified source/destination models and return maps. A dashed return uses a finite lookup table; no volcano level is inferred.](MAPS.pdf)\n';
  fs.writeFileSync(md, text);
}
const analysis = JSON.parse(fs.readFileSync(path.join(reports, 'ANALYSIS.json')));
const record = analysis['ecbench-20261022T0012Z'].sensitivity_ratios.find(x => x.candidate === 't0c0s0');
const data = [record.candidate_over_reference_at_0, record.candidate_over_reference_at_0_1, record.candidate_over_reference_at_1];
const xs = [150, 450, 750];
const y = value => 345 - (Math.log10(value) + 0.30103) / 3.30103 * 245;
let svg = '<svg xmlns="http://www.w3.org/2000/svg" width="900" height="450" viewBox="0 0 900 450"><rect width="100%" height="100%" fill="white"/><g font-family="DejaVu Sans,sans-serif" fill="#17324a"><text x="35" y="32" font-size="20">Effect of pricing the recorded native work</text><text x="35" y="58" font-size="14">t0c0s0 / same-base-t0c0 · 80 paired public workloads · L0</text><text x="35" y="82" font-size="13">Sensitivity model; no calibrated full-cost ranking or wall-clock result</text>';
for (const tick of [0.5, 1, 10, 100, 1000]) {
  svg += `<line x1="110" x2="810" y1="${y(tick)}" y2="${y(tick)}" stroke="${tick === 1 ? '#b25c17' : '#d7e0e8'}" stroke-dasharray="${tick === 1 ? '6 4' : 'none'}"/><text x="55" y="${y(tick) + 5}" font-size="14">${tick}</text>`;
}
svg += `<polyline points="${data.map((v, i) => `${xs[i]},${y(v)}`).join(' ')}" fill="none" stroke="#176b9c" stroke-width="3"/>`;
data.forEach((v, i) => { svg += `<circle cx="${xs[i]}" cy="${y(v)}" r="5" fill="#176b9c"/><text x="${xs[i] + 12}" y="${y(v) - 7}" font-size="15">${v.toFixed(3)}</text><text x="${xs[i] - 12}" y="377" font-size="15">${[0, 0.1, 1][i]}</text>`; });
svg += '<text x="200" y="410" font-size="15">GAE assigned to each recorded unpriced native unit, on both arms</text><text x="35" y="438" font-size="12">Y axis: candidate/reference cost ratio (log scale). Dashed line: equal modeled cost.</text></g></svg>\n';
fs.writeFileSync(path.join(reports, 'SENSITIVITY.svg'), svg);
fs.writeFileSync(path.join(reports, 'SENSITIVITY_DATA.json'), JSON.stringify(record, null, 2) + '\n');
