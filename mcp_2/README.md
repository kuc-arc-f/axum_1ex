# mcp_2

 Version: 0.9.1

 date    : 2025/10/16
 
 update :

***

axum + Rust , remoto MCP Server

* TURSO_DATABASE use
***
* rustc 1.90.0 
* cargo 1.90.0 

***
* build
```
cargo run  --release
```

***
### setup

* .env
* API_KEY: Authorization key set

```
API_KEY="123"
TURSO_DATABASE_URL=""
TURSO_AUTH_TOKEN=
```

***
* settings.json : GEMINI-CLI
```
"myRemoteServer": {
  "httpUrl": "http://localhost:3000/mcp", 
  "headers": {
    "Authorization": "" 
  },
  "timeout": 5000 
}  
```

***
* test-code: http

https://gist.github.com/kuc-arc-f/66c6ac29fe229e4ba495acb1ca14f196

***
