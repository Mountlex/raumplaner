

```
sea-orm-cli migrate up
sea-orm-cli generate entity -l -o entity/src
```


```
curl -s -w '\n' -H 'Content-Type: application/json' -d '{"client_id":"user","client_secret":"123"}' http://localhost:3000/login
curl -s -w '\n' -H 'Content-Type: application/json' -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyIiwiZXhwIjoyMDAwMDAwMDAwLCJyb2xlIjoiVXNlciJ9.LOvZgACEfkkMZt4B3Xub5QybG95xk82UickPZPPTVl8' http://localhost:3000/rooms
```


```
curl -s -w '\n' -H 'Content-Type: application/json' -d '{"client_id":"admin","client_secret":"123"}' http://localhost:3000/login
curl -s -w '\n' -H 'Content-Type: application/json' -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsImV4cCI6MjAwMDAwMDAwMCwicm9sZSI6IkFkbWluIn0.SlOkY1JTtHc-7e3uXmjglM4Q0gPWDjei0Lz7qLj8O_0' http://localhost:3000/rooms
```