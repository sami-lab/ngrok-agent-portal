package middleware

import (
	"encoding/json"
	"log"
	"net/http"
	"os"

	"github.com/joho/godotenv"
)

var agentID string
var agentToken string

func init() {
	if err := godotenv.Load(); err != nil {
		log.Fatal("Error loading .env file")
	}
	agentID = os.Getenv("AGENT_ID")
	agentToken = os.Getenv("AGENT_TOKEN")
}

func Authentication(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		id := r.Header.Get("AGENT_ID")
		token := r.Header.Get("AGENT_TOKEN")

		if id != agentID || token != agentToken {
			response := map[string]interface{}{
				"success": false,
				"error":   "Unauthorized",
			}
			jsonResponse, _ := json.Marshal(response)
			w.Header().Set("Content-Type", "application/json")
			w.WriteHeader(http.StatusUnauthorized)
			w.Write(jsonResponse)
			return
		}

		next.ServeHTTP(w, r)
	})
}
