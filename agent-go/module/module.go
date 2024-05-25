package module

import (
	"github.com/google/uuid"
)

var endpoints []map[string]interface{}

func init() {
	// Initialize the state with one endpoint
	endpoints = []map[string]interface{}{
		{
			"_id":      uuid.New().String(),
			"status":   "offline",
			"listener": nil,
		},
	}
}

// GetEndpoint returns the current state of endpoints
func GetEndpoint() []map[string]interface{} {
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

// AddEndpoint adds a new endpoint to the state
func AddEndpoint(endpoint map[string]interface{}) {
	endpoint["_id"] = uuid.New().String()
	endpoints = append(endpoints, endpoint)
}
