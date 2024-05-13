import os
from uvicorn import run
from fastapi import FastAPI
# Load environment variables
from dotenv import load_dotenv
load_dotenv()

#import endpointsmanager
from app import app




# Start the server
if __name__ == "__main__":
    port = int(os.getenv("PORT", 8001))
    run(app,host="0.0.0.0", port=port)
    print("In......")
    # Call initialize_agent_config after server has started
    #endpointsmanager.initialize_agent_config()