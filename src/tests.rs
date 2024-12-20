#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use utils::timestamp_to_seconds;

    use crate::{data::{APIEvent, EventData}, request::{APMQueryRequest, QueryType}, response::APMQueryResponse, utils};


    fn create_sample_events() -> EventData {
        let mut event_data = EventData {
            data: HashMap::new(),
        };

        event_data.insert_event(APIEvent {
            endpoint: "POST /v1/orders".to_string(),
            event_id: 1,
            timestamp: "10:00:00".to_string(),
            latency: 100.0,
            status_code: 200,
            is_error: false,
        }).unwrap();

        event_data.insert_event(APIEvent {
            endpoint: "POST /v1/orders".to_string(),
            event_id: 2,
            timestamp: "10:00:01".to_string(),
            latency: 200.0,
            status_code: 200,
            is_error: false,
        }).unwrap();

        event_data.insert_event(APIEvent {
            endpoint: "POST /v1/orders".to_string(),
            event_id: 3,
            timestamp: "10:00:02".to_string(),
            latency: 300.0,
            status_code: 500,
            is_error: true,
        }).unwrap();

        event_data.insert_event(APIEvent {
            endpoint: "GET /v1/orders/1".to_string(),
            event_id: 4,
            timestamp: "10:00:03".to_string(),
            latency: 400.0,
            status_code: 200,
            is_error: false,
        }).unwrap();

        event_data.insert_event(APIEvent {
            endpoint: "GET /v1/orders/2".to_string(),
            event_id: 5,
            timestamp: "10:00:04".to_string(),
            latency: 500.0,
            status_code: 401,
            is_error: true,
        }).unwrap();

        event_data.insert_event(APIEvent {
            endpoint: "GET /v1/orders/3".to_string(),
            event_id: 6,
            timestamp: "10:00:05".to_string(),
            latency: 600.0,
            status_code: 500,
            is_error: true,
        }).unwrap();

        event_data
    }


    #[test]
    fn test_insert_event() {
        let mut event_data = EventData {
            data: HashMap::new(),
        };

        let event = APIEvent {
            endpoint: "POST /v1/orders".to_string(),
            event_id: 1,
            timestamp: "10:00:00".to_string(),
            latency: 100.0,
            status_code: 200,
            is_error: false,
        };

        assert!(event_data.insert_event(event).is_ok());
        assert_eq!(event_data.data.len(), 1);
    }


    #[test]
    fn test_throughput_query() {
        let mut event_data = create_sample_events();

        let query_request = APMQueryRequest {
            endpoint: "POST /v1/orders".to_string(),
            query_type: QueryType::Throughput,
            from: timestamp_to_seconds("10:00:00".to_string()).unwrap(),
            to: timestamp_to_seconds("10:00:05".to_string()).unwrap(),
        };

        let response = query_request.query(&mut event_data);

        match response {
            Ok(APMQueryResponse::Throughput((total_requests, rps))) => {
                assert_eq!(total_requests, 3);
                assert_eq!(rps, 3.0 / 6.0);
            }
            _ => panic!("Expected throughput response"),
        }
    }

    #[test]
    fn test_error_query() {
        let mut event_data = create_sample_events();

        let query_str = "GET /v1/orders error 10:00:04 10:00:05";
        let query_request = APMQueryRequest::from_str(query_str).unwrap();

        let response = query_request.query(&mut event_data);

        match response {
            Ok(APMQueryResponse::Error((total_errors, error_rate))) => {
                assert_eq!(total_errors, 0);
                assert_eq!(error_rate, "0");
            }
            _ => panic!("Expected error response"),
        }
    }

    #[test]
    fn test_latency_query() {
        let mut event_data = create_sample_events();

        let query_str = "POST /v1/orders latency 10:00:00 10:00:05";
        let query_request = APMQueryRequest::from_str(query_str).unwrap();

        let response = query_request.query( &mut event_data);

        match response {
            Ok(APMQueryResponse::Latency((avg_latency, min_latency, max_latency))) => {
                assert_eq!(avg_latency, 200.0); 
                assert_eq!(min_latency, 100.0);
                assert_eq!(max_latency, 300.0);
            }
            _ => panic!("Expected latency response"),
        }
    }

    #[test]
    fn test_query_no_events() {
        let mut event_data = EventData {
            data: HashMap::new(),
        };
        let query_str = "POST /v1/noevents throughput 10:00:00 10:00:03";
        let query_request = APMQueryRequest::from_str(query_str).unwrap();

        let response = query_request.query(&mut event_data);
        match response {
            Ok(APMQueryResponse::Throughput((total_requests, rps))) => {
                assert_eq!(total_requests, 0);
                assert_eq!(rps, 0.0);
            }
            _ => panic!("Expected throughput response with zero events"),
        }
    }
}
