// Document layout only. All quantitative values come from frozen native output.
const fs=require('fs');
const sharp=require('/opt/codex/cua_node/lib/node_modules/sharp');
const rows=fs.readFileSync('../receipts/round-01/target-precision-01.jsonl','utf8')
  .trim().split('\n').map(JSON.parse).filter(x=>x.absolute_precision===5);
if(rows.length!==3)throw Error('Expected three recorded N=5 cases');
const head=(h)=>`<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="${h}" viewBox="0 0 1000 ${h}"><rect width="1000" height="${h}" rx="16" fill="#f5f8f7"/><g font-family="DejaVu Sans,sans-serif" fill="#17332d">`;
let route=head(360)+`<defs><marker id="arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M0 0L8 4L0 8" fill="#a66a14"/></marker></defs>
<text x="30" y="37" font-size="24" font-weight="bold">A common curve model does not yet give a compatible point image</text>
<rect x="30" y="68" width="370" height="110" rx="12" fill="white" stroke="#b9d1c9"/>
<text x="50" y="100" font-size="21">Source: Q ∈ ⟨P⟩ ⊂ E(Fₚ)</text><text x="50" y="130" font-size="18">Input: coordinates; scalar unknown</text><text x="50" y="157" font-size="17">Fixed setup must work for every Q</text>
<rect x="600" y="68" width="370" height="110" rx="12" fill="white" stroke="#b9d1c9"/>
<text x="620" y="100" font-size="21">Target: component group Z/rZ</text><text x="620" y="130" font-size="18">Classical decoder κ(local point)</text><text x="620" y="157" font-size="17">Missing: compatible image of Q</text>
<path d="M415 121H585" stroke="#a66a14" stroke-dasharray="7 4" stroke-width="3" marker-end="url(#arrow)"/>
<text x="423" y="102" font-size="17">point lift: open</text>
<rect x="30" y="207" width="940" height="108" rx="12" fill="#e5efeb"/>
<text x="50" y="237" font-size="20" font-weight="bold">Checked components and rejected shortcuts</text>
<text x="50" y="266" font-size="18">Compact model projections pass native checks; independent lifts fail negation.</text>
<text x="50" y="293" font-size="18">Fixed low absolute precision merges many distinct target component labels.</text>
<text x="30" y="343" font-size="16">Research goal active · no arbitrary-point transfer or speedup established · round 01</text></g></svg>`;
let chart=head(380)+`<text x="30" y="36" font-size="24" font-weight="bold">Target labels lost at five bits of absolute precision</text><text x="30" y="63" font-size="17">Classical Tate controls · full finite enumeration · counts, not runtimes</text>`;
rows.forEach((v,i)=>{const y=110+i*76,total=v.nonzero_auditor_labels,zero=v.labels_at_zero_pair;
  chart+=`<text x="30" y="${y+20}" font-size="20">r = ${v.r}</text><rect x="160" y="${y}" width="${5*total}" height="30" rx="3" fill="#b9d1c9"/><rect x="160" y="${y}" width="${5*zero}" height="30" rx="3" fill="#d29a43"/><text x="${175+5*total}" y="${y+21}" font-size="17">${zero} / ${total} labels</text><text x="160" y="${y+53}" font-size="15">${v.distinct_coordinate_pairs} coordinate pairs remain</text>`;
});
chart+=`<rect x="30" y="341" width="18" height="18" fill="#d29a43"/><text x="60" y="356" font-size="16">Labels encoded as (0,0) modulo 32</text><text x="560" y="356" font-size="15">Source: target-precision-01.jsonl · round 01</text></g></svg>`;
fs.writeFileSync('route.svg',route);fs.writeFileSync('precision.svg',chart);
Promise.all([sharp(Buffer.from(route),{density:200}).png().toFile('route.png'),
  sharp(Buffer.from(chart),{density:200}).png().toFile('precision.png')])
  .then(()=>process.stdout.write('Rendered route and recorded-count chart.\n')).catch(e=>{process.stderr.write(String(e));process.exitCode=1;});
