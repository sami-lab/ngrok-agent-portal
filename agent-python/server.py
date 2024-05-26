import os
import ngrok

from uvicorn import Config, Server, run
from fastapi import FastAPI
# Load environment variables
from dotenv import load_dotenv
load_dotenv()

#import endpointsmanager
from app import app




# Start the server
if __name__ == "__main__":
    port = int(os.getenv("PORT", 8001))
    print("-------------------------------------------",port)
    #listener =  ngrok.forward(authtoken_from_env=True, proto="http", addr="localhost:8001", domain="sami.tunnels.ctindel-ngrok.com")
    run(app, host="0.0.0.0", port=port)


