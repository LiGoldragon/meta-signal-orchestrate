//! The meta contract inside signal's exchange envelope, exercised as a wire.
//!
//! Every value is framed, read back off a byte stream and attributed to its
//! exchange. The digest oracle is computed outside the crate, by the published
//! FNV-1a algorithm over `ethos/signal.ethos`, not through the path under test.

use std::io::Cursor;

use meta_signal_orchestrate::{ETHOS, Query, Response};
use signal::{
    Answer, ByteViewable, ContractDigest, Contracted, Delivery, Dispatch, Ending, ExchangeLedger,
    ExchangeMinting, Exchanged, FrameCapacity, FrameReading, FrameWriting, Greeted, Handshake,
    HandshakeReceipt, HandshakeRejection, Opening, Restorable, Signal, Signalizable,
};
use signal_orchestrate::{ConfigurationReceipt, OrchestrateNexusConfiguration};

/// FNV-1a over exactly the bytes of `ethos/signal.ethos`, as a signed 64-bit
/// integer, computed in Python from the published algorithm.
const META_DIGEST: ContractDigest = -8_743_676_413_678_169_448;

/// Put a value on a byte stream the way a socket carries it, and read it back.
trait CrossesTheWire: Sized {
    fn across(&self) -> Self;
}

impl<T> CrossesTheWire for T
where
    T: Signalizable,
    Signal<T>: Restorable<T>,
{
    fn across(&self) -> Self {
        let capacity = FrameCapacity::default();
        let mut wire = Vec::new();
        wire.write_frame(&self.signalize().expect("signalize"), capacity)
            .expect("write the frame");
        let body = Cursor::new(wire)
            .read_frame(capacity)
            .expect("read the frame");
        Signal::<T>::from(body.bytes().to_vec())
            .restore()
            .expect("restore the value")
    }
}

trait Configures {
    fn temporary() -> Self;
}

impl Configures for OrchestrateNexusConfiguration {
    fn temporary() -> Self {
        Self {
            ordinary_socket_path: "/tmp/orchestrate.sock".into(),
            meta_socket_path: "/tmp/orchestrate-meta.sock".into(),
        }
    }
}

#[test]
fn the_meta_contract_is_identified_by_the_digest_of_its_own_source() {
    assert_eq!(<Query as Contracted>::contract_digest(), META_DIGEST);
    assert_eq!(
        <Query as Contracted>::greeting(),
        Handshake {
            contract_digest: META_DIGEST
        }
    );
    assert_eq!(<Query as Contracted>::CONTRACT_SOURCE, ETHOS);
}

#[test]
fn an_ordinary_peer_greeting_the_meta_socket_is_refused() {
    assert_eq!(
        <Query as Contracted>::receipt(&<signal_orchestrate::Query as Contracted>::greeting()),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(META_DIGEST))
    );
    assert_eq!(
        <Query as Contracted>::receipt(&<Query as Contracted>::greeting()),
        HandshakeReceipt::Greeted(META_DIGEST)
    );
}

#[test]
fn a_configure_is_one_exchange_answered_once_and_then_ended() {
    let mut ledger = ExchangeLedger::default();
    ledger.greet().expect("greet once");
    let exchange = ledger.open().expect("open the exchange");

    let opening = Dispatch::Open(Opening {
        exchange,
        query: Query::Configure(OrchestrateNexusConfiguration::temporary()),
    });
    assert_eq!(opening.across(), opening);

    let answer: Delivery<Response> = Delivery::Answer(Answer {
        exchange,
        response: Response::Configured(ConfigurationReceipt {
            orchestrate_nexus_configuration: OrchestrateNexusConfiguration::temporary(),
            meta_configure_done: true,
        }),
    });
    assert_eq!(answer.across(), answer);

    let ending: Delivery<Response> = Delivery::End(Ending::completed(exchange));
    let Delivery::End(received) = ending.across() else {
        panic!("a one-answer exchange ends after its answer");
    };
    assert_eq!(received.exchange(), exchange);
}
