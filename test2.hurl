# Do this command first to start server
# cargo watch -q -c -w src/ -x run

# # POST /question
# POST http://localhost:8000/question
# Content-Type: application/json
# {
#   "title": "What is Rust?",
#   "description": "A systems programming language focusing on safety and performance."
# }

# # POST /question
# POST http://localhost:8000/answer
# Content-Type: application/json
# {
#   "question_uuid": "abedf",
#   "content": "What is Rust?"
# }

GET http://localhost:8000/answers