#[allow(unused_doc_comments)]
use std::fmt;

pub struct Conversation {
	pub display_name: String,
	pub chat_identifier: String,
	pub latest_text: String,
	pub has_unread: bool,
	pub addresses: String, // Or maybe vec?
	pub is_selected: bool
}

impl Conversation {
	pub fn from_json(val: &serde_json::Map<String, serde_json::Value>) -> Conversation {
		// it's so ugly :(
		Conversation {
			display_name: val["display_name"].as_str().unwrap().to_owned(),
			chat_identifier: val["chat_identifier"].as_str().unwrap().to_owned(),
			latest_text: val["latest_text"].as_str().unwrap().to_owned(),
			has_unread: val["has_unread"].as_bool().unwrap(),
			addresses: val["addresses"].as_str().unwrap().to_owned().to_owned(),
			is_selected: false,
		}
	}
}

impl fmt::Debug for Conversation {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Conversation")
			.field("display_name", &self.display_name)
			.field("chat_identifier", &self.chat_identifier)
			.field("has_unread", &self.has_unread)
			.field("addresses", &self.addresses)
			.field("is_selected", &self.is_selected)
			.finish()
	}
}

pub struct Message {
	pub guid: String,
	pub date_read: i64,
	pub date: i64,
	pub balloon_bundle_id: String,
	pub cache_has_attachments: bool,
	pub imsg: bool,
	pub is_from_me: bool,
	pub subject: String,
	pub text: String,
	pub associated_message_guid: String,
	pub associated_message_type: i16,
	pub sender: Option<String>,
}

impl Message {
	pub fn from_json(val: &serde_json::Map<String, serde_json::Value>) -> Message {
		Message {
			guid: val["guid"].as_str().unwrap().to_owned(),
			date_read: val["date_read"].as_i64().unwrap(),
			date: val["date"].as_i64().unwrap(),
			balloon_bundle_id: val["balloon_bundle_id"].as_str().unwrap().to_owned(),
			cache_has_attachments: val["cache_has_attachments"].as_bool().unwrap(),
			imsg: val["service"].as_str().unwrap() == String::from("iMessage"),
			is_from_me: val["is_from_me"].as_bool().unwrap(),
			subject: val["subject"].as_str().unwrap().to_owned(),
			text: val["text"].as_str().unwrap().to_owned(),
			associated_message_guid: val["associated_message_guid"].as_str().unwrap().to_owned(),
			associated_message_type: val["associated_message_type"].as_i64().unwrap() as i16,
			sender: if val.contains_key("sender") { 
				Some(val["sender"].as_str().unwrap().to_owned()) 
			} else { 
				None 
			},
		}
	}
}

impl fmt::Debug for Message {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Message")
			.field("guid", &self.guid)
			.field("date_read", &self.date_read)
			.field("date", &self.date)
			.field("balloon_bundle_id", &self.balloon_bundle_id)
			.field("cache_has_attachments", &self.cache_has_attachments)
			.field("imsg", &self.imsg)
			.field("is_from_me", &self.is_from_me)
			.field("subject", &self.subject)
			.field("text", &self.text)
			.field("associated_message_guid", &self.associated_message_guid)
			.field("associated_message_type", &self.associated_message_type)
			.field("sender", &self.sender)
			.finish()
	}
}
