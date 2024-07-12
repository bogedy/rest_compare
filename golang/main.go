package main

import (
    "fmt"
    "net/http"

    "github.com/gin-gonic/gin"
)

func main() {
    // Create a Gin router
    r := gin.Default()

    // Define a handler for the root path "/"
    r.GET("/", func(c *gin.Context) {
        c.JSON(http.StatusOK, gin.H{
            "Hello": "World",
            "backend": "gin",
        })
    })

    gin.SetMode(gin.ReleaseMode)

    // Run the server
    if err := r.Run(); err != nil {
        fmt.Printf("Error starting server: %s\n", err)
    }
}