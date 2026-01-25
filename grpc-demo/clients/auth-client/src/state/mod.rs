use prometheus::{Counter, CounterVec, Gauge, Histogram, HistogramOpts, Registry, opts};
use serde::Serialize;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::time::Instant;

// 自定义指标结构
#[derive(Clone)]
pub struct Metrics {
    pub registry: Arc<Registry>,
    pub http_requests_total: CounterVec,
    pub http_request_duration_seconds: Histogram,
    pub active_connections: Gauge,
    pub custom_counter: Counter,
}

impl Metrics {
    fn new() -> Self {
        let registry = Registry::default();

        // HTTP 请求总数
        let http_requests_total = CounterVec::new(
            opts!("http_requests_total", "Total number of HTTP requests"),
            &["method", "path", "status"],
        )
        .unwrap();

        // HTTP 请求持续时间
        let http_request_duration_seconds = Histogram::with_opts(
            HistogramOpts::new("http_request_duration_seconds", "HTTP 请求持续时间（秒）")
                .buckets(vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0]),
        )
        .unwrap();

        // 活跃连接数
        let active_connections = Gauge::new("active_connections", "活跃连接数").unwrap();

        // 自定义计数器
        let custom_counter = Counter::new("custom_operations_total", "自定义操作总数").unwrap();

        // 注册指标
        registry
            .register(Box::new(http_requests_total.clone()))
            .unwrap();
        registry
            .register(Box::new(http_request_duration_seconds.clone()))
            .unwrap();
        registry
            .register(Box::new(active_connections.clone()))
            .unwrap();
        registry.register(Box::new(custom_counter.clone())).unwrap();

        Self {
            registry: Arc::new(registry),
            http_requests_total,
            http_request_duration_seconds,
            active_connections,
            custom_counter,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub metrics: Metrics,
    pub request_counter: Arc<AtomicUsize>,
    pub error_counter: Arc<AtomicUsize>,
    pub startup_time: Instant,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            metrics: Metrics::new(),
            request_counter: Arc::new(AtomicUsize::new(0)),
            error_counter: Arc::new(AtomicUsize::new(0)),
            startup_time: Instant::now(),
        }
    }

    pub fn get_uptime(&self) -> u64 {
        self.startup_time.elapsed().as_secs()
    }


    pub fn get_request_count(&self) -> usize {
        self.request_counter.load(Ordering::Relaxed)
    }
    pub fn get_error_count(&self) -> usize {
        self.error_counter.load(Ordering::Relaxed)
    }
}
