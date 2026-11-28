#!/usr/bin/env node
// Document layout only. All scientific values are recorded inputs in report_data.json.
// Usage: node render_report.cjs [path/to/report_data.json]
const fs = require('fs');
const path = require('path');
const { spawnSync } = require('child_process');
const { PDFDocument, StandardFonts, rgb, PDFName, PDFString } = require('pdf-lib');
const sharp = require('sharp');

const outDir = __dirname;
const dataPath = process.argv[2] ? path.resolve(process.argv[2]) : path.join(outDir, 'report_data.json');
const d = JSON.parse(fs.readFileSync(dataPath, 'utf8'));
if (d.schema !== 'prime31-admission-report/v1') throw new Error('Unrecognized report input schema');
if (!['pending', 'passed', 'failed', 'partial'].includes(d.native.state)) throw new Error('Invalid native status');
if (d.native.state === 'passed' && (!d.native.run_url || !d.native.source_sha || d.native.tests_passed === null)) {
  throw new Error('A passing report requires exact run, source and test receipt');
}

const C = {
  navy:'#183047', blue:'#365c79', teal:'#167a76', pale:'#eef6f5', gold:'#946414',
  amber:'#fff5df', gray:'#596772', light:'#e7edf0', paper:'#ffffff', ink:'#233646'
};
const base = 'https://github.com/isogeny-labs/crypto/blob/';
const refs = [
  ['1', 'Live scope and continuation', d.pr_url],
  ['2', 'Original group-admission source', `${base}${d.original_source_sha}/research/prime31_ic_20261115/native/examples/prime31_lp_group_admission.rs`],
  ['3', 'Preregistered native qualification', `${base}${d.qualification_plan_sha}/research/lp_graph_algorithms_20261123/PRIME31_GROUP_ADMISSION_NATIVE_20261123.md`],
  ['4', 'Capture source map and derived workload bound', d.capture_source_sha ? `${base}${d.capture_source_sha}/research/lp_graph_algorithms_20261123/PRIME31_CAPTURE_SOURCE_20261123.md` : '../../PRIME31_CAPTURE_SOURCE_20261123.md'],
  ['5', 'Generic residual-pair producer', `${base}${d.producer_source_sha}/src/ml_dataset/relations_generic.rs`],
  ['6', 'Existing transcript source map', `${base}${d.producer_source_sha}/research/ml_dataset_large_prime_trace_20261028/SOURCE_MAP.md`],
  ['7', 'Original Arithmetic and fixed-curve certificate', `${base}${d.original_source_sha}/research/prime31_ic_20261115/native/lib.rs`]
];
const nativeBadge = ({pending:'NATIVE GATE PENDING',passed:'NATIVE GATE PASSED',failed:'NATIVE GATE FAILED',partial:'NATIVE GATE PARTIAL'})[d.native.state];
const nativeEvidence = d.native.tests_passed === null ? 'Authored; not yet executed' : `${d.native.tests_passed}/${d.native.tests_total} admission tests passed; ${d.native.graph_tests_passed}/${d.native.graph_tests_total} graph tests passed`;
const nativeRunText = d.native.run_id ? `Run ${d.native.run_id}; job ${d.native.job_id || 'not supplied'}` : 'Run / job: pending receipt';
const mapping = [
 ['target', 'public_instance.target'],
 ['small_points', 'Enumerated small.columns; canonical points'],
 ['large_points', 'Enumerated residual.domain.columns'],
 ['row id', 'Accepted attempt.sequence'],
 ['a, b', 'attempt.coef_a, attempt.coef_b'],
 ['small terms', 'attempt.row.small; validate/remove target slot'],
 ['large terms', 'attempt.row.large']
];

function escapeXML(s) { return String(s).replace(/[&<>"']/g, ch => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&apos;'}[ch])); }
function svgText(x,y,lines,size=13,color=C.ink,weight=400) {
  return `<text x="${x}" y="${y}" text-anchor="middle" fill="${color}" font-family="Arial, Helvetica, sans-serif" font-size="${size}" font-weight="${weight}">${lines.map((t,i)=>`<tspan x="${x}" dy="${i?18:0}">${escapeXML(t)}</tspan>`).join('')}</text>`;
}
function makeDiagram() {
  const node = (x,y,w,h,title,sub,kind='plain') => {
    const fill = kind==='pending'?C.amber:kind==='control'?C.pale:'#f4f7fa';
    const stroke = kind==='pending'?C.gold:kind==='control'?C.teal:C.blue;
    return `<rect x="${x}" y="${y}" width="${w}" height="${h}" rx="9" fill="${fill}" stroke="${stroke}" stroke-width="1.5"/>${svgText(x+w/2,y+21,[title],14,stroke,700)}${svgText(x+w/2,y+40,[sub],11.5,C.gray)}`;
  };
  const arrow = (p, dashed=false) => `<path d="${p}" fill="none" stroke="${C.blue}" stroke-width="1.7"${dashed?' stroke-dasharray="5 4"':''} marker-end="url(#arrow)"/>`;
  let s = `<svg xmlns="http://www.w3.org/2000/svg" width="780" height="416" viewBox="0 0 780 416"><rect width="780" height="416" fill="white"/><defs><marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse"><path d="M0 0 L10 5 L0 10Z" fill="${C.blue}"/></marker></defs>`;
  s += node(85,9,280,57,'Natural collector capture','Missing; retain source + rejected attempts','pending');
  s += node(415,9,280,57,'Planted control bundle','5 known-answer rows; no natural yield','control');
  s += arrow('M225 66 L225 83 L390 83 L390 100',true);
  s += arrow('M555 66 L555 83 L390 83');
  s += node(225,103,330,55,'Prime31 group replay','Canonical points, residues, IDs, complete residual');
  s += arrow('M390 158 L390 176');
  s += node(225,179,330,55,'Protected target JSONL','small target coefficient = -b; rhs = a');
  s += arrow('M390 234 L390 252 L140 252 L140 270');
  s += arrow('M390 234 L390 270');
  s += arrow('M390 252 L640 252 L640 270');
  s += node(30,273,220,53,'Exact forest','Same rows + source replay');
  s += node(280,273,220,53,'Pivot filter','Same rows + source replay');
  s += node(530,273,220,53,'MST filter','Same rows or explicit refusal');
  s += arrow('M140 326 L140 343 L390 343 L390 361',true);
  s += arrow('M390 326 L390 361',true);
  s += arrow('M640 326 L640 343 L390 343',true);
  s += node(170,364,440,47,'Useful rank and cold recovery','Natural-input evaluation remains pending','pending');
  return s+'</svg>';
}

function markdown() {
  return `# Prime31 group admission: native qualification\n\n${d.date} | PR [#${d.pr_number}](${d.pr_url}) | ${nativeBadge}\n\n## Outcome and evidence scope\n\nThe continuation preserves the admission implementation byte for byte and expands its Rust test module from two tests to seven. It adds a five-row planted input, independently constructed expected JSONL, a negative-target companion and a deliberately corrupt final row. These qualify the group-to-graph boundary; they do not generate a natural relation stream. [1-3]\n\n**Native standing:** ${d.native.summary}\n\n| Evidence category | Recorded status |\n|---|---|\n| Source preservation | Original two tests retained; production prefix unchanged |\n| Authored controls | Seven Rust tests; fixed planted input of five rows |\n| Native execution | ${nativeEvidence}; ${d.native.cli_status} CLI gate; ${d.native.graph_status} graph gate |\n| Derived fixture structure | LP rank 2; LP left-nullity 3; projected small-plus-target rank 2 |\n| Natural measurement | No qualifying capture, timings, RSS or cold recovery |\n| Independent PCC | ${d.pcc_status} |\n\n### Fixed public instance\n\np = ${d.curve.p}; ell = ${d.curve.ell}. Curve y^2 = x^3 + ${d.curve.a} x + ${d.curve.b}; G = ${d.curve.generator}. The original Arithmetic/check_curve code is reused. [2]\n\n![Admission and evaluation boundary](admission_pipeline.svg)\n\nThe diagram separates missing natural input from the planted qualification branch. Dashed paths are pending natural-input work. No benchmark chart is included because this continuation has no measured performance series; canonical performance plots receive no new result point.\n\n### Exact control contract\n\nFor each accepted original row, verify sum(c_i P_i) + sum(d_j L_j) = aG + bQ. Export -b modulo ell in protected small column u32::MAX, with RHS a. Validate every row before emitting JSONL. A bad final row must cause unsuccessful exit and empty stdout.\n\nThe five planted rows have LP arities 0, 1, 2, 1, 2. With S, L, M the canonical orientations of 3G, 5G, 11G and Q = 7G, their equations are 2S = aG; -S + L = aG + Q; 2S - L + 3M = aG + 4Q; S + M = aG + 2Q; S - L + M = aG + 3Q. The negative-target control substitutes -Q and -b. Expected exports are built from these predetermined coefficients and known-answer RHS values, independently of the admission output.\n\nR41 + R42 - 3R43 cancels both LPs and leaves (-2, +1) on (S,Q). Together with R40 = (2,0), this gives the expected projected rank 2; the negative-target case changes +1 to -1. This is fixture analysis, not observed natural rank or a general performance result.\n\n## Natural source: mapping and workload gap\n\nSource inspection identifies the existing ml-ecdlp-generic-large-prime-transcript/v2 producer, not a qualifying fixed-curve capture. Its residual pair table belongs to the generic-decomposition track. The binary/Koblitz collector hook from #5029 is a separate field setting. [4-6]\n\n| Admission field | Existing transcript source |\n|---|---|\n${mapping.map(x=>'| '+x.join(' | ')+' |').join('\n')}\n\n**Target-column rule:** the source target index is small.columns.len() and must carry -b mod ell. Check and remove that column before conversion. The admission output writes u32::MAX. Retaining both would count the target twice.\n\nReplay the native transcript verifier and check the exact curve, G, Q and source binding first. Preserve the full transcript, rejected attempts, configurations, pair-table setup, counters and filter certificates in a hashed custody sidecar. Group-valid JSONL alone does not establish how rows were collected.\n\n### Derived upper bound, not a measurement\n\nThe current producer caps queries at 4096 and residual columns at 32. At most 64 signed points supply 64*65/2 = 2080 unordered pairs with repetition. Excluding 32 opposite-sign identity pairs leaves at most 2048 nonidentity pair sums; collisions reduce support.\n\nUnder independently uniform a in F_ell and independent b/small choices, the residual query is uniform, so E[accepted rows] <= 4096 * 2048 / 2147446831 = 8388608 / 2147446831 ~= 0.003906317. This bound precedes duplicate and rank rejection and is not a fixed-PRNG-seed guarantee. [4-5]\n\nThe unchanged capped producer cannot credibly furnish the intended 256/1024/4096-row checkpoints under that model. No cap increase or collection is authorized by this report. The next input must come with a separately admitted yield, support, memory and query-budget protocol.\n\n## Qualification receipt and next gate\n\n- ${nativeRunText}\n- Source commit: ${d.native.source_sha || 'pending exact native receipt'}\n- Checkout: ${d.native.checkout_sha || 'pending exact native receipt'}\n- Rust: ${d.native.rust_version || 'pending native receipt'}\n- Test result: ${nativeEvidence}\n- CLI status: ${d.native.cli_status}; graph status: ${d.native.graph_status}\n${d.native.run_url?'- Run evidence: '+d.native.run_url+'\n':''}${d.native.artifact_url?'- Artifact evidence: '+d.native.artifact_url+'\n':''}${d.native.failure?'- Failure retained: '+d.native.failure+'\n':''}\nStandalone gate: one existing hosted development job, one build/test thread, 4 GiB address-space cap, 300 seconds for adapter build/test and 60 seconds for fixed CLI checks. This does not qualify the full root crate. Retain source/lockfile/executable hashes, compiler version, complete stdout/stderr and failures. [3]\n\nRun the original example with:\n\n\`\`\`sh\ncargo test --locked --release \\\n  --manifest-path research/prime31_ic_20261115/native/Cargo.toml \\\n  --example prime31_lp_group_admission -- --nocapture --test-threads=1\n\`\`\`\n\nThe control bundle consists of valid.json, expected.jsonl, late-invalid.json, negative-target.json and negative-target-expected.jsonl. PRIME31_ADMISSION_CONTROL_DIR retains those files; without it the test uses and cleans a unique temporary directory. Validate CLI output bytes, final-row atomic rejection and original evaluator replay before recording a gate pass.\n\nAfter source qualification, the remaining dependency is an authentic fixed-curve capture with a feasible, admitted workload. Compare forest, pivot and MST on identical admitted input and include materialization, source replay, useful rank, memory and recovery costs. Preserve failures and explicit refusals. No measured speedup is claimed; no merge or paid compute is authorized.\n\n## Sources\n\n${refs.map(([id,label,url])=>`[${id}] [${label}](${url})`).join('\n\n')}\n\n## Re-render\n\nEdit report_data.json only when a source-bound receipt supports the update; run node render_report.cjs. The generator writes this Markdown, the diagram SVG and the PDF. It performs document layout only and does not derive scientific values or run cryptographic code.\n`;
}

async function main() {
  const svg = makeDiagram();
  fs.writeFileSync(path.join(outDir,'admission_pipeline.svg'),svg);
  let md = markdown().replace('The original Arithmetic/check_curve code is reused. [2]', 'The original Arithmetic/check_curve code is reused. [7]');
  if(d.native.state === 'passed') {
    md = md.replace('| Derived fixture structure | LP rank 2; LP left-nullity 3; projected small-plus-target rank 2 |',
      '| Planted execution | LP rank 2; LP left-nullity 3; all three arms accepted, with useful and augmented rank 2 |');
    md = md.replace('| Natural measurement | No qualifying capture, timings, RSS or cold recovery |',
      '| Natural measurement | No qualifying capture or qualified performance/RSS/cold comparison; timing fields not interpreted |');
    const graphRows = d.native.graph_observations.map(r => `| ${r.arm} | ${r.accepted?'yes':'no'} | ${r.useful_rank} | ${r.augmented_rank} | ${r.replayed} | ${r.replay_failures} |`).join('\n');
    md = md.replace('## Qualification receipt and next gate\n\n',
      `## Qualification receipt and next gate\n\n### Observed fixed-control results\n\n| Arm | Accepted | Useful rank | Augmented rank | Replayed | Replay failures |\n|---|---|---:|---:|---:|---:|\n${graphRows}\n\nBoth valid CLI controls exited 0 and matched their expected JSONL bytes. The invalid-final-row, malformed-json, oversized-input, missing-argument and extra-argument controls each exited 1 with empty stdout. No evaluator timing field is interpreted as a performance measurement.\n\n`);
    md = md.replace('Standalone gate:', `Artifact ZIP SHA-256: \`${d.native.artifact_zip_sha256}\`; ${d.native.artifact_zip_bytes} bytes. Retention is ${d.native.artifact_retention_days} days, expiring ${d.native.artifact_expires_utc} (${d.native.artifact_expires_display}). This is not durable archival; retain compact raw receipts with the PR.\n\nStandalone gate:`);
    md = md.replace('Validate CLI output bytes, final-row atomic rejection and original evaluator replay before recording a gate pass.',
      'The recorded run passed exact CLI byte comparisons, final-row atomic rejection and original evaluator replay on the fixed control.');
    md = md.replace('After source qualification, the remaining dependency', 'The remaining dependency');
  }
  fs.writeFileSync(path.join(outDir,'REPORT.md'),md);
  const pdf = await PDFDocument.create();
  pdf.setTitle('Prime31 group admission - native qualification');
  pdf.setAuthor('Isogeny Labs research');
  pdf.setSubject('PR #5076 source preservation, planted controls, native receipt and natural-source workload gap');
  pdf.setCreationDate(new Date('2026-11-23T00:00:00Z'));
  pdf.setModificationDate(new Date('2026-11-23T00:00:00Z'));
  const regular = await pdf.embedFont(StandardFonts.Helvetica);
  const bold = await pdf.embedFont(StandardFonts.HelveticaBold);
  const italic = await pdf.embedFont(StandardFonts.HelveticaOblique);
  const mono = await pdf.embedFont(StandardFonts.Courier);
  const W=595.28,H=841.89,M=43,CW=W-2*M;
  const col = h => {let v=h.replace('#','');return rgb(parseInt(v.slice(0,2),16)/255,parseInt(v.slice(2,4),16)/255,parseInt(v.slice(4,6),16)/255);};
  let page,y,commands;
  const layout=[];
  const text=(s,x,top,size=10.4,font=regular,color=C.ink)=>{
    page.drawText(String(s),{x,y:H-top-size,size,font,color:col(color)});
    commands.push({type:'text',text:String(s),x,y:H-top-size,size,font:font===bold?'bold':font===mono?'mono':'regular',color});
  };
  const rect=(x,top,w,h,fill,stroke)=>{
    page.drawRectangle({x,y:H-top-h,width:w,height:h,color:col(fill),...(stroke?{borderColor:col(stroke),borderWidth:0.65}:{})});
    commands.push({type:'rect',x,y:H-top-h,width:w,height:h,fill,stroke:stroke||null});
  };
  function wrap(s,width,size,font=regular) {
    const lines=[];
    for(const paragraph of String(s).split('\n')) {
      let line='';
      for(const word of paragraph.split(/\s+/)) {
        if(!word)continue;
        if(font.widthOfTextAtSize(line?line+' '+word:word,size)>width && line){lines.push(line);line=word;}else line=line?line+' '+word:word;
      }
      lines.push(line);
    }
    return lines;
  }
  function para(s,{size=10.4,width=CW,x=M,leading=14.5,color=C.ink,font=regular,gap=8}={}) {
    const ls=wrap(s,width,size,font);for(const line of ls){text(line,x,y,size,font,color);y+=leading;}y+=gap;return ls.length*leading+gap;
  }
  function heading(s){y+=3;text(s,M,y,14.3,bold,C.navy);y+=24;}
  function newPage(n,kicker,title,subtitle) {
    page=pdf.addPage([W,H]);
    commands=[];layout.push(commands);
    rect(0,0,W,8,C.teal);text('ISOGENY LABS  /  RESEARCH NOTE',M,26,8.1,bold,C.teal);
    text(`23 NOV 2026  /  PR #${d.pr_number}`,W-M-157,26,8.1,regular,C.gray);
    text(kicker.toUpperCase(),M,53,8.4,bold,C.gray);
    text(title,M,71,24,bold,C.navy);
    y=107;para(subtitle,{size:10.2,color:C.gray,leading:14,gap:9});
    page.drawLine({start:{x:M,y:39},end:{x:W-M,y:39},thickness:0.6,color:col(C.light)});
    commands.push({type:'line',x1:M,y1:39,x2:W-M,y2:39,width:0.6,color:C.light});
    text('Source-linked component report | Independent PCC: '+d.pcc_status,M,H-28,7.7,regular,C.gray);
    text(`${n} / 3`,W-M-25,H-28,8,regular,C.gray);
  }
  function table(rows,widths,{fontSize=9.4,rowMin=26}={}) {
    for(let i=0;i<rows.length;i++) {
      const header=i===0;
      const cells=rows[i].map((s,j)=>wrap(s,widths[j]-18,fontSize,header?bold:regular));
      const h=Math.max(rowMin,Math.max(...cells.map(x=>x.length))*12.5+12);
      rect(M,y,CW,h,header?C.navy:i%2===1?'#f1f5f7':'#ffffff');
      let x=M;
      cells.forEach((ls,j)=>{ls.forEach((line,k)=>text(line,x+9,y+6+k*12.5,fontSize,header?bold:regular,header?'#ffffff':C.ink));x+=widths[j];});
      y+=h;
    }y+=10;
  }
  function callout(title,body,kind='teal') {
    const color=kind==='amber'?C.gold:C.teal,fill=kind==='amber'?C.amber:C.pale;
    const ls=wrap(body,CW-24,10.2);
    const h=29+ls.length*14+10;
    rect(M,y,CW,h,fill);rect(M,y,3,h,color);
    text(title,M+12,y+9,10,bold,color);
    ls.forEach((l,i)=>text(l,M+12,y+29+i*14,10.2,regular,C.ink));y+=h+12;
  }
  function link(label,url,{size=9.4}={}) {
    const lines=wrap(label,CW,size,bold);
    for(const line of lines){text(line,M,y,size,bold,C.blue);if(url.startsWith('http')){const box=[M,H-y-size-2,M+bold.widthOfTextAtSize(line,size),H-y+2];const a=pdf.context.obj({Type:'Annot',Subtype:'Link',Rect:box,Border:[0,0,0],A:{Type:'Action',S:'URI',URI:PDFString.of(url)}});const ref=pdf.context.register(a);page.node.addAnnot(ref);commands.push({type:'link',url,box});}y+=13;}
  }

  newPage(1,'01 / Admission boundary','Group equations into graph-IC','Original group equations become explicit affine rows, with one protected target column.');
  const badgeColor=d.native.state==='passed'?C.teal:C.gold;
  rect(M,y,CW,27,d.native.state==='passed'?C.pale:C.amber);
  text(nativeBadge,M+10,y+8,9.3,bold,badgeColor);y+=39;
  para('The continuation preserves the admission implementation byte for byte and expands its Rust controls from 2 tests to 7. A fixed five-row bundle checks signed rows, target orientation and atomic rejection of a corrupt final row. [1-3]',{gap:8});
  table([
    ['Evidence class','What this continuation records'],
    ['SOURCE / AUTHORED','Original 2 tests retained; 5 added. Core admission prefix unchanged.'],
    ['NATIVE EXECUTION',nativeEvidence+'; CLI '+d.native.cli_status+'; graph '+d.native.graph_status+'.'],
    [d.native.state==='passed'?'PLANTED / OBSERVED':'PLANTED / DERIVED',d.native.state==='passed'?'5 rows; LP rank 2, left-nullity 3; all three arms accepted, useful rank 2.':'5 rows; expected LP rank 2, left-nullity 3, projected rank 2.'],
    ['NATURAL MEASUREMENT','Natural capture and qualified runtime/RSS/cold results remain absent.']
  ],[125,CW-125],{fontSize:9.1,rowMin:27});
  const diagramBytes=await sharp(Buffer.from(svg)).resize(1560,832).png().toBuffer();
  const diagram=await pdf.embedPng(diagramBytes);
  const qaDir=path.join(outDir,'qa');fs.mkdirSync(qaDir,{recursive:true});
  const diagramPath=path.join(qaDir,'diagram.png');fs.writeFileSync(diagramPath,diagramBytes);
  const dh=CW*416/780;
  page.drawImage(diagram,{x:M,y:H-y-dh,width:CW,height:dh});
  commands.push({type:'image',path:diagramPath,x:M,y:H-y-dh,width:CW,height:dh});y+=dh+5;
  para('Figure 1. Solid links define the control and replay contract. Dashed links require a natural capture and its admitted evaluation. The control branch is planted.',{size:8.3,leading:11,color:C.gray,gap:6});
  para(`Fixed instance: p=${d.curve.p}; ell=${d.curve.ell}; a=${d.curve.a}; b=${d.curve.b}; G=${d.curve.generator}. [7]`,{size:8.8,leading:12,gap:4});
  para('Every row verifies sum(cP) + sum(dL) = aG + bQ. The export stores -b mod ell at u32::MAX and rhs=a. All rows pass before any JSONL is emitted.',{size:9.4,leading:13,gap:0});
  if(y>H-53)throw new Error(`Page 1 overflow at ${y}`);

  newPage(2,'02 / Natural-input dependency','A producer is not yet a capture','The existing generic transcript offers a precise conversion path, but its current limits cannot furnish the intended checkpoints under a uniform-query model.');
  para('Source inspection identifies ml-ecdlp-generic-large-prime-transcript/v2 and its residual-pair lookup producer. This is the generic-decomposition track. The #5029 binary/Koblitz collector is a separate field setting. [4-6]',{size:10.1,leading:14,gap:8});
  table([['Admission field','Existing transcript source'],...mapping],[126,CW-126],{fontSize:9.2,rowMin:24});
  callout('The target must appear exactly once','The source column small.columns.len() must carry -b mod ell. Check and remove it; the adapter writes its protected u32::MAX column. Retaining both would count the unknown target twice.');
  para('Before conversion, replay the native transcript verifier and require the exact curve, G, Q and source binding. Keep the full transcript, rejected attempts, pair-table setup, configurations, counters and certificates as a hashed custody sidecar.',{size:10,leading:14,gap:8});
  heading('Derived support bound - no collection was run');
  para('The source caps residual columns at 32 and queries at 4096. Up to 64 signed points yield 2080 unordered pairs with repetition. Excluding 32 opposite-sign identity pairs leaves at most 2048 nonidentity pair sums; collisions reduce support. [4-5]',{size:10.1,leading:14,gap:9});
  rect(M,y,CW,69,C.pale);
  text('E[accepted rows] <= 4096 x 2048 / 2147446831',M+15,y+12,11.4,bold,C.teal);
  text('= 8388608 / 2147446831  ~=  0.003906317',M+15,y+34,12.5,bold,C.navy);y+=80;
  para('Assumption: a is independently uniform in F_ell, and b and the small choices are independent of a. The residual query is then uniform. This bound precedes duplicate/rank rejection; it is not a measured yield or a fixed-PRNG-seed guarantee.',{size:9.9,leading:14,gap:8});
  para('The unchanged capped producer is therefore not a credible source for the planned 256 / 1024 / 4096-row checkpoints under that model. The next capture needs its own feasible support, yield, memory and query-budget protocol. No caps are raised here.',{size:10.1,leading:14,gap:0});
  if(y>H-53)throw new Error(`Page 2 overflow at ${y}`);

  newPage(3,'03 / Reproducibility and next gate','Evidence to inspect before promotion','A native correctness receipt applies to its exact source and control inputs. The full crate and the natural-input comparison remain separate qualifications.');
  callout(nativeBadge,d.native.summary,d.native.state==='passed'?'teal':'amber');
  table([
    ['Receipt field','Current record'],
    ['Run / job',nativeRunText],
    ['Native source',d.native.source_sha||'Pending exact run receipt'],
    ['Compiler / host',d.native.rust_version?`rustc 1.100.0; ${d.native.host}; LLVM ${d.native.llvm_version}`:'Pending native receipt'],
    ['Full root / PCC','Standalone scope only / '+d.pcc_status]
  ],[108,CW-108],{fontSize:8.9,rowMin:24});
  if(d.native.run_url){link('Exact native run and source-bound job',d.native.run_url);}
  if(d.native.artifact_url){
    link('Download the retained qualification artifact',d.native.artifact_url);
    para(`${d.native.artifact_retention_days}-day retention; expires ${d.native.artifact_expires_display}. Not durable archival.`,{size:8.5,leading:12,color:C.gray,gap:4});
    para(`ZIP SHA-256: ${d.native.artifact_zip_sha256}`,{size:7.2,leading:10.5,font:mono,color:C.gray,gap:6});
  }
  if(d.native.failure){para('Retained failure: '+d.native.failure,{size:9.3,leading:13,color:C.gold,gap:6});}
  if(d.native.state==='passed') {
    heading('Observed on the same five planted rows');
    table([
      ['Arm','Accepted','Useful rank','Aug. rank','Replayed','Failures'],
      ...d.native.graph_observations.map(r=>[r.arm,r.accepted?'yes':'no',String(r.useful_rank),String(r.augmented_rank),String(r.replayed),String(r.replay_failures)])
    ],[112,67,79,79,85,CW-422],{fontSize:8.7,rowMin:22});
    para('Invalid final row, malformed JSON, oversized input, missing argument and extra argument: each exited 1 with empty stdout. Both valid exports matched exact expected bytes. Evaluator timing fields are not interpreted.',{size:9.2,leading:12.8,gap:6});
  } else {
    heading('Control bundle and adjudication');
    para('Require byte-identical valid CLI output, failed exit with empty stdout for the bad final row, and original evaluator source replay with the expected rank. Expected exports come from predetermined coefficients and known-answer RHS values.',{size:9.6,leading:13.5,gap:8});
  }
  heading('Sources and frozen identities');
  for(const [id,label,url] of refs){link(`[${id}] ${label}`,url,{size:8.8});}
  y+=5;
  para('Scope: one existing development job; one build/test thread, 4 GiB cap, 300-second adapter and 60-second CLI limits. [3] Natural input remains pending. Preserve compact raw receipts; no measured speedup, full-root pass, merge or paid compute is claimed.',{size:9.1,leading:12.5,gap:0});
  if(y>H-53)throw new Error(`Page 3 overflow at ${y}`);
  const pdfPath=path.join(outDir,'Prime31_Group_Admission_20261123.pdf');
  const layoutPath=path.join(qaDir,'render_layout.json');
  fs.writeFileSync(layoutPath,JSON.stringify({width:W,height:H,pages:layout}));
  // Embedded Liberation fonts avoid environment-dependent base-font substitution.
  // The Python helper renders these fixed document-layout commands only.
  const python=process.env.CODEX_PRIMARY_RUNTIME_PYTHON||'python3';
  const render=spawnSync(python,[path.join(outDir,'render_embedded_fonts.py'),layoutPath,pdfPath],{encoding:'utf8'});
  if(render.status!==0)throw new Error(render.stderr||render.stdout||'Embedded-font rendering failed');
  console.log(JSON.stringify({pdf:pdfPath,bytes:fs.statSync(pdfPath).size,pages:3,status:d.native.state,source:dataPath}));
}
main().catch(e=>{console.error(e);process.exit(1);});
