-- cputimes

SELECT add_continuous_aggregate_policy('cputimes_10m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '10 minutes',
  schedule_interval => INTERVAL '10 minutes');

SELECT add_continuous_aggregate_policy('cputimes_30m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '30 minutes',
  schedule_interval => INTERVAL '30 minutes');

-- cpustats

SELECT add_continuous_aggregate_policy('cpustats_10m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '10 minutes',
  schedule_interval => INTERVAL '10 minutes');

SELECT add_continuous_aggregate_policy('cpustats_30m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '30 minutes',
  schedule_interval => INTERVAL '30 minutes');

-- disks

SELECT add_continuous_aggregate_policy('disks_10m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '10 minutes',
  schedule_interval => INTERVAL '10 minutes');

SELECT add_continuous_aggregate_policy('disks_30m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '30 minutes',
  schedule_interval => INTERVAL '30 minutes');

-- ioblocks

SELECT add_continuous_aggregate_policy('ioblocks_10m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '10 minutes',
  schedule_interval => INTERVAL '10 minutes');

SELECT add_continuous_aggregate_policy('ioblocks_30m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '30 minutes',
  schedule_interval => INTERVAL '30 minutes');

-- ionet

SELECT add_continuous_aggregate_policy('ionets_10m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '10 minutes',
  schedule_interval => INTERVAL '10 minutes');

SELECT add_continuous_aggregate_policy('ionets_30m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '30 minutes',
  schedule_interval => INTERVAL '30 minutes');

-- loadavg

SELECT add_continuous_aggregate_policy('loadavg_10m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '10 minutes',
  schedule_interval => INTERVAL '10 minutes');

SELECT add_continuous_aggregate_policy('loadavg_30m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '30 minutes',
  schedule_interval => INTERVAL '30 minutes');

-- memory

SELECT add_continuous_aggregate_policy('memory_10m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '10 minutes',
  schedule_interval => INTERVAL '10 minutes');

SELECT add_continuous_aggregate_policy('memory_30m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '30 minutes',
  schedule_interval => INTERVAL '30 minutes');

-- swap

SELECT add_continuous_aggregate_policy('swap_10m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '10 minutes',
  schedule_interval => INTERVAL '10 minutes');

SELECT add_continuous_aggregate_policy('swap_30m',
  start_offset => INTERVAL '3 days',
  end_offset => INTERVAL '30 minutes',
  schedule_interval => INTERVAL '30 minutes');