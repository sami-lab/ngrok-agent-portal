package controller

import (
	"agent-go/server/module"
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"

	"github.com/gorilla/mux"
)

type Message struct {
	Text string `json:"text"`
}

func GetEndPointStatus(w http.ResponseWriter, r *http.Request) {
	logRequest("GET", r)
	vars := mux.Vars(r)
	id := vars["id"]

	endpoint := module.GetEndpointStatus(id)

	doc := []interface{}{}
	if endpoint != nil {
		doc = append(doc, endpoint)
	} else {
		doc = append(doc, map[string]interface{}{})
	}

	response := map[string]interface{}{
		"success": true,
		"data": map[string]interface{}{
			"doc": doc,
		},
	}
	jsonResponse, _ := json.Marshal(response)
	w.Header().Set("Content-Type", "application/json")
	w.Write(jsonResponse)
}
func GetAgentStatus(w http.ResponseWriter, r *http.Request) {
	logRequest("GET", r)
	response := map[string]interface{}{
		"success": true,
		"message": "Connected",
	}
	jsonResponse, _ := json.Marshal(response)
	w.Header().Set("Content-Type", "application/json")
	w.Write(jsonResponse)
}
func GetAllEndPoints(w http.ResponseWriter, r *http.Request) {
	logRequest("GET", r)
	endpointResponse := module.GetAllEndPoints()

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

	var requestData struct {
		EndpointYaml string      `json:"endpointYaml"`
		Listener     interface{} `json:"listener"`
		Id           string      `json:"_id"`
	}

	decoder := json.NewDecoder(r.Body)
	if err := decoder.Decode(&requestData); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	newEndpoint, err := module.AddEndpoint(requestData.Id, requestData.EndpointYaml, requestData.Listener)
	if err != nil {
		response := map[string]interface{}{
			"success": false,
			"error":   err.Error(),
		}
		jsonResponse, _ := json.Marshal(response)
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusBadRequest)
		w.Write(jsonResponse)
		return
	}

	response := map[string]interface{}{
		"success": true,
		"data":    newEndpoint,
	}
	jsonResponse, _ := json.Marshal(response)
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusCreated)
	w.Write(jsonResponse)
}
func UpdateStatus(w http.ResponseWriter, r *http.Request) {

	logRequest("PATCH", r)

	vars := mux.Vars(r)
	id := vars["id"]

	updatedEndpoint, err := module.UpdateEndpointStatus(id)
	if err != nil {
		response := map[string]interface{}{
			"success": false,
			"error":   err.Error(),
		}
		jsonResponse, _ := json.Marshal(response)
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusBadRequest)
		w.Write(jsonResponse)
		return
	}

	response := map[string]interface{}{
		"success": true,
		"data": map[string]interface{}{
			"doc": updatedEndpoint,
		},
	}
	jsonResponse, _ := json.Marshal(response)
	w.Header().Set("Content-Type", "application/json")
	w.Write(jsonResponse)
}

func DeleteEndpoint(w http.ResponseWriter, r *http.Request) {
	logRequest("DELETE", r)

	vars := mux.Vars(r)
	id := vars["id"]

	module.DeleteEndpoint(id)

	response := map[string]interface{}{
		"success": true,
	}
	jsonResponse, _ := json.Marshal(response)
	w.Header().Set("Content-Type", "application/json")
	w.Write(jsonResponse)
}
func logRequest(method string, r *http.Request) {
	fmt.Printf("%s request to %s\n", method, r.URL.Path)

	if method == http.MethodPost {
		bodyBytes, err := io.ReadAll(r.Body)
		if err != nil {
			fmt.Println("Error reading request body:", err)
			return
		}
		r.Body = io.NopCloser(bytes.NewBuffer(bodyBytes))

		if len(bodyBytes) > 0 {
			fmt.Printf("Request body: %s\n", string(bodyBytes))
		} else {
			fmt.Println("Request body is empty")
		}
	}
}
