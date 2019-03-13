const express = require("express");
const app = express();
const port = 8080;

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

app.get("/hello/:name", (req, res) =>
    res.send("Hello " + req.params.name + "!"),
);

app.get("/users/:id", (req, res) => {
    const user = users.find(u => u.userId === Number(req.params.id));
    res.json(user);
});

app.listen(port, () => console.log(`Example app listening on port ${port}!`));
