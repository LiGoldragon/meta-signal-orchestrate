//! The scalar newtype keeps the representation the type alias had: a value of
//! the contract archives to the same bytes, and writes the same Datom text, as a
//! mirror of it built from plain scalars.

use meta_signal_orchestrate::{PeerRejection, PeerUserId};
use rkyv::rancor::Error;

#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Composing)
)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
struct PlainRejection {
    peer_user_id: i64,
}

fn rejection() -> PeerRejection {
    PeerRejection {
        peer_user_id: PeerUserId(1000),
    }
}

fn plain_rejection() -> PlainRejection {
    PlainRejection { peer_user_id: 1000 }
}

#[test]
fn newtype_value_archives_to_the_bytes_of_a_plain_scalar() {
    let new = rkyv::to_bytes::<Error>(&rejection()).expect("archive rejection");
    let old = rkyv::to_bytes::<Error>(&plain_rejection()).expect("archive plain rejection");
    assert_eq!(new.as_slice(), old.as_slice());
}

#[cfg(feature = "datom")]
mod text {
    use super::*;
    use datom_codec::{Actualizing, Budget, Composing, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    fn written<T: Datomizable>(value: &T) -> String {
        value.datomize(Vec::new()).protosize().textualize()
    }

    fn read<T: Composing>(text: &str) -> T {
        let mut budget = Budget {
            remaining: 4096,
            reader: ReaderBudget::try_from(4096usize).unwrap(),
            depth: 0,
            maximum_depth: 64,
        };
        Potential::<T>::from(text)
            .actualize(&mut budget)
            .expect("read the datom text")
    }

    #[test]
    fn newtype_value_writes_the_text_of_a_plain_scalar() {
        assert_eq!(written(&rejection()), written(&plain_rejection()));
    }

    #[test]
    fn text_written_from_a_plain_scalar_reads_as_the_newtype_value() {
        assert_eq!(
            read::<PeerRejection>(&written(&plain_rejection())),
            rejection()
        );
    }
}
