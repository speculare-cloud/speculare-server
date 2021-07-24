CREATE TABLE alerts (
	id SERIAL PRIMARY KEY NOT NULL,
	_name VARCHAR(128) NOT NULL,
	_table VARCHAR(128) NOT NULL,
	lookup TEXT NOT NULL,
	timing INTEGER NOT NULL,
	warn TEXT NOT NULL,
	crit TEXT NOT NULL,
	info TEXT,
    host_uuid VARCHAR(48) NOT NULL,
	hostname VARCHAR(64) NOT NULL,
	where_clause TEXT
);

CREATE INDEX alerts_idx_uuid ON alerts(host_uuid);