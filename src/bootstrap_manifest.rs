//! Authority-seated identities for the strict privileged orchestration Interface.
//!
//! These opaque identities and canonical-order values are minted state. None
//! is derived from spelling, source position, or Rust representation.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    223, 249, 232, 223, 246, 205, 63, 153, 224, 202, 182, 254, 117, 43, 51, 58, 150, 132, 169, 138,
    131, 196, 166, 224, 174, 225, 20, 140, 80, 92, 60, 223,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 34298;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 3327;

pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 32454, 0xbb7fe9d3cd108da6);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 16336, 0xc25c06addae436bb);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 19921, 0x513aed4d35a42be6);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 61661, 0xc0f50ea4f4942bf2);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 9491, 0xa4e97ff3d80ac653);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 11474, 0xd36fa41134c3240d);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 3616, 0x7ebbfe8dc197f62c);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 14614, 0xf15ffc0946a9d779);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 11137, 0x39144a2a79cd6c7b);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 32915, 0x4c18720e10fc14d5);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 7446, 0xc58f2c1ba1bf397d);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 38567, 0xbe07cb9518951694);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 62484, 0xd2e0b709b5f12796);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 29366, 0xdbbeeeefd71b4c88);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 7959, 0x3f826d43d22debad);
pub const STREAM_IDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 9482, 0x6cfe9ae18240e7ac);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    52561, 25842, 50711, 38702, 59220, 26521, 8640, 58972, 25038, 21429,
];

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "MetaOperationKind", 5271, 0x1d28f95e9d6451ed),
    DeclarationSeat::new(Some(5271), "Refresh", 59319, 0xc8c9e7155e8242f7),
];
