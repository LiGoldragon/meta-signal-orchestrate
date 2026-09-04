use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

fn prepend_fault(fault: datomic::Fault, index: i64) -> datomic::Fault {
    match fault {
        datomic::Fault::Structural(f) => datomic::Fault::Structural(f),
        datomic::Fault::Conceptual(mut path, problem) => {
            path.insert(0, index);
            datomic::Fault::Conceptual(path, problem)
        }
        datomic::Fault::Corporal(mut path, problem) => {
            path.insert(0, index);
            datomic::Fault::Corporal(path, problem)
        }
    }
}

// Wire envelope
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Version(pub u16, pub u16, pub u16);

pub const SIGNAL_VERSION: Version = Version(1u16, 0u16, 0u16);

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    VersionMismatch(Version, Version),
    Unreadable,
}

// Domain types

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Configure(pub protos::Text, pub protos::Text);


#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum ConfigurationRefusal {
    InvalidConfiguration,
}


#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ConfigurationRejection(pub Configure, pub ConfigurationRefusal);


// Request / Reply / Body / Frame

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Configure(Configure),
}


#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Reply {
    Configured(Configure),
    ConfigurationRejected(ConfigurationRejection),
}


#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Body {
    Request(Request),
    Reply(Reply),
    Refusal(Refusal),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Frame(pub Version, pub Body);

// ---------------------------------------------------------------------------
// Datomic impls
// ---------------------------------------------------------------------------

impl datomic::Datomic for Configure {
    fn incorporate(datom: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        let datomic::Datom::Struct(fields) = datom else {
            return Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, datom),
            ));
        };
        if fields.len() != 2 {
            return Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(2, fields.len() as i64),
            ));
        }
        let mut iter = fields.into_iter();
        Ok(Self(
            <protos::Text as datomic::Datomic>::incorporate(iter.next().unwrap())
                .map_err(|f| prepend_fault(f, 0))?,
            <protos::Text as datomic::Datomic>::incorporate(iter.next().unwrap())
                .map_err(|f| prepend_fault(f, 1))?,
        ))
    }

    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
        ])
    }
}

impl datomic::Datomic for ConfigurationRefusal {
    fn incorporate(datom: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        match &datom {
            datomic::Datom::Bare(s) if s == "InvalidConfiguration" => {
                Ok(Self::InvalidConfiguration)
            }
            _ => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Variant, datom),
            )),
        }
    }

    fn datomize(&self) -> datomic::Datom {
        match self {
            Self::InvalidConfiguration => {
                datomic::Datom::Bare("InvalidConfiguration".to_owned())
            }
        }
    }
}

impl datomic::Datomic for ConfigurationRejection {
    fn incorporate(datom: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        let datomic::Datom::Struct(fields) = datom else {
            return Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Struct, datom),
            ));
        };
        if fields.len() != 2 {
            return Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Arity(2, fields.len() as i64),
            ));
        }
        let mut iter = fields.into_iter();
        Ok(Self(
            <Configure as datomic::Datomic>::incorporate(iter.next().unwrap())
                .map_err(|f| prepend_fault(f, 0))?,
            <ConfigurationRefusal as datomic::Datomic>::incorporate(iter.next().unwrap())
                .map_err(|f| prepend_fault(f, 1))?,
        ))
    }

    fn datomize(&self) -> datomic::Datom {
        datomic::Datom::Struct(vec![
            datomic::Datomic::datomize(&self.0),
            datomic::Datomic::datomize(&self.1),
        ])
    }
}

impl datomic::Datomic for Request {
    fn incorporate(datom: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        let datomic::Datom::Variant(ref head, ref sep, ref body) = datom else {
            return Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Variant, datom),
            ));
        };
        if *sep != datomic::Separator::Period {
            return Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Separator(*sep),
            ));
        }
        match head.as_str() {
            "Configure" => {
                let inner = body
                    .as_ref()
                    .map(|b| *b.clone())
                    .ok_or_else(|| {
                        datomic::Fault::Corporal(
                            vec![],
                            datomic::Problem::Shape(
                                datomic::Expected::Struct,
                                datomic::Datom::Bare(head.clone()),
                            ),
                        )
                    })?;
                Ok(Self::Configure(
                    <Configure as datomic::Datomic>::incorporate(inner)?,
                ))
            }
            _ => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::UnknownVariant(head.clone()),
            )),
        }
    }

    fn datomize(&self) -> datomic::Datom {
        match self {
            Self::Configure(v) => datomic::Datom::Variant(
                "Configure".to_owned(),
                datomic::Separator::Period,
                Some(Box::new(datomic::Datomic::datomize(v))),
            ),
        }
    }
}

impl datomic::Datomic for Reply {
    fn incorporate(datom: datomic::Datom) -> std::result::Result<Self, datomic::Fault> {
        let datomic::Datom::Variant(ref head, ref sep, ref body) = datom else {
            return Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Shape(datomic::Expected::Variant, datom),
            ));
        };
        if *sep != datomic::Separator::Period {
            return Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::Separator(*sep),
            ));
        }
        match head.as_str() {
            "Configured" => {
                let inner = body
                    .as_ref()
                    .map(|b| *b.clone())
                    .ok_or_else(|| {
                        datomic::Fault::Corporal(
                            vec![],
                            datomic::Problem::Shape(
                                datomic::Expected::Struct,
                                datomic::Datom::Bare(head.clone()),
                            ),
                        )
                    })?;
                Ok(Self::Configured(
                    <Configure as datomic::Datomic>::incorporate(inner)?,
                ))
            }
            "ConfigurationRejected" => {
                let inner = body
                    .as_ref()
                    .map(|b| *b.clone())
                    .ok_or_else(|| {
                        datomic::Fault::Corporal(
                            vec![],
                            datomic::Problem::Shape(
                                datomic::Expected::Struct,
                                datomic::Datom::Bare(head.clone()),
                            ),
                        )
                    })?;
                Ok(Self::ConfigurationRejected(
                    <ConfigurationRejection as datomic::Datomic>::incorporate(inner)?,
                ))
            }
            _ => Err(datomic::Fault::Corporal(
                vec![],
                datomic::Problem::UnknownVariant(head.clone()),
            )),
        }
    }

    fn datomize(&self) -> datomic::Datom {
        match self {
            Self::Configured(v) => datomic::Datom::Variant(
                "Configured".to_owned(),
                datomic::Separator::Period,
                Some(Box::new(datomic::Datomic::datomize(v))),
            ),
            Self::ConfigurationRejected(v) => datomic::Datom::Variant(
                "ConfigurationRejected".to_owned(),
                datomic::Separator::Period,
                Some(Box::new(datomic::Datomic::datomize(v))),
            ),
        }
    }
}
