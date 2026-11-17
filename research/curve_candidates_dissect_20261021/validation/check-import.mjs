// Packaging/document checks only. This does not replay mathematical certificates.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';

const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const zip=path.join(root,'evidence/curve-assessment-evidence.zip');
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
const member=name=>execFileSync('unzip',['-p',zip,'curve-assessment/'+name],{maxBuffer:12*1024*1024});
const requireCheck=(test,message)=>{if(!test)throw new Error(message);};
execFileSync('unzip',['-t',zip],{stdio:'pipe'});
const oldManifest=JSON.parse(member('manifest-continued.json'));
for(const [name,record] of Object.entries(oldManifest.files)){
  const bytes=execFileSync('unzip',['-p',zip,name],{maxBuffer:12*1024*1024});
  requireCheck(sha(bytes)===record.sha256,'Archive member hash mismatch: '+name);
}
for(const [file,inside] of [
 ['curves.icv1.json','curves.icv1.json'],
 ['assessment-certificates.json','assessment-certificates.json'],
 ['continuation-summary.json','continuation/continuation-summary.json'],
 ['continuation-validation.json','continuation/continuation-validation.json'],
 ['dissect-corrections.patch','continuation/dissect-corrections.patch'],
]){
 requireCheck(fs.readFileSync(path.join(root,'evidence',file)).equals(member(inside)),'Loose evidence differs: '+file);
}
const summary=JSON.parse(member('continuation/continuation-summary.json'));
const view=JSON.parse(member('continuation/assessed-results.json'));
requireCheck(view.data.length===292,'Unexpected assessed record count');
requireCheck(view.data.filter(r=>r.all_fields_available).length===290,'Availability count mismatch');
requireCheck(summary.remaining_incomplete_calls===2&&summary.corrected_original_records===10,'Summary mismatch');
const pdf=path.join(root,'REPORT.pdf');
const info=execFileSync('pdfinfo',[pdf],{encoding:'utf8'});
requireCheck(/Pages:\s+8\b/.test(info),'Unexpected PDF page count');
const text=execFileSync('pdftotext',['-layout',pdf,'-'],{encoding:'utf8'}).replace(/\s/g,'');
for(const row of JSON.parse(member('curves.icv1.json')).curves){
 requireCheck(text.includes(row.curve_id.icv1.replace(/\s/g,'')),'PDF omits a canonical identity');
 for(const name of ['p','a','b','Gx','Gy','n','seed'])
  requireCheck(text.includes(row.parameters[name]),'PDF omits an exact parameter '+name);
}
const sums=fs.readFileSync(path.join(root,'SHA256SUMS'),'utf8').trim().split('\n');
for(const row of sums){
 const match=/^([a-f0-9]{64})  (.+)$/.exec(row);
 requireCheck(match!==null,'Malformed checksum row');
 requireCheck(sha(fs.readFileSync(path.join(root,match[2])))===match[1],'Imported file hash mismatch: '+match[2]);
}
console.log('PASS: archive CRC and every archived member SHA-256');
console.log('PASS: all five loose evidence files match the frozen archive exactly');
console.log('PASS: 292 assessed records; 290 with all fields; two incomplete; ten corrected');
console.log('PASS: eight-page PDF contains every full ICV1 identity and exact supplied parameter');
console.log('PASS: imported report, figure, source and evidence checksums');
console.log('LIMIT: packaging/document checks do not constitute fresh arithmetic or independent-host replay');
