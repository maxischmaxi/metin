-- Add specialization column to characters table
ALTER TABLE characters ADD COLUMN specialization TEXT;

-- Create index for faster lookups
CREATE INDEX idx_characters_specialization ON characters(specialization);
