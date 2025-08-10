GET http://localhost:8000
HTTP/1.1 200

GET http://localhost:8000/mystr
HTTP/1.1 200


# GET /questions
GET http://localhost:8000/questions
HTTP/1.1 200

# POST /question
POST http://localhost:8000/question
Content-Type: application/json
{
  "title": "What is Rust?",
  "description": "A systems programming language focusing on safety and performance."
}
HTTP/1.1 200

# DELETE /question
DELETE http://localhost:8000/question
Content-Type: application/json
{
  "question_uuid": "1"
}
HTTP/1.1 200