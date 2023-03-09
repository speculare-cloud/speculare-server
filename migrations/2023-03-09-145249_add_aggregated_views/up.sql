-- cputimes

CREATE MATERIALIZED VIEW IF NOT EXISTS cputimes_10m WITH (timescaledb.continuous)
    AS SELECT
        host_uuid,
        time_bucket('10m', created_at) as time,
        avg(cuser)::int8 as cuser,
        avg(nice)::int8 as nice,
        avg(system)::int8 as system,
        avg(idle)::int8 as idle,
        avg(iowait)::int8 as iowait,
        avg(irq)::int8 as irq,
        avg(softirq)::int8 as softirq,
        avg(steal)::int8 as steal
    FROM cputimes
    GROUP BY host_uuid, time
	WITH NO DATA;

SELECT add_retention_policy('cputimes_10m', INTERVAL '4 days');

CREATE MATERIALIZED VIEW IF NOT EXISTS cputimes_30m WITH (timescaledb.continuous)
    AS SELECT
        host_uuid,
        time_bucket('30m', created_at) as time,
        avg(cuser)::int8 as cuser,
        avg(nice)::int8 as nice,
        avg(system)::int8 as system,
        avg(idle)::int8 as idle,
        avg(iowait)::int8 as iowait,
        avg(irq)::int8 as irq,
        avg(softirq)::int8 as softirq,
        avg(steal)::int8 as steal
    FROM cputimes
    GROUP BY host_uuid, time
	WITH NO DATA;

SELECT add_retention_policy('cputimes_30m', INTERVAL '1 month');

-- cpustats

CREATE MATERIALIZED VIEW IF NOT EXISTS cpustats_10m WITH (timescaledb.continuous)
    AS SELECT
		host_uuid,
		time_bucket('10m', created_at) as time,
		avg(interrupts)::int8 as interrupts,
		avg(ctx_switches)::int8 as ctx_switches,
		avg(soft_interrupts)::int8 as soft_interrupts,
		avg(processes)::int8 as processes,
		avg(procs_running)::int8 as procs_running,
		avg(procs_blocked)::int8 as procs_blocked
	FROM cpustats
    GROUP BY host_uuid, time
	WITH NO DATA;

SELECT add_retention_policy('cpustats_10m', INTERVAL '4 days');

CREATE MATERIALIZED VIEW IF NOT EXISTS cpustats_30m WITH (timescaledb.continuous)
    AS SELECT
		host_uuid,
		time_bucket('30m', created_at) as time,
		avg(interrupts)::int8 as interrupts,
		avg(ctx_switches)::int8 as ctx_switches,
		avg(soft_interrupts)::int8 as soft_interrupts,
		avg(processes)::int8 as processes,
		avg(procs_running)::int8 as procs_running,
		avg(procs_blocked)::int8 as procs_blocked
	FROM cpustats
    GROUP BY host_uuid, time
	WITH NO DATA;

SELECT add_retention_policy('cpustats_30m', INTERVAL '1 month');

-- disks

CREATE MATERIALIZED VIEW IF NOT EXISTS disks_10m WITH (timescaledb.continuous)
    AS SELECT
		host_uuid,
		time_bucket('10m', created_at) as time,
		disk_name,
		avg(total_space)::int8 as total_space,
		avg(avail_space)::int8 as avail_space
	FROM disks
    GROUP BY host_uuid, time, disk_name
	WITH NO DATA;

SELECT add_retention_policy('disks_10m', INTERVAL '4 days');

CREATE MATERIALIZED VIEW IF NOT EXISTS disks_30m WITH (timescaledb.continuous)
    AS SELECT
		host_uuid,
		time_bucket('30m', created_at) as time,
		disk_name,
		avg(total_space)::int8 as total_space,
		avg(avail_space)::int8 as avail_space
	FROM disks
    GROUP BY host_uuid, time, disk_name
	WITH NO DATA;

SELECT add_retention_policy('disks_30m', INTERVAL '1 month');

-- ioblocks

CREATE MATERIALIZED VIEW IF NOT EXISTS ioblocks_10m WITH (timescaledb.continuous)
    AS SELECT
		host_uuid,
		time_bucket('10m', created_at) as time,
		device_name,
		avg(read_bytes)::int8 as read_bytes,
		avg(write_bytes)::int8 as write_bytes
	FROM ioblocks
    GROUP BY host_uuid, time, device_name
	WITH NO DATA;

SELECT add_retention_policy('ioblocks_10m', INTERVAL '4 days');

CREATE MATERIALIZED VIEW IF NOT EXISTS ioblocks_30m WITH (timescaledb.continuous)
    AS SELECT
		host_uuid,
		time_bucket('30m', created_at) as time,
		device_name,
		avg(read_bytes)::int8 as read_bytes,
		avg(write_bytes)::int8 as write_bytes
	FROM ioblocks
    GROUP BY host_uuid, time, device_name
	WITH NO DATA;

SELECT add_retention_policy('ioblocks_30m', INTERVAL '1 month');

-- ionet

CREATE MATERIALIZED VIEW IF NOT EXISTS ionets_10m WITH (timescaledb.continuous)
    AS SELECT
		host_uuid,
		time_bucket('10m', created_at) as time,
		interface,
		avg(rx_bytes)::int8 as rx_bytes,
		avg(tx_bytes)::int8 as tx_bytes
	FROM ionets
    GROUP BY host_uuid, time, interface
	WITH NO DATA;

SELECT add_retention_policy('ionets_10m', INTERVAL '4 days');

CREATE MATERIALIZED VIEW IF NOT EXISTS ionets_30m WITH (timescaledb.continuous)
    AS SELECT
		host_uuid,
		time_bucket('30m', created_at) as time,
		interface,
		avg(rx_bytes)::int8 as rx_bytes,
		avg(tx_bytes)::int8 as tx_bytes
	FROM ionets
    GROUP BY host_uuid, time, interface
	WITH NO DATA;

SELECT add_retention_policy('ionets_30m', INTERVAL '1 month');

-- loadavg

CREATE MATERIALIZED VIEW IF NOT EXISTS loadavg_10m WITH (timescaledb.continuous)
    AS SELECT
		host_uuid,
		time_bucket('10m', created_at) as time,
		avg(one)::float8 as one,
		avg(five)::float8 as five,
		avg(fifteen)::float8 as fifteen
	FROM loadavg
    GROUP BY host_uuid, time
	WITH NO DATA;

SELECT add_retention_policy('loadavg_10m', INTERVAL '4 days');

CREATE MATERIALIZED VIEW IF NOT EXISTS loadavg_30m WITH (timescaledb.continuous)
    AS SELECT
		host_uuid,
		time_bucket('30m', created_at) as time,
		interface,
		avg(one)::float8 as one,
		avg(five)::float8 as five,
		avg(fifteen)::float8 as fifteen
	FROM loadavg
    GROUP BY host_uuid, time
	WITH NO DATA;

SELECT add_retention_policy('loadavg_30m', INTERVAL '1 month');

-- memory

CREATE MATERIALIZED VIEW IF NOT EXISTS memory_10m WITH (timescaledb.continuous)
    AS SELECT
		host_uuid,
		time_bucket('10m', created_at) as time,
		avg(free)::int8 as free,
		avg(used)::int8 as used,
		avg(buffers)::int8 as buffers,
		avg(cached)::int8 as cached
	FROM memory
    GROUP BY host_uuid, time
	WITH NO DATA;

SELECT add_retention_policy('memory_10m', INTERVAL '4 days');

CREATE MATERIALIZED VIEW IF NOT EXISTS memory_30m WITH (timescaledb.continuous)
    AS SELECT
		host_uuid,
		time_bucket('30m', created_at) as time,
		avg(free)::int8 as free,
		avg(used)::int8 as used,
		avg(buffers)::int8 as buffers,
		avg(cached)::int8 as cached
	FROM memory
    GROUP BY host_uuid, time
	WITH NO DATA;

SELECT add_retention_policy('memory_30m', INTERVAL '1 month');

-- swap

CREATE MATERIALIZED VIEW IF NOT EXISTS swap_10m WITH (timescaledb.continuous)
    AS SELECT
		host_uuid,
		time_bucket('10m', created_at) as time,
		avg(total)::int8 as total,
		avg(free)::int8 as free,
		avg(used)::int8 as used
	FROM swap
    GROUP BY host_uuid, time
	WITH NO DATA;

SELECT add_retention_policy('swap_10m', INTERVAL '4 days');

CREATE MATERIALIZED VIEW IF NOT EXISTS swap_30m WITH (timescaledb.continuous)
    AS SELECT
		host_uuid,
		time_bucket('30m', created_at) as time,
		avg(total)::int8 as total,
		avg(free)::int8 as free,
		avg(used)::int8 as used
	FROM swap
    GROUP BY host_uuid, time
	WITH NO DATA;

SELECT add_retention_policy('swap_30m', INTERVAL '1 month');