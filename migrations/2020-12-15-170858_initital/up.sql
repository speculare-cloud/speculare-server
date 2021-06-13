CREATE TABLE hosts (
	system VARCHAR(128) NOT NULL,
	os_version VARCHAR(128) NOT NULL,
	hostname VARCHAR(64) NOT NULL,
	uptime BIGINT NOT NULL,
    uuid VARCHAR(48) PRIMARY KEY NOT NULL,
	created_at TIMESTAMP NOT NULL
);

CREATE TABLE disks (
	id BIGSERIAL PRIMARY KEY,
	disk_name VARCHAR(128) NOT NULL,
	mount_point VARCHAR(128) NOT NULL,
	total_space BIGINT NOT NULL,
	avail_space BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
	created_at TIMESTAMP NOT NULL,
	CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE
) PARTITION BY RANGE (created_at);

CREATE TABLE disks_template (LIKE disks);
ALTER TABLE disks_template ADD PRIMARY KEY (id);

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
	created_at TIMESTAMP NOT NULL,
	CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE
) PARTITION BY RANGE (created_at);

CREATE TABLE cputimes_template (LIKE cputimes);
ALTER TABLE cputimes_template ADD PRIMARY KEY (id);

CREATE TABLE cpustats (
	id BIGSERIAL,
	interrupts BIGINT NOT NULL,
	ctx_switches BIGINT NOT NULL,
	soft_interrupts BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
	created_at TIMESTAMP NOT NULL,
	CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE
) PARTITION BY RANGE (created_at);

CREATE TABLE cpustats_template (LIKE cpustats);
ALTER TABLE cpustats_template ADD PRIMARY KEY (id);

CREATE TABLE iostats (
	id BIGSERIAL,
	device_name VARCHAR(128) NOT NULL,
	read_count BIGINT NOT NULL,
	read_bytes BIGINT NOT NULL,
	write_count BIGINT NOT NULL,
	write_bytes BIGINT NOT NULL,
	busy_time BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
	created_at TIMESTAMP NOT NULL,
	CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE
) PARTITION BY RANGE (created_at);

CREATE TABLE iostats_template (LIKE iostats);
ALTER TABLE iostats_template ADD PRIMARY KEY (id);

CREATE TABLE loadavg (
	id BIGSERIAL,
	one FLOAT NOT NULL,
	five FLOAT NOT NULL,
    fifteen FLOAT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
    created_at TIMESTAMP NOT NULL,
	CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE
) PARTITION BY RANGE (created_at);

CREATE TABLE loadavg_template (LIKE loadavg);
ALTER TABLE loadavg_template ADD PRIMARY KEY (id);

CREATE TABLE memory (
	id BIGSERIAL,
	total BIGINT NOT NULL,
	free BIGINT NOT NULL,
	used BIGINT NOT NULL,
	shared BIGINT NOT NULL,
	buffers BIGINT NOT NULL,
	cached BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
    created_at TIMESTAMP NOT NULL,
	CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE
) PARTITION BY RANGE (created_at);

CREATE TABLE memory_template (LIKE memory);
ALTER TABLE memory_template ADD PRIMARY KEY (id);

CREATE TABLE swap (
	id BIGSERIAL,
	total BIGINT NOT NULL,
	free BIGINT NOT NULL,
	used BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
    created_at TIMESTAMP NOT NULL,
	CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE
) PARTITION BY RANGE (created_at);

CREATE TABLE swap_template (LIKE swap);
ALTER TABLE swap_template ADD PRIMARY KEY (id);

CREATE TABLE iocounters (
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
	created_at TIMESTAMP NOT NULL,
	CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE
) PARTITION BY RANGE (created_at);

CREATE TABLE iocounters_template (LIKE iocounters);
ALTER TABLE iocounters_template ADD PRIMARY KEY (id);