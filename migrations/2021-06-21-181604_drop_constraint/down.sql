ALTER TABLE disks ADD CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE;
ALTER TABLE cputimes ADD CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE;
ALTER TABLE cpustats ADD CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE;
ALTER TABLE ioblocks ADD CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE;
ALTER TABLE loadavg ADD CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE;
ALTER TABLE memory ADD CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE;
ALTER TABLE swap ADD CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE;
ALTER TABLE ionets ADD CONSTRAINT host_uuid_fkey FOREIGN KEY (host_uuid) REFERENCES hosts (uuid) DEFERRABLE;