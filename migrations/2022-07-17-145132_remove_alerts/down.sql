ALTER TABLE incidents
	ADD COLUMN alerts_name VARCHAR(128) NOT NULL,
	ADD COLUMN alerts_table VARCHAR(128) NOT NULL,
	ADD COLUMN alerts_lookup TEXT NOT NULL,
	ADD COLUMN alerts_warn TEXT NOT NULL,
	ADD COLUMN alerts_crit TEXT NOT NULL,
	ADD COLUMN alerts_info TEXT,
	ADD COLUMN alerts_where_clause TEXT;