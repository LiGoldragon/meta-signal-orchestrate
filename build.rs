use std::{env, path::PathBuf};

use schema_rust::build::{CargoSchemaMetadata, DependencySchema, GenerationDriver, GenerationPlan};

fn main() {
    SchemaBuild::from_environment().run();
}

struct SchemaBuild {
    crate_root: PathBuf,
}

impl SchemaBuild {
    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set")),
        }
    }

    fn run(&self) {
        println!("cargo:rerun-if-changed=schema/lib.schema");
        println!("cargo:rerun-if-env-changed=DEP_SIGNAL_ORCHESTRATE_SCHEMA_DIR");
        let signal_orchestrate_schema = self.signal_orchestrate_schema();
        println!(
            "cargo::rustc-env=SIGNAL_ORCHESTRATE_SCHEMA_DIR={}",
            signal_orchestrate_schema.schema_directory().display()
        );
        CargoSchemaMetadata::new("meta-signal-orchestrate").emit_schema_directory(&self.crate_root);
        GenerationDriver::new(
            GenerationPlan::wire_contract(&self.crate_root, "meta-signal-orchestrate", "0.4.0")
                .with_dependency_schema(signal_orchestrate_schema),
        )
        .generate()
        .expect("generate meta-signal-orchestrate schema artifacts")
        .write_or_check("META_SIGNAL_ORCHESTRATE_UPDATE_SCHEMA_ARTIFACTS")
        .expect("checked-in meta-signal-orchestrate schema artifacts are fresh");
    }

    fn signal_orchestrate_schema(&self) -> DependencySchema {
        DependencySchema::from_cargo_metadata("signal-orchestrate", "signal-orchestrate", "0.5.0")
            .expect("read signal-orchestrate schema metadata")
            .expect("signal-orchestrate must emit schema metadata")
    }
}
