package routes

import (
	"agent-go/server/controller"
	"agent-go/server/middleware"

	"github.com/gorilla/mux"
)

func RegisterRoutes(router *mux.Router) {
	router.Use(middleware.Authentication)

	router.HandleFunc("/getEndPointStatus", controller.GetEndPointStatus).Methods("GET")
	router.HandleFunc("/getAllEndPoints", controller.GetAllEndPoints).Methods("GET")
	router.HandleFunc("/addEndpoint", controller.AddEndpoint).Methods("POST")
	router.HandleFunc("/updateStatus", controller.UpdateStatus).Methods("PATCH")
	router.HandleFunc("/deleteEndpoint", controller.DeleteEndpoint).Methods("DELETE")
}
