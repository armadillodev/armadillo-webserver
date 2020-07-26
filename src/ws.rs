use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpResponse, HttpRequest};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::Arc;
use crate::db::{data::DbData, Address};

const HEARTBEAT_INTERVAL: Duration =  Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

type SendData = Arc<dyn DbData>;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Update(pub SendData);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    subject: Address,
    recipient: Recipient<Update>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

pub struct UpdateServer {
    listener_map: HashMap<Address, HashMap<usize, Recipient<Update>>>,
    count: usize,
}

impl UpdateServer {
    pub fn new() -> Self {
        UpdateServer {
            listener_map: HashMap::new(),
            count: 0,
        }
    }

    fn send_update(&mut self, data: SendData) {
        if let Some(listeners) = self.listener_map.get(&data.id()) {
            for (_, addr) in listeners {
                addr.do_send(Update(Arc::clone(&data))).expect("failed to send update");
            }
        }
    }
}

impl Actor for UpdateServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for UpdateServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.count;
        self.count += 1;

        let address = msg.subject;

        info!("adding listener: {} for {:?}", id, address);

        self.listener_map.entry(address)
            .or_insert(HashMap::new())
            .insert(id, msg.recipient);

        id
    }
}

impl Handler<Update> for UpdateServer {
    type Result = ();

    fn handle(&mut self, msg: Update, _: &mut Context<Self>) {
        info!("update server recv data");
        self.send_update(msg.0);
    }
}

impl Handler<Disconnect> for UpdateServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        let id = msg.id;

        info!("removing listener: {}", id);

        for (_, listeners) in self.listener_map.iter_mut() {
            listeners.remove(&id);
        }
    }
}

struct WsBikeUpdates {
    id: usize,
    address: Address,
    hb: Instant,
    addr: Addr<UpdateServer>,
}

impl Actor for WsBikeUpdates {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("started web socket");
        self.hb(ctx);

        let addr = ctx.address();
        self.addr
            .send(Connect{
                recipient: addr.recipient(),
                subject: self.address,
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect{ id: self.id });
        Running::Stop
    }
}

impl Handler<Update> for WsBikeUpdates {
    type Result = ();

    fn handle(&mut self, msg: Update, ctx: &mut Self::Context) {
        info!("sending data");
        ctx.text(msg.0.to_packet());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsBikeUpdates {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        info!("Web socket message: {:?}", msg);

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => self.hb = Instant::now(),
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl WsBikeUpdates {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                info!("web socket heartbeat failed");
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

pub async fn ws_bike_updates(
    req: HttpRequest,
    stream: web::Payload,
    bike_id: web::Path<i32>,
    srv: web::Data<Addr<UpdateServer>>,
) -> Result<HttpResponse, Error> {
    let bike_id = bike_id.into_inner();
    let res = ws::start(
        WsBikeUpdates {
            id: 0,
            address: Address::Bike(bike_id),
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    );
    info!("{:?}", res);
    res
}