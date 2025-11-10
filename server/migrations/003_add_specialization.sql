-- Create index for faster lookups (IF NOT EXISTS supported)
-- Note: We only create the index here because the column might already exist
-- from a previous migration attempt. The server code handles the ALTER TABLE separately.
CREATE INDEX IF NOT EXISTS idx_characters_specialization ON characters(specialization);
