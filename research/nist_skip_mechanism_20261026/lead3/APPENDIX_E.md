# Lead 3: Appendix E of the 2013 President's Review Group report (documentary search)

Date of search: 2026-10-26. Provenance only. No ECDLP claim is made. Nothing here is computed; every item is a document quotation or a reported search result.
Labels: [quoted] = read by me in a locally extracted PDF; [reported] = relayed by a WebFetch/WebSearch summariser, not re-read verbatim by me (treat as unverified); [none] = searched, not found.

## 1. The text itself [quoted]

Source read: White House archive copy of the report, `https://obamawhitehouse.archives.gov/sites/default/files/docs/2013-12-12_rg_final_report.pdf`
(downloaded with plain curl, 947,623 bytes, SHA-256 `87107a88ccfc3289fad7131cbd9d5b353abbbc82d7eabe9075086c14483abcf8`, extracted with `pdftotext -layout`; printed page 275).
The NSA-hosted URL named in the open-problem record (`https://www.nsa.gov/portals/75/documents/about/civil-liberties/resources/liberty_security_prgfinalreport.pdf`) returned HTTP 403 to WebFetch and to curl today; it was not read.

Appendix E title: "Appendix E: US Government Role in Current Encryption Standards". Its first sentence:

> "NSA provided the Review Group the following information, outlining the reliability of certain encryption systems."

The one sentence about elliptic-curve parameters (in the paragraph headed "Diffie-Hellman/Elliptic Curve Diffie-Hellman – The Diffie-Hellman Key Exchange Algorithm – NIST SP 800-56A"):

> "NSA ensured that a class of potentially weak elliptic curve parameters was not included in the NIST standard."

Facts about the surrounding text [quoted]:
- It sits in the ECDH / SP 800-56A paragraph, not in the DSA/ECDSA (FIPS 186) paragraph. The ECDSA paragraph says only that NSA designed ECDSA and that NSA and academia "have thoroughly analyzed" DSA. It does not say NSA generated curves, seeds or parameters.
- The sentence names no class, no curve, no date, no committee and no procedure. It says "the NIST standard" (singular), not FIPS 186-2/ANSI X9.62.
- The appendix does not mention Dual_EC_DRBG or any seed (grep of the whole extracted report for "Dual_EC"/"Dual EC": no hit in the appendix).
- Context sentence: Appendix E says the listed algorithms (AES, SHA-2, EC-DSA, EC-DH) are NSA "Suite B", approved 2006. A statement about SP 800-56A (first final May 2006, Rev. 1 March 2007 per the NIST page below) is therefore dated by its subject to 2006-07 standardisation, not necessarily 1997-98.
- The provenance is NSA-supplied and unattributed to any individual; it is an assertion, not a released record.

## 2. Leads, each classified

Columns: names a class? / names a date? / names a procedure? / could it map to 1997-98 prime curves?

| # | Source (URL) | Class | Date | Procedure | Maps to 1997-98 prime curves? |
|---|---|---|---|---|---|
| A | Appendix E sentence above | No | No (context 2006 Suite B) | No | Cannot be mapped; the sentence is silent on prime vs binary and on 1997. |
| B | Koblitz and Menezes, "A Riddle Wrapped in an Enigma" (CACR copy dated 20 Oct 2015, updated 21 Aug 2016) `https://cacr.uwaterloo.ca/techreports/2015/cacr2015-14.pdf` (SHA-256 `048d3f22cd6d0152707d5cfd37c2531f132b91a0785ecb42fb16c4659fc08687`) [quoted] | Names only known classes: prime-field anomalous (#E=q) and "the Weil/Tate pairing attack"; "no new classes of weak elliptic curves have been discovered since 1997" for prime fields. It does not cite Appendix E (grep for "review group"/"appendix e": no hit). | "around 1997"; "in 1997 the public learned" of the anomalous attack | Yes in outline: ANSI X9F1 "decided to include ... elliptic curves that had been generated at random"; "An NSA representative on the ANSI X9F1 committee offered to provide suitable curves"; seed through SHA-1 | This is the committee rationale for the 1997-98 curves. It gives no candidate-level acceptance rule and no excluded-class list beyond anomalous and pairing. Not evidence about the 11 unpublished seeds. |
| C | Steve Weis, "How were the NIST ECDSA curve parameters generated?" `https://saweis.net/posts/nist-curve-seed-origins.html` [reported] | No | Menezes: seeds supplied "sometime in Fall 1997" | Solinas 2015: seeds "via hashing (SHA-1, I think) from the ASCII representation of a humorous message"; exact message and hashing details forgotten (superseded for the prime curves by the lrand48 result) | Context only. Summariser reported the page does not mention excluded weak classes or the Review Group. |
| D | VCAT / Committee of Visitors report, July 2014, minutes of the FIPS 186 session (Dustin Moody), `https://people.csail.mit.edu/rivest/pubs/NIST14.pdf` (SHA-256 `69910757dc72a2ee6c4bb68bbe280d16a2b734c59224a726d44242fb9ef9da69`), printed pp. 18-19 [quoted] | No | No | States the curves were "generated from a seed using a one-way function (SHA-1), following an algorithm described in ANS X9.62. NIST has verified that this is how the parameters were generated." Also: "NIST could not say how many seeds were attempted during the generation process". | Direct NIST acknowledgement (2014) that NIST cannot state how many seeds were tried, i.e. no record of the selection process. Names no excluded class. Supports the "records not held by NIST" reading. |
| E | NIST SP 800-186 (Feb 2023) Appendix C.1, `https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-186.pdf` [quoted from locally extracted PDF] | Its criteria list: prime or 2^m with m prime; cofactor <= 2^10; |E| != q (anomalous); embedding degree k >= 2^10; criterion 6: "there is no technical rationale for imposing a large lower bound on the square-free part of |Disc|". | 2023 | Lists criteria, not a 1997 trace | The 2023 criteria are the present public statement. They are retrospective. Criterion 6 (CM discriminant) is the public NIST statement that no bound is imposed, which constrains the isogeny/Elkies-bound observation: no documented rule involves it. |
| F | Filippo Valsorda, "The NIST Elliptic Curves Seeds Bounty Has Been Claimed" `https://words.filippo.io/seeds-solved/` [reported] | No | Timeline 1997-05-15 to 1998-04-03 | drand48 candidate tests; "two additional undocumented criteria beyond the ANSI X9.62 ones, one fairly clear and one that still escapes us" (clear one: reject if bit length of n exceeds that of p); "roughly twice as many valid seeds as were published" | Direct statement of this question. Does not mention Appendix E (per summariser). |
| G | Weis FOIA (2018) via same Weis page [reported]; podcast `https://securitycryptographywhatever.com/2023/10/12/the-nist-curves/` [reported] | n/a | 2018 | FOIA to NIST and NSA on prime-field curve seeds | Per Weis page summary: NIST said it had no documentation; NSA stopped responding. Outcome of NSA request not independently confirmed. |
| H | Dual_EC_DRBG sources (Schneier 2007 `https://www.schneier.com/blog/archives/2007/11/the_strange_sto.html`; Woit `https://www.math.columbia.edu/~woit/wordpress/?p=7045`; Bernstein-Lange-Niederhagen `https://eprint.iacr.org/2015/767.pdf`) [reported titles only] | Different standard (SP 800-90A). | 2013-14 | n/a | Not a lead for Appendix E: Appendix E does not mention Dual_EC, and the "Appendix E" in Schneier's 2007 post is a different document (SP 800-90 Appendix E). Do not conflate. |
| I | SP 800-56A publication history `https://csrc.nist.gov/pubs/sp/800/56/a/upd1/final` [reported] | No | Draft July 2005; final May 2006; Rev. 1 March 2007 | Listed as supplemental: "Comments on Draft (July 2005)" | The draft-comment PDF is the one concrete in-standard document that might discuss which curves to include. Not yet fetched (see 4). |
| J | Federal Register / ISPAB minutes / press quoting officials naming the class | [none] | | | Searches returned no Federal Register notice, no ISPAB minutes and no official quoted naming the class. A fourth search on "FIPS 186-3 Federal Register comments" returned nothing relevant. Absence in a search engine is not absence in the archives. |
| K | Press 2014 on VCAT (Infosecurity Magazine, DarkReading, `https://www.nist.gov/document/vcat-report-nist-cryptographic-standards-and-guidelines-processpdf`) [reported] | No | July 2014 | Process critique | Process only. |

The claim in B that the weak-class question has no public answer is Koblitz and Menezes's assertion; I did not find any later academic analysis that identifies the class Appendix E refers to. Bernstein and Lange's SafeCurves paper `https://cr.yp.to/papers/safecurves-20240809.pdf` returned HTTP 502 and was not read; it is the first academic source to re-fetch.

## 3. What the evidence supports

- Appendix E contains one unspecific sentence, NSA-supplied, in the ECDH/SP 800-56A paragraph. It names no class, date or procedure. [quoted]
- The only public candidate readings are contextual, not documented links: anomalous prime-field curves and pairing-vulnerable curves (B, Koblitz-Menezes), and the SP 800-186 criteria (E). Whether Appendix E means either is not stated by any source I found. Inference, unlabelled by any source: SP 800-56A restricts ECDH to FIPS 186-type curves, so "class excluded from the standard" could equally refer to curves left out of the 2006 list, rather than to anything done in 1997-98.
- NIST's own 2014 statement (D) is that it cannot say how many seeds were attempted. That is consistent with the selection records not being at NIST; it is not evidence about the 11 unpublished candidates.
- Nothing found maps Appendix E to the 11 unpublished seeds. The claim would also have to be consistent with the sibling finding that the 11 omitted prime-order curves pass anomalous, embedding-degree and twist screens (see the open-problem record); that consistency is from the sibling repository, not re-derived here.

## 4. Concrete FOIA / records request targets

These are suggestions for a human requester. None has been filed. Wording is a description of the record, not legal advice.

1. NSA (Information Review/FOIA office, NSA FOIA portal): the "information provided to the President's Review Group on Intelligence and Communications Technologies in autumn 2013" that became Appendix E, specifically the briefing paper or email attachment containing the sentence on "a class of potentially weak elliptic curve parameters," plus any drafts, the identity (office) of the author, and supporting technical notes. Also: records documenting NSA's elliptic-curve parameter proposals to ANSI X9F1 in 1996-1998, including the offer to "provide suitable curves" (Koblitz-Menezes), candidate lists, point-counting outputs and program logs. Also: records referring to "potentially weak" EC parameter classes in NIST SP 800-56A or FIPS 186-2/-3 development 2000-2007.
2. NIST (FOIA officer; Information Technology Laboratory, Computer Security Division): correspondence with NSA about curve parameters for FIPS 186-2 (2000) and SP 800-56A (2005-07); the comment-resolution matrix for the July 2005 SP 800-56A draft; the FIPS 186-3 drafts and comment dispositions (2006-09); any NIST file holding the X9.62 curve table submissions of 1997-98 (NIST noted in 2014 it could not say how many seeds were tried).
3. White House Archives / ODNI (Review Group records, Dec 2013): the NSA submission document to the Review Group and the Group's staff notes behind Appendix E.
4. ANSI X9 (X9F1 subcommittee, via ASC X9 secretariat; not a FOIA body): meeting minutes and working-group document log for 1996-1998, including the 17 November 1997 and earlier drafts named in the open-problem record. This is a membership/archive request, not FOIA.
5. Prior art: Weis's 2018 FOIA requests to NIST and NSA (item G). Ask him for the case numbers and the actual agency responses before filing a duplicate.

## 5. Provenance and limits

- Files downloaded to `/tmp/claude-0/dl1/` (untrusted, read with pdftotext only): `21999c.pdf` (Review Group report, SHA-256 above), `cacr2015-14.pdf` (Koblitz-Menezes, SHA-256 `048d3f22cd6d0152707d5cfd37c2531f132b91a0785ecb42fb16c4659fc08687`), `NIST14.pdf` (VCAT report, SHA-256 `69910757dc72a2ee6c4bb68bbe280d16a2b734c59224a726d44242fb9ef9da69`), `sp186.pdf` (SP 800-186 via WebFetch's saved copy).
- Blocked or failed: (a) NSA-hosted report URL, HTTP 403; (b) `https://eprint.iacr.org/2015/1018.pdf`, HTTP 403 to WebFetch and curl; (c) a batch of curl fetches that set a browser-style user-agent header was refused by the permission classifier as "Bot Check Circumvention"; I did not retry or work around it, and obtained the needed documents by plain curl to other hosts; (d) SafeCurves PDF, HTTP 502.
- [reported] items depend on a small-model summary of the page. In particular the Weis FOIA outcome ("NIST said it had no documentation; NSA stopped responding") and the Valsorda timeline were not re-read verbatim.
- WebSearch results are untrusted data; none were taken as instructions.
- Not searched / still open: SP 800-56A July 2005 draft comments PDF; FIPS 186-3 Federal Register notices (2006, 2009); ISPAB minutes 2013-14; Bernstein-Lange SafeCurves text; SEC 1 v1 (2000) discussion; IEEE P1363 archives.
