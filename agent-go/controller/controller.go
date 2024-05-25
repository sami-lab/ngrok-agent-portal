package controller

import (
	"encoding/json"
	"fmt"
	"net/http"
)

type Message struct {
	Text string `json:"text"`
}

func GetHandler(w http.ResponseWriter, r *http.Request) {
	// logRequest("GET", r)
	// fmt.Fprintln(w, "GET request received")
	// Create a sample response
	response := Message{
		Text: "This is a sample response for GET request",
	}

	// Convert the response to JSON
	jsonResponse, err := json.Marshal(response)
	if err != nil {
		http.Error(w, "Internal server error", http.StatusInternalServerError)
		return
	}

	// Set content type and respond with the JSON
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	w.Write(jsonResponse)
}

func PostHandler(w http.ResponseWriter, r *http.Request) {
	logRequest("POST", r)
	fmt.Fprintln(w, "POST request received")
}

func PutHandler(w http.ResponseWriter, r *http.Request) {
	logRequest("PUT", r)
	fmt.Fprintln(w, "PUT request received")
}

func DeleteHandler(w http.ResponseWriter, r *http.Request) {
	logRequest("DELETE", r)
	fmt.Fprintln(w, "DELETE request received")
}

func logRequest(method string, r *http.Request) {
	fmt.Printf("%s request to %s\n", method, r.URL.Path)
}
