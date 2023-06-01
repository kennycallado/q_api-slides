CREATE TABLE IF NOT EXISTS slides ();

ALTER TABLE slides 
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN slide_type VARCHAR NOT NULL,
  ADD COLUMN title VARCHAR NOT NULL,
  ADD COLUMN content TEXT,
  ADD COLUMN question_id INTEGER;

ALTER TABLE slides ADD CONSTRAINT slides_slide_type_check CHECK
  (slide_type IN ('content', 'input'));

-- ALTER TABLE slides REPLICA IDENTITY FULL;
-- CREATE PUBLICATION slides_pub FOR TABLE slides;

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

CREATE TABLE IF NOT EXISTS questions ();

ALTER TABLE questions
  ADD COLUMN IF NOT EXISTS id SERIAL PRIMARY KEY,
  ADD COLUMN IF NOT EXISTS question_type VARCHAR(10) NOT NULL,
  ADD COLUMN IF NOT EXISTS question VARCHAR NOT NULL;

ALTER TABLE questions ADD CONSTRAINT questions_question_type_check CHECK
  (question_type IN ('range', 'radio', 'checkbox', 'input'));

CREATE SUBSCRIPTION questions_sub
  CONNECTION 'host=questions_db port=5432 dbname=questions user=support password=support-pass'
  PUBLICATION questions_sub WITH (create_slot = false)
  ;
