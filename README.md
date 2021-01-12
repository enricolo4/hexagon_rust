# Hexagonal Rust Quickstart 

## Run
Run command in terminal ```cargo run```

## Endpoint
### http://localhost:8080
### Health: 
- GET /health
  - Response
    -  ```Hello World```
- POST /person
  - Body
    -  ```json
        {
            "name": " String",
            "age": 32,
            "cpf": "String",
            "email": "String"
        }
        ```
    - Response
        ```json
        {
            "id": "UUID",
            "name": " String",
            "age": 32,
            "cpf": "String",
            "email": "String"
        }
        ```
