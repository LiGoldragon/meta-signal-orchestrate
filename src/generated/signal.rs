#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ConfigurationRejectionReason {
    InvalidConfiguration,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigurationRejection {
    pub configuration_rejection_reason: ConfigurationRejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PeerUserId(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PeerRejection {
    pub peer_user_id: PeerUserId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Input {
    Configure(signal_orchestrate::OrchestrateNexusConfiguration),
    ReverseMetaConfiguration,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Output {
    Configured(signal_orchestrate::ConfigurationReceipt),
    OrdinaryConfigurationReopened(signal_orchestrate::ConfigurationReceipt),
    ConfigurationRejected(ConfigurationRejection),
    PeerRefused(PeerRejection),
    Refused(signal_orchestrate::Refusal),
}
#[rustfmt::skip]
#[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]
pub trait Answering {
    fn configure(
        &self,
        orchestrate_nexus_configuration: &signal_orchestrate::OrchestrateNexusConfiguration,
    ) -> std::result::Result<signal_orchestrate::ConfigurationReceipt, Output>;
    fn reverse_meta_configuration(
        &self,
    ) -> std::result::Result<signal_orchestrate::ConfigurationReceipt, Output>;
}
#[rustfmt::skip]
pub trait Answerable {
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Output;
}
#[rustfmt::skip]
impl Answerable for Input {
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Output {
        match self {
            Input::Configure(carried) => {
                nexus
                    .configure(carried)
                    .map_or_else(|refusal| refusal, Output::Configured)
            }
            Input::ReverseMetaConfiguration => {
                nexus
                    .reverse_meta_configuration()
                    .map_or_else(
                        |refusal| refusal,
                        Output::OrdinaryConfigurationReopened,
                    )
            }
        }
    }
}
