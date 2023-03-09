-- cputimes

DROP MATERIALIZED VIEW IF EXISTS cputimes_10m;

SELECT remove_retention_policy('cputimes_10m');

DROP MATERIALIZED VIEW IF EXISTS cputimes_30m;

SELECT remove_retention_policy('cputimes_30m');

-- cpustats

DROP MATERIALIZED VIEW IF EXISTS cpustats_10m;

SELECT remove_retention_policy('cpustats_10m');

DROP MATERIALIZED VIEW IF EXISTS cpustats_30m;

SELECT remove_retention_policy('cpustats_30m');

-- disks

DROP MATERIALIZED VIEW IF EXISTS disks_10m;

SELECT remove_retention_policy('disks_10m');

DROP MATERIALIZED VIEW IF EXISTS disks_30m;

SELECT remove_retention_policy('disks_30m');

-- ioblocks

DROP MATERIALIZED VIEW IF EXISTS ioblocks_10m;

SELECT remove_retention_policy('ioblocks_10m');

DROP MATERIALIZED VIEW IF EXISTS ioblocks_30m;

SELECT remove_retention_policy('ioblocks_30m');

-- ionet

DROP MATERIALIZED VIEW IF EXISTS ionets_10m;

SELECT remove_retention_policy('ionets_10m');

DROP MATERIALIZED VIEW IF EXISTS ionets_30m;

SELECT remove_retention_policy('ionets_30m');

-- loadavg

DROP MATERIALIZED VIEW IF EXISTS loadavg_10m;

SELECT remove_retention_policy('loadavg_10m');

DROP MATERIALIZED VIEW IF EXISTS loadavg_30m;

SELECT remove_retention_policy('loadavg_30m');

-- memory

DROP MATERIALIZED VIEW IF EXISTS memory_10m;

SELECT remove_retention_policy('memory_10m');

DROP MATERIALIZED VIEW IF EXISTS memory_30m;

SELECT remove_retention_policy('memory_30m');

-- swap

DROP MATERIALIZED VIEW IF EXISTS swap_10m;

SELECT remove_retention_policy('swap_10m');

DROP MATERIALIZED VIEW IF EXISTS swap_30m;

SELECT remove_retention_policy('swap_30m');