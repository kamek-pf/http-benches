## Hello world
`wrk -t12 -c400 -d5s http://localhost:8080/hello/noob`
Best out of 5:

Warp:
Requests/sec: 340858.06
Transfer/sec:     41.61MB

Actix:
Requests/sec: 333536.64
Transfer/sec:     40.71MB

Koa:
Requests/sec:  29132.11
Transfer/sec:      4.22MB

Express:
Requests/sec:  19182.19
Transfer/sec:      3.93MB



