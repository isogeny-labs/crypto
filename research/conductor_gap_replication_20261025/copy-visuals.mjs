// Browsing copies only; the study remains the source of truth.
import {readFileSync,writeFileSync,mkdirSync,readdirSync,copyFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import assert from 'node:assert/strict';
const source='research/conductor_gap_replication_20261025';
const destination='benchmarks/20261025-conductor-gap-replication';
const paths=['REPORT.pdf'];
function visit(directory){
  for(const entry of readdirSync(source+'/'+directory,{withFileTypes:true})){
    const path=directory+'/'+entry.name;
    if(entry.isDirectory())visit(path);
    else if(/\.(svg|png)$/.test(path)||path.endsWith('/REPORT.pdf'))paths.push(path);
  }
}
visit('figures');visit('reports');
const sha=path=>createHash('sha256').update(readFileSync(path)).digest('hex');
const files=paths.map(path=>{
  mkdirSync(destination+'/'+path.split('/').slice(0,-1).join('/'),{recursive:true});
  copyFileSync(source+'/'+path,destination+'/'+path);
  const sha256=sha(source+'/'+path);assert.equal(sha(destination+'/'+path),sha256);
  return {path,source:source+'/'+path,sha256};
});
writeFileSync(destination+'/COPY_MANIFEST.json',JSON.stringify({
  classification:'documentary counted-work study; no speed or complete-cost rank admitted',files
},null,2)+'\n');
console.log('Refreshed and checked '+files.length+' visual copies.');
