// Layout of existing admitted fields only; all mathematics is native Rust/GP.
import fs from 'node:fs';import crypto from 'node:crypto';
const s='research/twist_routes_1_3_20261021',v=s+'/execution-v13',dir=s+'/reports/quotient-cycle-20261028';
const d=JSON.parse(fs.readFileSync(v+'/SUMMARY.json'));fs.mkdirSync(dir,{recursive:true});
const t=(x,y,s,n=19)=>`<text x="${x}" y="${y}" font-size="${n}" fill="#23394b">${s}</text>`;
const a=d.maps[0];
let svg=`<svg xmlns="http://www.w3.org/2000/svg" width="1080" height="700" viewBox="0 0 1080 700"><rect width="1080" height="700" fill="white"/><g font-family="DejaVu Sans, sans-serif">${t(30,38,'The kernel cycle gives explicit maps between quotient curves',25)}${t(30,72,'All8 generic squares verified · one representative square below · p827 toy fixture',18)}`;
for(const [x,y,label]of [[45,100,'icv1-fp10-t5-192cb216'],[615,100,'icv1-fp10-t5-192cb216'],[45,263,a.source_model],[615,263,a.destination_model]])svg+=`<rect x="${x}" y="${y}" width="420" height="67" rx="8" fill="#edf5fb" stroke="#2674a9"/>`+t(x+18,y+41,label,21);
const arrow=(x1,y1,x2,y2)=>`<path d="M${x1},${y1} L${x2},${y2}" fill="none" stroke="#2674a9" stroke-width="2" marker-end="url(#arrow)"/>`;
svg+='<defs><marker id="arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10z" fill="#2674a9"/></marker></defs>';
svg+=arrow(473,132,606,132)+t(484,113,'α: degree17',17);
svg+=arrow(250,174,250,255)+t(265,221,'ψ0: degree7',18);
svg+=arrow(820,174,820,255)+t(835,221,'ψ4: degree7',18);
svg+=arrow(473,296,606,296)+t(480,279,'β0: degree17',17);
svg+=t(30,368,'Verified rational-point checks per edge (points; exact population, no uncertainty)',20);
for(let i=0;i<d.maps.length;i++){
 const m=d.maps[i],y=392+i*31;
 svg+=t(35,y+19,`${m.line} → ${m.next_line}`,17)+`<rect x="160" y="${y}" width="${m.source_rational_checks*0.9}" height="22" fill="#2674a9"/>`+t(916,y+18,m.source_rational_checks,16);
}
svg+=t(30,668,'Initial GP rejected; corrected GP passes. Native data reused. No charged solver-cost claim.',17)+'</g></svg>';
fs.writeFileSync(dir+'/SQUARES.svg',svg);fs.writeFileSync(dir+'/PLOT_DATA.json',JSON.stringify(d,null,2)+'\n');
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');fs.writeFileSync(dir+'/SOURCES.json',JSON.stringify({schema:'twist.report-sources/v1',source:v+'/SUMMARY.json',source_sha256:sha(v+'/SUMMARY.json'),svg_sha256:sha(dir+'/SQUARES.svg'),plot_operations:'Layout of admitted model names and recorded point-check counts only.'},null,2)+'\n');
