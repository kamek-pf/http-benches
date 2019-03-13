var Koa = require("koa");
var Router = require("koa-router");

var app = new Koa();
var router = new Router();

const users = [
    {
        userId: 1,
        userName: "bob",
    },
    {
        userId: 2,
        userName: "alice",
    },
];

router.get("/hello/:name", (ctx, next) => {
    ctx.body = "Hello " + ctx.params.name + "!";
});

router.get("/users/:id", (ctx, next) => {
    const user = users.find(u => u.userId === Number(ctx.params.id));
    ctx.body = JSON.stringify(user);
});

app.use(router.routes()).use(router.allowedMethods());

app.listen(8080);
