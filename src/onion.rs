use lightning::onion_message::CustomOnionMessageContents;
use lightning::util::ser::{Writeable, Writer};

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
