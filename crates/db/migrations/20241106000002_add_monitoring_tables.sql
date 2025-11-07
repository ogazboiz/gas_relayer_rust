-- Additional tables for monitoring and observability
-- This migration adds tables specifically for the monitoring system (E7)

-- Metrics snapshots table - stores periodic metric snapshots
CREATE TABLE IF NOT EXISTS metrics_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    metric_name VARCHAR(100) NOT NULL,
    metric_value NUMERIC(20, 6) NOT NULL,
    labels JSONB,                       -- Key-value pairs for metric labels
    recorded_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for metrics queries
CREATE INDEX IF NOT EXISTS idx_metrics_snapshots_name ON metrics_snapshots(metric_name);
CREATE INDEX IF NOT EXISTS idx_metrics_snapshots_recorded_at ON metrics_snapshots(recorded_at DESC);
CREATE INDEX IF NOT EXISTS idx_metrics_snapshots_name_time ON metrics_snapshots(metric_name, recorded_at DESC);

-- Health check results table
CREATE TABLE IF NOT EXISTS health_checks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    component VARCHAR(50) NOT NULL,     -- database, rpc_endpoint, queue, etc.
    status VARCHAR(20) NOT NULL,        -- healthy, degraded, unhealthy
    response_time_ms INTEGER,
    details JSONB,                      -- Additional health check details
    checked_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for health check queries
CREATE INDEX IF NOT EXISTS idx_health_checks_component ON health_checks(component);
CREATE INDEX IF NOT EXISTS idx_health_checks_checked_at ON health_checks(checked_at DESC);
CREATE INDEX IF NOT EXISTS idx_health_checks_status ON health_checks(status);

-- Performance metrics table - for detailed performance tracking
CREATE TABLE IF NOT EXISTS performance_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation VARCHAR(100) NOT NULL,    -- e.g., 'transaction_processing', 'db_query'
    duration_ms NUMERIC(10, 3) NOT NULL,
    success BOOLEAN NOT NULL DEFAULT true,
    error_message TEXT,
    metadata JSONB,                     -- Additional context
    recorded_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for performance metrics
CREATE INDEX IF NOT EXISTS idx_performance_metrics_operation ON performance_metrics(operation);
CREATE INDEX IF NOT EXISTS idx_performance_metrics_recorded_at ON performance_metrics(recorded_at DESC);
CREATE INDEX IF NOT EXISTS idx_performance_metrics_success ON performance_metrics(success);

-- Queue statistics table - tracks queue depth and processing stats
CREATE TABLE IF NOT EXISTS queue_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    queue_name VARCHAR(50) NOT NULL DEFAULT 'default',
    depth INTEGER NOT NULL DEFAULT 0,
    processing_rate NUMERIC(10, 2),     -- transactions per second
    avg_wait_time_ms NUMERIC(10, 2),
    recorded_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Index for queue stats
CREATE INDEX IF NOT EXISTS idx_queue_stats_recorded_at ON queue_stats(recorded_at DESC);
CREATE INDEX IF NOT EXISTS idx_queue_stats_queue_name ON queue_stats(queue_name);

-- Alert history table - tracks fired alerts
CREATE TABLE IF NOT EXISTS alert_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    alert_name VARCHAR(100) NOT NULL,
    severity VARCHAR(20) NOT NULL,      -- critical, warning, info
    message TEXT NOT NULL,
    details JSONB,
    fired_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    resolved_at TIMESTAMP WITH TIME ZONE,
    is_resolved BOOLEAN DEFAULT false
);

-- Indexes for alert history
CREATE INDEX IF NOT EXISTS idx_alert_history_fired_at ON alert_history(fired_at DESC);
CREATE INDEX IF NOT EXISTS idx_alert_history_severity ON alert_history(severity);
CREATE INDEX IF NOT EXISTS idx_alert_history_resolved ON alert_history(is_resolved);

-- Create a view for recent transaction statistics
CREATE OR REPLACE VIEW recent_tx_stats AS
SELECT 
    DATE_TRUNC('hour', created_at) as hour,
    status,
    COUNT(*) as count,
    AVG(gas_used) as avg_gas_used,
    AVG(EXTRACT(EPOCH FROM (updated_at - created_at))) as avg_processing_time_seconds
FROM tx_requests 
WHERE created_at >= NOW() - INTERVAL '24 hours'
GROUP BY DATE_TRUNC('hour', created_at), status
ORDER BY hour DESC, status;

-- Create a view for relayer performance metrics
CREATE OR REPLACE VIEW relayer_performance AS
SELECT 
    DATE_TRUNC('hour', recorded_at) as hour,
    operation,
    COUNT(*) as operation_count,
    AVG(duration_ms) as avg_duration_ms,
    PERCENTILE_CONT(0.95) WITHIN GROUP (ORDER BY duration_ms) as p95_duration_ms,
    SUM(CASE WHEN success THEN 1 ELSE 0 END)::FLOAT / COUNT(*) * 100 as success_rate_percent
FROM performance_metrics 
WHERE recorded_at >= NOW() - INTERVAL '24 hours'
GROUP BY DATE_TRUNC('hour', recorded_at), operation
ORDER BY hour DESC, operation;
