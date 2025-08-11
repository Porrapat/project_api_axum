-- Insert 100 questions
INSERT INTO questions (question_uuid, title, description, created_at)
SELECT
    gen_random_uuid(),
    'Question ' || i,
    'This is the description for question ' || i,
    NOW() - (i || ' days')::interval
FROM generate_series(1, 100) AS s(i);

-- Insert 300 answers (3 per question)
INSERT INTO answers (answer_uuid, question_uuid, content, created_at)
SELECT
    gen_random_uuid(),
    q.question_uuid,
    'This is answer ' || a_num || ' for question ' || q_num,
    NOW() - (q_num || ' days')::interval
FROM (
    SELECT question_uuid, row_number() OVER () AS q_num
    FROM questions
    ORDER BY created_at
    LIMIT 100
) q
CROSS JOIN generate_series(1, 3) AS a(a_num);