# py/service_data.py
from fastapi import APIRouter
from pydantic import BaseModel
import duckdb
import os

router = APIRouter(prefix="/data", tags=["data"])


class FileRequest(BaseModel):
    path: str
    page: int = 1
    page_size: int = 1000
    total: int = 0


@router.post("/preview")
def preview_data(req: FileRequest):
    path = req.path
    page = req.page
    page_size = req.page_size
    ext = os.path.splitext(path)[1].lower()

    if ext not in ['.csv', '.xlsx', '.parquet']:
        return {"error": "暂不支持该文件格式预览"}

    try:
        con = duckdb.connect()

        # 优化：如果总数已经大于0（说明不是第一页），直接跳过耗时的 count 查询
        total_rows = req.total
        if total_rows <= 0:
            count_sql = f"SELECT count(*) FROM '{path}'"
            total_rows = con.execute(count_sql).fetchone()[0]

        offset = (page - 1) * page_size
        sql = f"SELECT * FROM '{path}' LIMIT {page_size} OFFSET {offset}"

        result = con.execute(sql)
        columns = [desc[0] for desc in result.description]
        rows = result.fetchall()

        # 3. 转为字典列表
        data = [dict(zip(columns, row)) for row in rows]

        # 返回数据和分页信息
        return {
            "columns": columns,
            "data": data,
            "total": total_rows
        }

    except Exception as e:
        return {"error": str(e)}
