import os
from fastapi import FastAPI
from utils.config import load_config
from contextlib import asynccontextmanager
import endpointsmanager

# Ensure that NODE_ENV variable is set
assert os.getenv("NODE_ENV") in ["development", "production"], "You must set the NODE_ENV variable to development or production"



@asynccontextmanager
async def lifespan(app: FastAPI):
    await  endpointsmanager.initializeAgentConfig()
    yield 
# Create an instance of FastAPI
app = FastAPI(lifespan=lifespan)

# Load express configurations
load_config(app)


# Export the app instance
__all__ = ['app']
