use chrono::Duration;
use colored::*;
use std::{fmt};

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
	Top,
	Bottom,
	Center,
}

#[derive(Debug, Eq, PartialEq)]
 pub struct Notification {
	pub size: u32,
	pub color: (u8, u8, u8),
	pub position: Position,
	pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
	Remainder(&'a str),
	Registration(Duration),
	Appointment(&'a str),
	Holiday,
}


impl fmt::Display for Notification {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {}, {})",
			self.position.to_string(), self.size, self.content.truecolor(self.color.0,self.color.1,self.color.2))
	}
}

impl ToString for Position {

	fn to_string(&self) -> String {
		match self {
			Position::Top => "Top".to_string(),
			Position::Bottom => "Bottom".to_string(),
			Position::Center => "Center".to_string(),
		}
	}
}

impl<'a> Event <'_> {
	pub fn notify(&self) -> Notification {
		match self {
			&Event::Remainder(content) => Notification {
				size: 50,
				color: (50, 50, 50),
				position: Position::Bottom,
				content:  content.to_string(),
			},
			&Event::Appointment(content) => Notification {
				size: 100,
				color: (200, 200, 3),
				position: Position::Center,
				content: format!("{content}"),
			},
			&Event::Registration(duration) => Notification {
				size: 30,
				color: (255, 2, 22),
				position: Position::Top,
				content: format!("You have {}H:{}M:{}S left before the registration ends", duration.num_hours(),duration.num_minutes() % 60, duration.num_seconds() % 60),
			},
			Event::Holiday => Notification {
				size: 25,
				color: (0, 255, 0),
				position: Position::Top,
				content: "Enjoy your holiday".to_string(),
			},
		}
	}
}