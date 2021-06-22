CREATE INDEX swap_idx_created_at ON swap(host_uuid, created_at DESC);
CREATE INDEX memory_idx_created_at ON memory(host_uuid, created_at DESC);
CREATE INDEX loadavg_idx_created_at ON loadavg(host_uuid, created_at DESC);
CREATE INDEX ioblocks_idx_created_at ON ioblocks(host_uuid, created_at DESC);
CREATE INDEX cpustats_idx_created_at ON cpustats(host_uuid, created_at DESC);
CREATE INDEX cputimes_idx_created_at ON cputimes(host_uuid, created_at DESC);
CREATE INDEX disks_idx_created_at ON disks(host_uuid, created_at DESC);
CREATE INDEX ionets_idx_created_at ON ionets(host_uuid, created_at DESC);