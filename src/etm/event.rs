use std::error::Error;

use crate::device::Device;

/// ETM Allows up to 4 event fields.
#[derive(Debug)]
pub struct Event {
    event1: u8,
    event2: u8,
    event3: u8,
    event4: u8,
}

impl Event {
    fn from_hex(combination: u32) -> Self {
        let event1 = (combination >> 24) as u8;
        let event2 = (combination << 8 >> 24) as u8;
        let event3 = (combination << 16 >> 24) as u8;
        let event4 = (combination << 24 >> 24) as u8;
        Event {
            event1,
            event2,
            event3,
            event4,
        }
    }

    fn to_hex(&self) -> u32 {
        let combination = self.event1 as u32 << 24;
    }
}

fn get_events(d: &Device) -> Result<Event, Box<dyn Error>> {
    let event_combination: u32 = d.get_from_hex("event")?;
    Ok(Event::from_hex(event_combination))
}

fn set_events(d: &Device, events: &Event) -> Result<(), Box<dyn Error>> {
    todo!()
}
