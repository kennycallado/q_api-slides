CREATE TABLE IF NOT EXISTS media ();

ALTER TABLE media
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN name VARCHAR,
  ADD COLUMN media_type VARCHAR NOT NULL,
  ADD COLUMN url TEXT NOT NULL;

ALTER TABLE media ADD CONSTRAINT media_media_type_check CHECK
  (media_type IN ('image', 'video'));

INSERT INTO media (media_type, url) VALUES
  ('image', 'https://fastly.picsum.photos/id/967/200/300.jpg?hmac=N516I7nonEwIoem47eN9JDOoQOXjDLqpDz98BaZz3qc');
