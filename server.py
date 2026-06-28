# server.py
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
import uvicorn
import sys
from py import service_todo


class App(FastAPI):
    def __init__(self):
        super().__init__()

        # 配置 CORS 中间件
        self.add_middleware(
            CORSMiddleware,
            allow_origins=["*"],
            allow_credentials=True,
            allow_methods=["*"],
            allow_headers=["*"],
        )

        # 挂载待办事项的服务的路由
        self.include_router(service_todo.router)


if __name__ == "__main__":
    app = App()
    print("Python backend is running on http://127.0.0.1:8000", file=sys.stderr)
    uvicorn.run(app, host="127.0.0.1", port=8000)
