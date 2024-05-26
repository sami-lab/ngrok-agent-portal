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
		response := map[string]interface{}{
			"success": false,
			"error":   "Missing id query parameter",
		}
		jsonResponse, _ := json.Marshal(response)
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusBadRequest)
		w.Write(jsonResponse)
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
		Status   string      `json:"status"`
		Listener interface{} `json:"listener"`
	}

	decoder := json.NewDecoder(r.Body)
	if err := decoder.Decode(&requestData); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if requestData.Status == "" {
		response := map[string]interface{}{
			"success": false,
			"error":   "status is required",
		}
		jsonResponse, _ := json.Marshal(response)
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusBadRequest)
		w.Write(jsonResponse)
		return
	}

	if requestData.Listener == nil {
		response := map[string]interface{}{
			"success": false,
			"error":   "listener is required",
		}
		jsonResponse, _ := json.Marshal(response)
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusBadRequest)
		w.Write(jsonResponse)
		return
	}

	newEndpoint, err := module.AddEndpoint(requestData.Status, requestData.Listener)
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

	id := r.URL.Query().Get("id")

	if id == "" {
		response := map[string]interface{}{
			"success": false,
			"error":   "Missing id query parameter",
		}
		jsonResponse, _ := json.Marshal(response)
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusBadRequest)
		w.Write(jsonResponse)
		return
	}

	var requestData struct {
		Status   string      `json:"status"`
		Listener interface{} `json:"listener"`
	}

	decoder := json.NewDecoder(r.Body)
	if err := decoder.Decode(&requestData); err != nil {
		response := map[string]interface{}{
			"success": false,
			"error":   "Invalid request body",
		}
		jsonResponse, _ := json.Marshal(response)
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusBadRequest)
		w.Write(jsonResponse)
		return
	}

	if requestData.Status == "" {
		response := map[string]interface{}{
			"success": false,
			"error":   "status is required",
		}
		jsonResponse, _ := json.Marshal(response)
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusBadRequest)
		w.Write(jsonResponse)
		return
	}

	if requestData.Listener == nil {
		response := map[string]interface{}{
			"success": false,
			"error":   "listener is required",
		}
		jsonResponse, _ := json.Marshal(response)
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusBadRequest)
		w.Write(jsonResponse)
		return
	}

	updatedEndpoint, err := module.UpdateEndpoint(id, requestData.Status, requestData.Listener)
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
		"data":    updatedEndpoint,
	}
	jsonResponse, _ := json.Marshal(response)
	w.Header().Set("Content-Type", "application/json")
	w.Write(jsonResponse)

}

func DeleteEndpoint(w http.ResponseWriter, r *http.Request) {
	logRequest("DELETE", r)

	id := r.URL.Query().Get("id")

	if id == "" {
		response := map[string]interface{}{
			"success": false,
			"error":   "Missing id query parameter",
		}
		jsonResponse, _ := json.Marshal(response)
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusBadRequest)
		w.Write(jsonResponse)
		return
	}

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
}
