use lightning::io::Read;
use lightning::ln::msgs::DecodeError;
use lightning::onion_message::{CustomOnionMessageContents, CustomOnionMessageHandler};
use lightning::util::ser::{Writeable, Writer};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub(crate) struct UserOnionMessageContents {
	pub(crate) tlv_type: u64,
	pub(crate) data: Vec<u8>,
}

impl CustomOnionMessageContents for UserOnionMessageContents {
	fn tlv_type(&self) -> u64 {
		self.tlv_type
	}
}

impl Writeable for UserOnionMessageContents {
	fn write<W: Writer>(&self, w: &mut W) -> Result<(), std::io::Error> {
		w.write_all(&self.data)
	}
}

// An extremely basic message handler needed for our integration tests.
#[derive(Clone)]
pub(crate) struct OnionMessageHandler {
	pub(crate) messages: Arc<Mutex<VecDeque<UserOnionMessageContents>>>,
}

impl CustomOnionMessageHandler for OnionMessageHandler {
	type CustomMessage = UserOnionMessageContents;

	fn handle_custom_message(&self, msg: Self::CustomMessage) -> Option<UserOnionMessageContents> {
		println!("Received a new custom message!");
		self.messages.lock().unwrap().push_back(msg.clone());
		Some(msg)
	}

	fn read_custom_message<R: Read>(
		&self, _message_type: u64, _buffer: &mut R,
	) -> Result<Option<Self::CustomMessage>, DecodeError> {
		unimplemented!();
	}
}
