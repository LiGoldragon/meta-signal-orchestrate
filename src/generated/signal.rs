#![allow(dead_code)]
#![allow(clippy::redundant_closure)]
pub type OrdinarySocketPath = protos::Text;
pub type MetaSocketPath = protos::Text;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Configure(pub OrdinarySocketPath, pub MetaSocketPath);
impl datom_codec::Datomic for Configure {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: OrdinarySocketPath = datom_codec::Positional::position(&mut p)?;
        let p1: MetaSocketPath = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for Configure {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            datom_codec::Datom::Struct(vec![
                protos::Conceivable::conceive(&self.0)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.1)
                    .expect("infallible datom ascent")
                    .1,
            ]),
        ))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigurationRefusal {
    InvalidConfiguration,
}
impl datom_codec::Datomic for ConfigurationRefusal {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "InvalidConfiguration" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::InvalidConfiguration)
            }
            _ => std::result::Result::Err(datom_codec::Headed::reject(
                &v,
                datom_codec::Problem::UnknownVariant(
                    protos::Word::try_from(v.name).expect("variant name"),
                ),
            )),
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for ConfigurationRefusal {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            match self {
                Self::InvalidConfiguration => datom_codec::Datom::Word(
                    datom_codec::DatomWord::try_from(
                        protos::Word::try_from("InvalidConfiguration").expect("static variant"),
                    )
                    .expect("stable variant"),
                ),
            },
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigurationRejection(pub Configure, pub ConfigurationRefusal);
impl datom_codec::Datomic for ConfigurationRejection {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: Configure = datom_codec::Positional::position(&mut p)?;
        let p1: ConfigurationRefusal = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for ConfigurationRejection {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            datom_codec::Datom::Struct(vec![
                protos::Conceivable::conceive(&self.0)
                    .expect("infallible datom ascent")
                    .1,
                protos::Conceivable::conceive(&self.1)
                    .expect("infallible datom ascent")
                    .1,
            ]),
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Configure(Configure),
}
impl datom_codec::Datomic for Request {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Configure" => {
                std::result::Result::Ok(Self::Configure(datom_codec::Carrying::body(v)?))
            }
            _ => std::result::Result::Err(datom_codec::Headed::reject(
                &v,
                datom_codec::Problem::UnknownVariant(
                    protos::Word::try_from(v.name).expect("variant name"),
                ),
            )),
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Request {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            match self {
                Self::Configure(p0) => datom_codec::Datom::Variant(
                    protos::Symbol::try_from("Configure").expect("static variant"),
                    std::boxed::Box::new(
                        protos::Conceivable::conceive(p0)
                            .expect("infallible datom ascent")
                            .1,
                    ),
                ),
            },
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Response {
    Configured(Configure),
    ConfigurationRejected(ConfigurationRejection),
}
impl datom_codec::Datomic for Response {
    fn incorporate(site: datom_codec::Site<'_>) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Configured" => {
                std::result::Result::Ok(Self::Configured(datom_codec::Carrying::body(v)?))
            }
            "ConfigurationRejected" => std::result::Result::Ok(Self::ConfigurationRejected(
                datom_codec::Carrying::body(v)?,
            )),
            _ => std::result::Result::Err(datom_codec::Headed::reject(
                &v,
                datom_codec::Problem::UnknownVariant(
                    protos::Word::try_from(v.name).expect("variant name"),
                ),
            )),
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Response {
    type Fault = std::convert::Infallible;
    fn conceive(&self) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(protos::Situated(
            protos::Situation {
                extent: protos::Extent(0, 0),
                children: vec![],
            },
            match self {
                Self::Configured(p0) => datom_codec::Datom::Variant(
                    protos::Symbol::try_from("Configured").expect("static variant"),
                    std::boxed::Box::new(
                        protos::Conceivable::conceive(p0)
                            .expect("infallible datom ascent")
                            .1,
                    ),
                ),
                Self::ConfigurationRejected(p0) => datom_codec::Datom::Variant(
                    protos::Symbol::try_from("ConfigurationRejected").expect("static variant"),
                    std::boxed::Box::new(
                        protos::Conceivable::conceive(p0)
                            .expect("infallible datom ascent")
                            .1,
                    ),
                ),
            },
        ))
    }
}
pub trait WireConversion: Sized {
    type Wire;
    fn into_wire(self) -> Self::Wire;
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault>;
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WireFault {
    Text,
}
pub type OrdinarySocketPathWire = std::string::String;
pub type MetaSocketPathWire = std::string::String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ConfigureWire(pub OrdinarySocketPathWire, pub MetaSocketPathWire);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ConfigurationRefusalWire {
    InvalidConfiguration,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ConfigurationRejectionWire(pub ConfigureWire, pub ConfigurationRefusalWire);
impl WireConversion for Configure {
    type Wire = ConfigureWire;
    fn into_wire(self) -> Self::Wire {
        let Configure(p0, p1) = self;
        ConfigureWire(p0.to_string(), p1.to_string())
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let ConfigureWire(p0, p1) = wire;
        Ok(Configure(
            protos::Text::try_from(p0).map_err(|_| WireFault::Text)?,
            protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
        ))
    }
}
impl WireConversion for ConfigurationRefusal {
    type Wire = ConfigurationRefusalWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            ConfigurationRefusal::InvalidConfiguration => {
                ConfigurationRefusalWire::InvalidConfiguration
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            ConfigurationRefusalWire::InvalidConfiguration => {
                Ok(ConfigurationRefusal::InvalidConfiguration)
            }
        }
    }
}
impl WireConversion for ConfigurationRejection {
    type Wire = ConfigurationRejectionWire;
    fn into_wire(self) -> Self::Wire {
        let ConfigurationRejection(p0, p1) = self;
        ConfigurationRejectionWire(
            <Configure as WireConversion>::into_wire(p0),
            <ConfigurationRefusal as WireConversion>::into_wire(p1),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let ConfigurationRejectionWire(p0, p1) = wire;
        Ok(ConfigurationRejection(
            <Configure as WireConversion>::try_from_wire(p0)?,
            <ConfigurationRefusal as WireConversion>::try_from_wire(p1)?,
        ))
    }
}
impl WireConversion for Request {
    type Wire = RequestWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            Request::Configure(value) => {
                RequestWire::Configure(<Configure as WireConversion>::into_wire(value))
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            RequestWire::Configure(value) => Ok(Request::Configure(
                <Configure as WireConversion>::try_from_wire(value)?,
            )),
        }
    }
}
impl WireConversion for Response {
    type Wire = ResponseWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            Response::Configured(value) => {
                ResponseWire::Configured(<Configure as WireConversion>::into_wire(value))
            }
            Response::ConfigurationRejected(value) => ResponseWire::ConfigurationRejected(
                <ConfigurationRejection as WireConversion>::into_wire(value),
            ),
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            ResponseWire::Configured(value) => Ok(Response::Configured(
                <Configure as WireConversion>::try_from_wire(value)?,
            )),
            ResponseWire::ConfigurationRejected(value) => Ok(Response::ConfigurationRejected(
                <ConfigurationRejection as WireConversion>::try_from_wire(value)?,
            )),
        }
    }
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RequestWire {
    Configure(ConfigureWire),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ResponseWire {
    Configured(ConfigureWire),
    ConfigurationRejected(ConfigurationRejectionWire),
}
