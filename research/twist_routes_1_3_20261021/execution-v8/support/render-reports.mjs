// Layout of recorded native values only. No research aggregation or reruns.
import fs from 'node:fs';import crypto from 'node:crypto';
const s='research/twist_routes_1_3_20261021';
const main=JSON.parse(fs.readFileSync(s+'/execution-v8/ADMISSION.json'));
const rank=JSON.parse(fs.readFileSync(s+'/execution-v9/ADMISSION.json'));
const esc=x=>String(x).replaceAll('&','&amp;').replaceAll('<','&lt;');
const text=(x,y,t,size=18,color='#25384b')=>`<text x="${x}" y="${y}" font-size="${size}" fill="${color}">${esc(t)}</text>`;
const open=(title)=>`<svg xmlns="http://www.w3.org/2000/svg" width="1060" height="570" viewBox="0 0 1060 570"><rect width="1060" height="570" fill="white"/><g font-family="DejaVu Sans, sans-serif">${text(38,42,title,26)}`;
const blue='#2674a9',orange='#c66e23';
let svg=open('Transport preserves the complete signed relation system');
svg+=text(38,73,'p827; one public target; two fixed bases; arities2/3; offsets0–15',17);
svg+=`<rect x="48" y="97" width="18" height="18" fill="${blue}"/>`+text(74,113,'Ordinary base',17)+`<rect x="260" y="97" width="18" height="18" fill="${orange}"/>`+text(287,113,'Twist-progression base',17);
svg+=text(38,153,'Witness rows, including multiplicities',18);
for(const value of [0,20,40,60,80]){const y=340-value*2;svg+=`<line x1="62" y1="${y}" x2="1015" y2="${y}" stroke="#e0e7ee"/>`+text(24,y+6,value,14);}
for(const a of main.systems){const x=92+a.model*103+a.base*28;const y=340-a.relation_rows*2;svg+=`<rect x="${x}" y="${y}" width="24" height="${a.relation_rows*2}" fill="${a.base?orange:blue}"/>`+text(x-1,y-7,a.relation_rows,14);svg+=text(x+5,465,a.rank,20,a.base?orange:blue);}
svg+=text(38,420,'Matrix rank (nine unknowns)',18);
for(let i=0;i<9;i++)svg+=text(91+i*103,378,i===0?'Source':'Map'+(i-1),15);
svg+=text(38,513,'All576 query sets agree exactly across each transported base.',18);
svg+=text(38,544,'Exact finite values, not independent targets or a speed comparison. Run: kernel-v1',15);
svg+='</g></svg>';
let rsvg=open('Rank deficiency leaves the twist-base target ambiguous');
rsvg+=text(38,77,'Post-v8 diagnostic; all18 unchanged matrices; native RREF and independent GP',17);
for(const [i,a]of rank.cases.slice(0,2).entries()){
 const x=45+i*505;rsvg+=`<rect x="${x}" y="108" width="475" height="310" rx="9" fill="${i?'#fff3e7':'#edf5fb'}" stroke="${i?orange:blue}"/>`;
 rsvg+=text(x+22,148,i?'Twist-progression base':'Ordinary base',23);
 rsvg+=text(x+22,196,`Matrix rank: ${a.rank} / ${a.unknowns}`,22);
 rsvg+=text(x+22,240,a.target_identifiable?'Target fixed by the relation rows':'Target varies along the nullspace',20);
 rsvg+=text(x+22,284,a.target_identifiable?`Recovered scalar: ${a.recovered_scalar}`:'Free column: target log k (column8)',19);
 rsvg+=text(x+22,327,a.target_identifiable?'[37]G = Q verified on every image':'Retained direction has target entry1',18);
 rsvg+=text(x+22,371,i?'Same ambiguity on source and all8 maps':'Same recovery on source and all8 maps',17);
}
rsvg+=text(38,462,`Verified: ${rank.summary.target_identifiable} identifiable systems; ${rank.summary.target_ambiguous} ambiguous systems.`,21);
rsvg+=text(38,505,'Original GP field-initialization failure is preserved; pinned preload replay passes.',16);
rsvg+=text(38,543,'No new decomposition queries, changed bases or performance inference. Run: rank-v1',15);
rsvg+='</g></svg>';
for(const [name,data,fig]of [['kernel-relations-20261027',main,svg],['relation-rank-20261027',rank,rsvg]]){
 const dir=s+'/reports/'+name;fs.mkdirSync(dir,{recursive:true});fs.writeFileSync(dir+'/SYSTEMS.svg',fig);fs.writeFileSync(dir+'/PLOT_DATA.json',JSON.stringify(data,null,2)+'\n');
 const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
 fs.writeFileSync(dir+'/SOURCES.json',JSON.stringify({schema:'twist.report-sources/v1',created:new Date().toISOString(),source:name.startsWith('kernel')?s+'/execution-v8/ADMISSION.json':s+'/execution-v9/ADMISSION.json',source_sha256:hash(name.startsWith('kernel')?s+'/execution-v8/ADMISSION.json':s+'/execution-v9/ADMISSION.json'),plot_sha256:hash(dir+'/SYSTEMS.svg'),plotted_values:'Recorded native values; position/color/layout only; no scientific aggregation'},null,2)+'\n');
}
