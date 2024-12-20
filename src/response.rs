use crate::data::EventData;


pub enum APMQueryResponse {
    Throughput((u64, f64)),
    Error((u32, String)),
    Latency((f64, f64,f64)),
}



impl APMQueryResponse {

    pub fn query_throughput(
        data_store: &mut EventData,
        from: i64,
        to: i64,
        endpoint: &str,
    ) -> Result<APMQueryResponse,String> {
        let events = EventData::query_event(data_store, from, to, endpoint.to_string())?;
        let total_requests = events.len() as u64;
        let rps = if total_requests > 0 {
            total_requests as f64 / (to - from + 1) as f64
        } else {
            0.0
        };
    
        Ok(APMQueryResponse::Throughput((total_requests, rps)))
    }
    
    pub fn query_error(
        data_store: &mut EventData,
        from: i64,
        to: i64,
        endpoint: &str,
    ) -> Result<APMQueryResponse,String> {
        let events = EventData::query_event(data_store, from, to, endpoint.to_string())?;
        let total_errors = events.iter().filter(|e| e.is_error).count() as u64;
        let total_requests = events.len() as u64;
    
        let error_rate = if total_requests > 0 {
            (total_errors as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };
    
        Ok(APMQueryResponse::Error((total_errors.try_into().unwrap(), error_rate.to_string())))
    }
    
    pub fn query_latency(
        data_store: &mut EventData,
        from: i64,
        to: i64,
        endpoint: &str,
    ) -> Result<APMQueryResponse,String> {
        let events = EventData::query_event(data_store, from, to, endpoint.to_string())?;
        let latencies: Vec<f64> = events.iter().map(|e| e.latency).collect();
    
        let average_latency = if !latencies.is_empty() {
            latencies.iter().sum::<f64>() / latencies.len() as f64
        } else {
            0.0
        };
    
        let min_latency = latencies.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_latency = latencies.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    
        Ok(APMQueryResponse::Latency((average_latency, min_latency, max_latency)))
    }
    
    

}

