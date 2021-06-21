ALTER TABLE disks DROP CONSTRAINT host_uuid_fkey;
ALTER TABLE cputimes DROP CONSTRAINT host_uuid_fkey;
ALTER TABLE cpustats DROP CONSTRAINT host_uuid_fkey;
ALTER TABLE ioblocks DROP CONSTRAINT host_uuid_fkey;
ALTER TABLE loadavg DROP CONSTRAINT host_uuid_fkey;
ALTER TABLE memory DROP CONSTRAINT host_uuid_fkey;
ALTER TABLE swap DROP CONSTRAINT host_uuid_fkey;
ALTER TABLE ionets DROP CONSTRAINT host_uuid_fkey;