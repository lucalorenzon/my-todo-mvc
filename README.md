# My TODO MVC

Rust based implementation of the well known [todoMVC](https://todomvc.com/) application. The main purpose is to learn rust doing something concrete.
In particular I'd like to learn how to manage in rust the following topics:
- **Workspace monorepo**
- A **RESTful API Server** with OpenAPI specification
- A **Web App** that call the **RESTful API**
- A **Secret Manager**
- An **ID**entity **P**rovider to manage **Authentication** and **Authorization** for the **Web App** and for the **RESTful API**
- An **API Gateway** to manage the **RESTful API** access and separate **Authentication** and **Authorization** logic from the business code of the **RESTful API**
- A **Policy Engine** like **Open Policy Agent** in order to manage **Authorization** behaviour without changing code

~~I'll proceed in micro-steps with a branch per each step, in order to have a version of the repo in each meaningful moment.~~
I'll proceed in steps, tagging each significant step.

## Requirement \#1: basic "Hello World" api server
- Build an **API Server** that is able to return a json response for health request like the following:
```
curl -X GET http://localhost:8080/health
```

response:
```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 22

{"status": "ok"}
```
- All other requests should return an empty body with status code 404
- If request to `/health` has also parameters it should return an empty body with status code 400

It should be deployed in aws ecs service. So also all the necessary infrastructure and deploy pipeline are in scope.
It should be monitored by Opentelemetry and Grafana.
