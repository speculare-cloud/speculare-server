CREATE SCHEMA partman;

CREATE EXTENSION pg_partman SCHEMA partman;

SELECT partman.create_parent('public.disks', 'created_at', 'native', 'daily', p_template_table := 'public.disks_template');
SELECT partman.create_parent('public.cputimes', 'created_at', 'native', 'daily', p_template_table := 'public.cputimes_template');
SELECT partman.create_parent('public.cpustats', 'created_at', 'native', 'daily', p_template_table := 'public.cpustats_template');
SELECT partman.create_parent('public.iostats', 'created_at', 'native', 'daily', p_template_table := 'public.iostats_template');
SELECT partman.create_parent('public.loadavg', 'created_at', 'native', 'daily', p_template_table := 'public.loadavg_template');
SELECT partman.create_parent('public.memory', 'created_at', 'native', 'daily', p_template_table := 'public.memory_template');
SELECT partman.create_parent('public.swap', 'created_at', 'native', 'daily', p_template_table := 'public.swap_template');
SELECT partman.create_parent('public.iocounters', 'created_at', 'native', 'daily', p_template_table := 'public.iocounters_template');

UPDATE partman.part_config SET 
	retention = '14 days', 
	retention_keep_table = FALSE, 
	retention_keep_index = FALSE, 
	infinite_time_partitions = TRUE, 
	optimize_trigger = 7
WHERE parent_table = 'public.disks' 
	OR parent_table = 'public.cputimes' 
	OR parent_table = 'public.cpustats'
	OR parent_table = 'public.iostats'
	OR parent_table = 'public.loadavg'
	OR parent_table = 'public.memory'
	OR parent_table = 'public.swap'
	OR parent_table = 'public.iocounters';


