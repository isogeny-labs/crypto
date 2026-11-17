// Layout recorded native aggregates only. No research aggregation or reruns.
import fs from 'node:fs';import crypto from 'node:crypto';
const s='research/twist_routes_1_3_20261021',v=s+'/execution-v5',r=s+'/reports/overlap-20261027';
const read=p=>JSON.parse(fs.readFileSync(p)),sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const a=read(v+'/overlap-audit-v1/AUDIT.json'),g=read(v+'/ADMISSION.json'),archive=read(v+'/RAW_VERIFICATION.json');
if(fs.existsSync(r))throw Error('fresh report directory required');fs.mkdirSync(r,{recursive:true});
if(g.status!=='independent-all-shift-and-PARI-polynomials-verified'||!archive.fresh_remote_download)throw Error('admission/publication absent');
const esc=s=>String(s).replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('>','&gt;');
const tex=s=>String(s).replace(/[\\{}_$%&#~^]/g,c=>({'\\':'\\textbackslash{}','{':'\\{','}':'\\}','$':'\\$','_':'\\_','%':'\\%','&':'\\&','#':'\\#','~':'\\textasciitilde{}','^':'\\textasciicircum{}'}[c])).replace(/[a-f0-9]{40,64}/g,m=>m.match(/.{1,8}/g).join('\\allowbreak{}'));
const svg=(h,b)=>`<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="${h}" viewBox="0 0 1100 ${h}"><rect width="1100" height="${h}" fill="white"/><g font-family="DejaVu Sans,sans-serif" font-size="18" fill="#223544">${b}</g></svg>\n`;
const xpos=x=>380+(Math.log10(x)-Math.log10(5))*530/3;
let b='<text x="24" y="34" font-size="27" font-weight="bold">Full-universe mixed overlap: all nonzero shifts</text><text x="24" y="64" font-size="17">Exact minimum and maximum admissible u per shift; log count axis, not a confidence interval.</text>';
for(const x of [10,100,1000])b+=`<path d="M${xpos(x)} 85 V650" stroke="#dce3e7" stroke-dasharray="3 4"/><text x="${xpos(x)}" y="678" text-anchor="middle">${x}</text>`;
a.by_fixture.forEach((x,i)=>{const y=106+i*53;b+=`<text x="24" y="${y}" font-size="17">${esc(x.fixture)}</text><text x="24" y="${y+19}" font-size="14">p=${x.p}; ${x.nonzero_shifts} nonzero shifts</text><path d="M${xpos(x.mixed_min)} ${y} H${xpos(x.mixed_max)}" stroke="#1a7c72" stroke-width="5"/><circle cx="${xpos(x.mixed_min)}" cy="${y}" r="4" fill="#254e86"/><circle cx="${xpos(x.mixed_max)}" cy="${y}" r="4" fill="#254e86"/><text x="943" y="${y}" font-size="16">${x.mixed_min}–${x.mixed_max}</text>`;});
b+='<text x="24" y="718" font-size="16">27,130 nonzero shifts; all 10 singular zero shifts retained separately. overlap-v1, 2026-10-27.</text><text x="24" y="743" font-size="16">Counts describe the entire coordinate universe; finite orbit prefixes and solving costs are unmeasured.</text>';
fs.writeFileSync(r+'/OVERLAP.svg',svg(770,b));
const factorRows=[{label:'Overlap source factor',found:g.overlap_source_factor_count,total:g.genus_two_curves_tested},{label:'Overlap twist factor',found:g.overlap_twist_factor_count,total:g.genus_two_curves_tested},{label:'Known-cover source factor',found:g.positive_cover_controls,total:g.positive_cover_controls},{label:'Corrupted polynomial rejected',found:g.false_polynomial_controls,total:g.false_polynomial_controls}];
b='<text x="24" y="34" font-size="27" font-weight="bold">Fixed Frobenius-factor subset and detector controls</text><text x="24" y="65" font-size="17">PARI characteristic polynomials; exact native integer-division recheck; no transfer or cost claim.</text>';
factorRows.forEach((x,i)=>{const y=103+i*61;b+=`<text x="24" y="${y+23}">${esc(x.label)}</text><rect x="380" y="${y}" width="500" height="30" fill="#dce3e7"/><rect x="380" y="${y}" width="${500*x.found/x.total}" height="30" fill="#1a7c72"/><text x="925" y="${y+23}">${x.found}/${x.total}</text>`;});
b+='<text x="24" y="391" font-size="16">Overlap subset: exactly shifts 1 and 3 on all 10 fixtures; not every genus-two polynomial.</text><text x="24" y="418" font-size="16">Known covers: y²=x⁶+a x²+b to E via (x²,y). False control adds 1 to the polynomial.</text>';
fs.writeFileSync(r+'/FACTORS.svg',svg(445,b));
const paragraphs=[
  'The complete fixed panel has 27,140 shifts on all ten original public toy fixtures. Independent Euler-character and square-root-multiplicity implementations agree on every mixed/ordinary count and genus-two affine count. All 27,130 nonzero shifts pass polynomial coprimality and the integer Weil-bound check. Ten singular zero shifts remain explicit controls; 27,140 false plus-one counts are rejected. These are exact finite counts, not independent targets or timing samples.',
  'The classical connection is that four times the full-coordinate twist-to-source overlap equals the rational point count of D_c: y²=f(u)f(u+c)/d for nonzero shift c. Its nonsquare leading coefficient gives no rational points at infinity. The standard genus-two Weil bound puts this supply near p/4. This does not bound a short orbit prefix or a decomposition algorithm. For p=10111 the nonzero-shift range is 2444–2605 eligible coordinates from 5072 full twist x-coordinates.',
  'On the predeclared shifts 1 and 3, none of the twenty genus-two Frobenius polynomials has the source or twist elliptic factor. All ten known elliptic-cover positive controls are detected, and all ten corrupted-polynomial controls are rejected. PARI point counts match the native census; native integer division independently verifies factor flags. This does not test the remaining shifts for factors, prove absolute simplicity, construct a scalar map, or measure any DLP advantage.',
  'Scientific source 069fb43cd486b24b604ce9320d01caa9f8fb64b3 and binary/input hashes are frozen in execution-v5/FROZEN.json. Protocol b7e3139b59da38041808bdd4d0028b8d8ac8348e2baa9e9ec6f4da7fe436a61d precedes panel data. Two actual executions process the same complete panel: producer and independent verifier. The full structured export/protocol/file binding passes with zero findings; this does not repair the predecessor cost panel metadata.',
  'Commands: frozen worker transport RECEIPT produce PRIOR_PLAN RAW; transport RECEIPT verify PRIOR_PLAN RAW AUDIT; native GP transport; frozen admit; frozen export. Producer/verifier each have 600 seconds, 4 GiB and 64 MiB/file; GP has 300 seconds, initial 64 MB stack and 4 GiB maximum. L0 managed Xeon Platinum 8573C/KVM; the standing authorized local fallback applies. No native clock, cost, GAE, external speed ranking or independent-host claim.',
  'Before affected GP outputs, primary help exposed an API mismatch: polrem was undefined. A recorded preload polrem(a,b)=a%b supplies the intended remainder operation. The main script, native census, cases, thresholds and stderr gate remain unchanged. This explicit compatibility amendment is later than the initial freeze and is linked in the export. Source/hash receipts preserve each stage.',
  'Three Lean theorems prove the ring expansion, signed indicator and conditional finite permutation identity, with a fresh native axiom audit and no sorry, custom axioms or trusted native evaluation. IDC1ha81899274c038263 adds 32 issuer and 64 extra checker probes plus false-plus-one rejection. Legendre, projective-model and Weil formal bridges remain open. Local primary GP help and repository/atlas sources were read; online PARI/IACR/MIT requests failed, so no completed novelty review is claimed.',
  'The deterministic raw archive has '+archive.bytes+' bytes and SHA256 '+archive.sha256+'. A fresh Setec download verifies all '+archive.members_verified+' members and an identical repack. Figures use native aggregates only. No full-campaign, NIST, useful-relation rank or whole-IC/rho requirement is discharged; final E28 labels 32–63 remain unopened.'
];
const sources=['execution-v5/PROTOCOL.md','execution-v5/LITERATURE.md','execution-v5/ADMISSION.json','execution-v5/overlap-audit-v1/AUDIT.json','execution-v5/experiment.export.json','execution-v5/GP_COMPATIBILITY_AMENDMENT_V2.md','lean-v4/README.md'];
const heads=['Source identity','p','Nonzero shifts','Mixed min','Mixed max'];
const rows=a.by_fixture.map(x=>[x.fixture,x.p,x.nonzero_shifts,x.mixed_min,x.mixed_max]);
const table='| '+heads.join(' | ')+' |\n| '+heads.map(()=>'---').join(' | ')+' |\n'+rows.map(x=>'| '+x.join(' | ')+' |').join('\n');
fs.writeFileSync(r+'/RESULTS.md','# Shifted twist overlap and genus-two factor checks\n\nRun: **overlap-v1**, 2026-10-27.\n\n'+paragraphs.join('\n\n')+'\n\n![Exact overlap ranges](OVERLAP.svg)\n\n![Fixed Frobenius-factor checks and controls](FACTORS.svg)\n\n'+table+'\n\nSources: '+sources.map(p=>'['+p+'](../../'+p+')').join(', ')+'.\n\nCanonical IC leaderboard/scoreboard, progress timeline and curve/cover map graphs were checked: no validated pipeline row or new scalar-preserving map changes their data, so they remain unchanged. This finite geometry report adds no performance benchmark row. The next discriminator is a target-blind twist-generator order/finite-prefix comparison, with charged discovery and matched ordinary generators; the full-universe count does not answer it.\n');
fs.writeFileSync(r+'/RESULTS.tex',String.raw`\documentclass[10pt]{article}
\usepackage[T1]{fontenc}\usepackage[utf8]{inputenc}\usepackage{lmodern,amsmath,amssymb}
\usepackage[margin=18mm]{geometry}\usepackage{graphicx,hyperref,parskip}
\hypersetup{hidelinks}\setlength{\emergencystretch}{3em}
\begin{document}
\section*{Shifted twist overlap and genus-two factor checks}
\textbf{Run:} overlap-v1, 2026-10-27.
`+paragraphs.slice(0,3).map(x=>tex(x)+'\n\n').join('')+String.raw`For a root-free cubic and nonsquare $d$, put $chi_u=chi(f(u))$.
For $c\ne0$ the classical equalities and bound are
\[
4N_c=p-\sum_{u\in\mathbb F_p}\chi_u\chi_{u+c}=\#D_c(\mathbb F_p),
\qquad (4N_c-(p+1))^2\le16p.
\]
`+paragraphs.slice(3).map(x=>tex(x)+'\n\n').join('')+'\\textbf{Sources:} '+sources.map(p=>'\\path{'+p+'}').join('; ')+'\n\\clearpage\n\\section*{Complete finite overlap ranges}\n\\includegraphics[width=\\linewidth]{OVERLAP.pdf}\n\\clearpage\n\\section*{Fixed subset and explicit correctness controls}\n\\includegraphics[width=\\linewidth]{FACTORS.pdf}\n\\end{document}\n');
fs.writeFileSync(r+'/PLOT_DATA.json',JSON.stringify({by_fixture:a.by_fixture,factor_controls:factorRows},null,2)+'\n');
fs.writeFileSync(r+'/SOURCES.json',JSON.stringify({run:'overlap-v1',files:sources.map(p=>({path:s+'/'+p,sha256:sha(s+'/'+p)})),raw_archive_sha256:archive.sha256,scientific_aggregation_in_javascript:false},null,2)+'\n');
