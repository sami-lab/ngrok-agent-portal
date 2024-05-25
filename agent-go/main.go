package main

import (
	"log"
	"net/http"

	"agent-go/server/app"
)

func main() {
	router := app.SetupRouter()

	log.Println("Server listening on :8080")
	log.Fatal(http.ListenAndServe(":8080", router))
}
