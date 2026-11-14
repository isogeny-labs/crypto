// Reporting metadata only. This does not execute a mathematical census.
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {loadCorpus} from '../../docs/ecdlp-hardness-corpus/load.mjs';
const here=path.dirname(fileURLToPath(import.meta.url));
const corpusDir=path.resolve(here,'../../docs/ecdlp-hardness-corpus');
const sourcePath=path.join(corpusDir,'corpus.json');
const root=JSON.parse(fs.readFileSync(sourcePath));
const corpus=loadCorpus(sourcePath);
const hygiene=JSON.parse(fs.readFileSync(path.join(corpusDir,'weakness.json'))).hygiene_queue;
const priorities=new Map(hygiene.map(n=>[n.id,n]));
const followup=new Map([
 ['S-XT-BK-1998','Original proof/remarks and probability theorem read; actual excluded-boundary witness; full qualified theorem Lean bridges open'],
 ['S-FST-2010','Trace/embedding definitions and Algorithm3.3 read; actual r=2 field-degree counterexample; large-prime cost results not refuted'],
 ['S-WD-GAUDRY-2000','Sections2–3 and subgroup assumptions read; higher-genus exact counts/proportion theorem remain open'],
 ['S-GG-HILDEBRAND-1994','Fixed-support theorem/averaging hypotheses read; full mixing Lean proof and walk census remain open'],
 ['S-FGHR-2014','Earlier increment read; universal short-Weierstrass square-system witness only; generic and representation-restricted claims remain open']
]);
const sourceIndex=corpus.sources.map(s=>({id:s.id,title:s.title,kind:s.kind,url:s.url??null,
 corpus_verification:s.verification??null,referencing_nodes:corpus.nodes.filter(n=>(n.source_refs??[]).includes(s.id)).map(n=>n.id),
 this_study_review:followup.get(s.id)??'Not newly reviewed in this follow-up; historical corpus label is not a new audit',
 formal_status:'No full-paper Lean verification asserted'}));
const sourceMap=new Map(sourceIndex.map(s=>[s.id,s]));
const nodes=corpus.nodes.filter(n=>n.status==='proved_external_unread').map(n=>({id:n.id,label:n.label,corpus_status:n.status,
 corpus_queue_impact:priorities.get(n.id)?.impact??null,
 statement:n.statement,assumptions:n.assumptions??[],scope:n.scope??null,
 sources:(n.source_refs??[]).map(id=>{const s=sourceMap.get(id);return s?
   {id,title:s.title,url:s.url,this_study_review:followup.get(id)??null}:
   {id,missing_from_source_index:true};}),
 next_obligation:(n.source_refs??[]).some(id=>followup.has(id))?
   'Consult PRIMARY_AUDIT.md for the newly read scope; full theorem formalization is not discharged':
   'Retrieve/read the exact primary statement, bind hypotheses, then formalize or refute that same statement',
 signed_promotion:false
})).sort((a,b)=>(b.corpus_queue_impact??-1)-(a.corpus_queue_impact??-1)||a.id.localeCompare(b.id));
const inputs=['corpus.json',...root.fragments,'weakness.json','load.mjs'].map(p=>{const b=fs.readFileSync(path.join(corpusDir,p));return {path:'docs/ecdlp-hardness-corpus/'+p,sha256:createHash('sha256').update(b).digest('hex')};});
const out={schema:'paper-audit-backlog/v1',corpus_file_revision:'ca496f7898f4467a28b9733bd8a43f52f9e8e723',
 counts:{nodes:corpus.nodes.length,source_records:corpus.sources.length,primary_publication_records:corpus.sources.filter(s=>s.kind==='primary_publication').length,unread_label_nodes:nodes.length},
 interpretation:'Snapshot of existing corpus labels and existing hygiene impacts. Source records include books, standards and secondary/repository references; they are not all papers. No untouched claim or entire paper is promoted.',
 inputs,nodes,source_index:sourceIndex};
fs.writeFileSync(path.join(here,'paper-audit-backlog.json'),JSON.stringify(out,null,2)+'\n');
const esc=s=>String(s??'—').replaceAll('|','\\|').replaceAll('\n',' ');
let md='# Complete unread-label backlog snapshot\n\n'+`The pinned corpus has${out.counts.nodes}nodes,${out.counts.source_records}source records (${out.counts.primary_publication_records}primary-publication records), and${nodes.length}nodes labelled proved_external_unread. These are metadata counts, not completed audits. [pcc:unverified]\n\n`;
md+='This table preserves all44original unread-label tasks. Impact is copied from the existing hygiene queue; it is not a new downstream ranking. Full statements, assumptions, URLs and the complete source index are in [paper-audit-backlog.json](paper-audit-backlog.json). Newly read scopes are in [PRIMARY_AUDIT.md](PRIMARY_AUDIT.md); reading is distinct from full Lean coverage. [pcc:unverified]\n\n';
md+='| Existing impact | Node | Source ids | This follow-up |\n|---|---|---|---|\n';
for(const n of nodes)md+=`| ${esc(n.corpus_queue_impact)} | ${esc(n.id)} | ${n.sources.map(s=>esc(s.id)).join(', ')} | ${n.sources.some(s=>followup.has(s.id))?'Primary text reviewed at the documented scope; full Lean obligation open':'Not newly audited'} |\n`;
md+='\nNo signed scientific promotion or change to canonical corpus statuses occurs in this inventory. The original fourteen requested proof/experiment methods also remain tracked in README.md. [pcc:unverified]\n';
fs.writeFileSync(path.join(here,'UNREAD_BACKLOG.md'),md);
console.log(JSON.stringify(out.counts));
