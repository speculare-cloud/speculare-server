-- cputimes

SELECT remove_continuous_aggregate_policy('cputimes_10m');

SELECT remove_continuous_aggregate_policy('cputimes_30m');

-- cpustats

SELECT remove_continuous_aggregate_policy('cpustats_10m');

SELECT remove_continuous_aggregate_policy('cpustats_30m');

-- disks

SELECT remove_continuous_aggregate_policy('disks_10m');

SELECT remove_continuous_aggregate_policy('disks_30m');

-- ioblocks

SELECT remove_continuous_aggregate_policy('ioblocks_10m');

SELECT remove_continuous_aggregate_policy('ioblocks_30m');

-- ionet

SELECT remove_continuous_aggregate_policy('ionets_10m');

SELECT remove_continuous_aggregate_policy('ionets_30m');

-- loadavg

SELECT remove_continuous_aggregate_policy('loadavg_10m');

SELECT remove_continuous_aggregate_policy('loadavg_30m');

-- memory

SELECT remove_continuous_aggregate_policy('memory_10m');

SELECT remove_continuous_aggregate_policy('memory_30m');

-- swap

SELECT remove_continuous_aggregate_policy('swap_10m');

SELECT remove_continuous_aggregate_policy('swap_30m');