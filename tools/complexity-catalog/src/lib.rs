//! Source-grounded research retrieval; records are not proof or execution receipts.
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Paper {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub url: String,
    pub pdf: String,
    pub pdf_sha256: String,
    pub source_review: String,
    pub learning: String,
    pub assumptions: Vec<String>,
    pub integration: Vec<String>,
    pub status: String,
    pub reported_evidence: String,
    pub local_formal_verification: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub schema: String,
    pub retrieved_at: String,
    pub source: String,
    pub expected_total: usize,
    pub pagination_complete: bool,
    pub papers: Vec<Paper>,
}

/// Load the frozen snapshot, not the current live subject page.
pub fn catalog() -> Result<Catalog, String> {
    let value: Catalog = serde_json::from_str(include_str!(
        "../../../research/hexagon_cscc_20261030/catalog.json"
    ))
    .map_err(|e| e.to_string())?;
    value.validate()?;
    Ok(value)
}

impl Catalog {
    /// Detect incomplete ingestion, duplicate versions, lost assumptions and
    /// accidental promotion of literature into locally checked formal proofs.
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != "hexagon-complexity-catalog/v1"
            || !self.pagination_complete
            || self.papers.len() != self.expected_total
            || self.expected_total == 0
        {
            return Err("schema or complete subject coverage is missing".into());
        }
        let mut ids = BTreeSet::new();
        for paper in &self.papers {
            if !ids.insert(&paper.id) {
                return Err(format!("duplicate work: {}", paper.id));
            }
            let versioned = paper
                .id
                .strip_prefix("hexagon:")
                .ok_or("invalid Hexagon identifier")?;
            if !versioned.contains('v')
                || !paper.url.ends_with(versioned)
                || !paper.pdf.ends_with(versioned)
                || !paper.url.starts_with("https://hexagonmath.org/")
                || !paper.pdf.starts_with("https://hexagonmath.org/pdf/")
            {
                return Err(format!("unbound source/version: {}", paper.id));
            }
            if paper.pdf_sha256.len() != 64
                || !paper
                    .pdf_sha256
                    .bytes()
                    .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
            {
                return Err(format!("missing retrieved-PDF digest: {}", paper.id));
            }
            if paper.learning.trim().is_empty()
                || paper.assumptions.is_empty()
                || paper.integration.is_empty()
                || paper.reported_evidence.trim().is_empty()
                || paper.source_review != "statements_and_limits"
                || paper.local_formal_verification != "not_run"
            {
                return Err(format!(
                    "missing scope or unsupported proof promotion: {}",
                    paper.id
                ));
            }
            if ![
                "implemented_reduction",
                "reference_backend",
                "existing_backend_context",
                "research_constraint",
                "bounded_control",
                "cross_domain_constraint",
            ]
            .contains(&paper.status.as_str())
            {
                return Err(format!("unknown integration status: {}", paper.id));
            }
        }
        let inventory: serde_json::Value = serde_json::from_str(include_str!(
            "../../../research/hexagon_cscc_20261030/inventory.json"
        ))
        .map_err(|e| e.to_string())?;
        let source_ids = inventory["papers"]
            .as_array()
            .ok_or("missing inventory")?
            .iter()
            .map(|p| p["id"].as_str().ok_or("invalid inventory identifier"))
            .collect::<Result<BTreeSet<_>, _>>()?;
        let actual_ids: BTreeSet<_> = self.papers.iter().map(|p| p.id.as_str()).collect();
        if source_ids != actual_ids {
            return Err("catalog does not match the frozen subject inventory".into());
        }
        for paper in &self.papers {
            let source = inventory["papers"]
                .as_array()
                .ok_or("missing inventory")?
                .iter()
                .find(|p| p["id"].as_str() == Some(paper.id.as_str()))
                .ok_or("missing source work")?;
            let authors = serde_json::to_value(&paper.authors).map_err(|e| e.to_string())?;
            if source["title"].as_str() != Some(paper.title.as_str())
                || source["url"].as_str() != Some(paper.url.as_str())
                || source["pdf"].as_str() != Some(paper.pdf.as_str())
                || source["pdf_sha256"].as_str() != Some(paper.pdf_sha256.as_str())
                || source["authors"] != authors
            {
                return Err(format!("catalog source metadata drift: {}", paper.id));
            }
        }
        Ok(())
    }

    /// Case-insensitive retrieval across titles, source claims, assumptions and
    /// implementation paths. Matching a paper never changes its evidence status.
    pub fn search(&self, query: &str) -> Vec<&Paper> {
        let words: Vec<_> = query.split_whitespace().map(str::to_lowercase).collect();
        self.papers
            .iter()
            .filter(|paper| {
                let text = serde_json::to_string(paper)
                    .unwrap_or_default()
                    .to_lowercase();
                words.iter().all(|word| text.contains(word))
            })
            .collect()
    }
}

#[cfg(test)]
mod controls;
#[cfg(test)]
mod prime_ic;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_subject_snapshot_covers_every_version_including_cross_lists() {
        let c = catalog().unwrap();
        assert_eq!(c.papers.len(), 22);
        assert_eq!(c.search("").len(), 22);
        assert_eq!(c.papers.iter().filter(|p| p.id.ends_with("v2")).count(), 3);
        assert_eq!(c.search("2610.00301v2").len(), 1);
        assert_eq!(c.search("2610.00301v1").len(), 0);
        assert!(c.search("locality").len() >= 2);
    }

    #[test]
    fn incomplete_duplicate_and_promoted_records_are_rejected() {
        let c = catalog().unwrap();
        let mut bad = c.clone();
        bad.papers.pop();
        assert!(bad.validate().is_err());
        let mut bad = c.clone();
        bad.papers[1] = bad.papers[0].clone();
        assert!(bad.validate().is_err());
        let mut bad = c.clone();
        bad.papers[0].local_formal_verification = "checked".into();
        assert!(bad.validate().is_err());
        let mut bad = c;
        bad.papers[0].pdf_sha256 = "unretrieved".into();
        assert!(bad.validate().is_err());
        bad.papers[0].pdf_sha256 = "f".repeat(64);
        assert!(bad.validate().is_err());
    }

    #[test]
    fn integration_pointers_resolve_in_the_owning_repository() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        for p in catalog().unwrap().papers {
            for path in p.integration {
                assert!(root.join(&path).exists(), "{}: {}", p.id, path);
            }
        }
    }
}
