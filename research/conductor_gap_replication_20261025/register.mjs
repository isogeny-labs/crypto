// Metadata-only registration. Arithmetic and ICV1 identities come from the native preparer.
// Preserve every existing model and exact EC1 representation, including off-disk sources.
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import assert from 'node:assert/strict';
const study='research/conductor_gap_replication_20261025';
const path='docs/curves/registry.json', aliasPath='src/cryptanalysis/curve_aliases.json';
const sha=s=>createHash('sha256').update(s).digest('hex');
const original=readFileSync(path,'utf8'), aliasOriginal=readFileSync(aliasPath,'utf8');
const doc=JSON.parse(original), aliases=JSON.parse(aliasOriginal);
const frozen=new Map(doc.curves.map(c=>[c.slug,structuredClone(c)]));
const bySlug=new Map(doc.curves.map(c=>[c.slug,c]));
let added=0,representations=0;
for(const panel of ['a','b-v2']){
  const source=study+'/prepared-'+panel+'/identities.json';
  for(const row of JSON.parse(readFileSync(source,'utf8'))){
    const {normalized_aliases,...r}=row;
    let e=bySlug.get(r.slug);
    if(e){
      assert.equal(e.icv1,r.icv1);assert.equal(e.model_json,r.model_json);
      e.representations??=[];
      for(const rep of r.representations){
        const old=e.representations.find(x=>x.curve_uid===rep.curve_uid);
        if(old){assert.deepEqual(old.field,rep.field);assert.deepEqual(old.curve,rep.curve);}
        else{e.representations.push(rep);representations++;}
      }
    }else{
      // Keep the maintained registry's property order and one-space formatting.
      e={slug:r.slug,icv1:r.icv1,family:r.family,params:r.params,model_json:r.model_json,
         trace:r.trace,order:r.order,j:r.j,end:r.end,aliases:[],standard_names:[],
         sources:r.sources,representations:r.representations};
      doc.curves.push(e);bySlug.set(e.slug,e);added++;representations+=e.representations.length;
    }
    for(const a of normalized_aliases){
      if(aliases.aliases[a]!==undefined)assert.equal(aliases.aliases[a],r.slug);
      aliases.aliases[a]=r.slug;
    }
  }
}
for(const [slug,old]of frozen){
  const now=bySlug.get(slug);assert(now);
  for(const key of Object.keys(old).filter(k=>k!=='representations'))assert.deepEqual(now[key],old[key]);
  for(const rep of old.representations??[])assert(now.representations.some(r=>JSON.stringify(r)===JSON.stringify(rep)));
}
const rank={koblitz:0,subfield:1,binary:2,prime:3,extension:4};
doc.curves.sort((a,b)=>(rank[a.family]-rank[b.family])||
  Number((BigInt(a.params.n??a.params.m??0)-BigInt(b.params.n??b.params.m??0)))||
  (BigInt(a.params.p??0)<BigInt(b.params.p??0)?-1:BigInt(a.params.p??0)>BigInt(b.params.p??0)?1:0)||
  (a.family==='koblitz'?a.params.a-b.params.a:0)||a.slug.localeCompare(b.slug,'en'));
aliases.aliases=Object.fromEntries(Object.entries(aliases.aliases).sort(([a],[b])=>a<b?-1:a>b?1:0));
const encoded=JSON.stringify(doc,null,1)+'\n';
const aliasEncoded=JSON.stringify(aliases,null,1).replace(/^ +/gm,'')+'\n';
const receipt={schema:'conductor-gap.registry-append/v1',prior_models:frozen.size,added_models:added,
  added_representations:representations,prior_models_and_representations_preserved:true,
  registry_before_sha256:sha(original),registry_after_sha256:sha(encoded),
  aliases_before_sha256:sha(aliasOriginal),aliases_after_sha256:sha(aliasEncoded)};
writeFileSync(path,encoded);writeFileSync(aliasPath,aliasEncoded);
writeFileSync(study+'/registration.json',JSON.stringify(receipt,null,2)+'\n',{flag:'wx'});
console.log(JSON.stringify(receipt));

