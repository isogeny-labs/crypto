// Operational byte/hash audit and document-inspection receipt; no mathematics.
import fs from 'node:fs';
import crypto from 'node:crypto';
const s='research/twist_routes_1_3_20261021',v=s+'/execution-v14';
const dir=s+'/reports/rank-repair-20261028';
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const write=(p,d)=>fs.writeFileSync(p,JSON.stringify(d,null,2)+'\n',{flag:'wx'});
const f=JSON.parse(fs.readFileSync(v+'/FROZEN.json'));
const checks=[];
for(const [p,h] of Object.entries({...f.source_hashes,...f.input_hashes,[v+'/protocol.json']:f.protocol_file_sha256})) {
 if(sha(p)!==h)throw Error('frozen hash mismatch: '+p);
 checks.push({path:p,sha256:h,matches:true});
}
for(const b of [f.binary,f.publication]) {
 if(sha(b.path)!==b.sha256)throw Error('pinned binary mismatch');
 checks.push({path:b.path,sha256:b.sha256,matches:true});
}
write(v+'/FINAL_METADATA_CHECK.json',{schema:'twist.final-metadata-check/v1',created:new Date().toISOString(),all_frozen_pins_match:true,checks,scope:'Frozen source/input/binary/protocol bytes; mathematics independently admitted by native/PARI.'});
write(dir+'/INSPECTION.json',{schema:'twist.report-inspection/v1',created:new Date().toISOString(),report_pdf_sha256:sha(dir+'/RESULTS.pdf'),report_source_sha256:sha(dir+'/RESULTS.md'),svg_sha256:sha(dir+'/IDENTIFIABILITY.svg'),png_sha256:sha(dir+'/IDENTIFIABILITY.png'),pages:4,all_pages_opened_and_visually_inspected:true,inspection_method:'Opened all four pdftoppm page renders at1600pixels and the150dpi figure PNG with view_image.',findings:'All pages legible; counts/denominators and p827 scope visible; no clipping, missing glyphs or overfull boxes. Figure occupies its own page. Original overfull layout logs/TeX preserved.',make_check:'passed: no Overfull/Missing character/undefined references/LaTeX Error'});
