use std::str::FromStr;

use crate::{data::EventData, response::APMQueryResponse, utils};



#[derive(Debug, Clone)]
pub enum QueryType {
    Throughput,
    Error,
    Latency,
}

impl FromStr for QueryType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "throughput" => Ok(QueryType::Throughput),
            "error" => Ok(QueryType::Error),
            "latency" => Ok(QueryType::Latency),
            _ => Err(
                format!("Invalid query type: {}", s),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct APMQueryRequest {
    pub endpoint: String,
    pub query_type: QueryType,
    pub from: i64,
    pub to: i64,
}



impl APMQueryRequest {
   
    pub fn from_str(input: &str) -> Result<Self, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 5 {
            return Err( 
                 "Invalid input format. Expected 5 parts: method endpoint, query_type, from, to.".to_string(),
            );
        }

        let endpoint = parts[0..2].join(" ");
        let query_type = QueryType::from_str(parts[2])?;

        let from = utils::timestamp_to_seconds(parts[3].to_owned())?;
        let to = utils::timestamp_to_seconds(parts[4].to_owned())?;
        

        Ok(APMQueryRequest {
            endpoint,
            query_type,
            from,
            to,
        })
    }

    pub fn query(
        &self,
        data_store: &mut EventData
    ) -> Result<APMQueryResponse, String> {
        match self.query_type {
            QueryType::Throughput => APMQueryResponse::query_throughput(data_store,
                self.from, self.to,&self.endpoint),
            QueryType::Error => APMQueryResponse::query_error(data_store,
                 self.from, self.to,&self.endpoint),
            QueryType::Latency => APMQueryResponse::query_latency(data_store,
                self.from, self.to,&self.endpoint),
        }
    }
}



