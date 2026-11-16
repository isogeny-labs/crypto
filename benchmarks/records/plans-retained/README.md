# Retained empty plan records

These exact source bytes describe plans with no captured launcher, raw data or metric. They are outside the flat execution-record index read by benchrec. They remain open historical metadata, not completed or censored executions. No command or result is fabricated to close them.

The November15 genus-ladder plan was restored by PR4394 after an earlier cleanup had removed it. Retain it here while keeping the active execution ledger and its derived progress report consistent.
