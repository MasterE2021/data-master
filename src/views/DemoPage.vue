<template>
  <div class="table-container">
    <table class="draggable-table">
      <thead>
      <tr>
        <th
            v-for="(col, index) in columns"
            :key="col.key"
            :class="{dragging: dragIndex === index,'drag-over': dragOverIndex === index && dragIndex !== index}"
            @mousedown="onMouseDown(index, $event)"
            @mouseenter="onMouseEnter(index)"
        >
          <span class="drag-handle">⠿</span>
          {{ col.title }}
        </th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="row in data" :key="row.id">
        <td v-for="col in columns" :key="col.key">
          {{ row[col.key] }}
        </td>
      </tr>
      </tbody>
    </table>

    <!-- 拖拽时跟随鼠标的浮层 -->
    <div
        v-if="dragIndex !== -1"
        class="drag-ghost"
        :style="{ left: ghostX + 'px', top: ghostY + 'px' }"
    >
      {{ columns[dragIndex].title }}
    </div>

    <div class="order-info">
      当前列顺序: {{ columns.map(c => c.title).join(' → ') }}
    </div>
  </div>
</template>

<script setup>
import {ref, onUnmounted} from 'vue'

const columns = ref([
  {key: 'name', title: '姓名'},
  {key: 'age', title: '年龄'},
  {key: 'city', title: '城市'},
  {key: 'job', title: '职业'},
  {key: 'salary', title: '薪资'}
])

const data = ref([
  {id: 1, name: '张三', age: 28, city: '北京', job: '工程师', salary: '25k'},
  {id: 2, name: '李四', age: 32, city: '上海', job: '设计师', salary: '20k'},
  {id: 3, name: '王五', age: 25, city: '深圳', job: '产品经理', salary: '22k'},
  {id: 4, name: '赵六', age: 30, city: '杭州', job: '运营', salary: '18k'},
  {id: 5, name: '钱七', age: 35, city: '成都', job: '架构师', salary: '35k'}
])

const dragIndex = ref(-1)
const dragOverIndex = ref(-1)
const ghostX = ref(0)
const ghostY = ref(0)

let startX = 0
let startY = 0
let pendingIndex = -1
let isDragging = false

function onMouseDown(index, event) {
  // 只响应左键
  if (event.button !== 0) return
  pendingIndex = index
  startX = event.clientX
  startY = event.clientY
  document.addEventListener('mousemove', onMouseMove)
  document.addEventListener('mouseup', onMouseUp)
}

function onMouseMove(event) {
  // 移动超过5px才算开始拖拽，避免误触
  if (!isDragging) {
    const dx = Math.abs(event.clientX - startX)
    const dy = Math.abs(event.clientY - startY)
    if (dx > 5 || dy > 5) {
      isDragging = true
      dragIndex.value = pendingIndex
      document.body.style.userSelect = 'none'
      document.body.style.cursor = 'grabbing'
    }
  }
  if (isDragging) {
    ghostX.value = event.clientX + 12
    ghostY.value = event.clientY + 12
  }
}

function onMouseEnter(index) {
  if (isDragging) {
    dragOverIndex.value = index
  }
}

function onMouseUp() {
  if (
      isDragging &&
      dragOverIndex.value !== -1 &&
      dragIndex.value !== dragOverIndex.value
  ) {
    const cols = [...columns.value]
    const [moved] = cols.splice(dragIndex.value, 1)
    cols.splice(dragOverIndex.value, 0, moved)
    columns.value = cols
  }
  // 清理状态
  isDragging = false
  pendingIndex = -1
  dragIndex.value = -1
  dragOverIndex.value = -1
  document.body.style.userSelect = ''
  document.body.style.cursor = ''
  document.removeEventListener('mousemove', onMouseMove)
  document.removeEventListener('mouseup', onMouseUp)
}

onUnmounted(() => {
  document.removeEventListener('mousemove', onMouseMove)
  document.removeEventListener('mouseup', onMouseUp)
})
</script>

<style scoped>
.table-container {
  padding: 20px;
}

.draggable-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 14px;
}

.draggable-table th,
.draggable-table td {
  border: 1px solid #ddd;
  padding: 10px 16px;
  text-align: left;
}

.draggable-table th {
  background: #f5f7fa;
  cursor: grab;
  user-select: none;
  transition: background 0.2s;
}

.draggable-table th:hover {
  background: #e8ecf1;
}

.draggable-table th.dragging {
  opacity: 0.4;
  background: #d0e2ff;
}

.draggable-table th.drag-over {
  border-left: 3px solid #409eff;
  background: #ecf5ff;
}

.drag-handle {
  color: #999;
  margin-right: 6px;
}

/* 跟随鼠标的浮层 */
.drag-ghost {
  position: fixed;
  z-index: 9999;
  padding: 6px 14px;
  background: #409eff;
  color: #fff;
  border-radius: 4px;
  font-size: 13px;
  pointer-events: none;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

.order-info {
  margin-top: 12px;
  color: #666;
  font-size: 13px;
}
</style>