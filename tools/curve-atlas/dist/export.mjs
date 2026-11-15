const SVG_NS = 'http://www.w3.org/2000/svg';

// A complete, self-contained diagram, independent of the interactive zoom.
export function serializeGraph(graph, { title, subtitle }) {
  const svg = graph.cloneNode(true);
  const { width, height } = graph.viewBox.baseVal;
  const totalHeight = height + 160;
  svg.setAttribute('xmlns', SVG_NS);
  svg.setAttribute('viewBox', `0 -80 ${width} ${totalHeight}`);
  svg.setAttribute('width', width);
  svg.setAttribute('height', totalHeight);
  svg.querySelector(':scope > g')?.removeAttribute('transform');

  const element = (name, attributes, text) => {
    const child = document.createElementNS(SVG_NS, name);
    for (const [key, value] of Object.entries(attributes)) child.setAttribute(key, value);
    if (text !== undefined) child.textContent = text;
    return child;
  };
  svg.prepend(element('rect', {
    x: 0, y: -80, width, height: totalHeight, fill: '#101722',
  }));
  svg.prepend(element('style', {}, `
    .edge-line { fill: none; stroke-width: 1.4; opacity: .85; }
    .edge-hit { fill: none; stroke: none; }
    .halo { opacity: 0; }
    .edge text { font: 10px sans-serif; paint-order: stroke;
      stroke: #101722; stroke-width: 5px; stroke-linejoin: round; }
  `));
  svg.append(element('text', {
    x: 35, y: -45, fill: '#e4eaf2', 'font-family': 'sans-serif', 'font-size': 20,
  }, title));
  svg.append(element('text', {
    x: 35, y: -18, fill: '#9eacbd', 'font-family': 'sans-serif', 'font-size': 13,
  }, subtitle));
  const legend = [
    ['Elliptic', '#b4e98a'], ['Hyperelliptic', '#7bafff'],
    ['Higher-genus cyclic', '#b49aff'], ['Jacobian', '#edb779'],
  ];
  legend.forEach(([label, color], i) => {
    const x = 65 + i * 245;
    svg.append(element('circle', { cx: x, cy: height + 25, r: 4, fill: color }));
    svg.append(element('text', {
      x: x + 13, y: height + 29, fill: '#9eacbd',
      'font-family': 'sans-serif', 'font-size': 12,
    }, label));
  });
  svg.append(element('text', {
    x: 35, y: height + 62, fill: '#9eacbd',
    'font-family': 'sans-serif', 'font-size': 11,
  }, 'Bounded explicit families. Dashed arrows: structural Jacobian maps.'));
  return { source: new XMLSerializer().serializeToString(svg), width, height: totalHeight };
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
