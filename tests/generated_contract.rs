use dotos::{DotosEncode, DotosSource};
use meta_signal_orchestrate::{
    ConfigurationRefusal, ConfigurationRejected, Configure, Configured, Frame,
    MetaOrchestrateRequest, MetaOrchestrateWire, MetaSocketPath, OrdinarySocketPath,
};
use signal_frame::{
    ClientFrame, ExchangeIdentifier, ExchangeLane, LaneSequence, RequestPayload, SessionEpoch,
    WireContract,
};

fn wire_fixture(source: &str) -> Vec<u8> {
    source
        .split_ascii_whitespace()
        .map(|byte| byte.parse().expect("decimal wire byte"))
        .collect()
}

#[test]
fn generated_contract_textualizes_configure() {
    assert_eq!(MetaOrchestrateWire::BINDING.contract().value(), 2);
    assert_eq!(MetaOrchestrateWire::BINDING.revision().value(), 4);
    let configure_payload = Configure {
        ordinary_socket_path: OrdinarySocketPath("/tmp/orchestrate.sock".into()),
        meta_socket_path: MetaSocketPath("/tmp/meta-orchestrate.sock".into()),
    };
    let configure = MetaOrchestrateRequest::Configure(configure_payload.clone());
    let configured = Configured {
        configure: configure_payload.clone(),
    };
    let rejected = ConfigurationRejected {
        configure: configure_payload.clone(),
        configuration_refusal: ConfigurationRefusal::InvalidConfiguration,
    };

    assert_eq!(
        configure_payload.to_dotos(),
        "Configure.{/tmp/orchestrate.sock /tmp/meta-orchestrate.sock}"
    );
    assert_eq!(
        configured.to_dotos(),
        "Configured.{/tmp/orchestrate.sock /tmp/meta-orchestrate.sock}"
    );
    assert_eq!(
        rejected.to_dotos(),
        "ConfigurationRejected.{{/tmp/orchestrate.sock /tmp/meta-orchestrate.sock} InvalidConfiguration}"
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

#[test]
fn generated_contract_preserves_configure_wire_bytes() {
    let configure = Frame::request_frame(
        ExchangeIdentifier::new(
            SessionEpoch::new(42),
            ExchangeLane::Connector,
            LaneSequence::first(),
        ),
        MetaOrchestrateRequest::Configure(Configure {
            ordinary_socket_path: OrdinarySocketPath(
                "/tmp/orchestrate-wire-byte-fixture.sock".into(),
            ),
            meta_socket_path: MetaSocketPath("/tmp/meta-orchestrate-wire-byte-fixture.sock".into()),
        })
        .into_request(),
    )
    .expect("frame configure")
    .encode_client_frame()
    .expect("encode configure frame");

    assert_eq!(
        configure,
        wire_fixture(include_str!("fixtures/configure-wire.bytes"))
    );
}
