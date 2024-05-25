package app

import (
	"agent-go/server/routes"

	"github.com/gorilla/mux"
)

func SetupRouter() *mux.Router {
	router := mux.NewRouter()
	routes.RegisterRoutes(router)
	return router
}
