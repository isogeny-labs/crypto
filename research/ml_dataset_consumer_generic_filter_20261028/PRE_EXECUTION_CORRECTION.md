# Pre-execution wire correction

The first frozen config files accidentally included a descriptive schema field.
Production CaseConfig has no schema member. Removed only that field and refreshed
the config hashes before any fixture generation. All 40 mathematical parameters,
seeds, indices, limits, policies and family/twist choices are unchanged. The first
freeze remains in commit1202bf89359. No result informed this correction.
