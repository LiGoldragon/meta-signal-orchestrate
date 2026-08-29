use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}
impl ProtocolVersion {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChannelContractId(pub u32);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChannelWireRevision(pub u16);
pub const INTERFACE_VERSION: ProtocolVersion = ProtocolVersion::new(0u16, 2u16, 0u16);
pub const CHANNEL_CONTRACT_ID: ChannelContractId = ChannelContractId(2u32);
pub const CHANNEL_WIRE_REVISION: ChannelWireRevision = ChannelWireRevision(5u16);
pub const PROTOCOL_VERSION: ProtocolVersion = INTERFACE_VERSION;
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct OrdinarySocketPath(String);
impl OrdinarySocketPath {
    pub fn try_from_string(
        value: String,
    ) -> std::result::Result<Self, datomic::UnrepresentableString> {
        datomic::DatomicString::try_from(value).map(|value| Self(value.as_ref().to_owned()))
    }
}
impl std::convert::TryFrom<String> for OrdinarySocketPath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value)
    }
}
impl<'a> std::convert::TryFrom<&'a str> for OrdinarySocketPath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value.to_owned())
    }
}
impl AsRef<str> for OrdinarySocketPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct MetaSocketPath(String);
impl MetaSocketPath {
    pub fn try_from_string(
        value: String,
    ) -> std::result::Result<Self, datomic::UnrepresentableString> {
        datomic::DatomicString::try_from(value).map(|value| Self(value.as_ref().to_owned()))
    }
}
impl std::convert::TryFrom<String> for MetaSocketPath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value)
    }
}
impl<'a> std::convert::TryFrom<&'a str> for MetaSocketPath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value.to_owned())
    }
}
impl AsRef<str> for MetaSocketPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Configure {
    pub ordinary_socket_path: OrdinarySocketPath,
    pub meta_socket_path: MetaSocketPath,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum ConfigurationRefusal {
    InvalidConfiguration,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Configured {
    pub configure: Configure,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ConfigurationRejected {
    pub configure: Configure,
    pub configuration_refusal: ConfigurationRefusal,
}
impl datomic::Datomic for OrdinarySocketPath {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(
            <datomic::DatomicString as datomic::Datomic>::embody(portion)?
                .as_ref()
                .to_owned(),
        ))
    }
    fn portion(&self) -> protos::Portion {
        datomic::DatomicString::try_from(self.0.clone()).map_or_else(
            |_| datomic::PortionBuilding::bare("wire-invalid"),
            |value| datomic::Datomic::portion(&value),
        )
    }
}
impl datomic::Datomic for MetaSocketPath {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(
            <datomic::DatomicString as datomic::Datomic>::embody(portion)?
                .as_ref()
                .to_owned(),
        ))
    }
    fn portion(&self) -> protos::Portion {
        datomic::DatomicString::try_from(self.0.clone()).map_or_else(
            |_| datomic::PortionBuilding::bare("wire-invalid"),
            |value| datomic::Datomic::portion(&value),
        )
    }
}
impl datomic::Datomic for Configure {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 2usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            ordinary_socket_path: <OrdinarySocketPath as datomic::Datomic>::embody(&parts[0usize])?,
            meta_socket_path: <MetaSocketPath as datomic::Datomic>::embody(&parts[1usize])?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![
                datomic::Datomic::portion(&self.ordinary_socket_path),
                datomic::Datomic::portion(&self.meta_socket_path),
            ],
        )
    }
}
impl datomic::Datomic for ConfigurationRefusal {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(InvalidConfiguration)) {
            return Ok(Self::InvalidConfiguration);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::InvalidConfiguration => {
                datomic::PortionBuilding::bare(stringify!(InvalidConfiguration))
            }
        }
    }
}
impl datomic::Datomic for Configured {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 1usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            configure: <Configure as datomic::Datomic>::embody(&parts[0usize])?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![datomic::Datomic::portion(&self.configure)],
        )
    }
}
impl datomic::Datomic for ConfigurationRejected {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 2usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            configure: <Configure as datomic::Datomic>::embody(&parts[0usize])?,
            configuration_refusal: <ConfigurationRefusal as datomic::Datomic>::embody(
                &parts[1usize],
            )?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![
                datomic::Datomic::portion(&self.configure),
                datomic::Datomic::portion(&self.configuration_refusal),
            ],
        )
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Configure(Configure),
}
impl datomic::Datomic for Request {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Configure)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Configure(<Configure as datomic::Datomic>::embody(
                &headed.body,
            )?));
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Configure(value) => datomic::PortionBuilding::headed(
                stringify!(Configure),
                protos::Separator::Period,
                <Configure as datomic::Datomic>::portion(value),
            ),
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Reply {
    Configured(Configured),
}
impl datomic::Datomic for Reply {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Configured)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Configured(<Configured as datomic::Datomic>::embody(
                &headed.body,
            )?));
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Configured(value) => datomic::PortionBuilding::headed(
                stringify!(Configured),
                protos::Separator::Period,
                <Configured as datomic::Datomic>::portion(value),
            ),
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    ConfigurationRejected(ConfigurationRejected),
}
impl datomic::Datomic for Refusal {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(ConfigurationRejected)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::ConfigurationRejected(
                <ConfigurationRejected as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::ConfigurationRejected(value) => datomic::PortionBuilding::headed(
                stringify!(ConfigurationRejected),
                protos::Separator::Period,
                <ConfigurationRejected as datomic::Datomic>::portion(value),
            ),
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FrameBody {
    Request(Request),
    Reply(Reply),
    Refusal(Refusal),
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Frame {
    pub channel_contract_id: ChannelContractId,
    pub channel_wire_revision: ChannelWireRevision,
    pub protocol_version: ProtocolVersion,
    pub body: FrameBody,
}
