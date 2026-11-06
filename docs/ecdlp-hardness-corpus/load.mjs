import fs from "node:fs";
import path from "node:path";

export function loadCorpus(corpusPath) {
  const base = JSON.parse(fs.readFileSync(corpusPath, "utf8"));
  const fragments = base.fragments ?? [];
  delete base.fragments;

  for (const relative of fragments) {
    const fragmentPath = path.resolve(path.dirname(corpusPath), relative);
    const fragment = JSON.parse(fs.readFileSync(fragmentPath, "utf8"));
    for (const key of ["sources", "nodes", "edges", "research_targets"]) {
      if (fragment[key] !== undefined) {
        if (!Array.isArray(fragment[key])) {
          throw new Error(`${fragmentPath}: ${key} must be an array`);
        }
        base[key].push(...fragment[key]);
      }
    }
  }
  return base;
}
