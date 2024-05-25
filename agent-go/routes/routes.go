package routes

import (
	"agent-go/server/controller"

	"github.com/gorilla/mux"
)

func RegisterRoutes(router *mux.Router) {
	router.HandleFunc("/getEndPointStatus", controller.GetEndPointStatus).Methods("GET")
	router.HandleFunc("/addEndpoint", controller.AddEndpoint).Methods("POST")
	router.HandleFunc("/updateStatus", controller.UpdateStatus).Methods("PATCH")
	router.HandleFunc("/deleteEndpoint", controller.DeleteEndpoint).Methods("DELETE")
}
