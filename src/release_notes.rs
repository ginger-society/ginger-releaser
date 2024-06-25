use git2::{Oid, Repository};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::exit;

use crate::utils::Version;

pub fn generate_release_notes(
    git_url_prefix: &String,
    version: Version,
) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    let mut tags: HashMap<String, Oid> = HashMap::new();
    let mut release_notes: HashMap<String, Vec<String>> = HashMap::new();
    let mut sorted_tags: Vec<String> = vec![];
    // Get all tags in the repository and filter by semantic versioning
    for tag in repo.tag_names(None)?.iter() {
        match tag {
            Some(version) => {
                tags.insert(
                    version.to_string().clone(),
                    repo.refname_to_id(format!("refs/tags/{}", &version).as_str())?,
                );
                sorted_tags.push(version.to_string().clone());
                release_notes.insert(version.to_string().clone(), Vec::new());
            }
            None => {}
        }
    }

    sorted_tags.sort_by(|a, b| {
        let semver_a = Version::from_str(a);
        let semver_b = Version::from_str(b);
        semver_b.cmp(&semver_a)
    });

    // Iterate over tags and collect commit messages
    for (i, tag_name) in sorted_tags.iter().enumerate() {
        let mut revwalk = repo.revwalk()?;
        let mut commit_messages = Vec::new();

        if i < sorted_tags.len() - 1 {
            let next_tag_name = &sorted_tags[i + 1];

            let start_commit = repo.find_commit(*tags.get(next_tag_name).unwrap())?;

            let end_commit = repo.find_commit(*tags.get(tag_name).unwrap())?;

            revwalk.push(end_commit.id())?;
            revwalk.hide(start_commit.id())?;
        } else {
            let end_commit = repo.find_commit(*tags.get(tag_name).unwrap())?;
            revwalk.push(end_commit.id())?;
        }

        for commit_id in revwalk {
            let commit_id = commit_id?;
            let commit = repo.find_commit(commit_id)?;
            let commit_hash = commit.id().to_string();
            let author = commit.author();
            let message = commit.message().unwrap_or_default();
            let formatted_message = format!(
                " - [{}]({}{}) ({}) {}\n",
                &commit_hash[..10],
                git_url_prefix,
                commit_hash,
                author.name().unwrap_or_default(),
                message.replace("\n", "\n\t")
            );

            commit_messages.push(formatted_message);
        }

        release_notes
            .get_mut(tag_name)
            .unwrap()
            .extend(commit_messages);
    }

    // Collect commits since the last tag under "Unreleased commits"
    let mut revwalk = repo.revwalk()?;
    let head = repo.head()?;

    // Get the OID (commit ID) of the HEAD commit
    let head_oid = head
        .target()
        .ok_or_else(|| git2::Error::from_str("No commit in HEAD"))?;

    // Find the commit object using the OID
    let last_commit = repo.find_commit(head_oid)?;

    if let Some(last_tag_name) = sorted_tags.first().cloned() {
        let end_commit = repo.find_commit(*tags.get(&last_tag_name).unwrap())?;
        revwalk.push(last_commit.id())?;
        revwalk.hide(end_commit.id())?;
        let mut commit_messages = Vec::new();

        for commit_id in revwalk {
            let commit_id = commit_id?;
            let commit = repo.find_commit(commit_id)?;
            let commit_hash = commit.id().to_string();
            let author = commit.author();
            let message = commit.message().unwrap_or_default();

            let formatted_message = format!(
                " - [{}]({}{}) ({}) {}\n",
                &commit_hash[..10],
                git_url_prefix,
                commit_hash,
                author.name().unwrap_or_default(),
                message.replace("\n", "\n\t")
            );

            commit_messages.push(formatted_message);
        }

        release_notes.insert(String::from(version.formatted()), commit_messages);
    }

    match File::create("CHANGELOG.md") {
        Ok(mut release_notes_file) => {
            for (tag_name, notes) in release_notes.iter() {
                match write!(release_notes_file, "## {}\n", tag_name) {
                    Ok(()) => {}
                    Err(_) => exit(0),
                };
                for note in notes {
                    match write!(release_notes_file, "{}", note) {
                        Ok(()) => {}
                        Err(_) => exit(0),
                    };
                }
            }
        }
        Err(_) => exit(0),
    };

    Ok(())
}
