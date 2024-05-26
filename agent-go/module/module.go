package module

import (
	"errors"

	"github.com/google/uuid"
)

var endpoints []map[string]interface{}

func init() {
	endpoints = []map[string]interface{}{
		{
			"_id":      uuid.New().String(),
			"status":   "offline",
			"listener": nil,
		},
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

func AddEndpoint(status string, listener interface{}) (map[string]interface{}, error) {
	if status == "" {
		return nil, errors.New("status is required")
	}
	if listener == nil {
		return nil, errors.New("listener is required")
	}

	newEndpoint := map[string]interface{}{
		"_id":      uuid.New().String(),
		"status":   status,
		"listener": listener,
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

func UpdateEndpoint(id, status string, listener interface{}) (map[string]interface{}, error) {
	if status == "" {
		return nil, errors.New("status is required")
	}
	if listener == nil {
		return nil, errors.New("listener is required")
	}

	for _, endpoint := range endpoints {
		if endpoint["_id"] == id {
			endpoint["status"] = status
			endpoint["listener"] = listener
			return endpoint, nil
		}
	}

	return nil, errors.New("endpoint not found")
}
