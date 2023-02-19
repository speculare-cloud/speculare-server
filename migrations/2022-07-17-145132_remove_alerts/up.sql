ALTER TABLE incidents
	DROP COLUMN alerts_name,
	DROP COLUMN alerts_table,
	DROP COLUMN alerts_lookup,
	DROP COLUMN alerts_warn,
	DROP COLUMN alerts_crit,
	DROP COLUMN alerts_info,
	DROP COLUMN alerts_where_clause;