use meta_signal_orchestrate::{
    ApplicationFailure, ApplicationFailureReason, ApplicationSuccess, CreateRoleOrder,
    DownstreamComponent, DurationNanos, Frame, FrameBody, HarnessKind, LaneAlreadyRegistered,
    LaneAlreadyRegisteredResolution, LaneAssignment, LaneAuthority, LaneAuthorityChange,
    LaneAuthoritySet, LaneDetails, LaneIdentifier, LaneOwner, LaneProjection, LaneRegistered,
    LaneRegistration, LaneRegistrationMode, LaneRegistrationRequest, LaneResourceClaim, LaneRetired,
    LaneStatus, LaneUnregistered, LaneUnregistrationRequest, MetaOperationKind, MetaOrchestrateReply,
    MetaOrchestrateRequest,
    MetaOrchestrateRequestUnimplemented, MetaOrchestrateUnimplementedReason, PartialApplied,
    RefreshRepositoryIndexOrder, RepositoryIndexRefreshed, RetireRoleOrder, Retirement, Role,
    RoleCreated, RoleCreationRejected, RoleCreationRejectionReason, RoleIdentifier, RoleRetired,
    RoleToken, ScopeReason, ScopeReference, SessionIdentifier, TimestampNanos, WirePath,
    WorktreeIndexRefreshed,
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

fn session() -> SessionIdentifier {
    SessionIdentifier::from_camel_case_name("SessionLaneProtocolContracts").expect("session")
}

fn lane_details() -> LaneDetails {
    LaneDetails::from_text("session lane protocol contract worker").expect("lane details")
}

fn role_vector() -> Role {
    Role::try_new(vec![
        RoleToken::from_text("PersonaSignal").expect("role token"),
        RoleToken::from_text("Designer").expect("role token"),
    ])
    .expect("role vector")
}

fn lane_assignment() -> LaneAssignment {
    LaneAssignment {
        session: session(),
        lane: lane(),
        owner: LaneOwner {
            role: role_vector(),
            authority: LaneAuthority::Structural,
        },
        details: lane_details(),
    }
}

fn lane_registration_request(mode: LaneRegistrationMode) -> LaneRegistrationRequest {
    LaneRegistrationRequest {
        assignment: lane_assignment(),
        mode,
    }
}

fn lane_registration() -> LaneRegistration {
    LaneRegistration {
        assignment: lane_assignment(),
        registered_at: TimestampNanos::new(1_730_000_010_000_000_000),
        status: LaneStatus::Active,
    }
}

fn lane_projection() -> LaneProjection {
    LaneProjection {
        registration: lane_registration(),
        resource_claims: vec![LaneResourceClaim {
            scope: ScopeReference::Path(repository_path()),
            reason: ScopeReason::from_text("contract claim held by active lane")
                .expect("claim reason"),
            claimed_at: TimestampNanos::new(1_730_000_011_000_000_000),
        }],
        observed_at: TimestampNanos::new(1_730_000_012_000_000_000),
        age: DurationNanos::new(2_000_000_000),
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

    let register = MetaOrchestrateRequest::Register(lane_registration_request(
        LaneRegistrationMode::Fresh,
    ));
    assert_eq!(round_trip_request(register.clone()), register);

    let unregister = MetaOrchestrateRequest::Unregister(LaneUnregistrationRequest {
        session: session(),
        lane: lane(),
        details: lane_details(),
    });
    assert_eq!(round_trip_request(unregister.clone()), unregister);

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

    let refreshed =
        MetaOrchestrateReply::RepositoryIndexRefreshed(RepositoryIndexRefreshed::new(7));
    assert_eq!(round_trip_reply(refreshed.clone()), refreshed);

    let registered = MetaOrchestrateReply::LaneRegistered(LaneRegistered {
        registration: lane_registration(),
    });
    assert_eq!(round_trip_reply(registered.clone()), registered);

    let already_registered_fresh = MetaOrchestrateReply::LaneAlreadyRegistered(
        LaneAlreadyRegistered {
            requested: lane_registration_request(LaneRegistrationMode::Fresh),
            active: lane_projection(),
            resolution: LaneAlreadyRegisteredResolution::FreshConflict,
        },
    );
    assert_eq!(
        round_trip_reply(already_registered_fresh.clone()),
        already_registered_fresh
    );

    let already_registered_recovery = MetaOrchestrateReply::LaneAlreadyRegistered(
        LaneAlreadyRegistered {
            requested: lane_registration_request(LaneRegistrationMode::Recovery),
            active: lane_projection(),
            resolution: LaneAlreadyRegisteredResolution::RecoveryInherited,
        },
    );
    assert_eq!(
        round_trip_reply(already_registered_recovery.clone()),
        already_registered_recovery
    );

    let lane_unregistered = MetaOrchestrateReply::LaneUnregistered(LaneUnregistered {
        session: session(),
        lane: lane(),
        ended_at: TimestampNanos::new(1_730_000_013_000_000_000),
        details: lane_details(),
    });
    assert_eq!(round_trip_reply(lane_unregistered.clone()), lane_unregistered);

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
#[cfg(feature = "nota-text")]
fn meta_orchestrate_operations_encode_as_contract_local_nota_heads() {
    use nota::{NotaEncode, NotaSource};

    let operation = MetaOrchestrateRequest::Refresh(RefreshRepositoryIndexOrder {});
    let text = operation.into_request().to_nota();

    assert_eq!(text, "(Refresh ())");
    assert!(!text.contains("Mutate"));
    assert!(!text.contains("Retract"));

    let decoded = NotaSource::new(&text)
        .parse::<meta_signal_orchestrate::ChannelRequest>()
        .expect("decode");
    assert_eq!(
        decoded.payloads().head().operation_kind(),
        MetaOperationKind::Refresh
    );
}

#[test]
#[cfg(feature = "nota-text")]
fn index_refresh_replies_round_trip_through_schema_derived_nota() {
    use meta_signal_orchestrate::schema::lib as generated;
    use nota::{NotaEncode, NotaSource};

    let repository_refreshed = RepositoryIndexRefreshed::new(7);
    let repository_text = repository_refreshed.to_nota();
    let repository_decoded = NotaSource::new(&repository_text)
        .parse::<RepositoryIndexRefreshed>()
        .expect("decode repository refresh");
    assert_eq!(repository_decoded, repository_refreshed);
    assert_eq!(repository_decoded.repositories(), 7);
    assert_eq!(
        repository_text,
        generated::RepositoryIndexRefreshed::new(7).to_nota()
    );

    let worktree_refreshed = WorktreeIndexRefreshed::new(11);
    let worktree_text = worktree_refreshed.to_nota();
    let worktree_decoded = NotaSource::new(&worktree_text)
        .parse::<WorktreeIndexRefreshed>()
        .expect("decode worktree refresh");
    assert_eq!(worktree_decoded, worktree_refreshed);
    assert_eq!(worktree_decoded.worktrees(), 11);
    assert_eq!(
        worktree_text,
        generated::WorktreeIndexRefreshed::new(11).to_nota()
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

    let register = MetaOrchestrateRequest::Register(lane_registration_request(
        LaneRegistrationMode::Fresh,
    ));
    assert_eq!(register.operation_kind(), MetaOperationKind::Register);

    let unregister = MetaOrchestrateRequest::Unregister(LaneUnregistrationRequest {
        session: session(),
        lane: lane(),
        details: lane_details(),
    });
    assert_eq!(unregister.operation_kind(), MetaOperationKind::Unregister);

    let set_authority = MetaOrchestrateRequest::SetAuthority(LaneAuthorityChange {
        lane: lane(),
        authority: LaneAuthority::Support,
    });
    assert_eq!(
        set_authority.operation_kind(),
        MetaOperationKind::SetAuthority
    );
}
