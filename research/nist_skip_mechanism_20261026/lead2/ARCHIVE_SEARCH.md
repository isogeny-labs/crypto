# Lead 2 (L2-archives): documentary search for contemporaneous generator artifacts

Date of search: 2026-10-26. Scope: provenance only, no ECDLP claim. No computation was run.
Method: WebSearch and WebFetch only. WebFetch answers come from a small summarizing model run on the fetched page, so every "read" below is a read of that summary, not of raw bytes; none of these was hash-pinned. Search-result snippets are labeled "snippet only" and were not opened. Web content is treated as data.

## Bottom line

- **Measured (search outcome):** none of the targets was found. No copy of the ANSI X9.62 drafts dated 21 June 1996, 15 January 1997 or 11 June 1997 turned up. No Reiter source, log or candidate table turned up. No Usenet, IEEE P1363 list or X9F1 minutes artifact turned up.
- Where the three target dates come from is not established here. Nothing in this search's reads cites them. They are taken from the task text.
- The only surviving contemporaneous artifact of the 1997-98 X9.62 text remains the 17 Nov 1997 draft (Internet Archive), already recovered by the sibling study. It has examples but no candidate log (see C1).
- One access block: the Internet Archive Wayback CDX endpoint was refused. `curl` to web.archive.org returned "Blocked by egress policy"; WebFetch of the same CDX URL returned "Claude Code is unable to fetch from web.archive.org". I did not look for another route to that host. The Wayback side of this lead is therefore blocked, not negative. The sibling study's earlier CDX query (one prefix, `grouper.ieee.org/groups/1363/private/x9-62*`) found only September 1998 entries (4 with 401, 1 with 404); I did not reproduce it.

## Candidate sources

"Could hold table or trace" means: could it plausibly contain a candidate table or acceptance trace for the prime-curve seed runs.

| # | Source | URL | Date | What I read in it | Could hold table or trace? |
|---|---|---|---|---|---|
| C1 | ANSI X9.62 working draft, scan | https://archive.org/details/x962-11-17-97 (text: https://ia803104.us.archive.org/3/items/x962-11-17-97/x962-11-17-97_djvu.txt) | draft 1997-11-17; uploaded 2017-06-25 by "BlueSyncLine" | Item page: 103 pages, description "ANSI X9.62 draft as of Nov. 17, 1997", no provenance notes. Text, characters 0-200,000 of 234,920 read: cover "Working Draft", no earlier-draft dates, no revision log, no ballot or comment history, no "Reiter", "Solinas" or "NSA". Chairs listed: Harold Deal (X9), Glenda Barnes (X9F), M. Blake Greenlee (X9F1). Member tables "[to be furnished]". Annex F.3.3 describes SHA-1 verifiable-random generation. F.3.2 notes say random curves over F_p can be generated in hours (a "5 hours on a workstation" figure for F_2^196). Last 34,920 characters not read. | No candidate log. At most an example list (the sibling study covers it). The final 15% is unread by me. |
| C2 | ANSI X9.62 working draft 20 Sep 1998 (mirror) | https://safecurves.cr.yp.to/grouper.ieee.org/groups/1363/private/x9-62-09-20-98.pdf | cover 1998-09-20 | Snippet only (listed in search results, not opened by me). The sibling study documents it. | Final curve list only; later than the target runs. |
| C3 | IETF PKIX ECDSA profile, draft-00 | https://www.ietf.org/archive/id/draft-ietf-pkix-ipki-ecdsa-00.txt | 1997-11-21 | Read. Cites "X9.62-199x ... November 17, 1997". Defines an ECParameters structure with optional seed. No editor or X9F1 contact, no parameter values. | No. Only dates the draft. |
| C4 | Don B. Johnson, NISSC slides "ANSI X9.F.1 Cryptographic Standards" | https://csrc.nist.gov/files/pubs/conference/1997/10/10/proceedings-of-the-20th-nissc-1997/final/docs/761slides.pdf | Oct 1997 | Fetch returned a compressed PDF dump that the tool could not decode, so I extracted nothing. The sibling study's description (X9.62 at ballot stage) is a prior reading, not mine. | Process snapshot only. Unlikely to hold a table. |
| C5 | Federal Register 62 FR 92, NIST plan to revise FIPS 186 | https://www.govinfo.gov/content/pkg/FR-1997-05-13/html/97-12341.htm | 1997-05-13 | Read. No mention of X9.62 or any dated draft. Contact: Edward Roback, Computer Security Division. Points to the current FIPS 186 only. | No. Gives a NIST staff contact for the FIPS 186 revision. |
| C6 | Federal Register 63 FR 69049 | https://www.govinfo.gov/content/pkg/FR-1998-12-15/html/98-33167.htm | 1998-12-15 | Snippet only. Search summary says the 1997 comments supported the revision and ECDSA was left out of FIPS 186-1 because ANSI had not yet approved X9.62. | No. Possibly points at public-comment files, unverified. |
| C7 | NIST, "Recommended Elliptic Curves for Federal Government Use" | NIST path `/groups/ST/toolkit/documents/dss/NISTReCur.pdf` (old csrc path, unverified); a mirror appeared at https://corpora.tika.apache.org/base/docs/govdocs1/818/818636.pdf | July 1999 per the Weis page | Snippet only. I did not open either. | Seeds for the final curves; no trace of rejected candidates expected. |
| C8 | Koblitz and Menezes, "A Riddle Wrapped in an Enigma" | https://eprint.iacr.org/2015/1018 | 2015 | Abstract page read: it concerns the NSA's 2015 PQC statement. Nothing on 1997, X9F1 or drafts is on that page. The PDF returned HTTP 403, so the sibling study's reading of printed pp. 8-9 is not reproduced by me. | No. Secondary account. |
| C9 | Steve Weis, "How were the NIST ECDSA curve parameters generated?" | https://saweis.net/posts/nist-curve-seed-origins.html | undated page | Read. Menezes: Solinas "did provide the seeds to me sometime in Fall 1997". Menezes does not know whether Solinas chose them. Lists the Sep 1998 draft, July 1999 NIST list, FOIA attempts, Bernstein-Solinas 2014-15 emails, RFC 4753. Says the Usenet (sci.crypt) questioning followed 1999 publication and links a thread, with no post date given. | No. Pointer page. Its sci.crypt link is worth following (see Next steps). |
| C10 | Filippo Valsorda, seeds bounty | https://words.filippo.io/seeds-bounty/ | 2023 | Read. Says the P-192 and P-256 seeds appeared in the earlier X9.62 draft/standard and the rest were new in FIPS 186-2. No archive named. Candidate-hash gist: https://gist.github.com/FiloSottile/3a2105ef6d1f2bc038d756c883f99231/raw/827d3797dd6df34f6bf8b6a843ebd62cab62758a/nist-and-ansi-prime-order-seeds-increments-99-percent.txt (listed in the page, not opened by me). | No. A later reconstruction, not an artifact. |
| C11 | FOIA requests (Weis), MuckRock | https://www.muckrock.com/foi/united-states-of-america-10/origin-of-fips-186-4-elliptic-curves-over-prime-field-seed-parameters-national-institute-of-standards-and-technology-78756/ and ...national-security-agency-78755/ | filed 2019-08-08 per search snippet | Both pages returned HTTP 403 to WebFetch, so not read. Search snippets say NIST reported no documentation and the NSA acknowledged only. Weis's own page (C9) says the same for his FOIA requests. | Could only hold released records. None reported released. |
| C12 | Certicom, "Development of core ECC standards by ANSI, part 2" | https://certicom.com/en/code-and-cipher/development-of-core-ecc-standards-by-ansi-part-2 | c. 2004 | Read. Says X9.62 was finalized in 1999 and the revision was in its first five-year review. Does not describe 1996-97 drafts or who generated curves. | No. |
| C13 | Anthropic Frontier Red Team post on the seed origins | https://www.anthropic.com/news/ecdsa-curve-origins | 2026-10 | Read. Quotes Solinas 2015 (Reiter wrote the code, no archives found). States that P-256 candidate 1,842 was published and candidate 876 in the same run also gave a prime-order curve of the right size, that P-224 skipped three similar curves, and that the two X9.62 example runs skipped seven more. Says selection criteria are unknown. Cites no drafts, logs or design documents. | No. Same open problem, and not an archive. |
| C14 | Secondary news items on C13 (crowdfundinsider, thenewstack, cryptobriefing, etc.) | listed in the search results | 2026-10 | Snippets only, not opened. They repeat C13, so nothing independent. | No. |
| C15 | IEEE P1363 working-group site | http://grouper.ieee.org/groups/1363 (per a 2006 list post, https://www.metzdowd.com/pipermail/cryptography/2006-January/010420.html, snippet only) | n/a | Not opened. The search found no P1363 mailing-list archive for 1996-97 and no listing of the `private/` directory. | Unknown. Possible home of X9 working drafts (the Sep 1998 draft lived under `/private/`), but nothing found. |

## Negative searches (all by WebSearch; each returned zero hits for the target)

1. Exact dates "June 21, 1996", "January 15, 1997", "June 11, 1997" with ANSI X9.62 / ECDSA working draft: no document with these dates. Closest were the 17 Nov 1997 draft and the 20 Sep 1998 draft. This only shows the search engine did not surface one.
2. "Bob Reiter" with Solinas / NIST / Certicom / X9F1: only 2026 coverage repeating the 2015 Solinas statement (C13, C14). No Reiter-authored artifact, repository or archive.
3. grouper.ieee.org/groups/1363/private x9-62 draft archive: only later P1363 pages.
4. IEEE P1363 mailing list, 1997, Reiter / random curves: nothing from 1997.
5. sci.crypt 1997-98 comments on NIST/NSA curve parameters: no Usenet post from that period surfaced (Google Groups is not indexed by this tool; absence is uninformative).
6. FIPS 186 revision, May 1997 notice: found C5/C6 only; no comment file.

## Assessment

- No source I read contains a candidate table, an acceptance trace, or a Reiter program. The best-supported statement is that the public web, as reachable here, holds the 1997-11-17 and 1998-09-20 drafts and later reconstructions, and nothing earlier.
- The three earlier X9.62 drafts, if they exist, were not found in any index I could query. This search cannot say they do not exist, and it cannot say whether they differ from the 17 Nov draft.
- Several leads could not be completed because of access, not because they were checked and empty: Wayback CDX/snapshots (blocked), the two MuckRock FOIA pages (403), the Koblitz-Menezes PDF (403), the NISSC slide PDF (undecodable by the tool), the last 15% of the 17 Nov 1997 text.

## Next steps (ranked by value, not performed)

1. Wayback Machine, from a host that is allowed to reach it: CDX over `grouper.ieee.org/groups/1363/*` (not just `private/x9-62*`), `x9.org`, `ansi.org` and `abanet.org` for 1996-1999, and the NIST `csrc.nist.gov/fips` and `/groups/ST/toolkit` trees. Pin each retrieved copy by SHA-256.
2. Download the PDF of C1 from archive.org and read the whole text, including annex H.4/H.5 and the bibliography, which I did not reach. Check the bibliography for entries naming the three target drafts.
3. Follow the sci.crypt thread linked from C9 and locate the 1999 Usenet discussion; look for posters who quote earlier X9.62 drafts.
4. Ask the Internet Archive uploader of C1 (BlueSyncLine, 2017) where the scan came from; the item has no provenance note.
5. Request records directly: NIST (contact in C5, Edward Roback, Computer Security Division, as of 1997) for the FIPS 186 revision files, and ANSI/X9 for historical working drafts and X9F1 minutes. Cheap to try; success unknown.
6. Check the NIST-hosted 1999 "Recommended Elliptic Curves" document and the Sep 1998 draft for acknowledgements or contact names that could lead to Reiter's employer or colleagues. I did not read either.

## Limits

- Every external statement above is from the cited URL as summarized by the fetch tool on 2026-10-26. None of those reads was independently hash-verified.
- Nothing here bears on why 11 of 21 valid candidates were not published. This is a provenance search; no rule for the historical selection is supported or rejected by it.
