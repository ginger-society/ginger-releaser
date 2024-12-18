use std::{fs, path::Path};

use ginger_shared_rs::{FileType, ReleaserConfig};

use crate::utils::{update_json, update_py, update_toml};

pub fn update_references(config: &ReleaserConfig) {
    for reference in &config.references {
        let mut contents = fs::read_to_string(&reference.file_name).unwrap();
        let var_name = &reference.variable;

        let extension = Path::new(&reference.file_name)
            .extension()
            .and_then(|ext| ext.to_str());

        let updated_content = match FileType::from_extension(extension) {
            FileType::Py => update_py(
                &mut contents,
                &config.version,
                &var_name,
                &reference.output_type,
            )
            .unwrap(),
            FileType::Toml => update_toml(&mut contents, &config.version, &var_name).unwrap(),
            FileType::Json => update_json(&mut contents, &config.version, &var_name).unwrap(),
            FileType::Unknown => {
                println!(
                    "Unknown file type encountered {}, cannot update. However, continuing to update other possible references.",
                     reference.file_name
                );
                continue;
            }
        };

        fs::write(&reference.file_name, updated_content).unwrap();
    }
}
