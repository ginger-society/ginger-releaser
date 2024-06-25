use chrono::{Datelike, Local}; // Import chrono to handle date and time
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
    let mut sorted_tags: Vec<(String, chrono::NaiveDateTime)> = vec![]; // Change sorted_tags to hold datetime too

    // Get all tags in the repository and filter by semantic versioning
    for tag in repo.tag_names(None)?.iter() {
        match tag {
            Some(version) => {
                let tag_name = version.to_string().clone();
                let tag_id = repo.refname_to_id(format!("refs/tags/{}", &tag_name).as_str())?;
                tags.insert(tag_name.clone(), tag_id);

                // Get tag date
                let tag_date = repo.find_commit(tag_id)?.time().seconds();
                let tag_datetime = chrono::NaiveDateTime::from_timestamp(tag_date, 0);

                sorted_tags.push((tag_name.clone(), tag_datetime));
                release_notes.insert(tag_name.clone(), Vec::new());
            }
            None => {}
        }
    }

    sorted_tags.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by datetime in descending order

    // Iterate over tags and collect commit messages
    for (i, (tag_name, _)) in sorted_tags.iter().enumerate() {
        let mut revwalk = repo.revwalk()?;
        let mut commit_messages = Vec::new();

        if i < sorted_tags.len() - 1 {
            let next_tag_name = &sorted_tags[i + 1].0;

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

    match File::create("CHANGELOG.md") {
        Ok(mut release_notes_file) => {
            for (tag_name, tag_datetime) in sorted_tags.iter() {
                match release_notes.get(tag_name) {
                    Some(notes) => {
                        match write!(
                            release_notes_file,
                            "## {} ({})\n",
                            String::from(tag_name),
                            tag_datetime.date() // Get date part only
                        ) {
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
                    None => {}
                }
            }
        }
        Err(_) => exit(0),
    };

    Ok(())
}
