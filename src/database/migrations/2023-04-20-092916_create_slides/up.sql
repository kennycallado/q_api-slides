CREATE TABLE IF NOT EXISTS slides ();
-- CREATE PUBLICATION slides_pub FOR TABLE slides;

ALTER TABLE slides 
  -- ADD COLUMN created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  -- ADD COLUMN updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN slide_type VARCHAR NOT NULL,
  ADD COLUMN title VARCHAR NOT NULL,
  ADD COLUMN content TEXT,
  ADD COLUMN question_id INTEGER;

ALTER TABLE slides ADD CONSTRAINT slides_slide_type_check CHECK
  (slide_type IN ('content', 'input'));

-- ALTER TABLE slides REPLICA IDENTITY FULL;

-- SELECT diesel_manage_updated_at('slides');
INSERT INTO slides (slide_type, title, question_id) VALUES
  ('input', 'Título 1', 1);

INSERT INTO slides (slide_type, title, content) VALUES
  ('content', 'Título 2', 'Contenido del título 2'),
  ('content', 'Título 3', null),
  ('content', 'Título 4', 'Contenido del título 4'),
  ('content', 'Título 5', 'Contenido del título 5'),
  ('content', 'Título 6', 'Contenido del título 6'),
  ('content', 'Título 7', 'Contenido del título 7'),
  ('content', 'Título 8', 'Contenido del título 8')
  ;
