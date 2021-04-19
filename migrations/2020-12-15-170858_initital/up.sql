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
);
CREATE TABLE cpustats (
	id BIGSERIAL PRIMARY KEY,
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
);
CREATE TABLE iostats (
	id BIGSERIAL PRIMARY KEY,
	device_name VARCHAR(128) NOT NULL,
	bytes_read BIGINT NOT NULL,
	bytes_wrtn BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
	created_at TIMESTAMP NOT NULL,
	CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE
);
CREATE TABLE load_avg (
	id BIGSERIAL PRIMARY KEY,
	one FLOAT NOT NULL,
	five FLOAT NOT NULL,
    fifteen FLOAT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
    created_at TIMESTAMP NOT NULL,
	CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE
);
CREATE TABLE memory (
	id BIGSERIAL PRIMARY KEY,
	total_virt BIGINT NOT NULL,
	avail_virt BIGINT NOT NULL,
	total_swap BIGINT NOT NULL,
	avail_swap BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
    created_at TIMESTAMP NOT NULL,
	CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE
);