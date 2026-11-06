# Brief for the mechanical-tier (Haiku 5.5) verification cards

You verify one dossier, `leads/LEAD-NN.md`, in
`/home/user/crypto/research/pdp_literature_leads_20261022/`. You are the
mechanical tier of the owner's model-tier topology: you execute a fixed
procedure and relay tool output. You compute nothing, judge nothing beyond the
match tests below, change no file except the one you write, and run no `git`.

## Procedure (do every step, in order)

1. Read `leads/LEAD-NN.md` once.
2. **Citations.** List every literature citation in the dossier (anything with
   authors and a title, year, venue or identifier). For each one:
   - If it carries a URL or an identifier (arXiv id, ePrint id, DOI, HAL id),
     fetch that page with `WebFetch` and ask it to return the title, authors,
     year and venue shown on the page. If the fetch fails (403, timeout, "Just
     a moment"), try one `WebSearch` with the exact title in quotes and take
     the top result's title/authors/year.
   - Record `VERIFIED` when the page's title and at least one author match the
     dossier's; `MISMATCH` when the page shows different title, authors or
     year (quote what the page shows); `UNREACHABLE` when neither route
     returned a page (quote the error). Do not infer; do not fill gaps from
     memory.
3. **Repository paths.** List every path in the dossier that begins with
   `/home/user/` or with `src/`, `research/`, `docs/`, `papers/`, `knowledge/`,
   `ledger/`, `experiments/`, `csd/`, `ecc2k130/`, `suite/`, `groebner_compare/`
   or `examples/`. For a relative path, test it under `/home/user/crypto`,
   then `/home/user/crypto-autoresearcher`, `/home/user/cryptanalysis`,
   `/home/user/ml-cryptanalysis`. Run `test -e <path> && echo EXISTS || echo
   MISSING` through Bash for each. Record the result verbatim. Paths with a
   `:line` suffix are tested without the suffix.
4. **Numbers quoted from repository files.** For each sentence in the dossier
   that quotes a number "from" a named repository file (e.g. "`README.md`
   reports 48×"), run `grep -n -- '<the number>' <file>` through Bash and
   record `FOUND` with the line or `NOT FOUND`. Skip numbers with no named
   file.
5. Write `leads/LEAD-NN.verify.md` in exactly this layout:

```
# Verification of LEAD-NN (mechanical tier)

Dossier: leads/LEAD-NN.md. Procedure: BRIEF_HAIKU.md. Date: 2026-10-22.

## Citations
| # | citation as written | route checked (URL or query) | outcome | page shows |
|---|---|---|---|---|

## Repository paths
| # | path as written | tested as | outcome |
|---|---|---|---|

## Quoted numbers
| # | number | file | grep outcome (line) |
|---|---|---|---|

## Totals
citations: N verified / N mismatch / N unreachable; paths: N exist / N missing;
numbers: N found / N not found.
```

No prose outside the tables and the totals line. If a table would be empty,
write one row reading `none`.
