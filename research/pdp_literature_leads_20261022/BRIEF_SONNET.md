# Brief for the director-tier (Sonnet 5.5) lead dossiers

You are chasing one lead card from `LEADS.md` in this directory
(`/home/user/crypto/research/pdp_literature_leads_20261022/`). You are the
director tier of the owner's model-tier topology
(`/home/user/crypto-autoresearcher/docs/model-tiers.md`): bounded generation,
gated downstream. You change no research state, run no experiment, commit
nothing, and write exactly one file: `leads/LEAD-NN.md` (your lead's number).

## Rules

1. **Literature first** (ml-cryptanalysis AGENTS.md, owner 2026-10-19): read
   the primary sources that matter; verify their statements rather than
   relying on memory. Mark every citation as one of `read-at-source`
   (you fetched and read the text), `abstract-only` (you read the abstract or
   metadata page), or `recalled` (from memory, not fetched). Never invent a
   citation. If a source is unreachable, say which URL failed and how.
2. **Network facts.** IACR ePrint PDFs and some abstract pages are
   Cloudflare-blocked from this container (HTTP 403 or "Just a moment"). arXiv
   abstract pages and PDFs work through `WebFetch`; HAL, author pages and
   publisher abstract pages usually work; Semantic Scholar's API is
   rate-limited. Try alternatives, record failures, move on.
3. **Repository first for prior work.** Before claiming novelty, read the
   repository files the card names (paths are under `/home/user/crypto`,
   `/home/user/crypto-autoresearcher`, `/home/user/cryptanalysis`,
   `/home/user/ml-cryptanalysis`). Use `Grep`/`Glob` to find more. The
   repositories hold hundreds of negative results; a lead that re-opens a
   closed one must say what is different, or conclude "closed".
4. **No Python research code in `/home/user/crypto`** (AGENTS.md). You may run
   small illustrative calculations in the scratchpad
   (`/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/`),
   but any number from them is labelled "unverified desk check" in the
   dossier, is not a measurement, and the script is not committed.
5. **Do not edit** `LEADS.md`, `BRIEF_*.md`, any other dossier, or any file
   outside `leads/LEAD-NN.md`. Do not run `git`.
6. **Honesty of tier.** A result that only rearranges accounting is not an
   improvement. An oracle built from group operations and membership tests is
   on the generic line; say so. Report the gap to rho as distance, not as
   pass/fail.
7. Keep the dossier to roughly 1,500–3,000 words. Precision beats volume.

## Dossier format (`leads/LEAD-NN.md`)

```
# LEAD-NN — <title from the card>

**Verdict.** one of: exponent-candidate | constant-factor candidate |
closed-by-theorem | closed-by-measurement | unverifiable-from-here
**Confidence.** low | medium | high, with one sentence why.
**Expected gain if it works.** (number or range, with the unit and the
baseline row it would move; "unknown" is allowed)
**Cost of the discriminating test.** (person-days, CPU-hours, memory)

## 1. Mechanism (precise)
The mathematics: objects, equations, variable and equation counts, degrees,
for m = 3 at a named n or ℓ, and for m = 4 where it matters. State the
degree-of-regularity or solving-degree bound the formulation is expected to
obey and where that bound comes from.

## 2. Prior art: literature
One bullet per source: authors, title, venue, year, identifier, URL, access
status (read-at-source / abstract-only / recalled), and the one claim you
take from it.

## 3. Prior art: these repositories
One bullet per file read: path, what it measured or proved, and how it
bears on the lead (supports / closes / unrelated).

## 4. Why it could lower the Gröbner cost or the degree of regularity
Quantitative where possible. Name the assumption each step rests on.

## 5. Obstructions
Theorems, exact computations and measured negatives that bound the lead.
Say which generic bound covers the oracle and what it allows.

## 6. Minimal discriminating test (frozen protocol)
Instances (field, curve conditions, m, ℓ or digit count, n), instrument
(which repository binary or external tool), what is recorded, the
prediction, the kill criterion, and the cost. The test must be runnable by
the mechanical tier from a runbook, so no judgement calls inside it.

## 7. Ranking inputs
expected gain | confidence | cost | tier (exponent / constant) | depends on

## 8. Open questions
What you could not settle from here and why.

## 9. Sources
Every URL you fetched, with the HTTP outcome, and every repository path read.
```
