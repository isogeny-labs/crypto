-- Document rendering only: make relative source/evidence links usable in PDFs.
-- The report source itself retains repository-relative links for GitHub.
local source_ref = os.getenv("RR5_SOURCE_REF") or "5e536ed93302def2122968aa880b22c4b675d516"

function Link(link)
  if link.target:match("^[%a][%w+.-]*:") or link.target:match("^#") then
    return link
  end
  local parts = {"research", "prime31_four_summand_20261122", "scaling-b677"}
  for part in link.target:gmatch("[^/]+") do
    if part == ".." then
      assert(#parts > 0, "report link leaves the repository")
      table.remove(parts)
    elseif part ~= "." then
      table.insert(parts, part)
    end
  end
  link.target = "https://github.com/isogeny-labs/crypto/blob/" .. source_ref .. "/" .. table.concat(parts, "/")
  return link
end
