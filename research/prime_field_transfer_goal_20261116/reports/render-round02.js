// Document rendering only. Plot the frozen native node counts, never a rerun.
const fs=require('fs');
const base='../receipts/round-02/';
const read=(p)=>fs.readFileSync(base+p,'utf8').trim().split('\n').map(JSON.parse);
const rows=[read('native-03-p17.jsonl').find(x=>x.precision_digits===64),
 read('native-03-p257.jsonl').find(x=>x.precision_digits===64),
 read('native-04-public-0.jsonl').find(x=>x.precision_digits===64)];
if(rows.some(x=>!x))throw Error('Missing completed precision64 record');
const start=(height)=>`<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="${height}" viewBox="0 0 1000 ${height}"><rect width="1000" height="${height}" rx="14" fill="#f5f8f7"/><g font-family="DejaVu Sans,sans-serif" fill="#17332d">`;
let chart=start(380)+`<text x="30" y="35" font-size="24" font-weight="bold">Compressed division-polynomial evaluation</text><text x="30" y="64" font-size="17">Recorded memoized nodes · 64 p-adic digits · deterministic counts, no timing</text>`;
const y=n=>310-53*Math.log10(n);
for(const n of [1,10,100,1000,10000])chart+=`<path d="M95 ${y(n)}H950" stroke="#d2dfda"/><text x="30" y="${y(n)+5}" font-size="16">${n}</text>`;
rows.forEach((r,i)=>{const x=240+280*i;chart+=`<rect x="${x-45}" y="${y(r.max_recurrence_nodes)}" width="90" height="${310-y(r.max_recurrence_nodes)}" fill="#498a76"/><text x="${x}" y="${y(r.max_recurrence_nodes)-10}" text-anchor="middle" font-size="21">${r.max_recurrence_nodes}</text><text x="${x}" y="338" text-anchor="middle" font-size="18">${r.curve}</text><text x="${x}" y="364" text-anchor="middle" font-size="16">${r.order_bits}-bit order</text>`;});
chart+='</g></svg>';
let route=start(330)+`<defs><marker id="arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M0 0L8 4L0 8" fill="#a66a14"/></marker></defs><text x="30" y="35" font-size="24" font-weight="bold">Local lifting is implemented; cross-prime selection remains open</text>
<rect x="30" y="70" width="290" height="125" rx="10" fill="white" stroke="#b9d1c9"/><text x="47" y="100" font-size="21">Q ∈ E(Fₚ)[r]</text><text x="47" y="130" font-size="17">Coordinates supplied</text><text x="47" y="158" font-size="17">No scalar or subgroup table</text><text x="47" y="183" font-size="16">Fixed compact global model</text>
<rect x="365" y="70" width="290" height="125" rx="10" fill="#deeee7" stroke="#b9d1c9"/><text x="382" y="100" font-size="21">Source-local torsion lift</text><text x="382" y="130" font-size="17">Newton + compact recurrence</text><text x="382" y="158" font-size="17">Known classical construction</text><text x="382" y="183" font-size="16">Native checks through p⁶⁴</text>
<rect x="700" y="70" width="270" height="125" rx="10" fill="white" stroke="#b9d1c9"/><text x="717" y="100" font-size="20">Fixed 2-adic image</text><text x="717" y="130" font-size="17">Common-field branch needed</text><text x="717" y="158" font-size="17">Then κ → Z/rZ</text><text x="717" y="183" font-size="16">Input-point evaluator: open</text>
<path d="M325 132H356" stroke="#498a76" stroke-width="3" marker-end="url(#arrow)"/><path d="M662 132H692" stroke="#a66a14" stroke-dasharray="6 4" stroke-width="3" marker-end="url(#arrow)"/>
<text x="30" y="240" font-size="19">A p-adic residue does not specify its compatible algebraic image at another prime.</text><text x="30" y="271" font-size="17">Lean: exact model degrees and two residue-ring obstructions. Native lift correctness: unsigned.</text><text x="30" y="306" font-size="16">Round 02 · sources, raw outputs and proof map linked in the report · no target advantage measured</text></g></svg>`;
fs.writeFileSync('round-02-nodes.svg',chart);fs.writeFileSync('round-02-route.svg',route);
console.log('Rendered two SVGs from frozen records.');
