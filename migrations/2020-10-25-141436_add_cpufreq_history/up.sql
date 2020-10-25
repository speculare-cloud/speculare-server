CREATE TABLE cpu_info (
	id SERIAL PRIMARY KEY,
	cpu_freq BIGINT NOT NULL,
	data_uuid VARCHAR(48) NOT NULL,
    created_at TIMESTAMP NOT NULL
);