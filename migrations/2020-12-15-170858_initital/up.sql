CREATE TABLE hosts (
	os VARCHAR(128) NOT NULL,
	hostname VARCHAR(64) NOT NULL,
	uptime BIGINT NOT NULL,
    uuid VARCHAR(48) PRIMARY KEY NOT NULL,
	created_at TIMESTAMP NOT NULL
);
CREATE TABLE disks (
	id SERIAL PRIMARY KEY,
	disk_name VARCHAR(128) NOT NULL,
	mount_point VARCHAR(128) NOT NULL,
	total_space BIGINT NOT NULL,
	avail_space BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
	created_at TIMESTAMP NOT NULL,
	CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE
);
CREATE TABLE load_avg (
	id SERIAL PRIMARY KEY,
	one FLOAT NOT NULL,
	five FLOAT NOT NULL,
    fifteen FLOAT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
    created_at TIMESTAMP NOT NULL,
	CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE
);
CREATE TABLE cpu_info (
	id SERIAL PRIMARY KEY,
	cpu_freq BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
    created_at TIMESTAMP NOT NULL,
	CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE
);
CREATE TABLE memory (
	id SERIAL PRIMARY KEY,
	total_virt BIGINT NOT NULL,
	avail_virt BIGINT NOT NULL,
	total_swap BIGINT NOT NULL,
	avail_swap BIGINT NOT NULL,
	host_uuid VARCHAR(48) NOT NULL,
    created_at TIMESTAMP NOT NULL,
	CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE
);