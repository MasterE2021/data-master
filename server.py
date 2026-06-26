from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
import uvicorn
import sys

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

fake_db = []
current_id = 1


class TodoCreate(BaseModel):
    title: str


@app.get("/todos")
def get_todos():
    return fake_db


@app.post("/todos")
def create_todo(todo: TodoCreate):
    global current_id
    new_todo = {"id": current_id, "title": todo.title, "completed": False}
    fake_db.append(new_todo)
    current_id += 1
    return new_todo


@app.put("/todos/{todo_id}")
def toggle_todo(todo_id: int):
    for todo in fake_db:
        if todo["id"] == todo_id:
            todo["completed"] = not todo["completed"]
            return todo
    return {"error": "Todo not found"}


@app.delete("/todos/{todo_id}")
def delete_todo(todo_id: int):
    global fake_db
    fake_db = [todo for todo in fake_db if todo["id"] != todo_id]
    return {"status": "deleted"}


if __name__ == "__main__":
    print("Python backend is running on http://127.0.0.1:8000", file=sys.stderr)
    uvicorn.run(app, host="127.0.0.1", port=8000)
