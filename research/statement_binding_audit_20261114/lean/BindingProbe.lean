module

public import HeaderControls
public import Lean

public section

open Lean Matrix Polynomial

namespace StatementBindingAudit

partial def theoremNodes (s : Syntax) : Array Syntax :=
  if s.isOfKind ``Lean.Parser.Command.theorem then #[s]
  else s.getArgs.foldl (fun found child => found ++ theoremNodes child) #[]

def require {α : Type} (x : Except String α) : IO α :=
  match x with
  | .ok a => pure a
  | .error e => throw <| IO.userError e

def stringField (j : Json) (key : String) : IO String := require (j.getObjValAs? String key)

def normalizeWhitespace (s : String) : String :=
  String.intercalate " " (((String.ofList (s.toList.map (fun c => if c.isWhitespace then ' ' else c))).splitOn " ").filter
    (fun word => !word.isEmpty))

def restrictedNormalizationSupported (s : String) : Bool :=
  !s.contains '"' && (s.splitOn "/-").length=1 && (s.splitOn "--").length=1

def sha256File (path : String) : IO String := do
  let r ← IO.Process.output {cmd := "sha256sum",args := #[path]}
  unless r.exitCode=0 do throw <| IO.userError s!"sha256sum failed: {r.stderr}"
  let hash := (r.stdout.splitOn " ").head!
  unless hash.toList.length=64 do throw <| IO.userError "invalid SHA256 output"
  pure hash

def hashText (directory caseId suffix text : String) : IO String := do
  let path := s!"{directory}/{caseId}-{suffix}.txt"
  IO.FS.writeFile path text
  sha256File path

def parseFile (context : Parser.ParserModuleContext) (path : String) : IO (Array Json) := do
  let source ← IO.FS.readFile path
  let ctx := Parser.mkInputContext source path
  let (_, initial, headerMessages) ← Parser.parseHeader ctx
  let mut results := #[]
  if headerMessages.hasErrors then
    results := results.push <| Json.mkObj [("file",toJson path),("header_error",toJson true)]
  let mut state := initial
  let mut currentNamespace := Name.anonymous
  let mut scopes : Array Name := #[]
  repeat
    let (command, next, messages) := Parser.parseCommand ctx
      {context with currNamespace := currentNamespace} state {}
    for t in theoremNodes command do
      let signature := t[2]
      match t.getPos?, signature.getTailPos? with
      | some start, some finish =>
        results := results.push <| Json.mkObj [("file",toJson path),
          ("decl_id",toJson t[1][0].getId.toString),
          ("qualified_decl_id",toJson (currentNamespace ++ t[1][0].getId).toString),
          ("signature",toJson ((ctx.substring start finish).toString)),
          ("start_byte",toJson start.byteIdx),("end_byte",toJson finish.byteIdx),
          ("command_parse_error",toJson messages.hasErrors)]
      | _,_ =>
        results := results.push <| Json.mkObj [("file",toJson path),("missing_range",toJson true)]
    if command.isOfKind ``Lean.Parser.Command.eoi then break
    if command.isOfKind ``Lean.Parser.Command.namespace then
      scopes := scopes.push currentNamespace
      currentNamespace := currentNamespace ++ command[1].getId
    else if command.isOfKind ``Lean.Parser.Command.section then
      scopes := scopes.push currentNamespace
    else if command.isOfKind ``Lean.Parser.Command.end then
      if let some previous := scopes.back? then
        currentNamespace := previous
        scopes := scopes.pop
    if next.pos ≤ state.pos then throw <| IO.userError s!"parser made no progress in {path}"
    state := next
  pure results

def auditManifest (context : Parser.ParserModuleContext) (manifestPath : String) : IO Unit := do
  let manifest ← require <| Json.parse (← IO.FS.readFile manifestPath)
  let cases ← require <| manifest.getObjValAs? (Array Json) "cases"
  let directory := manifestPath ++ ".hashes"
  IO.FS.createDirAll directory
  let mut outcomes : Array Json := #[]
  let mut unresolved := 0
  for c in cases do
    let caseId ← stringField c "case_id"
    let name ← stringField c "theorem"
    let expectedHash ← stringField c "expected_decl_sha256"
    let candidates ← require <| c.getObjValAs? (Array Json) "candidates"
    let mut found : Array Json := #[]
    let mut issues : Array Json := #[]
    for candidate in candidates do
      let path ← stringField candidate "export_path"
      let expectedSourceHash ← stringField candidate "sha256"
      let actualSourceHash ← sha256File path
      let declaredSourceHash := (candidate.getObjValAs? String "declared_source_sha256").toOption
      if actualSourceHash != expectedSourceHash ||
          declaredSourceHash.any (fun h => h != actualSourceHash) then
        issues := issues.push <| Json.mkObj [("file",toJson path),("source_hash_mismatch",toJson true)]
        continue
      for header in (← parseFile context path) do
        if (header.getObjValAs? Bool "header_error").toOption.getD false ||
            (header.getObjValAs? Bool "missing_range").toOption.getD false then
          issues := issues.push header
        if let some declaredName := (header.getObjValAs? String "qualified_decl_id").toOption then
          if declaredName = name then
            if (header.getObjValAs? Bool "command_parse_error").toOption.getD true then
              issues := issues.push header
            else found := found.push header
    let baseFields := [("case_id",toJson caseId),("theorem",toJson name),
      ("source_plan",← c.getObjVal? "source_plan" |> require),("expected_decl_sha256",toJson expectedHash)]
    if found.size != 1 || !issues.isEmpty then
      unresolved := unresolved+1
      outcomes := outcomes.push <| Json.mkObj (baseFields ++ [("status",toJson "unsupported_source_or_parse"),
        ("matches",toJson found.size),("issues",toJson issues)])
      continue
    let header := found[0]!
    let signature ← stringField header "signature"
    if !restrictedNormalizationSupported signature then
      unresolved := unresolved+1
      outcomes := outcomes.push <| Json.mkObj (baseFields ++ [("status",toJson "unsupported_normalization"),("parsed_header",header)])
      continue
    let full := normalizeWhitespace signature
    let fullHash ← hashText directory caseId "full" full
    let recorded := (c.getObjValAs? String "recorded_statement").toOption
    let mut status := ""
    let mut recordedHash := ""
    let mut omittedSuffix := ""
    match recorded with
    | some statement =>
      recordedHash ← hashText directory caseId "recorded" statement
      if recordedHash != expectedHash then
        status := "recorded_hash_mismatch"
        unresolved := unresolved+1
      else if full = statement then status := "complete_under_frozen_normalization"
      else if full.startsWith statement then
        status := "truncated_signature_prefix"
        omittedSuffix := String.ofList (full.toList.drop statement.toList.length)
      else
        status := "signature_text_mismatch"
        unresolved := unresolved+1
    | none =>
      if fullHash=expectedHash then status := "complete_hash_match_without_recorded_text"
      else
        status := "missing_recorded_text_hash_diff"
        unresolved := unresolved+1
    outcomes := outcomes.push <| Json.mkObj (baseFields ++ [("status",toJson status),
      ("parsed_header",header),("full_normalized_signature",toJson full),("full_decl_sha256",toJson fullHash),
      ("recorded_statement",toJson recorded),("recorded_text_sha256",toJson recordedHash),
      ("full_characters",toJson full.toList.length),("omitted_suffix",toJson omittedSuffix)])
  for outcome in outcomes do IO.println outcome.compress
  let count (status : String) := (outcomes.filter (fun j => (j.getObjValAs? String "status").toOption=some status)).size
  IO.println <| (Json.mkObj [("summary",toJson true),("cases",toJson cases.size),
    ("complete",toJson (count "complete_under_frozen_normalization"+count "complete_hash_match_without_recorded_text")),
    ("truncated_prefixes",toJson (count "truncated_signature_prefix")),("unresolved",toJson unresolved),
    ("audit_complete",toJson (unresolved==0))]).compress
  unless unresolved=0 do throw <| IO.userError s!"{unresolved} unresolved cases; bounded audit not complete"

end StatementBindingAudit

run_cmd do
  if let some path ← IO.getEnv "STATEMENT_BINDING_INPUT" then
    let env ← getEnv
    let options ← getOptions
    let currentNamespace ← getCurrNamespace
    let declarations ← getOpenDecls
    let context : Parser.ParserModuleContext := {env,options,currNamespace := currentNamespace,openDecls := declarations}
    if path.endsWith ".json" then
      let _ ← StatementBindingAudit.auditManifest context path
      pure ()
    else
      let headers ← StatementBindingAudit.parseFile context path
      for header in headers do
        IO.println header.compress
        if (header.getObjValAs? Bool "command_parse_error").toOption.getD true then
          throwError "parser control failed"
