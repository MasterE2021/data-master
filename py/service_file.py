# py/service_file.py
from fastapi import APIRouter, Request
from pydantic import BaseModel

router = APIRouter(prefix="/file", tags=["file"])


class HistoryCreate(BaseModel):
    path: str
    name: str


@router.post("/history")
def save_history(history: HistoryCreate, request: Request):
    """保存导入过的根文件夹历史"""
    db_sqlite = request.app.db_sqlite
    sql = "INSERT INTO import_history (path, name) VALUES (?, ?)"
    db_sqlite.exec_sql(sql, (history.path, history.name))
    return {"status": "success"}


@router.get("/history/all")
def get_all_history(request: Request):
    """获取所有导入过的根文件夹历史记录（去重）"""
    db_sqlite = request.app.db_sqlite
    # 按 path 去重，并按导入顺序排列
    sql = "SELECT path, name FROM import_history GROUP BY path, name ORDER BY max(id) ASC"
    res = db_sqlite.run_sql(sql)
    return [{"path": row[0], "name": row[1]} for row in res]
