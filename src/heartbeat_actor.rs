
use actix::{Actor, Recipient, Handler};
use crate::actor_tools::MsgEnum;
use crossbeam_channel::{tick, Receiver};
use std::time::{Duration, Instant};

pub struct HeartbeatActor {
	pub logger:Recipient<MsgEnum>,
}
impl Actor for HeartbeatActor {
	type Context = actix::Context<Self>;
	fn started(&mut self, _ctx: &mut Self::Context) {
		println!("[CounterActor] started");
	}
}
impl Handler<MsgEnum> for HeartbeatActor {
	type Result = (); // Result<bool, std::io::Error>;
	fn handle(&mut self, msg: MsgEnum, _ctx: &mut Self::Context) -> Self::Result {
		// println!("[CounterActor...MsgEnum] received message: {:?}", msg);
		match msg {
			// MsgEnum::StartCounter(recipient) => {
			MsgEnum::StartCounter => {

				println!("[CounterActor] received StartCounter message");

				let recip = self.logger.clone();


				let ticker: Receiver<Instant> = tick(Duration::from_millis(500));

				std::thread::spawn(move || {
					loop {
						crossbeam_channel::select! {
							recv(ticker) -> _ => {
								let _ = recip.do_send(MsgEnum::LogThis("hello from counter".to_string()));
							}
						}
					}
				});


			},
			_ => {
				println!("[CounterActor.handle] unknown message")
			}
		}
	}
}
