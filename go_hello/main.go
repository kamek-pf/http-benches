package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strconv"
	"strings"
)

type User struct {
	UserId   int    `json:"userId"`
	UserName string `json:"userName"`
}

var users = []User{
	User{
		UserId:   1,
		UserName: "bob",
	},
	User{
		UserId:   2,
		UserName: "alice",
	},
}

func findUser(key int) *User {
	for i := range users {
		if users[i].UserId == key {
			return &users[i]
		}
	}

	return nil
}

func main() {
	http.HandleFunc("/hello/", func(w http.ResponseWriter, r *http.Request) {
		p := strings.Split(r.URL.Path, "/")
		name := p[len(p)-1]
		fmt.Fprintf(w, "Hello, %s!", name)
	})

	http.HandleFunc("/users/", func(w http.ResponseWriter, r *http.Request) {
		p := strings.Split(r.URL.Path, "/")
		strId := p[len(p)-1]

		if i, err := strconv.Atoi(strId); err == nil {
			user := findUser(i)
			b, err := json.Marshal(user)
			if err != nil {
				fmt.Fprintf(w, "")
			} else {
				fmt.Fprintf(w, "%s", string(b))
			}
		}
	})

	http.ListenAndServe(":8080", nil)
}
