# py/service_data.py
from fastapi import APIRouter
from pydantic import BaseModel
import duckdb
import os

router = APIRouter(prefix="/data", tags=["data"])


class FileRequest(BaseModel):
    path: str


@router.post("/preview")
def preview_data(req: FileRequest):
    path = req.path
    ext = os.path.splitext(path)[1].lower()

    try:
        # 连接 DuckDB（内存模式）
        con = duckdb.connect()

        # 根据不同后缀构建不同的查询 SQL (限制前 100 行预览)
        if ext in ['.csv', '.xlsx', '.parquet']:
            sql = f"select * from '{path}' limit 1000"
        else:
            return {"error": "暂不支持该文件格式预览"}

        # 执行查询并获取列名和数据
        result = con.execute(sql)
        columns = [desc[0] for desc in result.description]
        rows = result.fetchall()

        # 转为字典列表方便 Vue 表格渲染
        data = [dict(zip(columns, row)) for row in rows]
        return {"columns": columns, "data": data}

    except Exception as e:
        return {"error": str(e)}
