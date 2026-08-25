use dotos::{DotosEncode, DotosSource};
use meta_signal_orchestrate::{
    ConfigurationRefusal, ConfigurationRejected, Configure, Configured, Frame,
    MetaOrchestrateRequest, MetaOrchestrateWire, MetaSocketPath, OrdinarySocketPath, StorePath,
};
use signal_frame::{
    ClientFrame, ExchangeIdentifier, ExchangeLane, LaneSequence, RequestPayload, SessionEpoch,
    WireContract,
};

#[test]
fn build_generates_only_in_cargo_out_dir_and_checks_committed_projection() {
    let build = include_str!("../build.rs");

    assert!(build.contains("env::var_os(\"OUT_DIR\")"));
    assert!(build.contains("ComponentGeneration::new(root.join(\"ethos\"), &generated_directory)"));
    assert!(build.contains("fs::read(root.join(\"src/generated\").join(module))"));
    assert!(
        !build.contains(
            "ComponentGeneration::new(root.join(\"ethos\"), root.join(\"src/generated\"))"
        )
    );
}

#[test]
fn generated_contract_textualizes_configure() {
    assert_eq!(MetaOrchestrateWire::BINDING.contract().value(), 2);
    assert_eq!(MetaOrchestrateWire::BINDING.revision().value(), 3);
    let configure_payload = Configure {
        store_path: StorePath("/tmp/orchestrate.redb".into()),
        ordinary_socket_path: OrdinarySocketPath("/tmp/orchestrate.sock".into()),
        meta_socket_path: MetaSocketPath("/tmp/meta-orchestrate.sock".into()),
    };
    let configure = MetaOrchestrateRequest::Configure(configure_payload.clone());
    let configured = Configured {
        configure: configure_payload.clone(),
    };
    let rejected = ConfigurationRejected {
        configure: configure_payload.clone(),
        configuration_refusal: ConfigurationRefusal::StorePathImmutable,
    };

    assert_eq!(
        configure_payload.to_dotos(),
        "Configure.{/tmp/orchestrate.redb /tmp/orchestrate.sock /tmp/meta-orchestrate.sock}"
    );
    assert_eq!(
        configured.to_dotos(),
        "Configured.{/tmp/orchestrate.redb /tmp/orchestrate.sock /tmp/meta-orchestrate.sock}"
    );
    assert_eq!(
        rejected.to_dotos(),
        "ConfigurationRejected.{{/tmp/orchestrate.redb /tmp/orchestrate.sock /tmp/meta-orchestrate.sock} StorePathImmutable}"
    );
    assert_eq!(
        DotosSource::new(&configure_payload.to_dotos())
            .parse::<Configure>()
            .expect("decode configure"),
        configure_payload
    );
    assert_eq!(
        DotosSource::new(&rejected.to_dotos())
            .parse::<ConfigurationRejected>()
            .expect("decode configuration rejection"),
        rejected
    );
    let exchange = ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    );
    let frame = Frame::request_frame(exchange, configure.into_request()).expect("frame configure");
    let bytes = frame.encode_client_frame().expect("encode configure frame");
    assert_eq!(
        Frame::decode_client_frame(&bytes).expect("decode configure frame"),
        frame
    );
}
