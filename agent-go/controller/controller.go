package controller

import (
	"agent-go/server/module"
	"encoding/json"
	"fmt"
	"net/http"
)

type Message struct {
	Text string `json:"text"`
}

func GetEndPointStatus(w http.ResponseWriter, r *http.Request) {
	logRequest("GET", r)
	id := r.URL.Query().Get("id")
	if id == "" {
		http.Error(w, "Missing id query parameter", http.StatusBadRequest)
		return
	}
	endpoint := module.GetEndpointStatus(id)
	response := map[string]interface{}{
		"success": true,
		"data": map[string]interface{}{
			"doc": endpoint,
		},
	}
	jsonResponse, _ := json.Marshal(response)
	w.Header().Set("Content-Type", "application/json")
	w.Write(jsonResponse)
}
func GetAllEndPoints(w http.ResponseWriter, r *http.Request) {
	logRequest("GET", r)
	endpointResponse := module.GetEndpoint()

	response := map[string]interface{}{
		"success": true,
		"data": map[string]interface{}{
			"doc": endpointResponse,
		},
	}
	jsonResponse, _ := json.Marshal(response)
	w.Header().Set("Content-Type", "application/json")
	w.Write(jsonResponse)
}

func AddEndpoint(w http.ResponseWriter, r *http.Request) {
	logRequest("POST", r)
	fmt.Fprintln(w, "POST request received")
}

func UpdateStatus(w http.ResponseWriter, r *http.Request) {
	logRequest("PUT", r)
	fmt.Fprintln(w, "PUT request received")
}

func DeleteEndpoint(w http.ResponseWriter, r *http.Request) {
	logRequest("DELETE", r)
	fmt.Fprintln(w, "DELETE request received")
}

func logRequest(method string, r *http.Request) {
	fmt.Printf("%s request to %s\n", method, r.URL.Path)
}
