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
    db_sqlite = request.app.state.db_sqlite
    sql = "INSERT INTO import_history (path, name) VALUES (?, ?)"
    db_sqlite.exec_sql(sql, (history.path, history.name))
    return {"status": "success"}


@router.get("/history/latest")
def get_latest_history(request: Request):
    """获取最后一次导入的根文件夹历史记录"""
    db_sqlite = request.app.state.db_sqlite
    # 按照 id 倒序获取最新的一条
    sql = "SELECT path, name FROM import_history ORDER BY id DESC LIMIT 1"
    res = db_sqlite.run_sql(sql)
    if res and len(res) > 0:
        return {"path": res[0][0], "name": res[0][1]}
    return None
