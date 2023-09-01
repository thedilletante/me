
mod control {
    tonic::include_proto!("proxy");

    use std::collections::{HashMap, HashSet};
    use std::io::Cursor;
    use tokio::sync::Mutex;
    use tonic::{Request, Response, Status};
    use webrtc::sdp::SessionDescription;
    use log::{trace, debug};

    use crate::control::proxy_server::Proxy;

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct SessionId(String);

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct LegId(String);

    #[derive(Debug)]
    struct Session {
        leg_ids: HashSet<LegId>,
    }

    impl Session {
        fn new() -> Self {
            Session {
                leg_ids: HashSet::new(),
            }
        }
    }

    #[derive(Debug)]
    struct Leg {

    }

    impl Leg {

        fn new() -> Self {
            Leg {
            }
        }

        async fn set_remote_description(&self, _: SessionDescription) {
            todo!()
        }

        async fn set_local_description(&self, _: SessionDescription) {
            todo!()
        }

    }


    pub(crate) struct ProxyService {
        sessions: Mutex<HashMap<SessionId, Session>>,
        legs: Mutex<HashMap<LegId, Leg>>,
    }

    impl ProxyService {
        pub(crate) fn new() -> Self {
            ProxyService {
                sessions: Mutex::new(HashMap::new()),
                legs: Mutex::new(HashMap::new()),
            }
        }

        async fn generate_unique_session_id(&self) -> Result<SessionId, String> {
            // Generate a unique session ID using UUID checking that it does not clash with any existing session
            let sessions = self.sessions.lock().await;
            for _ in 0..100 {
                let session_id = SessionId(uuid::Uuid::new_v4().to_string());
                if !sessions.contains_key(&session_id) {
                    trace!("Generated unique session ID: {:?}", session_id);
                    return Ok(session_id);
                }
                trace!("Session ID clash ({:?}), retrying", session_id);
            }

            trace!("Failed to generate unique session ID using 100 attempts");
            Err("Failed to generate unique session ID".to_string())
        }

        async fn generate_unique_leg_id(&self) -> Result<LegId, String> {
            // Generate a unique leg ID using UUID checking that it does not clash with any existing leg
            let legs = self.legs.lock().await;
            for _ in 0..100 {
                let leg_id = LegId(uuid::Uuid::new_v4().to_string());
                if !legs.contains_key(&leg_id) {
                    trace!("Generated unique leg ID: {:?}", leg_id);
                    return Ok(leg_id);
                }
                trace!("Leg ID clash ({:?}), retrying", leg_id);
            }

            trace!("Failed to generate unique leg ID using 100 attempts");
            Err("Failed to generate unique leg ID".to_string())
        }
    }

    #[tonic::async_trait]
    impl Proxy for ProxyService {

        async fn create_session(&self, _: Request<CreateSessionRequest>) -> Result<Response<CreateSessionResponse>, Status> {
            match self.generate_unique_session_id().await {
                Ok(session_id) => {
                    let session = Session::new();
                    let mut sessions = self.sessions.lock().await;
                    debug!("Created session, {:?}, {:?}", session_id, session);
                    sessions.insert(session_id.clone(), session);
                    let response = CreateSessionResponse {
                        session_id: session_id.0,
                    };
                    Ok(Response::new(response))
                },
                Err(err) => {
                    debug!("Failed to create session: {}", err);
                    Err(Status::internal(err))
                }
            }
        }

        async fn destroy_session(&self, request: Request<DestroySessionRequest>) -> Result<Response<DestroySessionResponse>, Status> {
            todo!()
        }

        async fn create_leg(&self, request: Request<CreateLegRequest>) -> Result<Response<CreateLegResponse>, Status> {
            // Find session
            let session_id = SessionId(request.get_ref().session_id.clone());
            let mut sessions = self.sessions.lock().await;
            let session = match sessions.get_mut(&session_id) {
                Some(session) => session,
                None => {
                    debug!("Session not found: {:?}", session_id);
                    return Err(Status::not_found("Session not found"))
                }
            };
            match self.generate_unique_leg_id().await {
                Ok(leg_id) => {
                    let leg = Leg::new();
                    let mut legs = self.legs.lock().await;
                    debug!("Created leg, {:?}, {:?}", leg_id, leg);
                    legs.insert(leg_id.clone(), leg);
                    session.leg_ids.insert(leg_id.clone());
                    let response = CreateLegResponse {
                        leg_id: leg_id.0,
                    };
                    Ok(Response::new(response))
                },
                Err(err) => {
                    debug!("Failed to create leg: {}", err);
                    Err(Status::internal(err))
                }
            }
        }

        async fn destroy_leg(&self, request: Request<DestroyLegRequest>) -> Result<Response<DestroyLegResponse>, Status> {
            todo!()
        }

        async fn set_remote_description(&self, request: Request<SetRemoteDescriptionRequest>) -> Result<Response<SetRemoteDescriptionResponse>, Status> {
            todo!()
        }

        async fn set_local_description(&self, request: Request<SetLocalDescriptionRequest>) -> Result<Response<SetLocalDescriptionResponse>, Status> {
            let mut reader = Cursor::new(request.get_ref().sdp.as_bytes());
            let sdp = match SessionDescription::unmarshal(&mut reader) {
                Ok(sdp) => sdp,
                Err(err) => {
                    debug!("Failed to parse SDP: {}", err);
                    return Err(Status::invalid_argument(err.to_string()))
                }
            };

            let mut legs = self.legs.lock().await;
            if let Some(leg) = legs.get_mut(&LegId(request.get_ref().leg_id.clone())) {
                leg.set_local_description(sdp).await;
                Ok(Response::new(SetLocalDescriptionResponse {}))
            } else {
                debug!("Leg not found: {}", request.get_ref().leg_id);
                Err(Status::not_found("Leg not found"))
            }
        }

        async fn get_local_description(&self, request: Request<GetLocalDescriptionRequest>) -> Result<Response<GetLocalDescriptionResponse>, Status> {
            todo!()
        }
    }
}

use log::info;


#[tokio::main]
async fn main() {
    env_logger::init();
    // Start ProxyService as a gRPC server
    let proxy_service = control::ProxyService::new();
    let svc = control::proxy_server::ProxyServer::new(proxy_service);
    let addr = "0.0.0.0:7777".parse().unwrap();
    info!("Starting gRPC server on {}", addr);
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .unwrap();
}
