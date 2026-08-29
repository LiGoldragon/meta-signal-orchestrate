use datomic::Datomic;
use meta_signal_orchestrate::{
    CHANNEL_CONTRACT_ID, CHANNEL_WIRE_REVISION, ChannelContractId, ChannelWireRevision,
    ConfigurationRefusal, ConfigurationRejected, Configure, Configured, Frame, FrameBody,
    FrameCodecError, INTERFACE_VERSION, MetaSocketPath, OrdinarySocketPath, ProtocolVersion,
    Refusal, Reply, Request, SignalFrameCodec,
};
use protos::PortionText;

fn ordinary_path(value: &str) -> OrdinarySocketPath {
    OrdinarySocketPath::try_from(value).expect("representable ordinary socket path")
}

fn meta_path(value: &str) -> MetaSocketPath {
    MetaSocketPath::try_from(value).expect("representable meta socket path")
}

fn configure() -> Configure {
    Configure {
        ordinary_socket_path: ordinary_path("/tmp/orchestrate.sock"),
        meta_socket_path: meta_path("/tmp/meta-orchestrate.sock"),
    }
}

fn assert_datom_root<Value>(value: Value, expected: &str)
where
    Value: Datomic + Clone + std::fmt::Debug + PartialEq,
{
    let portion = value.portion();
    assert_eq!(portion.canonical_text().as_ref(), expected);
    assert_eq!(
        Value::embody(&portion).expect("Datomic root realizes"),
        value
    );
}

#[test]
fn all_privileged_datom_roots_round_trip_through_request_reply_and_refusal() {
    let configure = configure();
    assert_datom_root(
        Request::Configure(configure.clone()),
        "Configure.{/tmp/orchestrate.sock /tmp/meta-orchestrate.sock}",
    );
    assert_datom_root(
        Reply::Configured(Configured {
            configure: configure.clone(),
        }),
        "Configured.{{/tmp/orchestrate.sock /tmp/meta-orchestrate.sock}}",
    );
    assert_datom_root(
        Refusal::ConfigurationRejected(ConfigurationRejected {
            configure,
            configuration_refusal: ConfigurationRefusal::InvalidConfiguration,
        }),
        "ConfigurationRejected.{{/tmp/orchestrate.sock /tmp/meta-orchestrate.sock} InvalidConfiguration}",
    );
}

#[test]
fn rkyv_frame_is_length_prefixed_validated_and_bound_to_privileged_constants() {
    assert_eq!(CHANNEL_CONTRACT_ID, ChannelContractId(2));
    assert_eq!(CHANNEL_WIRE_REVISION, ChannelWireRevision(5));
    assert_eq!(INTERFACE_VERSION, ProtocolVersion::new(0, 2, 0));
    let frame = Frame {
        channel_contract_id: CHANNEL_CONTRACT_ID,
        channel_wire_revision: CHANNEL_WIRE_REVISION,
        protocol_version: INTERFACE_VERSION,
        body: FrameBody::Refusal(Refusal::ConfigurationRejected(ConfigurationRejected {
            configure: configure(),
            configuration_refusal: ConfigurationRefusal::InvalidConfiguration,
        })),
    };
    let bytes = frame.encode_length_prefixed().expect("rkyv frame encodes");
    assert_eq!(
        Frame::decode_length_prefixed(&bytes).expect("rkyv frame validates"),
        frame
    );
    let wrong_revision = Frame {
        channel_wire_revision: ChannelWireRevision(99),
        ..frame
    };
    assert!(matches!(
        wrong_revision.encode_length_prefixed(),
        Err(FrameCodecError::WrongChannelWireRevision { .. })
    ));
}
