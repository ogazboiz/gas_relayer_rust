use prometheus::{
    Counter, Gauge, Histogram, IntCounter, IntGauge, Registry, Encoder, TextEncoder,
    HistogramOpts, Opts,};
use std::sync::Arc;

#[derive(Clone)]

/// This struct holds all Prometheus metric objects and provides methods that update and export them

pub struct MetricsCollector {
    pub registry: Arc<Registry>,
    
    // Transaction metrics
    /*
    These track:
    Total number of transactions processed.
    How many succeeded or failed.
    How many are still pending.
    Time taken per transaction.
    */
    pub transactions_total: IntCounter,
    pub transactions_success: IntCounter,
    pub transactions_failed: IntCounter,
    pub transactions_pending: IntGauge,
    pub transaction_processing_duration: Histogram,
    

    // Gas metrics
    /*
    These monitor:
    Gas usage across all relayed transactions.
    Current gas price (could be updated periodically).
    Violations (when transactions exceed allowed gas limits).
    */
    pub gas_used_total: Counter,
    pub gas_price_current: Gauge,
    pub gas_limit_violations: IntCounter,
    

    // Queue metrics
    /*
    Track:
    How many transactions are currently queued.
    How long they wait in the queue.
    How many times retries occurred.
    */
    pub queue_depth: IntGauge,
    pub queue_processing_time: Histogram,
    pub queue_retries_total: IntCounter,
    


    // Database metrics
    /*
    Measure:
    Number of open DB connections.
    Time to run a DB query.
    Total DB query errors.
    */
    pub db_connections_active: IntGauge,
    pub db_query_duration: Histogram,
    pub db_errors_total: IntCounter,
    

    // RPC metrics
    /*
    Track:
    Number of RPC requests made.
    How many failed.
    Latency for each RPC call.
    */
    pub rpc_requests_total: IntCounter,
    pub rpc_errors_total: IntCounter,
    pub rpc_latency: Histogram,


    
    // Relayer metrics
    /*
    Monitor:
    Relayer’s ETH balance.
    Current nonce (to detect stuck txs or misalignment).
    Total sent transactions.
    */
    pub relayer_balance: Gauge,
    pub relayer_nonce_current: IntGauge,
    pub relayer_tx_sent: IntCounter,
    

    // Security metrics
    /*
    Count:
    Invalid signatures detected.
    Replay attacks prevented.
    Rate-limit rule hits.
    */
    pub invalid_signatures: IntCounter,
    pub replay_attacks: IntCounter,
    pub rate_limit_hits: IntCounter,
}

impl MetricsCollector {
    pub fn new() -> anyhow::Result<Self> {
        let registry = Arc::new(Registry::new());
        
        // Transaction metrics
        let transactions_total = IntCounter::with_opts(
            Opts::new("gas_relayer_transactions_total", "Total number of transactions processed")
        )?;
        
        let transactions_success = IntCounter::with_opts(
            Opts::new("gas_relayer_transactions_success_total", "Total number of successful transactions")
        )?;
        
        let transactions_failed = IntCounter::with_opts(
            Opts::new("gas_relayer_transactions_failed_total", "Total number of failed transactions")
        )?;
        
        let transactions_pending = IntGauge::with_opts(
            Opts::new("gas_relayer_transactions_pending", "Number of transactions currently pending")
        )?;
        
        let transaction_processing_duration = Histogram::with_opts(
            HistogramOpts::new("gas_relayer_transaction_processing_duration_seconds", "Time spent processing transactions")
                .buckets(vec![0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0])
        )?;
        
        // Gas metrics
        let gas_used_total = Counter::with_opts(
            Opts::new("gas_relayer_gas_used_total", "Total gas used by relayed transactions")
        )?;
        
        let gas_price_current = Gauge::with_opts(
            Opts::new("gas_relayer_gas_price_gwei", "Current gas price in Gwei")
        )?;
        
        let gas_limit_violations = IntCounter::with_opts(
            Opts::new("gas_relayer_gas_limit_violations_total", "Number of transactions exceeding gas limits")
        )?;
        
        // Queue metrics
        let queue_depth = IntGauge::with_opts(
            Opts::new("gas_relayer_queue_depth", "Number of transactions in processing queue")
        )?;
        
        let queue_processing_time = Histogram::with_opts(
            HistogramOpts::new("gas_relayer_queue_processing_time_seconds", "Time transactions spend in queue")
                .buckets(vec![1.0, 5.0, 10.0, 30.0, 60.0, 300.0, 600.0])
        )?;
        
        let queue_retries_total = IntCounter::with_opts(
            Opts::new("gas_relayer_queue_retries_total", "Total number of transaction retries")
        )?;
        
        // Database metrics
        let db_connections_active = IntGauge::with_opts(
            Opts::new("gas_relayer_db_connections_active", "Number of active database connections")
        )?;
        
        let db_query_duration = Histogram::with_opts(
            HistogramOpts::new("gas_relayer_db_query_duration_seconds", "Database query execution time")
                .buckets(vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0])
        )?;
        
        let db_errors_total = IntCounter::with_opts(
            Opts::new("gas_relayer_db_errors_total", "Total number of database errors")
        )?;
        
        // RPC metrics
        let rpc_requests_total = IntCounter::with_opts(
            Opts::new("gas_relayer_rpc_requests_total", "Total number of RPC requests")
        )?;
        
        let rpc_errors_total = IntCounter::with_opts(
            Opts::new("gas_relayer_rpc_errors_total", "Total number of RPC errors")
        )?;
        
        let rpc_latency = Histogram::with_opts(
            HistogramOpts::new("gas_relayer_rpc_latency_seconds", "RPC request latency")
                .buckets(vec![0.1, 0.5, 1.0, 2.0, 5.0, 10.0, 30.0])
        )?;
        
        // Relayer metrics
        let relayer_balance = Gauge::with_opts(
            Opts::new("gas_relayer_balance_eth", "Relayer wallet balance in ETH")
        )?;
        
        let relayer_nonce_current = IntGauge::with_opts(
            Opts::new("gas_relayer_nonce_current", "Current nonce of relayer wallet")
        )?;
        
        let relayer_tx_sent = IntCounter::with_opts(
            Opts::new("gas_relayer_tx_sent_total", "Total transactions sent by relayer")
        )?;
        
        // Security metrics
        let invalid_signatures = IntCounter::with_opts(
            Opts::new("gas_relayer_invalid_signatures_total", "Total number of invalid signatures detected")
        )?;
        
        let replay_attacks = IntCounter::with_opts(
            Opts::new("gas_relayer_replay_attacks_total", "Total number of replay attacks detected")
        )?;
        
        let rate_limit_hits = IntCounter::with_opts(
            Opts::new("gas_relayer_rate_limit_hits_total", "Total number of rate limit violations")
        )?;
        
        // Register all metrics
        registry.register(Box::new(transactions_total.clone()))?;
        registry.register(Box::new(transactions_success.clone()))?;
        registry.register(Box::new(transactions_failed.clone()))?;
        registry.register(Box::new(transactions_pending.clone()))?;
        registry.register(Box::new(transaction_processing_duration.clone()))?;
        registry.register(Box::new(gas_used_total.clone()))?;
        registry.register(Box::new(gas_price_current.clone()))?;
        registry.register(Box::new(gas_limit_violations.clone()))?;
        registry.register(Box::new(queue_depth.clone()))?;
        registry.register(Box::new(queue_processing_time.clone()))?;
        registry.register(Box::new(queue_retries_total.clone()))?;
        registry.register(Box::new(db_connections_active.clone()))?;
        registry.register(Box::new(db_query_duration.clone()))?;
        registry.register(Box::new(db_errors_total.clone()))?;
        registry.register(Box::new(rpc_requests_total.clone()))?;
        registry.register(Box::new(rpc_errors_total.clone()))?;
        registry.register(Box::new(rpc_latency.clone()))?;
        registry.register(Box::new(relayer_balance.clone()))?;
        registry.register(Box::new(relayer_nonce_current.clone()))?;
        registry.register(Box::new(relayer_tx_sent.clone()))?;
        registry.register(Box::new(invalid_signatures.clone()))?;
        registry.register(Box::new(replay_attacks.clone()))?;
        registry.register(Box::new(rate_limit_hits.clone()))?;
        
        Ok(Self {
            registry,
            transactions_total,
            transactions_success,
            transactions_failed,
            transactions_pending,
            transaction_processing_duration,
            gas_used_total,
            gas_price_current,
            gas_limit_violations,
            queue_depth,
            queue_processing_time,
            queue_retries_total,
            db_connections_active,
            db_query_duration,
            db_errors_total,
            rpc_requests_total,
            rpc_errors_total,
            rpc_latency,
            relayer_balance,
            relayer_nonce_current,
            relayer_tx_sent,
            invalid_signatures,
            replay_attacks,
            rate_limit_hits,
        })
    }
    
    pub fn export_metrics(&self) -> anyhow::Result<String> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
    
    // Helper methods for common metric operations
    pub fn record_transaction_success(&self, processing_time: f64, gas_used: f64) {
        self.transactions_total.inc();
        self.transactions_success.inc();
        self.transaction_processing_duration.observe(processing_time);
        self.gas_used_total.inc_by(gas_used);
    }
    
    pub fn record_transaction_failure(&self, processing_time: f64) {
        self.transactions_total.inc();
        self.transactions_failed.inc();
        self.transaction_processing_duration.observe(processing_time);
    }
    
    pub fn record_rpc_call(&self, latency: f64, success: bool) {
        self.rpc_requests_total.inc();
        self.rpc_latency.observe(latency);
        if !success {
            self.rpc_errors_total.inc();
        }
    }
    
    pub fn record_db_query(&self, duration: f64, success: bool) {
        self.db_query_duration.observe(duration);
        if !success {
            self.db_errors_total.inc();
        }
    }
}