use datomic::Textualizable;
use meta_signal_orchestrate::*;

fn configure() -> Configure {
    Configure(
        "/tmp/orchestrate.sock".to_owned(),
        "/tmp/meta-orchestrate.sock".to_owned(),
    )
}

#[test]
fn all_meta_datom_roots_round_trip() {
    // Request
    let req = Request::Configure(configure());
    let text = req.textualize();
    assert_eq!(
        text,
        "Configure.{ /tmp/orchestrate.sock /tmp/meta-orchestrate.sock }"
    );

    // Reply Configured
    let reply = Reply::Configured(configure());
    let text = reply.textualize();
    assert_eq!(
        text,
        "Configured.{ /tmp/orchestrate.sock /tmp/meta-orchestrate.sock }"
    );

    // Reply ConfigurationRejected
    let rejection = Reply::ConfigurationRejected(ConfigurationRejection(
        configure(),
        ConfigurationRefusal::InvalidConfiguration,
    ));
    let text = rejection.textualize();
    assert_eq!(
        text,
        "ConfigurationRejected.{ { /tmp/orchestrate.sock /tmp/meta-orchestrate.sock } InvalidConfiguration }"
    );
}

#[test]
fn rkyv_frame_version_only_validation() {
    let frame = Frame(
        SIGNAL_VERSION,
        Body::Request(Request::Configure(configure())),
    );
    let bytes = frame.encode_length_prefixed().expect("encodes");
    assert_eq!(
        Frame::decode_length_prefixed(&bytes).expect("decodes"),
        frame
    );

    let wrong = Frame(
        Version(99, 0, 0),
        Body::Request(Request::Configure(configure())),
    );
    assert!(matches!(
        wrong.encode_length_prefixed(),
        Err(FrameCodecError::VersionMismatch { .. })
    ));
}
