use ginger_shared_rs::{
    read_package_metadata_file, read_service_config_file, utils::get_token_from_file_storage,
    ReleaserConfig,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    process::exit,
};
use MetadataService::{
    apis::default_api::{
        metadata_create_snapshot, metadata_get_dbschemas_and_tables,
        metadata_get_services_and_envs, metadata_get_user_packages, MetadataCreateSnapshotParams,
        MetadataGetDbschemasAndTablesParams, MetadataGetServicesAndEnvsParams,
        MetadataGetUserPackagesParams,
    },
    get_configuration,
    models::CreateSnapshotRequest,
};

// Struct to hold the snapshot data
#[derive(Serialize, Deserialize, Debug)]
struct Snapshot {
    services: Vec<ServiceSnapshot>,
    packages: Vec<PackageSnapshot>,
    databases: Vec<DatabaseSnapshot>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServiceSnapshot {
    identifier: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PackageSnapshot {
    identifier: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DatabaseSnapshot {
    name: String,
    version: String,
}

pub async fn generate_snapshot(config: &ReleaserConfig) {
    let token = get_token_from_file_storage();
    let metadata_config = get_configuration(Some(token));

    let service_config = read_service_config_file(Path::new("services.toml")).unwrap();
    let mut services = Vec::new();
    let mut packages = Vec::new();
    let mut databases = Vec::new();

    // Fetch and process services
    match metadata_get_services_and_envs(
        &metadata_config,
        MetadataGetServicesAndEnvsParams {
            org_id: service_config.organization_id.clone(),
            page_number: None,
            page_size: None,
        },
    )
    .await
    {
        Ok(service_responses) => {
            for service in service_responses {
                if let Some(stage_env) = service.envs.iter().find(|env| env.env_key == "stage") {
                    if let Some(Some(version)) = &stage_env.version {
                        let identifier =
                            format!("@{}/{}", service_config.organization_id, service.identifier);
                        services.push(ServiceSnapshot {
                            identifier,
                            version: version.clone(),
                        });
                    }
                }
            }
        }
        Err(_) => {
            println!("Failed to fetch services");
        }
    }

    // Fetch and process packages
    match metadata_get_user_packages(
        &metadata_config,
        MetadataGetUserPackagesParams {
            org_id: service_config.organization_id.clone(),
            env: "stage".to_string(),
        },
    )
    .await
    {
        Ok(package_responses) => {
            for package in package_responses {
                packages.push(PackageSnapshot {
                    identifier: format!(
                        "@{}/{}",
                        service_config.organization_id,
                        package.identifier.clone()
                    ),
                    version: package.version.clone(),
                });
            }
        }
        Err(_) => {
            println!("Failed to fetch packages");
        }
    };

    // Fetch and process databases
    match metadata_get_dbschemas_and_tables(
        &metadata_config,
        MetadataGetDbschemasAndTablesParams {
            org_id: service_config.organization_id.clone(),
            env: "stage".to_string(),
        },
    )
    .await
    {
        Ok(db_schema_responses) => {
            for db_schema in db_schema_responses {
                if let Some(Some(version)) = db_schema.version {
                    databases.push(DatabaseSnapshot {
                        name: format!(
                            "@{}/{}",
                            service_config.organization_id,
                            db_schema.name.clone()
                        ),
                        version: version.clone(),
                    });
                }
            }
        }
        Err(_) => {
            println!("Failed to fetch databases");
        }
    }

    // Create the snapshot structure
    let snapshot = Snapshot {
        services,
        packages,
        databases,
    };

    let package_metadata = read_package_metadata_file("metadata.toml").unwrap();

    let links_str = serde_json::to_string(&package_metadata.links).unwrap();

    match metadata_create_snapshot(
        &metadata_config,
        MetadataCreateSnapshotParams {
            create_snapshot_request: CreateSnapshotRequest {
                version: config.version.formatted(),
                org_id: service_config.organization_id.clone(),
                infra_repo_origin: config.settings.git_url_prefix.clone().unwrap(),
                quick_links: links_str,
            },
        },
    )
    .await
    {
        Ok(_) => {
            println!("Snapshot created on the dev portal")
        }
        Err(e) => {
            println!("{:?}", e);
            println!("Error creating snapshot on the portal");
            exit(1);
        }
    }

    // Create the .snapshots directory if it doesn't exist
    create_dir_all("snapshots").unwrap();

    // Save the snapshot to .snapshots/data.json
    let json_data = serde_json::to_string_pretty(&snapshot).unwrap();
    let mut file = File::create(format!("snapshots/{}.json", config.version.formatted())).unwrap();
    file.write_all(json_data.as_bytes()).unwrap();

    println!("Snapshot saved to snapshots");
}
