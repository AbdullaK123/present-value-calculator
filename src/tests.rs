

#[cfg(test)]
mod tests {
    use crate::models::{CashFlow, CompoundingType, PresentValueRequest, PresentValueResponse};
    use crate::utils::*;

    #[test]
    fn test_present_value_discrete() {
        // $5,000 in 3 years at 6%
        assert_eq!(
            (present_value_discrete(5000.0, 0.06, 3.0) * 100.0).round() / 100.0,
            4198.10
        );
    }

    #[test]
    fn test_cts_compounding() {
        // $10,000 in 5 years at 7% continuous
        assert_eq!(
            (present_value_cts(10000.0, 0.07, 5.0) * 100.0).round() / 100.0,
            7046.88
        );
    }

    #[test]
    fn test_stream_compounding() {
        // Quiz problem 2: $1,000 in 1yr, $1,500 in 2yr, $2,000 in 3yr at 5%
        let cash_flows = vec![
            CashFlow::new(1000.0, 1.0, CompoundingType::Discrete),
            CashFlow::new(1500.0, 2.0, CompoundingType::Discrete),
            CashFlow::new(2000.0, 3.0, CompoundingType::Discrete)
        ];
        let result = (present_value_stream(cash_flows, 0.05) * 100.0).round() / 100.0;
        assert!((result - 4040.59).abs() < 1.0, "Expected ~4040.59, got {}", result);
    }

    #[test]
    fn test_future_value() {
        // Invest $40,360.27 for 4 years at 5.5%
        let result = (future_value_discrete(40360.27, 0.055, 4.0) * 100.0).round() / 100.0;
        assert!((result - 50000.00).abs() < 1.0, "Expected ~50000.00, got {}", result);
    }

    #[test]
    fn test_real_rate() {
        // 7% nominal, 3% inflation
        assert_eq!(
            (real_rate(0.07, 0.03) * 10000.0).round() / 10000.0,
            0.0388  // More precise Fisher equation: (1.07/1.03 - 1) = 3.88%
        );
    }

    // --- HTTP endpoint tests ---
    use axum::{http::{Request, StatusCode}, body::Body};
    use tower::ServiceExt;
    use tracing::info;

    // for `oneshot`
    
    #[tokio::test]
    async fn health_check_works() {
        let app = crate::app();
        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn present_value_endpoint_works() {
        let app = crate::app();
        let payload = PresentValueRequest {
            cash_flows: vec![
                CashFlow::new(1000.0, 1.0, CompoundingType::Discrete),
                CashFlow::new(1500.0, 2.0, CompoundingType::Discrete),
            ],
            nominal_rate: 0.06,
            inflation_rate: 0.02,
        };
        let body = serde_json::to_vec(&payload).unwrap();
        let request = Request::builder()
            .method("POST")
            .uri("/present-value")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp: PresentValueResponse = serde_json::from_slice(&bytes).unwrap();
        assert!(resp.result > 0.0);
    }
}