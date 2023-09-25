CREATE TABLE IF NOT EXISTS slides ();

ALTER TABLE slides 
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN order_number INTEGER,
  ADD COLUMN slide_type VARCHAR NOT NULL,
  ADD COLUMN title VARCHAR NOT NULL,
  ADD COLUMN media_id INTEGER,
  ADD COLUMN content TEXT,
  ADD COLUMN question_id INTEGER;

ALTER TABLE slides ADD CONSTRAINT slides_slide_type_check CHECK
  (slide_type IN ('content', 'input'));

ALTER TABLE slides
  ADD CONSTRAINT fk_media_id FOREIGN KEY (media_id) REFERENCES media (id);

-- ALTER TABLE slides REPLICA IDENTITY FULL;
-- CREATE PUBLICATION slides_pub FOR TABLE slides;

-- SELECT diesel_manage_updated_at('slides');
INSERT INTO slides (slide_type, title, question_id) VALUES
  ('input', 'Título 1', 1)
  ;

INSERT INTO slides (slide_type, title, content) VALUES
  ('content', 'Título 2', 'Contenido del título 2')
  ;
