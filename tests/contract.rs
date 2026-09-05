use datom_codec::{Actualizable, IncorporationBudget, Potential, Textualizable};
use meta_signal_orchestrate::*;
use protos::Text;

fn text(value: &str) -> Text {
    value.try_into().expect("fixture text")
}

fn configure() -> Configure {
    Configure(
        text("/tmp/orchestrate.sock"),
        text("/tmp/meta-orchestrate.sock"),
    )
}

fn assert_datom_round_trip<T>(value: T, expected: &str)
where
    T: datom_codec::Datomic
        + Textualizable<datom_codec::Datom>
        + Clone
        + std::fmt::Debug
        + PartialEq,
{
    let rendered = <T as Textualizable<datom_codec::Datom>>::textualize(&value);
    assert_eq!(rendered, expected);
    let decoded = Potential::<T>::from(rendered)
        .actualize(IncorporationBudget::try_from(1_024).expect("fixed positive budget"))
        .expect("round-trip actualize");
    assert_eq!(decoded, value);
}

#[test]
fn all_meta_datom_roots_round_trip() {
    assert_datom_round_trip(
        Request::Configure(configure()),
        "Configure.{ /tmp/orchestrate.sock /tmp/meta-orchestrate.sock }",
    );
    assert_datom_round_trip(
        Response::Configured(configure()),
        "Configured.{ /tmp/orchestrate.sock /tmp/meta-orchestrate.sock }",
    );
    assert_datom_round_trip(
        Response::ConfigurationRejected(ConfigurationRejection(
            configure(),
            ConfigurationRefusal::InvalidConfiguration,
        )),
        "ConfigurationRejected.{ { /tmp/orchestrate.sock /tmp/meta-orchestrate.sock } InvalidConfiguration }",
    );
}

#[test]
fn rkyv_request_wire_round_trips_and_validates_back_to_the_public_contract() {
    let request = Request::Configure(configure());
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&request.clone().into_wire())
        .expect("request wire encodes");
    let wire =
        rkyv::from_bytes::<RequestWire, rkyv::rancor::Error>(&bytes).expect("request wire decodes");
    assert_eq!(
        Request::try_from_wire(wire).expect("wire validates"),
        request
    );
}
