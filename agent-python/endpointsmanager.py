import logging
import yaml
import ngrok

# Import logger from utils.logger module
from utils.logger import logger

# Import agentEndpointController from controllers.agentEndpointController module
#from controllers.agentEndpointController import fetchAgentConfig

# Initialize ngrok
#ngrok.set_auth_token(token=ngrok.get_auth_token())

endpoints = []

async def initializeAgentConfig():
    global endpoints
     # Lazy import fetchAgentConfig
    from controllers.agentEndpointController import fetchAgentConfig
    response = await fetchAgentConfig()
    if response.get("success")==True:
        endpoints = [{
            **x,
            "status": "offline"
        } for x in response.get("data")]

async def changeEndpointsStatus(id):
    global endpoints
    success = False
    endpoint = next((e for e in endpoints if e["id"] == id), None)
    if endpoint:
        if endpoint.get("status") == "offline":
            logger.debug(endpoint)
            try:
                endpointYaml = yaml.safe_load(endpoint.get("endpointYaml"))
                logger.debug(f"Starting endpoint {endpoint.get('name')} with options: {endpointYaml}")
                # listener = ngrok.connect(**{**{"authtoken_from_env": True}, **endpointYaml})
                # logger.info(f"Ingress established for endpoint {endpoint.get('name')} at: {listener.get("public_url")}")
                # endpoint["listener"] = listener
                endpoint["status"] = "online"
                success = True
            except Exception as e:
                logger.error(f"Listener setup error: {e}")
        else:
            logger.debug(f"Stopping endpoint {endpoint['name']}")
            try:
                #endpoint["listener"].close()
                logger.info(f"Ingress closed")
                endpoint["status"] = "offline"
                success = True
            except Exception as e:
                logger.error(f"Listener close error: {e}")
    return {"success": success, "data": endpoints}

def getEndpoints():
    return endpoints

def addEndpoint(endpoint):
    global endpoints
    endpoints.append({
        **endpoint,
        "status": "offline",
        "listener": None
    })
    return endpoints

def deleteEndpoint(id):
    global endpoints
    endpoints = [e for e in endpoints if e["id"] != id]
    print(endpoints)
    return endpoints
