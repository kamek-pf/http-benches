package main

import (
	"strings"
	"fmt"
	"net/http"
)

func main() {
	http.HandleFunc("/hello/", func(w http.ResponseWriter, r *http.Request) {
		p := strings.Split(r.URL.Path, "/")
		name := p[len(p) - 1]
		fmt.Fprintf(w, "Hello, %s!", name)
	})

	http.ListenAndServe(":8080", nil)
}
