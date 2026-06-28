# server.py
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
import uvicorn
import sys
import os
from py import service_todo
from py import db_manager


class App(FastAPI):
    def __init__(self):
        super().__init__()
        self.db_duckdb = None
        self.db_sqlite = None

        # 加载数据库
        self.init_db()

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

    def init_db(self):
        data_dir = os.path.join(os.path.expanduser("~"), ".data-master")
        os.makedirs(data_dir, exist_ok=True)
        db_path = os.path.join(data_dir, "dm-file.data")

        self.db_duckdb = db_manager.DuckDBManager()
        self.db_sqlite = db_manager.SQLiteManager(db_path)


if __name__ == "__main__":
    app = App()
    print("Python backend is running on http://127.0.0.1:8000", file=sys.stderr)
    uvicorn.run(app, host="127.0.0.1", port=8000)
