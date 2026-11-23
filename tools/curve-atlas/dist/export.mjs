const SVG_NS = 'http://www.w3.org/2000/svg';
const palette = { ink: '#20334c', muted: '#63758b', border: '#dce4ed', paper: '#f8fafc',
  elliptic: '#315fa8', hyperelliptic: '#167b79', superelliptic: '#87539c', jacobian: '#7160a8' };
const sub = value => String(value).replace(/\d/g, d => '₀₁₂₃₄₅₆₇₈₉'[d]);
const exponent = value => String(value).replace(/\d/g, d => '⁰¹²³⁴⁵⁶⁷⁸⁹'[d]);
const label = node => node.label.replace(/\d+/g, sub);
const equation = node => `y² = x³ + ${node.a}x + ${node.b}`;

// An atlas layout repeats the same E model locally so no unrelated arrows cross.
export function renderDiagram(data, { selected = null } = {}) {
  const width = 1000;
  const maps = data.edges.filter(edge => edge.kind === 'isogeny');
  const covers = data.nodes.filter(node => node.kind === 'hyperelliptic' || node.kind === 'superelliptic');
  const base = data.nodes.find(node => node.id === 'E0');
  const isogenyHeight = maps.length ? maps.length * 112 : 58;
  const coverStart = 286 + isogenyHeight;
  const height = coverStart + 72 + Math.max(covers.length * 224, 80) + 76;
  const svg = document.createElementNS(SVG_NS, 'svg');
  const add = (parent, name, attrs = {}, text) => {
    const el = document.createElementNS(SVG_NS, name);
    for (const [key, value] of Object.entries(attrs)) el.setAttribute(key, value);
    if (text !== undefined) el.textContent = text;
    parent.append(el);
    return el;
  };
  const text = (parent, x, y, value, size = 14, fill = palette.ink, extra = {}) =>
    add(parent, 'text', { x, y, fill, 'font-size': size, ...extra }, value);
  const rect = (parent, x, y, w, h, fill = '#fff', stroke = palette.border, radius = 12) =>
    add(parent, 'rect', { x, y, width: w, height: h, rx: radius, fill, stroke });
  svg.setAttribute('xmlns', SVG_NS);
  svg.setAttribute('class', 'diagram');
  svg.setAttribute('width', width);
  svg.setAttribute('height', height);
  svg.setAttribute('viewBox', `0 0 ${width} ${height}`);
  add(svg, 'title', {}, 'Curve Atlas · Elliptic transfers');
  add(svg, 'desc', {}, 'Explicit isogenies and higher-genus covers, with one panel per connection. The same elliptic source is repeated for clarity.');
  add(svg, 'style', {}, `
    .diagram text { font-family: Arial, Helvetica, sans-serif; }
    .diagram .math { font-family: Georgia, 'Times New Roman', serif; }
    .diagram .edge text { font: 12px Arial, sans-serif; stroke: none; paint-order: normal; }
    .diagram .edge-line { fill: none; stroke-width: 1.8; opacity: 1; }
    .diagram .edge-hit { fill: none; stroke: transparent; stroke-width: 18; }
    .diagram .node, .diagram .edge { cursor: pointer; }
    .diagram .node.selected > rect, .diagram .node:focus-visible > rect { stroke-width: 3; }
    .diagram .edge.selected .edge-line, .diagram .edge:hover .edge-line { stroke-width: 3; }
  `);
  rect(svg, 0, 0, width, height, palette.paper, 'none', 0);
  const defs = add(svg, 'defs');
  for (const [name, color] of Object.entries(palette)) {
    const marker = add(defs, 'marker', { id: `tip-${name}`, viewBox: '0 0 10 10', refX: 9, refY: 5,
      markerWidth: 7, markerHeight: 7, orient: 'auto' });
    add(marker, 'path', { d: 'M0 0L10 5L0 10Z', fill: color });
  }
  const group = (kind, item) => add(svg, 'g', {
    class: `${kind}${selected === item.id ? ' selected' : ''}`, [`data-${kind}`]: item.id,
    tabindex: 0, role: 'button', 'aria-label': kind === 'node' ? `${label(item)}, ${item.kind}` : `${item.kind}: ${item.source} to ${item.target}`,
  });
  function node(item, x, y, w, title, detail, color, fill) {
    const g = group('node', item);
    rect(g, x, y, w, 66, fill, color, 10);
    text(g, x + w / 2, y + 28, title, 21, color, { 'text-anchor': 'middle', class: 'math' });
    text(g, x + w / 2, y + 49, detail, 12, palette.muted, { 'text-anchor': 'middle' });
  }
  function arrow(edge, x1, x2, y, caption, colorName = 'elliptic', dashed = false, below = false) {
    const g = group('edge', edge), color = palette[colorName];
    const path = `M${x1} ${y}H${x2}`;
    add(g, 'path', { d: path, class: 'edge-hit' });
    add(g, 'path', { d: path, class: 'edge-line', stroke: color,
      'marker-end': `url(#tip-${colorName})`, ...(dashed ? { 'stroke-dasharray': '5 4' } : {}) });
    text(g, (x1 + x2) / 2, y + (below ? 20 : -13), caption, 12, color, { 'text-anchor': 'middle' });
  }
  text(svg, 40, 40, 'CURVE ATLAS', 11, palette.elliptic, { 'letter-spacing': 2.5, 'font-weight': 700 });
  text(svg, 40, 85, 'Elliptic transfers', 34, palette.ink, { 'font-weight': 600 });
  text(svg, 40, 113, 'Explicit curve maps and the Jacobian maps they induce', 15, palette.muted);
  rect(svg, 40, 140, 920, 80, '#eef3fb', '#d6e1f2');
  text(svg, 62, 162, 'SOURCE CURVE  E₀', 10, palette.elliptic, { 'letter-spacing': 1.3, 'font-weight': 700 });
  text(svg, 62, 196, equation(base), 25, palette.ink, { class: 'math' });
  text(svg, 930, 170, `F${sub(data.parameters.p)}  ·  ${base.count} rational points`, 14, palette.elliptic, { 'text-anchor': 'end' });
  text(svg, 930, 196, `c = ${data.parameters.c}  ·  n ≤ ${data.parameters.maxN}  ·  isogeny depth ${data.parameters.depth}`, 12, palette.muted, { 'text-anchor': 'end' });
  text(svg, 40, 260, '01   ISOGENIES', 12, palette.muted, { 'letter-spacing': 1.6, 'font-weight': 700 });
  maps.forEach((edge, i) => {
    const y = 280 + i * 112;
    const source = data.nodes.find(n => n.id === edge.source), target = data.nodes.find(n => n.id === edge.target);
    rect(svg, 40, y, 920, 96);
    node(source, 60, y + 15, 260, label(source), equation(source), palette.elliptic, '#f2f6fc');
    node(target, 680, y + 15, 260, label(target), equation(target), palette.elliptic, '#f2f6fc');
    arrow(edge, 340, 660, y + 44, 'degree 2 · Vélu isogeny');
    text(svg, 500, y + 75, `kernel {O, (${edge.kernel}, 0)}`, 12, palette.muted, { 'text-anchor': 'middle' });
  });
  if (!maps.length) text(svg, 40, 296, 'No rational 2-isogenies displayed at these search settings.', 14, palette.muted);
  text(svg, 40, coverStart + 24, '02   HIGHER-GENUS COVERS', 12, palette.muted, { 'letter-spacing': 1.6, 'font-weight': 700 });
  text(svg, 40, coverStart + 49, 'E₀ is the same source in every panel; repeated labels keep each connection readable.', 13, palette.muted);
  covers.forEach((curve, i) => {
    const y = coverStart + 70 + i * 224;
    const edge = data.edges.find(e => e.source === curve.id && e.kind === 'cover');
    const jac = data.nodes.find(n => n.curve === curve.id);
    const colorName = curve.kind, color = palette[colorName];
    rect(svg, 40, y, 920, 208);
    text(svg, 62, y + 31, label(curve), 23, color, { class: 'math' });
    text(svg, 120, y + 30, curve.kind === 'hyperelliptic' ? 'Hyperelliptic cover' : curve.n === 1 ? 'Nonhyperelliptic plane quartic' : 'Cyclic cover', 14, palette.muted);
    rect(svg, 864, y + 14, 74, 26, '#f0f3f7', 'none', 13);
    text(svg, 901, y + 32, `genus ${curve.genus}`, 12, palette.muted, { 'text-anchor': 'middle' });
    node(curve, 62, y + 68, 218, label(curve), 'cover curve', color, curve.kind === 'hyperelliptic' ? '#eef8f6' : '#f7f1fa');
    node(base, jac ? 410 : 730, y + 68, jac ? 180 : 208, 'E₀', 'source elliptic curve', palette.elliptic, '#f2f6fc');
    arrow(edge, 294, jac ? 396 : 716, y + 101, `π · degree ${edge.degree}`, colorName);
    if (jac) {
      node(jac, 730, y + 68, 208, `J(${label(curve)})`, `dimension ${jac.dimension}`, palette.jacobian, '#f4f1fa');
      arrow(data.edges.find(e => e.id === 'p' + curve.id), 605, 716, y + 87, 'pullback', 'jacobian', true);
      arrow(data.edges.find(e => e.id === 't' + curve.id), 716, 605, y + 118, 'norm', 'jacobian', true, true);
    }
    const u = curve.n === 1 ? 'u' : `u${exponent(curve.n)}`, v = curve.m === 2 ? 'v' : 'v²';
    const lifted = curve.c === 0 ? u : `${u} + ${curve.c}`;
    text(svg, 62, y + 168, `π(u, v) = (${lifted}, ${v})`, 16, palette.ink, { class: 'math' });
    text(svg, 62, y + 191, curve.c === 0 ? `v${exponent(curve.m)} = u${exponent(3 * curve.n)} + ${base.a}${u} + ${base.b}` : `v${exponent(curve.m)} = (${lifted})³ + ${base.a}(${lifted}) + ${base.b}`, 14, palette.muted, { class: 'math' });
    if (jac) text(svg, 938, y + 179, `norm ∘ pullback = [${edge.degree}] on E₀`, 13, palette.jacobian, { 'text-anchor': 'end' });
  });
  if (!covers.length) text(svg, 40, coverStart + 97, 'No smooth covers at these settings. Choose a shift with f(c) ≠ 0.', 14, palette.muted);
  text(svg, 40, height - 41, 'Solid arrows: explicit curve maps. Dashed arrows: structural Jacobian homomorphisms.', 12, palette.muted);
  text(svg, 40, height - 19, 'Bounded families over the declared field; not a classification of all transfers.', 11, palette.muted);
  return svg;
}

export function serializeGraph(graph, { data }) {
  const svg = renderDiagram(data);
  return { source: new XMLSerializer().serializeToString(svg),
    width: Number(svg.getAttribute('width')), height: Number(svg.getAttribute('height')) };
}

export async function rasterizeSvg({ source, width, height }, scale = 2) {
  const url = URL.createObjectURL(new Blob([source], { type: 'image/svg+xml' }));
  try {
    const image = new Image();
    await new Promise((resolve, reject) => {
      image.onload = resolve;
      image.onerror = () => reject(new Error('The browser could not load the diagram.'));
      image.src = url;
    });
    const canvas = document.createElement('canvas');
    canvas.width = Math.round(width * scale);
    canvas.height = Math.round(height * scale);
    const context = canvas.getContext('2d');
    if (!context) throw new Error('PNG export requires canvas support.');
    context.drawImage(image, 0, 0, canvas.width, canvas.height);
    const blob = await new Promise(resolve => canvas.toBlob(resolve, 'image/png'));
    if (!blob) throw new Error('The browser could not encode a PNG.');
    return blob;
  } finally {
    URL.revokeObjectURL(url);
  }
}
