import sys
sys.path.append(".....")

from fastapi import Request

from BaseModel import BaseModel


class KNN(BaseModel):
    """
    Implementation of KNN extends BaseModel
    """

    def __init__(self, name:str='KNN', endpointPath:str='/KNN'):
        super().__init__(name=name, endpointPath=endpointPath)
        self.name = name
        self.endpointPath = endpointPath

    async def train(self, data:Request):
        data = await data.json()
        print(data)
        return {'data':'hi from train function in KNN'}

    async def predict(self, data:Request):
        data = await data.json()
        print(data)
        return {'data': 'hi from predict function in KNN'}

