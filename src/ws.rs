use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpResponse, HttpRequest};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use crate::db::models::BikeData;
use std::collections::HashMap;

const HEARTBEAT_INTERVAL: Duration =  Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub trait UpdateData: Send {}

impl UpdateData for BikeData {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Update<T: UpdateData>(pub T);


#[derive(Message)]
#[rtype(usize)]
pub struct Connect<T: UpdateData> {
    bike_id: i32,
    addr: Recipient<Update<T>>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

pub struct UpdateServer {
    bike_listeners: HashMap<i32, HashMap<usize, Recipient<Update<BikeData>>>>,
    count: usize,
}

impl UpdateServer {
    pub fn new() -> Self {
        UpdateServer {
            bike_listeners: HashMap::new(),
            count: 0,
        }
    }

    fn send_update(&mut self, bike_id: i32, bike_data: BikeData) {
        if let Some(bike_listeners) = self.bike_listeners.get(&bike_id) {
            for (_, addr) in bike_listeners {
                addr.do_send(Update(bike_data.clone()));
            }
        }
    }
}

impl Actor for UpdateServer {
    type Context = Context<Self>;
}

impl Handler<Connect<BikeData>> for UpdateServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect<BikeData>, _: &mut Context<Self>) -> Self::Result {
        let id = self.count;
        self.count += 1;

        let bike_id = msg.bike_id;

        info!("adding listener: {} for bike: {}", id, bike_id);

        self.bike_listeners.entry(bike_id)
            .or_insert(HashMap::new())
            .insert(id, msg.addr);

        id
    }
}

impl Handler<Update<BikeData>> for UpdateServer {
    type Result = ();

    fn handle(&mut self, msg: Update<BikeData>, _: &mut Context<Self>) {
        info!("bike server recv data");
        let bike_id = msg.0.bike;
        self.send_update(bike_id, msg.0);
    }
}

impl Handler<Disconnect> for UpdateServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        let id = msg.id;

        info!("removing listener: {}", id);

        for (_, listeners) in self.bike_listeners.iter_mut() {
            listeners.remove(&id);
        }
    }
}

struct WsBikeUpdates {
    id: usize,
    bike_id: i32,
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
                addr: addr.recipient(),
                bike_id: self.bike_id,
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

impl Handler<Update<BikeData>> for WsBikeUpdates {
    type Result = ();

    fn handle(&mut self, msg: Update<BikeData>, ctx: &mut Self::Context) {
        info!("sending data");
        ctx.text(serde_json::to_string(&msg.0).unwrap());
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
            bike_id,
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    );
    info!("{:?}", res);
    res
}