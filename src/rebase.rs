use anyhow::{anyhow, Result};

use std::collections::HashSet;

// expects any fixup! commits starting from HEAD,
// tries to find the earliest target.
pub fn find_last_fixup_target(repo: &git2::Repository) -> Result<git2::Commit> {
    let mut fixup_targets: HashSet<String> = HashSet::new();
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid_res in revwalk {
        let oid = oid_res?;
        let commit = repo.find_commit(oid)?;

        let id = &commit.id().to_string();
        let summary = commit.summary().unwrap_or("");
        let prefix = "fixup! ";

        if summary.starts_with(prefix) {
            let dest_commit_locator = summary.split_at(prefix.len()).1;
            fixup_targets.insert(dest_commit_locator.to_string());
        }

        fixup_targets.remove(summary);
        fixup_targets.remove(id);

        if fixup_targets.is_empty() {
            return Ok(commit);
        }
    }

    return Err(anyhow!("did not find all fixup targets"));
}

pub fn find_last_fixup_target_old(repo: &git2::Repository) -> Result<git2::Commit> {
    let mut fixup_targets: HashSet<String> = HashSet::new();
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid_res in revwalk.by_ref() {
        let oid = oid_res?;
        let commit = repo.find_commit(oid)?;

        let summary = commit.summary().unwrap_or("");
        let prefix = "fixup! ";

        if summary.starts_with(prefix) {
            let dest_commit_locator = summary.split_at(prefix.len()).1;
            fixup_targets.insert(dest_commit_locator.to_string());
        } else {
            revwalk.reset()?;
            revwalk.push(oid)?;
            break;
        }
    }

    if fixup_targets.is_empty() {
        return Err(anyhow!("HEAD commit not a fixup!"));
    }

    for oid_res in revwalk {
        let oid = oid_res?;
        let commit = repo.find_commit(oid)?;

        let summary = commit.summary().unwrap_or("");
        let id = &commit.id().to_string();

        fixup_targets.remove(summary);
        fixup_targets.remove(id);

        if fixup_targets.is_empty() {
            return Ok(commit);
        }
    }

    return Err(anyhow!("did not find all fixup targets"));
}
