CREATE EXTENSION IF NOT EXISTS timescaledb;

CREATE TABLE hosts (
	system VARCHAR(128) NOT NULL,
	os_version VARCHAR(128) NOT NULL,
	hostname VARCHAR(64) NOT NULL,
	uptime BIGINT NOT NULL,
    uuid VARCHAR(48) PRIMARY KEY NOT NULL,
	created_at TIMESTAMP NOT NULL
);

CREATE TABLE disks (
	id BIGSERIAL,
	disk_name VARCHAR(128) NOT NULL,
	mount_point VARCHAR(128) NOT NULL,
	total_space BIGINT NOT NULL,
	avail_space BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
	created_at TIMESTAMP NOT NULL
);

SELECT create_hypertable('disks', 'created_at', chunk_time_interval => INTERVAL '1 day');
SELECT add_retention_policy('disks', INTERVAL '10 days');

CREATE TABLE cputimes (
	id BIGSERIAL,
	cuser BIGINT NOT NULL,
	nice BIGINT NOT NULL,
	system BIGINT NOT NULL,
	idle BIGINT NOT NULL,
	iowait BIGINT NOT NULL,
	irq BIGINT NOT NULL,
	softirq BIGINT NOT NULL,
	steal BIGINT NOT NULL,
	guest BIGINT NOT NULL,
	guest_nice BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
	created_at TIMESTAMP NOT NULL
);

SELECT create_hypertable('cputimes', 'created_at', chunk_time_interval => INTERVAL '1 day');
SELECT add_retention_policy('cputimes', INTERVAL '10 days');

CREATE TABLE cpustats (
	id BIGSERIAL,
	interrupts BIGINT NOT NULL,
	ctx_switches BIGINT NOT NULL,
	soft_interrupts BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
	created_at TIMESTAMP NOT NULL
);

SELECT create_hypertable('cpustats', 'created_at', chunk_time_interval => INTERVAL '1 day');
SELECT add_retention_policy('cpustats', INTERVAL '10 days');

CREATE TABLE ioblocks (
	id BIGSERIAL,
	device_name VARCHAR(128) NOT NULL,
	read_count BIGINT NOT NULL,
	read_bytes BIGINT NOT NULL,
	write_count BIGINT NOT NULL,
	write_bytes BIGINT NOT NULL,
	busy_time BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
	created_at TIMESTAMP NOT NULL
);

SELECT create_hypertable('ioblocks', 'created_at', chunk_time_interval => INTERVAL '1 day');
SELECT add_retention_policy('ioblocks', INTERVAL '10 days');

CREATE TABLE loadavg (
	id BIGSERIAL,
	one FLOAT NOT NULL,
	five FLOAT NOT NULL,
    fifteen FLOAT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
    created_at TIMESTAMP NOT NULL
);

SELECT create_hypertable('loadavg', 'created_at', chunk_time_interval => INTERVAL '1 day');
SELECT add_retention_policy('loadavg', INTERVAL '10 days');

CREATE TABLE memory (
	id BIGSERIAL,
	total BIGINT NOT NULL,
	free BIGINT NOT NULL,
	used BIGINT NOT NULL,
	shared BIGINT NOT NULL,
	buffers BIGINT NOT NULL,
	cached BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
    created_at TIMESTAMP NOT NULL
);

SELECT create_hypertable('memory', 'created_at', chunk_time_interval => INTERVAL '1 day');
SELECT add_retention_policy('memory', INTERVAL '10 days');

CREATE TABLE swap (
	id BIGSERIAL,
	total BIGINT NOT NULL,
	free BIGINT NOT NULL,
	used BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
    created_at TIMESTAMP NOT NULL
);

SELECT create_hypertable('swap', 'created_at', chunk_time_interval => INTERVAL '1 day');
SELECT add_retention_policy('swap', INTERVAL '10 days');

CREATE TABLE ionets (
	id BIGSERIAL,
	interface VARCHAR(128) NOT NULL,
	rx_bytes BIGINT NOT NULL,
	rx_packets BIGINT NOT NULL,
	rx_errs BIGINT NOT NULL,
	rx_drop BIGINT NOT NULL,
	tx_bytes BIGINT NOT NULL,
	tx_packets BIGINT NOT NULL,
	tx_errs BIGINT NOT NULL,
	tx_drop BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
	created_at TIMESTAMP NOT NULL
);

SELECT create_hypertable('ionets', 'created_at', chunk_time_interval => INTERVAL '1 day');
SELECT add_retention_policy('ionets', INTERVAL '10 days');