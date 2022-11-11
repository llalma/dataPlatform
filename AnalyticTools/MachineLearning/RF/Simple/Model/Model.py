import sys
sys.path.append(".....")

from fastapi import APIRouter, Request

from BaseModel import BaseModel


class RF(BaseModel):
    """
    Implementation of RF extends BaseModel
    """

    def __init__(self, name:str='RF', endpointPath:str='/RF'):
        super().__init__(name=name, endpointPath=endpointPath)
        self.name = name
        self.endpointPath = endpointPath

    async def train(self, data:Request):
        data = await data.json()
        print(data)
        return {'data':'hi from train function in RF'}

    async def predict(self, data:Request):
        data = await data.json()
        print(data)
        return {'data': 'hi from predict function in RF'}

