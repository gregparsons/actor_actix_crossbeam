

pub enum MsgEnum{
	// Pingy,
	// Pongy,
	// StartCounter(Recipient<MsgEnum>),
	StartCounter,
	LogThis(String),
}
impl actix::Message for MsgEnum{
	type Result = ();
}