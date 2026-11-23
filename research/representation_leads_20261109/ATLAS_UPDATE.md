# Atlas update to apply (crypto-autoresearcher, 2026-11-14)

The atlas repository encrypts tracked files (vault, AES-256-SIV, commit f40008bd); the session
that wrote this had no vault key, so these edits are recorded here for someone who has one.
Apply to `knowledge/leads/ecdlp/ic-engine/` and run `python3 tools/lead_atlas.py --check`.

## LEAD-ICE-subspace-frobenius-chain-mod-n: open_blocked -> closed_obstructed

Replace `disposition` and `next_action` with:

```yaml
disposition: closed_obstructed
obstruction:
  kind: literature
  statement: >-
    Dominated inside Huang-Kosters-Yang-Yeo (arXiv:1505.02532, Thm 1.1): for a zero-dimensional
    radical big-field system in M variables with at most s solutions over the closure, the last
    fall degree of its full Weil descent is at most max(tau(max(d_F, deg F, (M+1)s, 1), q, M),
    M tau(2s, q, 1), q), independent of n; Sec. 4.4 states (without proof) the same with added
    constraints of small Frobenius weight, "such as linear subspace constraints". L_V is
    linearized (weight 1), so the standard system (S_(m+1) plus L_V(x_i) = 0, M = m) already lies
    in that scope; the squaring chain lowers only the ordinary degree, which the bound does not
    use, and adds l m big-field variables, which enter it linearly. Scope: the subspace case is a
    remark there, and d_F is bounded for neither system.
  revisit_when: >-
    A measured last fall or closure degree for the chain encoding below that of the standard
    descended system at equal (n, m, l), or a published bound in ordinary degree that the chain
    improves.
```

Set `last_reviewed: '2026-11-14'`.

## LEAD-ICE-quadratised-chain-representation: open_next_step -> open_blocked

```yaml
disposition: open_blocked
next_action:
  step: >-
    Run the frozen research/representation_leads_20261109/PROTOCOL.md (RUN_CARD.md): closure
    refutation degree of two matched pairs with identical variable sets (mu = e2 vs w = x_a x_b;
    chain S_3 plus the implied trace equations vs chain S_3) on 8 cells (n, m, l) in {(11,3,3),
    (11,3,4), (13,3,3), (11,4,3)} x Koblitz a in {0, 1}, 16 exhaustively unsatisfiable and 4
    planted draws each, with examples/replead_closure.rs on 640 systems pinned by sha256.
  discriminates: >-
    A candidate refuting at a strictly lower degree than its matched control on >= 12/16 draws in
    every m >= 4 cell is a representation effect to price at m >= 5; equal degree on >= 12/16 in
    every cell closes the lead as a relabelled degree reduction (resp. a redundant equation).
  blocked_by: resume checkpoint credential (CHECKPOINT_UPLOAD_TOKEN) for any experiment run
```

Add evidence `{repo: crypto, path: research/representation_leads_20261109/PROTOCOL.md, note:
frozen 2026-11-14 with instrument binding}`, note on REPORT.pdf that the lambda part is the
Lenstra-Kosters trace homomorphism, and set `last_reviewed: '2026-11-14'`.
