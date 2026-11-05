#!/usr/bin/env node

import crypto from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const input = process.argv[2];
const output = process.argv[3] ?? path.join(here, "galbraith-2018-inventory.json");
const expectedSha256 = "ca75ee62b96daa0d46faaa2d4dc28205683455ad3a2a48586b9b39299c25022d";

if (!input) {
  process.stderr.write("usage: node extract-galbraith-inventory.mjs BOOK.pdf [OUTPUT.json]\n");
  process.exit(2);
}

const chapterStarts = [
  ["1", 27, "Introduction"],
  ["2", 37, "Basic Algorithmic Number Theory"],
  ["3", 75, "Hash Functions and MACs"],
  ["4", 83, "Preliminary Remarks on Algebraic Groups"],
  ["5", 87, "Varieties"],
  ["6", 109, "Tori, LUC and XTR"],
  ["7", 123, "Curves and Divisor Class Groups"],
  ["8", 145, "Rational Maps on Curves and Divisors"],
  ["9", 165, "Elliptic Curves"],
  ["10", 201, "Hyperelliptic Curves"],
  ["11", 237, "Basic Algorithms for Algebraic Groups"],
  ["12", 261, "Primality Testing and Integer Factorisation using Algebraic Groups"],
  ["13", 269, "Basic Discrete Logarithm Algorithms"],
  ["14", 285, "Factoring and Discrete Logarithms using Pseudorandom Walks"],
  ["15", 323, "Factoring and Discrete Logarithms in Subexponential Time"],
  ["16", 355, "Lattices"],
  ["17", 365, "Lattice Basis Reduction"],
  ["18", 383, "Algorithms for the Closest and Shortest Vector Problems"],
  ["19", 397, "Coppersmith's Method and Related Applications"],
  ["20", 435, "The Diffie-Hellman Problem and Cryptographic Applications"],
  ["21", 447, "The Diffie-Hellman Problem"],
  ["22", 477, "Digital Signatures Based on Discrete Logarithms"],
  ["23", 493, "Public Key Encryption Based on Discrete Logarithms"],
  ["24", 507, "The RSA and Rabin Cryptosystems"],
  ["25", 545, "Isogenies of Elliptic Curves"],
  ["26", 573, "Pairings on Elliptic Curves"],
  ["A", 589, "Background Mathematics"],
  ["B", 605, "Hints and Solutions to Exercises"]
];

const coreChapters = new Set(["11", "13", "14", "15", "21", "25", "26"]);
const protocolChapters = new Set(["20", "22", "23", "24"]);
const backgroundChapters = new Set(["1", "3", "B"]);

function chapterForPage(pdfPage) {
  let chapter = { id: "front", title: "Front matter" };
  for (const [id, start, title] of chapterStarts) {
    if (pdfPage >= start) chapter = { id, title };
  }
  return chapter;
}

function relevanceForChapter(chapter) {
  if (coreChapters.has(chapter)) return "ecdlp_core";
  if (protocolChapters.has(chapter)) return "protocol_context";
  if (backgroundChapters.has(chapter)) return "background";
  return "mathematical_dependency";
}

function statementScore(line, number) {
  const index = line.indexOf(number);
  const tail = index < 0 ? "" : line.slice(index + number.length).replace(/^\.\s*/, "").trim();
  let score = 0;
  if (line.includes(`${number}.`)) score += 2;
  if (/^(?:\([^)]{1,50}\)\s*)?(?:Let|Suppose|Assume|If|For|There|Every|Any|Fix|Given|The|A |An |No |One |We |Write |Consider |Letting )/.test(tail)) score += 4;
  if (/^(?:shows?|states?|implies?|will|is proved|therefore|can also|below|holds?|has the|justifies?|follows?)/i.test(tail)) score -= 5;
  if (tail.length === 0) score -= 2;
  return score;
}

const bytes = fs.readFileSync(input);
const sha256 = crypto.createHash("sha256").update(bytes).digest("hex");
if (sha256 !== expectedSha256) {
  throw new Error(`unexpected PDF SHA-256 ${sha256}; expected ${expectedSha256}`);
}

const text = execFileSync("pdftotext", ["-layout", input, "-"], {
  encoding: "utf8",
  maxBuffer: 32 * 1024 * 1024
});
const pages = text.split("\f");
const resultPattern = /^\s*(Definition|Theorem|Lemma|Proposition|Corollary|Conjecture|Assumption|Heuristic)\s+([^\s]+)(?:\s|$)/;
const resultNumberPattern = /^((?:[0-9]+|A|B)(?:\.[0-9]+)+)/;
const resultCandidates = new Map();

for (let pageIndex = 0; pageIndex < pages.length; pageIndex += 1) {
  const pdfPage = pageIndex + 1;
  const chapter = chapterForPage(pdfPage);
  for (const line of pages[pageIndex].split(/\r?\n/)) {
    const match = line.match(resultPattern);
    if (!match) continue;
    const number = match[2].match(resultNumberPattern)?.[1];
    if (!number || number.split(".")[0] !== chapter.id) continue;
    const key = `${match[1]}:${number}`;
    const candidate = { pdf_page: pdfPage, line, score: statementScore(line.trim(), number) };
    if (!resultCandidates.has(key)) resultCandidates.set(key, []);
    resultCandidates.get(key).push(candidate);
  }
}

// Three headings contain extraction noise in this PDF. Two also have clean same-page
// cross-references; 19.6.5 has no clean occurrence and must be restored explicitly.
const repairs = new Map([
  ["Theorem:11.1.12", { pdf_page: 240, note: "heading token extracted as 11.1.12.Pl" }],
  ["Conjecture:19.6.5", { pdf_page: 414, note: "heading extracted as 'Conjecture p 19.6.5'" }],
  ["Theorem:21.7.11", { pdf_page: 473, note: "heading extracted as 'Theorem p 21.7.11'" }],
  ["Theorem:A.5.5", { pdf_page: 592, note: "heading token extracted as PmA.5.5.i" }]
]);
for (const [key, repair] of repairs) {
  if (!resultCandidates.has(key)) resultCandidates.set(key, []);
  resultCandidates.get(key).push({ pdf_page: repair.pdf_page, line: "", score: 100, repair: repair.note });
}

const results = [];
for (const [key, candidates] of resultCandidates) {
  const [type, number] = key.split(":");
  candidates.sort((left, right) => right.score - left.score || left.pdf_page - right.pdf_page);
  const selected = candidates[0];
  const chapter = chapterForPage(selected.pdf_page);
  const row = {
    id: `GAL-${type.toUpperCase()}-${number.replaceAll(".", "-")}`,
    type,
    number,
    chapter: chapter.id,
    chapter_title: chapter.title,
    section: number.split(".").slice(0, 2).join("."),
    pdf_page: selected.pdf_page,
    relevance: relevanceForChapter(chapter.id)
  };
  const repair = candidates.find(candidate => candidate.repair);
  if (repair) row.extraction_note = repair.repair;
  results.push(row);
}

const algorithmPattern = /^\s*Algorithm\s+([0-9]+)(?:\s+|\.\s*$)/;
const algorithms = new Map();
for (let pageIndex = 0; pageIndex < pages.length; pageIndex += 1) {
  for (const line of pages[pageIndex].split(/\r?\n/)) {
    const match = line.match(algorithmPattern);
    if (!match) continue;
    const number = Number(match[1]);
    if (!algorithms.has(number)) algorithms.set(number, []);
    algorithms.get(number).push({ pdf_page: pageIndex + 1, line: line.trim() });
  }
}
for (const [number, candidates] of algorithms) {
  // A numbered algorithm's display line has a title (except Algorithm 27, whose title is
  // intentionally blank). Prefer it over prose such as "Algorithm 6 computes ...".
  candidates.sort((left, right) => {
    const leftTitle = new RegExp(`^Algorithm ${number}\\s+[A-Z]`).test(left.line) ? 1 : 0;
    const rightTitle = new RegExp(`^Algorithm ${number}\\s+[A-Z]`).test(right.line) ? 1 : 0;
    return rightTitle - leftTitle || left.pdf_page - right.pdf_page;
  });
  const selected = candidates[0];
  const chapter = chapterForPage(selected.pdf_page);
  results.push({
    id: `GAL-ALGORITHM-${number}`,
    type: "Algorithm",
    number: String(number),
    chapter: chapter.id,
    chapter_title: chapter.title,
    section: chapter.id,
    pdf_page: selected.pdf_page,
    relevance: relevanceForChapter(chapter.id)
  });
}

results.sort((left, right) => left.pdf_page - right.pdf_page || left.id.localeCompare(right.id));

const sectionPattern = /^\s*((?:[0-9]+|A)\.[0-9]+(?:\.[0-9]+)?)\s{2,}(.+?)\s*$/;
const sectionsByNumber = new Map();
for (let pageIndex = 0; pageIndex < pages.length; pageIndex += 1) {
  for (const line of pages[pageIndex].split(/\r?\n/)) {
    const match = line.match(sectionPattern);
    if (!match || sectionsByNumber.has(match[1])) continue;
    const chapter = chapterForPage(pageIndex + 1);
    if (match[1].split(".")[0] !== chapter.id) continue;
    sectionsByNumber.set(match[1], {
      number: match[1],
      title: match[2].trim().replace(/\s+/g, " "),
      chapter: chapter.id,
      pdf_page: pageIndex + 1,
      relevance: relevanceForChapter(chapter.id)
    });
  }
}
const sections = [...sectionsByNumber.values()].sort((left, right) => left.pdf_page - right.pdf_page || left.number.localeCompare(right.number));

function countBy(items, key) {
  const counts = new Map();
  for (const item of items) counts.set(item[key], (counts.get(item[key]) ?? 0) + 1);
  return Object.fromEntries([...counts].sort());
}
const inventory = {
  schema: "ecdlp.galbraith-book-inventory/v1",
  generated: "2026-11-01",
  source: {
    id: "S-GALBRAITH-MPKC-2018",
    title: "Mathematics of Public Key Cryptography",
    author: "Steven D. Galbraith",
    version: "2.0",
    date: "2018-10-31",
    pdf_pages: 696,
    sha256
  },
  method: {
    command: "pdftotext -layout",
    completeness_unit: "Distinct numbered Definition, Theorem, Lemma, Proposition, Corollary, Conjecture, Assumption, Heuristic, and Algorithm labels, plus numbered section headings.",
    limitation: "This is a location inventory, not a reproduction of theorem prose. Unnumbered security claims are curated separately in the claim graph and audit. Appendix B solution references are excluded as results."
  },
  counts: {
    labeled_results: results.length,
    sections: sections.length,
    by_type: countBy(results, "type"),
    by_chapter: countBy(results, "chapter"),
    by_relevance: countBy(results, "relevance")
  },
  sections,
  results
};

fs.writeFileSync(output, `${JSON.stringify(inventory, null, 2)}\n`);
process.stdout.write(`wrote ${output}: ${results.length} labeled results and ${sections.length} sections\n`);
