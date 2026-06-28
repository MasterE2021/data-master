# service_todo.py
from fastapi import APIRouter
from pydantic import BaseModel

# 创建 APIRouter 实例，用于替代直接使用 app 注册路由
router = APIRouter()

# 模拟数据库与自增ID
fake_db = []
current_id = 1


# 数据模型
class TodoCreate(BaseModel):
    title: str


@router.get("/todos")
def get_todos():
    """获取所有待办事项"""
    return fake_db


@router.post("/todos")
def create_todo(todo: TodoCreate):
    """创建新的待办事项"""
    global current_id
    new_todo = {"id": current_id, "title": todo.title, "completed": False}
    fake_db.append(new_todo)
    current_id += 1
    return new_todo


@router.put("/todos/{todo_id}")
def toggle_todo(todo_id: int):
    """切换待办事项的完成状态"""
    for todo in fake_db:
        if todo["id"] == todo_id:
            todo["completed"] = not todo["completed"]
            return todo
    return {"error": "Todo not found"}


@router.delete("/todos/{todo_id}")
def delete_todo(todo_id: int):
    """删除待办事项"""
    global fake_db
    fake_db = [todo for todo in fake_db if todo["id"] != todo_id]
    return {"status": "deleted"}
