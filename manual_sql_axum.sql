INSERT INTO "questions" ("question_uuid", "title", "description", "created_at") VALUES ('82152280-b5ce-44b1-aac4-7c4a2fae8bfe', 'Question 1', 'This is the description for question 1', NOW());
INSERT INTO "questions" ("question_uuid", "title", "description", "created_at") VALUES ('9fc23a11-9411-4bf9-beda-6d52d688c409', 'Question 2', 'This is the description for question 2', NOW());
INSERT INTO "questions" ("question_uuid", "title", "description", "created_at") VALUES ('bec90455-57b8-4acd-9a93-d295d43289fe', 'Question 3', 'This is the description for question 3', NOW());

INSERT INTO "answers" ("answer_uuid", "question_uuid", "content", "created_at") VALUES ('55552280-b5ce-44b1-aac4-7c4a2fae8bfe', '82152280-b5ce-44b1-aac4-7c4a2fae8bfe', 'Answer 1', NOW());
INSERT INTO "answers" ("answer_uuid", "question_uuid", "content", "created_at") VALUES ('55652280-b5ce-44b1-aac4-7c4a2fae8bfe', '82152280-b5ce-44b1-aac4-7c4a2fae8bfe', 'Answer 1.1', NOW());
INSERT INTO "answers" ("answer_uuid", "question_uuid", "content", "created_at") VALUES ('55752280-b5ce-44b1-aac4-7c4a2fae8bfe', '82152280-b5ce-44b1-aac4-7c4a2fae8bfe', 'Answer 1.2', NOW());
INSERT INTO "answers" ("answer_uuid", "question_uuid", "content", "created_at") VALUES ('55523a11-9411-4bf9-beda-6d52d688c409', '9fc23a11-9411-4bf9-beda-6d52d688c409', 'Answer 2', NOW());
INSERT INTO "answers" ("answer_uuid", "question_uuid", "content", "created_at") VALUES ('55623a11-9411-4bf9-beda-6d52d688c409', '9fc23a11-9411-4bf9-beda-6d52d688c409', 'Answer 2.1', NOW());
INSERT INTO "answers" ("answer_uuid", "question_uuid", "content", "created_at") VALUES ('55723a11-9411-4bf9-beda-6d52d688c409', '9fc23a11-9411-4bf9-beda-6d52d688c409', 'Answer 2.2', NOW());
INSERT INTO "answers" ("answer_uuid", "question_uuid", "content", "created_at") VALUES ('55590455-57b8-4acd-9a93-d295d43289fe', 'bec90455-57b8-4acd-9a93-d295d43289fe', 'Answer 3', NOW());
INSERT INTO "answers" ("answer_uuid", "question_uuid", "content", "created_at") VALUES ('55690455-57b8-4acd-9a93-d295d43289fe', 'bec90455-57b8-4acd-9a93-d295d43289fe', 'Answer 3.1', NOW());
INSERT INTO "answers" ("answer_uuid", "question_uuid", "content", "created_at") VALUES ('55790455-57b8-4acd-9a93-d295d43289fe', 'bec90455-57b8-4acd-9a93-d295d43289fe', 'Answer 3.2', NOW());
