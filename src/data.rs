use std::collections::HashMap;

use crate::utils;



#[derive(Debug, Clone)]
pub struct APIEvent {
    pub endpoint: String,
    pub event_id: u64,
    pub timestamp: String,
    pub latency: f64,
    pub status_code: u16,
    pub is_error: bool,
}

#[derive(Debug, Clone)]
pub struct DataStore {
    pub endpoint: String,
    pub timestamp: i64,
    pub latency: f64,
    pub status_code: u16,
    pub is_error: bool,
}


pub struct EventData{
    pub(crate) data: HashMap<i64, DataStore>,
}

impl EventData {
    
    pub fn insert_event(&mut self, event_data: APIEvent) -> Result<(),String>{
        let time_in_secs = utils::timestamp_to_seconds(event_data.timestamp.clone())?;
       let data = DataStore {
            endpoint: event_data.endpoint,
            timestamp: time_in_secs,
            latency: event_data.latency,
            status_code: event_data.status_code,
            is_error: event_data.is_error

       };
        self.data.insert(time_in_secs, data);
        Ok(())
    }

    pub fn query_event(&mut self, from: i64, to: i64, endpoint: String) -> Result<Vec<DataStore>, String> {
        let data_list = self.data
            .iter() // iterate over key-value pairs
            .filter_map(|(_, value)| { // filter based on timestamp and endpoint
                if value.timestamp >= from && value.timestamp <= to  && endpoint == value.endpoint{
                    Some(value.clone()) 
                } else {
                    None // Otherwise, discard
                }
            })
            .collect::<Vec<DataStore>>(); // Collect into a Vec<DataStore>
    
        Ok(data_list)

    }


    
}