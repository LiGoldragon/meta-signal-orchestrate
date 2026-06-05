use meta_signal_orchestrate::{
    ApplicationFailure, ApplicationFailureReason, ApplicationSuccess, ChannelRequest,
    CreateRoleOrder, DownstreamComponent, Frame, FrameBody, HarnessKind, LaneAuthority,
    LaneAuthorityChange, LaneAuthoritySet, LaneIdentifier, LaneRegistered, LaneRegistration,
    LaneRegistrationRequest, LaneRetired, MetaOperationKind, MetaOrchestrateReply,
    MetaOrchestrateRequest, MetaOrchestrateRequestUnimplemented,
    MetaOrchestrateUnimplementedReason, PartialApplied, RefreshRepositoryIndexOrder,
    RepositoryIndexRefreshed, RetireRoleOrder, Retirement, Role, RoleCreated, RoleCreationRejected,
    RoleCreationRejectionReason, RoleIdentifier, RoleRetired, RoleToken, ScopeReason, WirePath,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SubReply,
};

fn role() -> RoleIdentifier {
    RoleIdentifier::from_wire_token("primary-hrhz").expect("role")
}

fn repository_path() -> WirePath {
    WirePath::from_absolute_path("/git/github.com/LiGoldragon/persona-role-primary-hrhz-reports")
        .expect("repository path")
}

fn lane_path() -> WirePath {
    WirePath::from_absolute_path("/home/li/primary/reports/primary-hrhz").expect("lane path")
}

fn lane() -> LaneIdentifier {
    LaneIdentifier::from_wire_token("persona-signal-designer").expect("lane")
}

fn role_vector() -> Role {
    Role::try_new(vec![
        RoleToken::from_text("PersonaSignal").expect("role token"),
        RoleToken::from_text("Designer").expect("role token"),
    ])
    .expect("role vector")
}

fn lane_registration() -> LaneRegistration {
    LaneRegistration {
        lane: lane(),
        role: role_vector(),
        authority: LaneAuthority::Structural,
    }
}

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn round_trip_request(request: MetaOrchestrateRequest) -> MetaOrchestrateRequest {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => {
            let operation = request.payloads().head();
            operation.clone()
        }
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_reply(reply: MetaOrchestrateReply) -> MetaOrchestrateReply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply operation, got {other:?}"),
    }
}

#[test]
fn meta_orchestrate_requests_round_trip() {
    let create = MetaOrchestrateRequest::Create(CreateRoleOrder {
        role: role(),
        harness: HarnessKind::Codex,
    });
    assert_eq!(round_trip_request(create.clone()), create);

    let retire = MetaOrchestrateRequest::Retire(Retirement::Role(RetireRoleOrder { role: role() }));
    assert_eq!(round_trip_request(retire.clone()), retire);

    let refresh = MetaOrchestrateRequest::Refresh(RefreshRepositoryIndexOrder {});
    assert_eq!(round_trip_request(refresh.clone()), refresh);

    let register = MetaOrchestrateRequest::Register(LaneRegistrationRequest {
        role: role_vector(),
        authority: LaneAuthority::Structural,
    });
    assert_eq!(round_trip_request(register.clone()), register);

    let retire_lane = MetaOrchestrateRequest::Retire(Retirement::Lane(lane()));
    assert_eq!(round_trip_request(retire_lane.clone()), retire_lane);

    let set_authority = MetaOrchestrateRequest::SetAuthority(LaneAuthorityChange {
        lane: lane(),
        authority: LaneAuthority::Support,
    });
    assert_eq!(round_trip_request(set_authority.clone()), set_authority);
}

#[test]
fn meta_orchestrate_replies_round_trip() {
    let created = MetaOrchestrateReply::RoleCreated(RoleCreated {
        role: role(),
        harness: HarnessKind::Codex,
        report_repository_path: repository_path(),
        report_lane_path: lane_path(),
    });
    assert_eq!(round_trip_reply(created.clone()), created);

    let retired = MetaOrchestrateReply::RoleRetired(RoleRetired { role: role() });
    assert_eq!(round_trip_reply(retired.clone()), retired);

    let rejected = MetaOrchestrateReply::RoleCreationRejected(RoleCreationRejected {
        role: role(),
        reason: RoleCreationRejectionReason::RoleAlreadyExists,
    });
    assert_eq!(round_trip_reply(rejected.clone()), rejected);

    let refreshed = MetaOrchestrateReply::RepositoryIndexRefreshed(RepositoryIndexRefreshed {
        repositories: 7,
    });
    assert_eq!(round_trip_reply(refreshed.clone()), refreshed);

    let registered = MetaOrchestrateReply::LaneRegistered(LaneRegistered {
        registration: lane_registration(),
    });
    assert_eq!(round_trip_reply(registered.clone()), registered);

    let lane_retired = MetaOrchestrateReply::LaneRetired(LaneRetired { lane: lane() });
    assert_eq!(round_trip_reply(lane_retired.clone()), lane_retired);

    let authority_set = MetaOrchestrateReply::LaneAuthoritySet(LaneAuthoritySet {
        lane: lane(),
        authority: LaneAuthority::Support,
    });
    assert_eq!(round_trip_reply(authority_set.clone()), authority_set);

    let partial = MetaOrchestrateReply::PartialApplied(PartialApplied {
        succeeded: vec![ApplicationSuccess {
            component: DownstreamComponent::Router,
            detail: ScopeReason::from_text("channel 42 installed").expect("success detail"),
        }],
        failed: vec![ApplicationFailure {
            component: DownstreamComponent::Harness,
            reason: ApplicationFailureReason::Unreachable,
            detail: ScopeReason::from_text("codex-7 transcript is gone").expect("failure detail"),
        }],
    });
    assert_eq!(round_trip_reply(partial.clone()), partial);

    let unimplemented = MetaOrchestrateReply::MetaOrchestrateRequestUnimplemented(
        MetaOrchestrateRequestUnimplemented {
            operation: MetaOperationKind::Create,
            reason: MetaOrchestrateUnimplementedReason::NotBuiltYet,
        },
    );
    assert_eq!(round_trip_reply(unimplemented.clone()), unimplemented);
}

#[test]
fn meta_orchestrate_operations_encode_as_contract_local_nota_heads() {
    use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};

    let operation = MetaOrchestrateRequest::Refresh(RefreshRepositoryIndexOrder {});
    let mut encoder = Encoder::new();
    operation
        .into_request()
        .encode(&mut encoder)
        .expect("encode");
    let text = encoder.into_string();

    assert_eq!(text, "(Refresh ())");
    assert!(!text.contains("Mutate"));
    assert!(!text.contains("Retract"));

    let mut decoder = Decoder::new(&text);
    let decoded = ChannelRequest::decode(&mut decoder).expect("decode");
    assert_eq!(
        decoded.payloads().head().operation_kind(),
        MetaOperationKind::Refresh
    );
}

#[test]
fn meta_orchestrate_request_exposes_contract_owned_operation_kind() {
    let create = MetaOrchestrateRequest::Create(CreateRoleOrder {
        role: role(),
        harness: HarnessKind::Codex,
    });
    assert_eq!(create.operation_kind(), MetaOperationKind::Create);

    let retire = MetaOrchestrateRequest::Retire(Retirement::Role(RetireRoleOrder { role: role() }));
    assert_eq!(retire.operation_kind(), MetaOperationKind::Retire);

    let refresh = MetaOrchestrateRequest::Refresh(RefreshRepositoryIndexOrder {});
    assert_eq!(refresh.operation_kind(), MetaOperationKind::Refresh);

    let register = MetaOrchestrateRequest::Register(LaneRegistrationRequest {
        role: role_vector(),
        authority: LaneAuthority::Structural,
    });
    assert_eq!(register.operation_kind(), MetaOperationKind::Register);

    let set_authority = MetaOrchestrateRequest::SetAuthority(LaneAuthorityChange {
        lane: lane(),
        authority: LaneAuthority::Support,
    });
    assert_eq!(
        set_authority.operation_kind(),
        MetaOperationKind::SetAuthority
    );
}
