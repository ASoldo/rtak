use actix::prelude::*;
use std::collections::HashSet;

// Message to broadcast text to WebSocket sessions
#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastMessage(pub String);

// Our broadcaster actor holds connected sessions
pub struct Broadcaster {
    sessions: HashSet<Addr<crate::api::WsSession>>,
}

impl Broadcaster {
    pub fn new() -> Broadcaster {
        Broadcaster {
            sessions: HashSet::new(),
        }
    }
}

impl Actor for Broadcaster {
    type Context = Context<Self>;
}

// Register a session
#[derive(Message)]
#[rtype(result = "()")]
pub struct RegisterSession(pub Addr<crate::api::WsSession>);

impl Handler<RegisterSession> for Broadcaster {
    type Result = ();

    fn handle(&mut self, msg: RegisterSession, _: &mut Context<Self>) {
        self.sessions.insert(msg.0);
    }
}

// Unregister a session
#[derive(Message)]
#[rtype(result = "()")]
pub struct UnregisterSession(pub Addr<crate::api::WsSession>);

impl Handler<UnregisterSession> for Broadcaster {
    type Result = ();

    fn handle(&mut self, msg: UnregisterSession, _: &mut Context<Self>) {
        self.sessions.remove(&msg.0);
    }
}

// Handle broadcast
impl Handler<BroadcastMessage> for Broadcaster {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Context<Self>) {
        for session in &self.sessions {
            session.do_send(crate::api::SendText(msg.0.clone()));
        }
    }
}
