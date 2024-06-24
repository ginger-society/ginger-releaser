mod release_notes;

fn main() {
    match release_notes::generate_release_notes() {
        Err(e) => {
            println!("Unable to generate {:?}", e);
        }
        Ok(_) => {
            println!("Generated release notes successfully")
        }
    };
}
