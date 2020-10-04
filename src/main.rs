mod heartbeat_actor;
mod actor_tools;
mod logger_actor;/*
	Learning Actix actors.
	Ref:	https://actix.rs/book/actix/sec-2-actor.html
	9/23/2020

*/


use actix::{Actor};
use actor_tools::MsgEnum;
use logger_actor::LoggerActor;
use heartbeat_actor::HeartbeatActor;

fn main(){

	println!("[main] before system");
	actix::System::run(||{

		let logger = LoggerActor.start();
		logger.do_send(MsgEnum::LogThis("hello from main".to_string()));

		// Get a Recipient for the printer to send to the counter to be able to send
		// a message directly to the printer.
		// https://github.com/actix/actix/blob/master/examples/ring.rs

		let counter = HeartbeatActor {
			logger: logger.clone().recipient(),
		};

		let counter_addr = counter.start();
		// &counter_addr.do_send(MsgEnum::StartCounter(c_clone.recipient()));
		&counter_addr.do_send(MsgEnum::StartCounter);

		// actix::System::current().stop();

	}).expect("System didn't start.");
	println!("[main] after system");

	// let cnt_addr = SyncArbiter::start(1,move ||
	// 	CounterActor{
	// 		name:String::from("counter001"),
	// 		printer_actor:printer_addr.clone(),
	// 	}
	// );
	// let _ = cnt_addr.send(MsgEnum::StartCounter).await;

}
