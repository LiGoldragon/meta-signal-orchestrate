use schema_next::{ImportResolver, SchemaEngine, SchemaIdentity, SchemaSourceArtifact};
use std::path::PathBuf;

fn schema_file() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("schema")
        .join("lib.schema")
}

fn signal_orchestrate_schema_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("meta-signal-orchestrate has a parent directory")
        .join("signal-orchestrate")
        .join("schema")
}

#[test]
fn meta_signal_orchestrate_schema_lowers_meta_routes_and_imports_shared_nouns() {
    let source = std::fs::read_to_string(schema_file()).expect("read meta schema");
    let artifact = SchemaSourceArtifact::from_schema_text(&source).expect("schema source decodes");
    let resolver = ImportResolver::new().with_dependency(
        "signal-orchestrate",
        signal_orchestrate_schema_directory(),
        "0.2.0",
    );
    let engine = SchemaEngine::default();
    let schema = engine
        .lower_schema_source_with_resolver(
            artifact.source(),
            SchemaIdentity::new("meta-signal-orchestrate:lib", "0.2.0"),
            &resolver,
        )
        .expect("schema lowers");

    assert_eq!(schema.input().variants.len(), 5);
    assert_eq!(schema.output().variants.len(), 9);
    assert_eq!(schema.resolved_imports().len(), 8);

    let create = &schema.input().variants[0];
    assert_eq!(create.name.as_str(), "Create");
    assert_eq!(
        create
            .payload
            .as_ref()
            .and_then(schema_next::TypeReference::plain_name)
            .map(schema_next::Name::as_str),
        Some("CreateRoleOrder")
    );

    assert!(schema.resolved_imports().iter().any(|import| {
        import
            .use_item()
            .contains("signal_orchestrate::schema::lib::Role")
    }));
}
