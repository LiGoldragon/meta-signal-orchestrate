use schema::{Leg, LoadedSchema, RouteBody};
use std::path::PathBuf;

fn schema_file() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("schema")
        .join("owner-signal-orchestrate.concept.schema")
}

#[test]
fn owner_signal_orchestrate_concept_schema_lowers_owner_routes() {
    let loaded =
        LoadedSchema::read_path(schema_file()).expect("owner-signal-orchestrate schema reads");
    let assembled = loaded.assembled();

    assert_eq!(assembled.routes().len(), 5);

    let create = assembled
        .route_for_short_header(Leg::Owner, u64::from_le_bytes([0, 0, 0, 0, 0, 0, 0, 0]))
        .expect("create route");
    assert_eq!(create.root().as_str(), "Create");
    assert_eq!(create.endpoint().name().as_str(), "CreateRoleOrder");
    assert!(matches!(create.body(), RouteBody::Type(name) if name.as_str() == "CreateRoleOrder"));

    assert!(
        assembled
            .route_for_short_header(Leg::Ordinary, u64::from_le_bytes([0, 0, 0, 0, 0, 0, 0, 0]))
            .is_none()
    );
}
