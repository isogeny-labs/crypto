import { renderDiagram, serializeGraph, rasterizeSvg } from './export.mjs';
import {build} from './math.mjs';
const $=s=>document.querySelector(s),colors={elliptic:'#b4e98a',hyperelliptic:'#7bafff',superelliptic:'#b49aff',jacobian:'#edb779'},edgeColors={isogeny:'#b4e98a',cover:'#7bafff',pullback:'#edb779',norm:'#edb779'};let data,zoom=1,selected='E0';
const sup=n=>String(n).split('').map(x=>'⁰¹²³⁴⁵⁶⁷⁸⁹'[+x]||x).join('');
const eq=e=>`y² = x³ + ${e.a}x + ${e.b}`;
function draw() {
  const diagram = renderDiagram(data, { selected });
  const graph = $('#graph');
  graph.setAttribute('class', 'diagram');
  graph.setAttribute('viewBox', diagram.getAttribute('viewBox'));
  graph.setAttribute('height', diagram.getAttribute('height'));
  graph.style.width = Math.round(zoom * 100) + '%';
  graph.innerHTML = diagram.innerHTML;
  $('#zoomLabel').textContent = Math.round(zoom * 100) + '%';
  $('#empty').hidden = true;
  document.querySelectorAll('[data-node],[data-edge]').forEach(el => {
    const select = () => { selected = el.dataset.node || el.dataset.edge; inspect(); draw(); };
    el.onclick = select;
    el.onkeydown = e => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); select(); } };
  });
}
function inspect(){const n=data.nodes.find(n=>n.id===selected),e=data.edges.find(e=>e.id===selected);if(n){let title=n.kind==='elliptic'?eq(n):n.kind==='jacobian'?`J(${data.nodes.find(c=>c.id===n.curve).label})`:n.equation.replace(/\^(\d+)/g,(_,a)=>sup(a));$('#details').innerHTML=`<span class="tag">${n.kind} · ${n.label}</span><div class="math">${title}</div><div class="detail-grid"><div><small>${n.kind==='jacobian'?'Dimension':'Genus'}</small><strong>${n.genus}</strong></div><div><small>Base field</small><strong>𝔽${data.parameters.p}</strong></div>${n.count?`<div><small>#E(𝔽ₚ)</small><strong>${n.count}</strong></div>`:''}${n.id==='E0'?`<div><small>j-invariant</small><strong>${data.j}</strong></div>`:''}</div><p>${n.kind==='elliptic'?'Select an edge to inspect its kernel, degree and map formula. Rational 2-torsion kernels generate the displayed isogenies.':n.kind==='jacobian'?'The degree-zero Picard variety of this cover. Dashed arrows are pullback and norm homomorphisms, whose coordinates are not explicitly computed.':`${n.classification}. Select its arrow to E₀ for the explicit covering map. Genus refers to the smooth projective model.`}</p>`}else if(e){$('#details').innerHTML=`<span class="tag">${e.kind} · ${e.source} → ${e.target}</span><div class="math">${e.formula}</div><div class="detail-grid"><div><small>Map type</small><strong>${e.kind}</strong></div>${e.degree?`<div><small>Degree</small><strong>${e.degree}</strong></div>`:''}${e.kernel!==undefined?`<div><small>Kernel point</small><strong>(${e.kernel}, 0)</strong></div>`:''}</div><p>${e.detail}</p>`}}
function generate(e){e?.preventDefault();try{const next=build({p:Number($('#p').value),a:Number($('#a').value),b:Number($('#b').value),c:Number($('#c').value),maxN:Number($('#n').value),depth:Number($('#depth').value),quartic:$('#quartic').checked,jacobians:$('#jacobians').checked});data=next;selected='E0';zoom=1;$('#error').textContent='';$('#heroEquation').textContent=eq(data.nodes[0]);$('#field').textContent=`Over 𝔽${data.parameters.p} · nonsingular`;$('#graphCount').textContent=`${data.nodes.length} objects / ${data.edges.length} maps`;$('#statE').textContent=data.nodes.filter(n=>n.kind==='elliptic').length;$('#statC').textContent=data.nodes.filter(n=>n.kind==='hyperelliptic'||n.kind==='superelliptic').length;$('#statG').textContent=Math.max(...data.nodes.map(n=>n.genus));$('#statM').textContent=data.edges.length;$('#scopeText').textContent=data.scope;$('#warnings').textContent=data.skipped.join(' ');inspect();draw()}catch(err){$('#error').textContent=err.message}}
function download(content,type,name){let u=URL.createObjectURL(new Blob([content],{type})),a=document.createElement('a');a.href=u;a.download=name;a.click();setTimeout(()=>URL.revokeObjectURL(u),1000)}
$('.graph-toolbar').append($('.zoom'));
$('#form').onsubmit=generate;$('#n').oninput=()=>$('#nValue').textContent=`n ≤ ${$('#n').value}`;$('#reset').onclick=()=>{$('#form').reset();$('#nValue').textContent='n ≤ 3';generate()};$('#plus').onclick=()=>{zoom=Math.min(2,zoom+.15);draw()};$('#minus').onclick=()=>{zoom=Math.max(.5,zoom-.15);draw()};$('#fit').onclick=()=>{zoom=1;draw()};$('#export').onclick=()=>download(JSON.stringify(data,null,2),'application/json','curve-atlas.json');$('#svgExport').onclick=()=>exportDiagram('svg');$('#pngExport').onclick=()=>exportDiagram('png');$('#method').onclick=()=>$('#about').showModal();$('#closeAbout').onclick=()=>$('#about').close();generate();

async function exportDiagram(format) {
  const button = $('#' + format + 'Export');
  button.disabled = true;
  $('#exportStatus').textContent = 'Preparing ' + format.toUpperCase() + '…';
  try {
    const diagram = serializeGraph($('#graph'), { data });
    if (format === 'png') download(await rasterizeSvg(diagram), 'image/png', 'curve-atlas.png');
    else download(diagram.source, 'image/svg+xml', 'curve-atlas.svg');
    $('#exportStatus').textContent = format.toUpperCase() + ' downloaded.';
  } catch (error) {
    $('#exportStatus').textContent = 'Export failed: ' + error.message + (format === 'png' ? ' Try the SVG download.' : '');
  } finally {
    button.disabled = false;
  }
}
