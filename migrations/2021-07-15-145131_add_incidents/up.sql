CREATE TABLE incidents (
	id SERIAL PRIMARY KEY NOT NULL,
	result TEXT NOT NULL,
	started_at TIMESTAMP NOT NULL,
	updated_at TIMESTAMP NOT NULL,
	resolved_at TIMESTAMP,
	host_uuid VARCHAR(48) NOT NULL,
	status INTEGER DEFAULT 0 NOT NULL,
	severity INTEGER DEFAULT 0 NOT NULL,
	alerts_id SERIAL NOT NULL,
	alerts_name VARCHAR(128) NOT NULL,
	alerts_table VARCHAR(128) NOT NULL,
	alerts_lookup TEXT NOT NULL,
	alerts_warn TEXT NOT NULL,
	alerts_crit TEXT NOT NULL,
	alerts_info TEXT,
	alerts_where_clause TEXT
);

CREATE INDEX incidents_idx_uuid ON incidents(host_uuid);
CREATE INDEX incidents_idx_update_at ON incidents(updated_at);