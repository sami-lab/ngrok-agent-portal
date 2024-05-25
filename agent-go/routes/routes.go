package routes

import (
	"agent-go/server/controller"

	"github.com/gorilla/mux"
)

func RegisterRoutes(router *mux.Router) {
	router.HandleFunc("/get", controller.GetHandler).Methods("GET")
	router.HandleFunc("/post", controller.PostHandler).Methods("POST")
	router.HandleFunc("/put", controller.PutHandler).Methods("PUT")
	router.HandleFunc("/delete", controller.DeleteHandler).Methods("DELETE")
}
