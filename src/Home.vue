<!-- src/Home.vue -->
<script setup>
import {ref, onMounted, onBeforeUnmount, onUnmounted} from 'vue';
import Signal from './utils/SignalCenter.js'
import {SignalName} from './utils/Common.js'
import FilePage from "./views/FilePage.vue";

/*
ref	            单一值、需要整体替换对象、在模板中需自动解包时。
reactive	      聚合多个相关状态、不希望用 .value 时，但注意不能解构。
computed	      任何派生状态，避免在模板中写复杂表达式，提升性能。
onMounted	      初始化需要 DOM 存在的操作（图表、地图、原生事件）。
onBeforeUnmount	清理所有在 onMounted 或其它地方创建的“外部资源”（定时器、订阅、事件监听）。
nextTick	      当你改变数据后需要立刻读取新的 DOM 状态，或者需要确保子组件已更新完毕时。
*/


// ========== 组件状态 ==========
const is_open_list_space = ref(false)
const file_folder = ref([]);

// ========== 信号系统 ==========
function btn_file() {
  is_open_list_space.value = !is_open_list_space.value
  // Signal.emit(SignalName.btn_file_click, ['点击按钮', 1, 2, 3], new Date(), {__source: 'Home.vue'});
}

// ========== 启动时加载历史记录 ==========
onMounted(async () => {
  try {
    const res = await fetch('http://127.0.0.1:8000/file/history/all'); // 调用新接口
    file_folder.value = await res.json();
  } catch (err) {
    console.error('获取历史导入记录失败:', err);
  }
});

</script>

<template>
  <div class="root">
    <div class="work-space">
      <div class="tool-bar">
        <button class="btn-file" @click="btn_file">文件</button>
        <button class="btn-1">按钮</button>
        <button class="btn-2">按钮</button>
      </div>

      <div class="list-space" v-if="is_open_list_space">
        <div class="file-tree">
          <FilePage :workspace-folders="file_folder"/>
        </div>
      </div>

      <div class="core-space">
        工作空间
      </div>

    </div>

    <div class="state-space">
      状态空间
    </div>
  </div>
</template>

<style>
html, body {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
}
</style>

<style scoped>
.root {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.work-space {
  flex: 1;
  display: flex;
  border-top: 1px solid #dcdfe6;
  border-bottom: 1px solid #dcdfe6;
  overflow: hidden;
}

.tool-bar {
  font-size: 8px;
  width: 40px;
  background: #939090;
}

.tool-bar button {
  white-space: nowrap; /* 强制文字在一行显示 */
  width: 100%;
  text-align: center;
  font-size: 14px; /* 原先 8px 太小，可适当放大 */
  padding: 10px 5px;
  border: none;
  background-color: transparent;
}

/* 悬停效果：轻微背景色，表明可交互 */
.tool-bar button:hover {
  background: #8d9c70;
}

/* 点击/激活状态（如果将来需要高亮当前选中的按钮） */
.tool-bar button:active {
  background: #ffffff;
}

.list-space {
  background: #b6b8bf;
}

.core-space {
  flex: 1;
  text-align: center;
}

.state-space {
  height: 30px;
  text-align: center;
}

.file-tree {
  flex: 1;
  width: calc(100% - 4px);
  height: 100%;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

</style>