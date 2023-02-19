ALTER TABLE hosts DROP COLUMN updated_at;

DROP TRIGGER IF EXISTS set_timestamp on hosts;

DROP FUNCTION IF EXISTS trigger_set_timestamp;