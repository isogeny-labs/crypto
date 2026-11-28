# Sources and construction scope

The source review began before implementation. Published inputs are pinned in `UPSTREAM_INPUTS.json`; no shared KB MCP is enabled, and the fallback wrapper enters the locked VLT1 volume, which remains untouched. No novelty or absence conclusion follows from that access gap.

The actual curve, covers, polarized Prym identification and integral half map are in crypto #5114 at bbc546251822d3c9cca73e8c51446de098d2a956: `ACTUAL_GENUS2_PRYM_PROOF_D.md` and `GLOBAL_RETRACTION_SHARPENING_E.md`. Canonical inert-sextic arithmetic and the complete original class data are in cryptanalysis #1964 at da07dc2ae4d92567ade4e6c8c4ab9e2c35b61fe2: `ACTUAL_DIVISOR_ARITHMETIC_PROOF_A.md`. These are unsigned producer sources with the recorded formal/custody gaps, not independent verdicts.

Primary mathematical sources already read for this project:

- Galbraith, corrected *Mathematics of Public Key Cryptography*, chapter 10, https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch10.pdf : full affine composition proof (10.3.14), reduction and degree bounds. Apply the affine identities with equal infinity counterweights; the odd-degree model theorem does not directly apply to this sextic.
- Galbraith--Harrison--Mireles Morales, *Efficient Hyperelliptic Arithmetic Using Balanced Representation for Divisors*, ANTS VIII, https://cspages.ucalgary.ca/~rscheidl/pims/handouts/GalbraithHarrisonMorales.pdf : sections 2--4, especially the even-genus nonsplit-infinity specialization on page 352. This supplies established balanced arithmetic, extended here to a composition of two actual effective cubic fibres.
- Shaska, arXiv:math/0109155v1, https://arxiv.org/pdf/math/0109155 : section 2.1, equations (2),(5),(7), Lemma 3 and its proof; the alternate degeneration in section 2.2. Its classification assumes algebraically closed characteristic zero. Use it as prior architecture only; exact F7 coefficient identities, coprimality, separability and proper extensions must establish this particular cover independently.
- Pollard's rho with negation and r-adding/Floyd controls is an existing generic method. The frozen implementation uses the identical eight-jump engine for the elliptic and Jacobian groups, charges table construction and all restarts, and validates recovery using only public coordinate inputs. No new solver or strongest-external-backend result is asserted.

Cantor's AMS primary PDF and Kuhn's 1988 primary were inaccessible in earlier review. Do not cite their unread text as evidence. Accessible complete affine/balanced proofs above establish the arithmetic contract. Generic Lean geometry remains open; finite checks and coefficient certificates will not substitute for it.
