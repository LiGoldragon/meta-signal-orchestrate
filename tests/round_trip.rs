use meta_signal_orchestrate::{
    Frame, FrameBody, MetaOperationKind, MetaOrchestrateReply, MetaOrchestrateRequest,
    RefreshRepositoryIndexOrder, RepositoryIndexRefreshed,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, LogVariant, NonEmpty, Reply, RootCode,
    SessionEpoch, SubReply, VariantCode, WireRoute,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn round_trip_request(request: MetaOrchestrateRequest) -> MetaOrchestrateRequest {
    let route = WireRoute::try_from_log_variant(request.log_variant()).expect("request route");
    let frame = request.into_frame(exchange()).expect("request frame");
    assert_eq!(frame.short_header().route(), route);

    let bytes = frame.encode_length_prefixed().expect("encode frame");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode frame");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn round_trip_reply(reply: MetaOrchestrateReply) -> MetaOrchestrateReply {
    let frame = Frame::new(
        WireRoute::new(RootCode::new(0), VariantCode::new(0)),
        FrameBody::Reply {
            exchange: exchange(),
            reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
        },
    );
    let bytes = frame.encode_length_prefixed().expect("encode frame");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode frame");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply frame, got {other:?}"),
    }
}

#[test]
fn literal_refresh_request_frame_round_trips() {
    let request = MetaOrchestrateRequest::Refresh(RefreshRepositoryIndexOrder {});
    assert_eq!(round_trip_request(request.clone()), request);
    assert_eq!(request.operation_kind(), MetaOperationKind::Refresh);
}

#[test]
fn literal_refresh_reply_frame_round_trips() {
    let reply = MetaOrchestrateReply::RepositoryIndexRefreshed(RepositoryIndexRefreshed::new(7));
    assert_eq!(round_trip_reply(reply.clone()), reply);
}
