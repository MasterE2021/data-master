<template>
  <div class="todo-container">
    <h2>📝 待办事项 (Vue + Python)</h2>

    <div class="input-group">
      <input
          v-model="newTodo"
          @keyup.enter="addTodo"
          placeholder="添加新的待办事项..."
      />
      <button @click="addTodo">添加</button>
    </div>

    <ul class="todo-list">
      <li v-for="todo in todos" :key="todo.id" :class="{ completed: todo.completed }">
        <span @click="toggleTodo(todo.id)" class="text">
          {{ todo.completed ? '✅' : '⏳' }} {{ todo.title }}
        </span>
        <button @click="deleteTodo(todo.id)" class="delete-btn">删除</button>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import {ref, onMounted} from 'vue';

// Python FastAPI 后端地址
const API_URL = 'http://127.0.0.1:8000/todos';

interface Todo {
  id: number;
  title: string;
  completed: boolean;
}

const todos = ref<Todo[]>([]);
const newTodo = ref('');

// 获取待办列表
const fetchTodos = async () => {
  try {
    const res = await fetch(API_URL);
    todos.value = await res.json();
  } catch (error) {
    console.error("无法连接到 Python 后端:", error);
  }
};

// 添加待办
const addTodo = async () => {
  if (!newTodo.value.trim()) return;
  await fetch(API_URL, {
    method: 'POST',
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify({title: newTodo.value})
  });
  newTodo.value = '';
  await fetchTodos();
};

// 切换完成状态
const toggleTodo = async (id: number) => {
  await fetch(`${API_URL}/${id}`, {method: 'PUT'});
  await fetchTodos();
};

// 删除待办
const deleteTodo = async (id: number) => {
  await fetch(`${API_URL}/${id}`, {method: 'DELETE'});
  await fetchTodos();
};

// 组件挂载时获取数据
onMounted(() => {
  fetchTodos();
});
</script>

<style scoped>
.todo-container {
  background: #574c4c;
  padding: 20px;
  border-radius: 12px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  width: 400px;
}

.input-group {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}

input {
  flex: 1;
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
}

button {
  padding: 8px 16px;
  background: #4f46e5;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:hover {
  background: #4338ca;
}

.todo-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

li {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px;
  border-bottom: 1px solid #eee;
}

.text {
  cursor: pointer;
  user-select: none;
}

.completed .text {
  text-decoration: line-through;
  color: #888;
}

.delete-btn {
  background: #ef4444;
  font-size: 12px;
  padding: 4px 8px;
}

.delete-btn:hover {
  background: #dc2626;
}
</style>