
mod control {
    tonic::include_proto!("proxy");

    use std::collections::HashMap;
    use std::io::Cursor;
    use tokio::sync::Mutex;
    use tonic::{Request, Response, Status};
    use webrtc::sdp::SessionDescription;
    use log::{trace, debug};

    use crate::control::proxy_server::Proxy;

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct SessionId(String);

    #[derive(Debug)]
    struct Session {

    }

    impl Session {
        fn new() -> Self {
            Session {}
        }

        async fn process_offer(&mut self, offer: SessionDescription) -> Result<SessionDescription, String> {
            todo!()
        }
    }

    pub(crate) struct ProxyService {
        sessions: Mutex<HashMap<SessionId, Session>>,
    }

    impl ProxyService {
        pub(crate) fn new() -> Self {
            ProxyService {
                sessions: Mutex::new(HashMap::new()),
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

        async fn process_offer(&self, request: Request<ProcessOfferRequest>) -> Result<Response<ProcessOfferResponse>, Status> {
            let mut reader = Cursor::new(request.get_ref().sdp.as_bytes());
            let sdp = match SessionDescription::unmarshal(&mut reader) {
                Ok(sdp) => sdp,
                Err(err) => {
                    debug!("Failed to parse SDP: {}", err);
                    return Err(Status::invalid_argument(err.to_string()))
                }
            };

            let mut sessions = self.sessions.lock().await;
            if let Some(session) = sessions.get_mut(&SessionId(request.get_ref().session_id.clone())) {
                debug!("Processing offer for session: {:?}", session);
                return match session.process_offer(sdp).await {
                    Ok(sdp) => {
                        Ok(Response::new(ProcessOfferResponse {
                            sdp: sdp.marshal(),
                        }))
                    }
                    Err(err) => {
                        debug!("Failed to process offer: {}", err);
                        Err(Status::internal(err))
                    }
                }
            } else {
                debug!("Session not found: {}", request.get_ref().session_id);
                Err(Status::not_found("Session not found"))
            }
        }

        async fn process_answer(&self, request: Request<ProcessAnswerRequest>) -> Result<Response<ProcessAnswerResponse>, Status> {
            todo!()
        }

        async fn destroy_session(&self, request: Request<DestroySessionRequest>) -> Result<Response<()>, Status> {
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
