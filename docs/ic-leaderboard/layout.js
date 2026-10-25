/* Presentation only. Frozen measurements and evidence links stay in the document.
 * Without JavaScript the complete generated leaderboard remains readable. */
(() => {
  'use strict';
  const root = document.querySelector('.wrap');
  const header = root?.querySelector(':scope > header');
  const board = document.getElementById('board');
  if (!header || !board) return;
  const element = (tag, className, text) => {
    const node = document.createElement(tag);
    if (className) node.className = className;
    if (text) node.textContent = text;
    return node;
  };
  const link = (label, href, className) => {
    const node = element('a', className, label);
    node.href = href;
    return node;
  };
  const disclosure = (label, nodes, className = '') => {
    const details = element('details', `disclosure ${className}`.trim());
    details.append(element('summary', '', label));
    const body = element('div', 'disclosure-body');
    body.append(...nodes);
    details.append(body);
    return details;
  };
  const group = (id, title, description) => {
    const section = element('section', 'page-section');
    section.id = id;
    const heading = element('h2', '', title);
    heading.id = `${id}-title`;
    section.setAttribute('aria-labelledby', heading.id);
    section.append(heading, element('p', 'section-intro', description));
    return section;
  };

  const libraryURL = 'https://aburan28.github.io/crypto/browser/#curves';
  const originalSections = Array.from(root.querySelectorAll(':scope > section'));
  const footer = root.querySelector(':scope > footer');
  const nav = root.querySelector('.site-nav');
  nav.append(link('Curve library', libraryURL));
  root.before(link('Skip to ranked results', '#board-A', 'skip-link'));

  header.querySelector('.eyebrow').textContent = 'Research / Index calculus';
  const originalVerdict = header.querySelector('.verdict');
  const intro = element('p', 'verdict',
    'Measured index-calculus recipes against Pollard rho on the same curve and point. Start with the cold cost: setup through a verified answer.');
  originalVerdict.before(intro);
  const context = disclosure('Comparison scope and online results', [originalVerdict]);
  header.append(context);
  // Keep the scientific verdict tied to the generated source as new results land.
  const coldVerdict = Array.from(originalVerdict.querySelectorAll('strong'))
    .find(node => /cold/i.test(node.textContent))?.textContent || '';
  const notice = element('p', 'comparison-notice',
    `${coldVerdict} Each table has its own unit and reference; compare ratios within that table only.`.trim());
  intro.after(notice);
  const actions = element('div', 'header-actions');
  actions.append(link('Browse the curve library →', libraryURL, 'primary-link'),
    link('View ranked results', '#board-A'));
  notice.after(actions);
  header.querySelectorAll('.fact').forEach(fact => {
    const description = fact.querySelector('p');
    if (description) {
      const unit = description.lastChild;
      if (unit?.nodeType === Node.TEXT_NODE && unit.textContent.trim().startsWith('× ')) {
        fact.append(element('p', 'fact-unit', unit.textContent));
      }
      fact.append(disclosure('Curve and comparison', [description], 'fact-detail'));
    }
  });
  const lastFact = header.querySelector('.fact:last-child');
  if (lastFact) lastFact.append(link('Open curve library →', libraryURL));

  const pageNav = element('nav', 'page-nav');
  pageNav.setAttribute('aria-label', 'On this page');
  for (const [label, href] of [['Ranked results', '#board-A'], ['Research notes', '#research-notes'],
    ['Methods & accounting', '#methods'], ['Curve records', '#curves']]) pageNav.append(link(label, href));
  header.after(pageNav);
  const main = element('main', 'leaderboard-content');
  pageNav.after(main);

  // Move the existing sections; no result, source or historical row is discarded.
  board.querySelector('h2').textContent = 'Ranked results';
  const boardExplanation = board.querySelector(':scope > p');
  if (boardExplanation) {
    const details = disclosure('How to read the rankings', [boardExplanation]);
    board.querySelector('h2').after(element('p', '',
      'Lower cold-cost ratios are closer to the matched reference. A ratio of 1× means equal cost.'));
    board.append(details);
  }
  main.append(board);
  for (const id of ['board-A', 'board-B']) {
    const section = document.getElementById(id);
    if (!section) continue;
    main.append(section);
    const defs = section.querySelector('.defs');
    // Keep the unit and matched reference visible, with the complete protocol one click away.
    if (defs) {
      const summary = element('p', 'table-context');
      for (const term of ['Unit', 'Reference']) {
        const row = Array.from(defs.children).find(node => node.querySelector('dt')?.textContent === term);
        if (!row) continue;
        const line = element('span');
        line.append(element('strong', '', `${term}: `));
        const description = row.querySelector('dd').textContent;
        line.append(term === 'Unit' ? description.split('. ')[0] + '.' : description);
        summary.append(line);
      }
      defs.before(summary);
      summary.after(disclosure('Full units, targets and accounting', [defs]));
    }
    const table = section.querySelector('table');
    if (!table) continue;
    const columns = Array.from(table.tHead.rows[0].cells);
    const compact = new Set(['curve', 'log₂ r', 'm', '|F|', '× ref', 'validation']);
    const extra = columns.map((cell, i) => compact.has(cell.textContent.trim()) ? -1 : i).filter(i => i >= 0);
    for (const row of table.rows) {
      if (row.cells.length !== columns.length) continue; // preserve family labels
      extra.forEach(i => row.cells[i].classList.add('extra-column'));
      if (row.parentElement === table.tHead) continue;
      const curve = row.cells[1];
      const slug = curve.querySelector('.slug');
      if (slug) {
        const metadata = Array.from(curve.childNodes).filter(node => node !== slug);
        curve.append(disclosure('Sources & base', metadata, 'cell-detail'));
      }
      const ratioCell = row.cells[columns.findIndex(cell => cell.textContent.trim() === '× ref')];
      if (ratioCell) {
        const extraNotes = Array.from(ratioCell.querySelectorAll(':scope > .muted'))
          .filter(note => !/^\s*\[/.test(note.textContent)); // keep the cold interval beside its ratio
        if (extraNotes.length) {
          for (const note of extraNotes) {
            if (note.previousSibling?.nodeName === 'BR') note.previousSibling.remove();
          }
          ratioCell.append(disclosure('Online & history', extraNotes, 'cell-detail'));
        }
      }
    }
    const controls = element('div', 'table-controls');
    const label = element('label', 'column-toggle');
    const toggle = element('input');
    toggle.type = 'checkbox';
    toggle.setAttribute('aria-controls', `${id}-table`);
    table.id = `${id}-table`;
    label.append(toggle, 'Show all columns');
    toggle.addEventListener('change', () => section.classList.toggle('all-columns', toggle.checked));
    controls.append(label, element('span', 'muted', 'Cold cost · setup included'));
    table.closest('.scroll').before(controls);
    section.classList.add('ranked-table');
  }

  const notes = group('research-notes', 'Research notes',
    'Component studies, audits and historical comparisons. Each entry retains its own scope, validation status and evidence.');
  const methods = group('methods', 'Methods & accounting',
    'Recipes, phase costs and decomposition methods behind the measurements.');
  const methodIds = new Set(['recipes', 'steps', 'pdp']);
  for (const section of originalSections) {
    if (['board', 'board-A', 'board-B', 'curves'].includes(section.id)) continue;
    const heading = section.querySelector('h2');
    const title = heading?.textContent || section.id;
    const item = disclosure(title, [section], 'study-detail');
    (methodIds.has(section.id) ? methods : notes).append(item);
  }
  main.append(notes, methods);
  const curves = document.getElementById('curves');
  if (curves) {
    main.append(curves);
    curves.querySelector('h2').textContent = 'Curve library & records';
    const library = element('p', 'library-link');
    library.append(link('Search all curves in the library →', libraryURL, 'primary-link'),
      ' Field, subgroup, identity, methods and factor-base records.');
    curves.querySelector('h2').after(library);
    curves.querySelectorAll('details').forEach(details => { details.open = false; });
  }
  if (footer) {
    const sourceParagraph = Array.from(footer.children).find(node => node.textContent.includes('Built by'));
    if (sourceParagraph) footer.append(disclosure('Build provenance and source files', [sourceParagraph]));
  }
  document.querySelectorAll('table').forEach(table => {
    if (table.closest('.scroll')) return;
    const scroller = element('div', 'scroll');
    table.before(scroller);
    scroller.append(table);
  });
  document.querySelectorAll('.scroll').forEach(scroller => {
    scroller.tabIndex = 0;
    scroller.setAttribute('role', 'region');
    const section = scroller.closest('section');
    scroller.setAttribute('aria-label', `${section?.querySelector('h2')?.textContent || 'Results'} — scrollable data`);
  });

  // Old links into folded sections still open the whole ancestor chain.
  function revealHash() {
    let id;
    try { id = decodeURIComponent(location.hash.slice(1)); } catch { return; }
    const target = document.getElementById(id);
    if (!target) return;
    for (let node = target; node; node = node.parentElement) {
      if (node.tagName === 'DETAILS') node.open = true;
    }
    requestAnimationFrame(() => target.scrollIntoView({block: 'start'}));
  }
  window.addEventListener('hashchange', revealHash);
  root.classList.add('organized');
  revealHash();
})();
