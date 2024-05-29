package module

import (
	"errors"

	"encoding/json"
	"log"
	"net/http"

	"os"

	"github.com/google/uuid"
)

var endpoints []map[string]interface{}

func FetchAgentConfig() {
	agentID := os.Getenv("AGENT_ID")
	agentToken := os.Getenv("AGENT_TOKEN")
	baseUrl := os.Getenv("BACKEND_URL")

	if agentID == "" || agentToken == "" || baseUrl == "" {
		log.Fatal("Environment variables AGENT_ID, AGENT_TOKEN, or BACKEND_URL not set")
	}

	url := baseUrl + "/api/v1/endpoint/" + agentID

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		log.Fatalf("Error creating HTTP request: %v", err)
	}
	req.Header.Set("Token", agentToken)

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		log.Fatalf("Error making HTTP request: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		log.Fatalf("Non-200 response from server: %d %s", resp.StatusCode, resp.Status)
	}

	var apiResp map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&apiResp); err != nil {
		log.Fatalf("Error decoding JSON response: %v", err)
	}

	data, ok := apiResp["data"].(map[string]interface{})
	if !ok {
		log.Fatalf("Unexpected JSON structure: missing 'data' field")
	}

	doc, ok := data["doc"].([]interface{})
	if !ok {
		log.Fatalf("Unexpected JSON structure: 'doc' field is not an array")
	}

	endpoints = make([]map[string]interface{}, len(doc))
	for i, item := range doc {
		endpoint, ok := item.(map[string]interface{})
		if !ok {
			log.Fatalf("Unexpected JSON structure: item in 'doc' array is not an object")
		}
		endpoint["status"] = "offline"
		endpoints[i] = endpoint
	}
}

func GetAllEndPoints() []map[string]interface{} {
	return endpoints
}

func GetEndpointStatus(id string) map[string]interface{} {
	for _, endpoint := range endpoints {
		if endpoint["_id"] == id {
			return endpoint
		}
	}
	return map[string]interface{}{}
}

func AddEndpoint(endpointYaml string, listener interface{}) (map[string]interface{}, error) {

	newEndpoint := map[string]interface{}{
		"_id":          uuid.New().String(),
		"status":       "offline",
		"listener":     listener,
		"endpointYaml": endpointYaml,
	}

	endpoints = append(endpoints, newEndpoint)
	return newEndpoint, nil
}

func DeleteEndpoint(id string) {
	for i, endpoint := range endpoints {
		if endpoint["_id"] == id {
			endpoints = append(endpoints[:i], endpoints[i+1:]...)
			break
		}
	}
}

func UpdateEndpointStatus(id string) (map[string]interface{}, error) {
	for _, endpoint := range endpoints {
		if endpoint["_id"] == id {
			if endpoint["status"] == "offline" {
				endpoint["status"] = "online"
				return endpoint, nil
			} else {
				endpoint["status"] = "offline"
				return endpoint, nil

			}
		}
	}
	return nil, errors.New("endpoint not found")
}
