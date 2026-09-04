#![allow(dead_code)]
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
pub type OrdinarySocketPath = protos::Text;
pub type MetaSocketPath = protos::Text;
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Configure(pub OrdinarySocketPath, pub MetaSocketPath);
impl datomic::Corporal<datomic::Datom> for Configure {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 2usize => {
                let mut iter = fields.into_iter();
                Ok(Self(
                    <OrdinarySocketPath as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <MetaSocketPath as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(2i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for Configure {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
        ])
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum ConfigurationRefusal {
    InvalidConfiguration,
}
impl datomic::Corporal<datomic::Datom> for ConfigurationRefusal {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Bare(s) if s == stringify!(InvalidConfiguration) => {
                Ok(Self::InvalidConfiguration)
            }
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Variant, other),
            )),
        }
    }
}
impl datomic::Datomic for ConfigurationRefusal {
    fn datomize(&self) -> datomic::Datom {
        match self {
            Self::InvalidConfiguration => {
                datomic::Datom::Bare(stringify!(InvalidConfiguration).to_owned())
            }
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ConfigurationRejection(pub Configure, pub ConfigurationRefusal);
impl datomic::Corporal<datomic::Datom> for ConfigurationRejection {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 2usize => {
                let mut iter = fields.into_iter();
                Ok(Self(
                    <Configure as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                    <ConfigurationRefusal as datomic::Corporal<datomic::Datom>>::incorporate(
                        iter.next().unwrap(),
                    )?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(2i64, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for ConfigurationRejection {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
        ])
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Configure(Configure),
}
impl datomic::Corporal<datomic::Datom> for Request {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Variant(head, protos::Separator::Period, Some(body))
                if head == stringify!(Configure) =>
            {
                Ok(Self::Configure(<Configure as datomic::Corporal<
                    datomic::Datom,
                >>::incorporate(*body)?))
            }
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Variant, other),
            )),
        }
    }
}
impl datomic::Datomic for Request {
    fn datomize(&self) -> datomic::Datom {
        match self {
            Self::Configure(value) => datomic::Datom::Variant(
                stringify!(Configure).to_owned(),
                protos::Separator::Period,
                Some(Box::new(datomic::Datomic::datomize(value))),
            ),
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Reply {
    Configured(Configure),
    ConfigurationRejected(ConfigurationRejection),
}
impl datomic::Corporal<datomic::Datom> for Reply {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Variant(head, protos::Separator::Period, Some(body))
                if head == stringify!(Configured) =>
            {
                Ok(Self::Configured(<Configure as datomic::Corporal<
                    datomic::Datom,
                >>::incorporate(*body)?))
            }
            datomic::Datom::Variant(head, protos::Separator::Period, Some(body))
                if head == stringify!(ConfigurationRejected) =>
            {
                Ok(Self::ConfigurationRejected(
                    <ConfigurationRejection as datomic::Corporal<datomic::Datom>>::incorporate(
                        *body,
                    )?,
                ))
            }
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Variant, other),
            )),
        }
    }
}
impl datomic::Datomic for Reply {
    fn datomize(&self) -> datomic::Datom {
        match self {
            Self::Configured(value) => datomic::Datom::Variant(
                stringify!(Configured).to_owned(),
                protos::Separator::Period,
                Some(Box::new(datomic::Datomic::datomize(value))),
            ),
            Self::ConfigurationRejected(value) => datomic::Datom::Variant(
                stringify!(ConfigurationRejected).to_owned(),
                protos::Separator::Period,
                Some(Box::new(datomic::Datomic::datomize(value))),
            ),
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Version(pub u16, pub u16, pub u16);
pub const SIGNAL_VERSION: Version = Version(1u16, 0u16, 0u16);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    VersionMismatch(Version, Version),
    Unreadable,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Body {
    Request(Request),
    Reply(Reply),
    Refusal(Refusal),
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Frame(pub Version, pub Body);
impl datomic::Corporal<datomic::Datom> for Version {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 3 => {
                let mut it = fields.into_iter();
                let a = <protos::Integer as datomic::Corporal<datomic::Datom>>::incorporate(
                    it.next().unwrap(),
                )? as u16;
                let b = <protos::Integer as datomic::Corporal<datomic::Datom>>::incorporate(
                    it.next().unwrap(),
                )? as u16;
                let c = <protos::Integer as datomic::Corporal<datomic::Datom>>::incorporate(
                    it.next().unwrap(),
                )? as u16;
                Ok(Self(a, b, c))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(3, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for Version {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&(self.0 as protos::Integer)),
            datomic::Datomic::datomize(&(self.1 as protos::Integer)),
            datomic::Datomic::datomize(&(self.2 as protos::Integer)),
        ])
    }
}
impl datomic::Corporal<datomic::Datom> for Refusal {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Variant(head, protos::Separator::Period, Some(body))
                if head == "VersionMismatch" =>
            {
                match *body {
                    datomic::Datom::Struct(fields) if fields.len() == 2 => {
                        let mut it = fields.into_iter();
                        Ok(Self::VersionMismatch(
                            <Version as datomic::Corporal<datomic::Datom>>::incorporate(
                                it.next().unwrap(),
                            )?,
                            <Version as datomic::Corporal<datomic::Datom>>::incorporate(
                                it.next().unwrap(),
                            )?,
                        ))
                    }
                    other => Err(datomic::Fault::Corporal(
                        vec![],
                        datomic::Problem::Shape(datomic::Expected::Struct, other),
                    )),
                }
            }
            datomic::Datom::Bare(s) if s == "Unreadable" => Ok(Self::Unreadable),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Variant, other),
            )),
        }
    }
}
impl datomic::Datomic for Refusal {
    fn datomize(&self) -> datomic::Datom {
        match self {
            Self::VersionMismatch(a, b) => datomic::Datom::Variant(
                "VersionMismatch".to_owned(),
                protos::Separator::Period,
                Some(Box::new(datomic::Datom::Struct(vec![
                    datomic::Datomic::datomize(a),
                    datomic::Datomic::datomize(b),
                ]))),
            ),
            Self::Unreadable => datomic::Datom::Bare("Unreadable".to_owned()),
        }
    }
}
impl datomic::Corporal<datomic::Datom> for Body {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Variant(head, protos::Separator::Period, Some(body)) => {
                match head.as_str() {
                    "Request" => Ok(Self::Request(<Request as datomic::Corporal<
                        datomic::Datom,
                    >>::incorporate(*body)?)),
                    "Reply" => Ok(Self::Reply(
                        <Reply as datomic::Corporal<datomic::Datom>>::incorporate(*body)?,
                    )),
                    "Refusal" => Ok(Self::Refusal(<Refusal as datomic::Corporal<
                        datomic::Datom,
                    >>::incorporate(*body)?)),
                    _ => Err(datomic::Fault::Corporal(
                        vec![],
                        datomic::Problem::UnknownVariant(head),
                    )),
                }
            }
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Variant, other),
            )),
        }
    }
}
impl datomic::Datomic for Body {
    fn datomize(&self) -> datomic::Datom {
        match self {
            Self::Request(v) => datomic::Datom::Variant(
                "Request".to_owned(),
                protos::Separator::Period,
                Some(Box::new(datomic::Datomic::datomize(v))),
            ),
            Self::Reply(v) => datomic::Datom::Variant(
                "Reply".to_owned(),
                protos::Separator::Period,
                Some(Box::new(datomic::Datomic::datomize(v))),
            ),
            Self::Refusal(v) => datomic::Datom::Variant(
                "Refusal".to_owned(),
                protos::Separator::Period,
                Some(Box::new(datomic::Datomic::datomize(v))),
            ),
        }
    }
}
impl datomic::Corporal<datomic::Datom> for Frame {
    type Fault = datomic::Fault;
    fn incorporate(concept: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match concept {
            datomic::Datom::Struct(fields) if fields.len() == 2 => {
                let mut it = fields.into_iter();
                Ok(Self(
                    <Version as datomic::Corporal<datomic::Datom>>::incorporate(
                        it.next().unwrap(),
                    )?,
                    <Body as datomic::Corporal<datomic::Datom>>::incorporate(it.next().unwrap())?,
                ))
            }
            datomic::Datom::Struct(fields) => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(2, fields.len() as i64),
            )),
            other => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, other),
            )),
        }
    }
}
impl datomic::Datomic for Frame {
    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
        ])
    }
}
