# Create User
POST http://127.0.0.1:3000/api/users/create
Content-Type: application/json
{
    "username": "testuser",
    "first_name": "Test",
    "last_name": "User",
    "age": 25,
    "weight": 70,
    "height": 175,
    "gender": "Male"
}
HTTP 201
[Asserts]
jsonpath "$.username" == "testuser"
jsonpath "$.first_name" == "Test"
jsonpath "$.last_name" == "User"
jsonpath "$.age" == 25
jsonpath "$.weight" == 70.0
jsonpath "$.height" == 175.0
