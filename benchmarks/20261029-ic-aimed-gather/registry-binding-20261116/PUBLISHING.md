# Prospective report publication

The new self-contained validator report is171195 bytes, SHA256`c355a3414f75b2796d24ee9c48824de46e58c583d9bc4ab7212e651fc7630f6e`. Its destination is the repository's existing content-addressed artifact store. The initial candidate commit supplies the data bytes to the trusted artifact-autofix workflow; the manifest is a locator, not a claim that upload already succeeded.

The trusted base tool uploads and independently replays remote bytes before its pointer-only commit removes the Git payload. The report must be hydrated before the validation gate runs. The workflow declaration derives its pull prefix from the new manifest. Publication is pending until that trusted migration commit and current-head CI complete. Original reports and stored objects are unchanged.
