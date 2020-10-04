
use crate::actor_tools::MsgEnum;

pub struct LoggerActor;
impl actix::Actor for LoggerActor {
	type Context = actix::Context<Self>;

	fn started(&mut self, _ctx: &mut Self::Context) {
		println!("[PrinterActor] started");
	}
}
impl actix::Handler<MsgEnum> for LoggerActor {
	type Result = (); // Result<bool, std::io::Error>;
	fn handle(&mut self, msg: MsgEnum, _ctx: &mut Self::Context) -> Self::Result {

		match msg {
			MsgEnum::LogThis(m) => {
				println!("[PrinterActor] Received: {}", &m);
			},
			_ => {
				println!("[PrinterActor] unhandled message received");
			}
		}
	}
}