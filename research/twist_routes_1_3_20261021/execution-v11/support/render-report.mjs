// Layout of admitted values only. No orbit, algebra or research aggregation.
import fs from 'node:fs';import crypto from 'node:crypto';
const s='research/twist_routes_1_3_20261021',v=s+'/execution-v11',dir=s+'/reports/kernel-symmetry-20261028';
const d=JSON.parse(fs.readFileSync(v+'/ADMISSION.json'));fs.mkdirSync(dir,{recursive:true});
const t=(x,y,s,n=19)=>`<text x="${x}" y="${y}" font-size="${n}" fill="#23394b">${s}</text>`;
let svg=`<svg xmlns="http://www.w3.org/2000/svg" width="1080" height="650" viewBox="0 0 1080 650"><rect width="1080" height="650" fill="white"/><g font-family="DejaVu Sans, sans-serif">${t(32,40,'The degree17 map cycles through all eight twist kernels',26)}${t(32,76,'Exact p827 toy fixture · symmetry-v1 · native and independent PARI agree',18)}`;
svg+=t(32,120,'Eight-line cycle (archived line labels; arrows are the induced permutation)',19);
const cycle=d.line_cycles[0];
for(let i=0;i<cycle.length;i++){
 const x=65+i*125;
 svg+=`<circle cx="${x}" cy="174" r="29" fill="#edf5fb" stroke="#2674a9" stroke-width="2"/>`+t(x-8,182,cycle[i],23);
 if(i<cycle.length-1)svg+=`<path d="M${x+32},174 h57" fill="none" stroke="#2674a9" stroke-width="2"/>`+t(x+78,181,'›',25);
}
svg+=`<path d="M940,210 V235 H65 V210" fill="none" stroke="#2674a9" stroke-width="2"/>`+t(483,260,'returns to line0',17);
svg+=t(32,303,'Point order and projective order (exact counts; no sampling uncertainty)',20);
const panels=[...d.controls.filter(x=>x.name!=='companion'),{name:'Actual twist action',point_order:d.point_order,projective_order:d.projective_order}];
for(let i=0;i<panels.length;i++){
 const a=panels[i],y=338+i*75;svg+=t(32,y+21,a.name,18);
 svg+=`<rect x="255" y="${y}" width="${a.point_order*13}" height="23" fill="#2674a9"/>`+t(265+a.point_order*13,y+19,a.point_order,17);
 svg+=`<rect x="255" y="${y+29}" width="${a.projective_order*13}" height="18" fill="#c66e23"/>`+t(265+a.projective_order*13,y+44,a.projective_order,17);
}
svg+=`<rect x="32" y="579" width="18" height="14" fill="#2674a9"/>`+t(58,592,'Point order',17)+`<rect x="230" y="579" width="18" height="14" fill="#c66e23"/>`+t(256,592,'Projective order',17);
svg+=t(32,632,'49 images · 2401 sums · 8 kernel checks. No solver-cost or IC speed claim.',18)+'</g></svg>';
fs.writeFileSync(dir+'/SYMMETRY.svg',svg);fs.writeFileSync(dir+'/PLOT_DATA.json',JSON.stringify(d,null,2)+'\n');
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');fs.writeFileSync(dir+'/SOURCES.json',JSON.stringify({schema:'twist.report-sources/v1',source:v+'/ADMISSION.json',source_sha256:sha(v+'/ADMISSION.json'),svg_sha256:sha(dir+'/SYMMETRY.svg'),plot_operations:'Layout of recorded cycles/orders only; no scientific aggregation.'},null,2)+'\n');
