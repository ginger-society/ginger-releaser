use chrono::Utc;
use ginger_shared_rs::Version;
use git2::{Oid, Repository};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::exit;

pub fn generate_release_notes(
    git_url_prefix: &String,
    version: Version,
) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    let mut tags: HashMap<String, Oid> = HashMap::new();
    let mut release_notes: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    let mut tag_dates: HashMap<String, String> = HashMap::new();
    let mut sorted_tags: Vec<String> = vec![];
    // Get all tags in the repository and filter by semantic versioning
    for tag in repo.tag_names(None)?.iter() {
        match tag {
            Some(version) => {
                let tag_ref = format!("refs/tags/{}", &version);
                match repo.refname_to_id(&tag_ref) {
                    Ok(tag_id) => {
                        // Proceed to find the commit and its date
                        tags.insert(version.to_string(), tag_id);
                        match repo.find_commit(tag_id) {
                            Ok(commit) => {
                                let tag_date = commit.time().seconds();
                                let tag_datetime = chrono::DateTime::from_timestamp(tag_date, 0);
                                tag_dates.insert(
                                    version.to_string(),
                                    tag_datetime.unwrap().date_naive().to_string(),
                                );
                                sorted_tags.push(version.to_string());
                                release_notes.insert(version.to_string(), HashMap::new());
                            }
                            Err(e) => {
                                eprintln!("Failed to find commit for tag {}: {}", version, e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to get tag ID for {}: {}", version, e);
                    }
                }
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
        // let mut commit_messages: HashMap<String, Vec<String>> = HashMap::new();

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

            let prefix = message
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .to_string();

            let formatted_message = format!(
                " - [{}]({}{}) ({}) {}\n",
                &commit_hash[..10],
                git_url_prefix,
                commit_hash,
                author.name().unwrap_or_default(),
                message.replace("\n", "\n\t")
            );

            // commit_messages
            //     .get_mut(&prefix)
            //     .unwrap()
            //     .push(formatted_message);

            // release_notes
            //     .entry(tag_name)
            //     .or_insert_with(Vec::new)
            //     .push(formatted_message);

            release_notes
                .entry(tag_name.clone())
                .or_insert_with(HashMap::new)
                .entry(prefix)
                .or_insert_with(Vec::new)
                .push(formatted_message);
        }
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
        // let mut commit_messages = Vec::new();

        for commit_id in revwalk {
            let commit_id = commit_id?;
            let commit = repo.find_commit(commit_id)?;
            let commit_hash = commit.id().to_string();
            let author = commit.author();
            let message = commit.message().unwrap_or_default();
            let prefix = message
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .to_string();
            let formatted_message = format!(
                " - [{}]({}{}) ({}) {}\n",
                &commit_hash[..10],
                git_url_prefix,
                commit_hash,
                author.name().unwrap_or_default(),
                message.replace("\n", "\n\t")
            );

            release_notes
                .entry(version.formatted())
                .or_insert_with(HashMap::new)
                .entry(prefix)
                .or_insert_with(Vec::new)
                .push(formatted_message);

            // commit_messages.push(formatted_message);
        }

        // release_notes.insert(String::from(version.formatted()), commit_messages);
    }

    match File::create("CHANGELOG.md") {
        Ok(mut release_notes_file) => {
            match release_notes.get(&version.formatted()) {
                Some(notes) => {
                    let current_date_time = Utc::now();

                    match write!(
                        release_notes_file,
                        "## {} - {}\n",
                        String::from(version.formatted()),
                        current_date_time.date_naive()
                    ) {
                        Ok(()) => {}
                        Err(_) => exit(0),
                    };

                    for (section_heading, section_notes) in notes.iter() {
                        match write!(release_notes_file, "{}\n", section_heading) {
                            Ok(()) => {}
                            Err(_) => exit(0),
                        };
                        for note in section_notes {
                            match write!(release_notes_file, "{}", note) {
                                Ok(()) => {}
                                Err(_) => exit(0),
                            };
                        }
                    }
                }
                None => {}
            }

            for tag_name in sorted_tags.iter() {
                match release_notes.get(tag_name) {
                    Some(notes) => {
                        match write!(
                            release_notes_file,
                            "## {} - {}\n",
                            tag_name,
                            tag_dates.get(tag_name).unwrap()
                        ) {
                            Ok(()) => {}
                            Err(_) => exit(0),
                        };
                        for (section_heading, section_notes) in notes.iter() {
                            match write!(release_notes_file, "{}\n", section_heading) {
                                Ok(()) => {}
                                Err(_) => exit(0),
                            };
                            for note in section_notes {
                                match write!(release_notes_file, "{}", note) {
                                    Ok(()) => {}
                                    Err(_) => exit(0),
                                };
                            }
                        }
                    }
                    None => {}
                }
            }
        }
        Err(_) => exit(0),
    };

    Ok(())
}
