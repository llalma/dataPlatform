from fastapi import APIRouter, Request

class BaseModel():
    """
    Base Model class that other Analytic Tools inherit from
    """

    def __init__(self, name:str, endpointPath:str):
        self.name = name
        self.endpointPath = endpointPath

        #Create router
        self.router = APIRouter()

        #Add train function to api
        self.router.add_api_route(f"{self.endpointPath}/train", self.train, methods=["POST"])

        #Add predict function to api
        self.router.add_api_route(f"{self.endpointPath}/predict", self.predict, methods=["POST"])


    async def train(self, data:Request):
        raise NotImplementedError()

    async def predict(self, data:Request):
        raise NotImplementedError()

