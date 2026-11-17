// Layout of admitted exact counts only; no research arithmetic or collection.
import fs from 'node:fs';
import crypto from 'node:crypto';
const study='research/twist_routes_1_3_20261021';
const source=study+'/execution-v14/SUMMARY.json';
const dir=study+'/reports/rank-repair-20261028';
const d=JSON.parse(fs.readFileSync(source));
fs.mkdirSync(dir,{recursive:true});
const text=(x,y,s,n=18)=>`<text x="${x}" y="${y}" font-size="${n}" fill="#20364a">${s}</text>`;
const label={ 'curve-orbit':'Curve progression', 'low-x':'Ascending x', 'twist-maximal':'Twist maximal seed' };
let svg='<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="700" viewBox="0 0 1100 700"><rect width="1100" height="700" fill="white"/><g font-family="DejaVu Sans, sans-serif">';
svg+=text(30,38,'Known endomorphism equations resolve some target ambiguities',25);
svg+=text(30,70,'Exact p827 census · same public target · three arity modes per filled configuration',18);
svg+='<rect x="440" y="93" width="22" height="16" fill="#8d9aa9"/>'+text(470,108,'Equations off',16);
svg+='<rect x="665" y="93" width="22" height="16" fill="#1879a2"/>'+text(695,108,'Equations on',16);
svg+=text(890,108,'Filled pairs',16);
for (const x of [0,3,6,9,12,15,18]) {
 const px=440+x*23;
 svg+=`<path d="M${px},135 L${px},570" stroke="#e1e7ec"/>`+text(px-6,595,x,15);
}
for (let i=0;i<d.descriptive_groups.length;i++) {
 const g=d.descriptive_groups[i], y=148+i*70;
 svg+=text(30,y+18,label[g.kind],18)+text(30,y+40,g.transform==='plain'?'plain base':'half seeds + alpha images',15);
 svg+=`<rect x="440" y="${y}" width="${g.off_identifiable*23}" height="17" fill="#8d9aa9"/>`;
 svg+=`<rect x="440" y="${y+23}" width="${g.on_identifiable*23}" height="17" fill="#1879a2"/>`;
 svg+=text(448+g.off_identifiable*23,y+14,g.off_identifiable,15);
 svg+=text(448+g.on_identifiable*23,y+37,g.on_identifiable,15);
 svg+=text(925,y+28,g.analyzed_pairs,19);
}
svg+=text(440,624,'Target-identifiable matrix cases (count)',18);
svg+=text(30,659,'42 bases: 29 filled, 13 underfilled. Underfills excluded by the frozen rule; no refills.',17);
svg+=text(30,685,'One toy DLP; correlated cases. No uncertainty or solving-cost measurements.',16)+'</g></svg>';
fs.writeFileSync(dir+'/IDENTIFIABILITY.svg',svg,{flag:'wx'});
fs.writeFileSync(dir+'/PLOT_DATA.json',JSON.stringify(d,null,2)+'\n',{flag:'wx'});
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
fs.writeFileSync(dir+'/SOURCES.json',JSON.stringify({schema:'twist.report-sources/v1',source,source_sha256:sha(source),svg_sha256:sha(dir+'/IDENTIFIABILITY.svg'),plot_operations:'Layout of frozen native descriptive_groups counts only; no new aggregation.'},null,2)+'\n',{flag:'wx'});
