# Sources: fetch log and exact statements (2026-11-07)

All raw files are under `sources/raw/` with SHA-256 in `sources/SHA256SUMS`. Nothing here was executed. The PDF was read with `pdftotext` (text in `sources/cheng.txt`).

## What was tried for Zhang-Liu (ECDLP via list decoding), and what came back

| # | URL | Result | File |
|---|-----|--------|------|
| 1 | https://eprint.iacr.org/2018/795.pdf | HTTP 403, Cloudflare "Just a moment..." page (5473 B). Not bypassed. | raw/eprint_2018_795.pdf (HTML challenge page, not a PDF) |
| 2 | https://eprint.iacr.org/2018/795 | HTTP 200, abstract page only (no full text) | raw/eprint_2018_795_abs.html |
| 3 | https://eprint.iacr.org/archive/2018/795/20180829:000000 | HTTP 403, same challenge. Not bypassed. | raw/eprint_2018_795_v2.pdf (HTML) |
| 4 | https://ia.cr/2018/795.pdf | HTTP 404 | raw/ia_cr.pdf (HTML) |
| 5 | https://link.springer.com/chapter/10.1007/978-3-030-31919-9_13 (ProvSec 2019, LNCS 11821, pp. 222-244) | HTTP 200, landing page: abstract plus "Buy Chapter USD 29.95". Paywalled; the `citation_pdf_url` was not requested. | raw/springer.html |
| 6 | https://api.crossref.org/works/10.1007/978-3-030-31919-9_13 | HTTP 200, metadata only: title, pages 222-244, 38 references, TDM link only | raw/core.json |
| 7 | https://archive.org/wayback/available?url=eprint.iacr.org/2018/795.pdf | HTTP 200, reports a 2026-01-02 snapshot | raw/wayback_avail.json |
| 8 | web.archive.org snapshot (id_ form) of that PDF | HTTP 403 "Blocked" from the session proxy (24 B). One attempt, not retried, not circumvented. | (deleted, 24-byte text) |
| 9 | https://dblp.org/search/publ/api?... | HTTP 202 "Making sure you're not a bot!" page. Not bypassed. | raw/dblp.json (HTML) |
| 10 | api.semanticscholar.org search | HTTP 429 (rate limit). One attempt; not retried. | (deleted) |
| 11 | author page / Google Scholar | not tried: the contact-author domain is not given on the abstract page except as "mail sysu edu cn"; Scholar is bot-gated. | - |

Also searched: the local sparse checkout of crypto-autoresearcher and /home/user/crypto for a copy of the PDF (`2018-795*`): none. KN-LIT-643 names `downloads/2018-795.pdf` but that directory is not in the checkout, and the note says it was generated from the PDF's first two pages only.

**Result: the Zhang-Liu full text was NOT obtained and NOT read.** What is known about it is the abstract (raw/eprint_2018_795_abs.html and the Springer landing page, identical text):

> "We provide a new approach to the elliptic curve discrete logarithm problem (ECDLP). First, we construct Elliptic Codes (EC codes) from the ECDLP. Then we propose an algorithm of finding the minimum weight codewords for algebraic geometry codes, especially for the elliptic code, via list decoding. Finally, with the minimum weight codewords, we show how to solve ECDLP. This work may provide a potential approach to speeding up the computation of ECDLP."

Metadata: Fangguo Zhang and Shengli Liu; ePrint received 2018-09-01, revised 2018-09-02; CC BY; Springer chapter 13 of LNCS 11821, pp. 222-244 (Crossref). The atlas lead also cites the author's 2019 talk slide ("not efficient currently, and it is even not of square-root time"), from the 2026-11-04 desk report (docs/reports/ecdlp-leads-desk-20261104/elliptic-code-desk.md); I did not re-fetch the slide.

Not available to this analysis: the paper's received word(s), parameters, decoder, stated complexity, experiments.

## Cheng, "Hard Problems of Algebraic Geometry Codes" (read in full)

Fetched https://arxiv.org/pdf/cs/0507026 (HTTP 200, 152584 B, sha256 241e5184...; arXiv v1, 8 Jul 2005; abs page in raw/cheng_arxiv_abs.html). Text locators are to `sources/cheng.txt` (pdftotext -layout of the arXiv v1). The IEEE Trans. Inf. Theory 2008 version was not obtained; statements below are from arXiv v1 only.

- Theorem 1 (cheng.txt line 238 ff.). Code of evaluating L(Q + (k-1)O) at P_i = a_i G, with G of order q and Q = bG: min distance n-k iff some k of the a_i sum to b mod q (else n-k+1). Proof: a weight n-k codeword has k zeros in D; divisor (f) = P_i1+...+P_ik - Q - (k-1)O, so the points sum to Q.
- One-point divisors (line 298 ff.): "We can also use one point divisor codes by reducing the following problem to MDPAGC. The detail will be left in the full paper." (subset sum with target 0.) So the one-point case is only announced in arXiv v1, as the lead says.
- Lemma 3 (line 365 ff.). Received word R = (f'(P_1),...,f'(P_n)) with f' in L(Q+(k-1)O) \ L((k-1)O), code from L((k-1)O) (dimension k-1). (1) distance(R, code) is n-k or n-k+1; (2) it is n-k iff some k of the P_i sum to Q. Proof of (1): f'-f in L(Q+(k-1)O) has at most k poles so at most k zeros. Suggested f': f' = l1/l2, l1 the line through Q' and Q'' = Q - Q', l2 the line through Q and -Q.
- Theorem 3 (line 411): computing the distance from a received vector to an elliptic code is NP-hard.
- Section 4 / Theorem 2 (line 333 ff.): ECDLP over F_q reduces to min distance of an elliptic code of length n <= floor(log q), by taking P_i = 2^i P and R = rQ with a random r; time 2^{cn} for min distance gives time q^c for ECDLP.
- Section 6 (line 440): "In the proof of Lemma 3, we use f' to generate a received word, it has poles at a place other than O."

Consequence used in the README: Cheng's Lemma 3(1) says the received word is at distance >= n-k from the code, i.e. agreement with any codeword is at most k.
