var Koa = require("koa");
var Router = require("koa-router");

var app = new Koa();
var router = new Router();

router.get("/hello/:name", (ctx, next) => {
  ctx.body = "Hello " + ctx.params.name + "!";
});

app.use(router.routes()).use(router.allowedMethods());

app.listen(8080);

