use owner_signal_persona_orchestrate::{
    CreateRoleOrder, Frame, FrameBody, HarnessKind, OwnerOperationKind, OwnerOrchestrateReply,
    OwnerOrchestrateRequest, OwnerOrchestrateRequestUnimplemented,
    OwnerOrchestrateUnimplementedReason, RefreshRepositoryIndexOrder, RepositoryIndexRefreshed,
    RetireRoleOrder, RoleCreated, RoleCreationRejected, RoleCreationRejectionReason,
    RoleIdentifier, RoleRetired, WirePath,
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

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn round_trip_request(request: OwnerOrchestrateRequest) -> OwnerOrchestrateRequest {
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

fn round_trip_reply(reply: OwnerOrchestrateReply) -> OwnerOrchestrateReply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::completed(NonEmpty::single(SubReply::Ok { payload: reply })),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok { payload, .. } => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply operation, got {other:?}"),
    }
}

#[test]
fn owner_orchestrate_requests_round_trip() {
    let create = OwnerOrchestrateRequest::Create(CreateRoleOrder {
        role: role(),
        harness: HarnessKind::Codex,
    });
    assert_eq!(round_trip_request(create.clone()), create);

    let retire = OwnerOrchestrateRequest::Retire(RetireRoleOrder { role: role() });
    assert_eq!(round_trip_request(retire.clone()), retire);

    let refresh = OwnerOrchestrateRequest::Refresh(RefreshRepositoryIndexOrder {});
    assert_eq!(round_trip_request(refresh.clone()), refresh);
}

#[test]
fn owner_orchestrate_replies_round_trip() {
    let created = OwnerOrchestrateReply::RoleCreated(RoleCreated {
        role: role(),
        harness: HarnessKind::Codex,
        report_repository_path: repository_path(),
        report_lane_path: lane_path(),
    });
    assert_eq!(round_trip_reply(created.clone()), created);

    let retired = OwnerOrchestrateReply::RoleRetired(RoleRetired { role: role() });
    assert_eq!(round_trip_reply(retired.clone()), retired);

    let rejected = OwnerOrchestrateReply::RoleCreationRejected(RoleCreationRejected {
        role: role(),
        reason: RoleCreationRejectionReason::RoleAlreadyExists,
    });
    assert_eq!(round_trip_reply(rejected.clone()), rejected);

    let refreshed = OwnerOrchestrateReply::RepositoryIndexRefreshed(RepositoryIndexRefreshed {
        repositories: 7,
    });
    assert_eq!(round_trip_reply(refreshed.clone()), refreshed);

    let unimplemented = OwnerOrchestrateReply::OwnerOrchestrateRequestUnimplemented(
        OwnerOrchestrateRequestUnimplemented {
            operation: OwnerOperationKind::Create,
            reason: OwnerOrchestrateUnimplementedReason::NotBuiltYet,
        },
    );
    assert_eq!(round_trip_reply(unimplemented.clone()), unimplemented);
}

#[test]
fn owner_orchestrate_operations_encode_as_contract_local_nota_heads() {
    use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};

    let operation = OwnerOrchestrateRequest::Refresh(RefreshRepositoryIndexOrder {});
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
    let decoded =
        owner_signal_persona_orchestrate::OwnerOrchestrateChannelRequest::decode(&mut decoder)
            .expect("decode");
    assert_eq!(
        decoded.payloads().head().operation_kind(),
        OwnerOperationKind::Refresh
    );
}

#[test]
fn owner_orchestrate_request_exposes_contract_owned_operation_kind() {
    let create = OwnerOrchestrateRequest::Create(CreateRoleOrder {
        role: role(),
        harness: HarnessKind::Codex,
    });
    assert_eq!(create.operation_kind(), OwnerOperationKind::Create);

    let retire = OwnerOrchestrateRequest::Retire(RetireRoleOrder { role: role() });
    assert_eq!(retire.operation_kind(), OwnerOperationKind::Retire);

    let refresh = OwnerOrchestrateRequest::Refresh(RefreshRepositoryIndexOrder {});
    assert_eq!(refresh.operation_kind(), OwnerOperationKind::Refresh);
}
