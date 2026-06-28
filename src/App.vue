<template>
  <el-container class="layout-container">
    <el-container class="main-body">
      <!-- 1. 左侧工具栏 -->
      <el-aside width="40px" class="toolbar-aside">
        <div class="toolbar">
          <div class="toolbar-top">
            <el-button type="primary" :icon="FolderOpened" class="tool-btn"
                       :class="{ 'is-active': activePanel === 'file' }" @click="togglePanel('file')"/>
            <el-button type="warning" :icon="List" class="tool-btn" :class="{ 'is-active': activePanel === 'todo' }"
                       @click="togglePanel('todo')"/>
          </div>
          <div class="toolbar-spacer"></div>
          <div class="toolbar-bottom">
            <el-button type="success" :icon="Download" class="tool-btn" @click="handleExport"/>
            <el-button type="success" :icon="FolderAdd" class="tool-btn" @click="handleImport"/>
          </div>
        </div>
      </el-aside>

      <!-- 2. 动态侧边栏 -->
      <el-aside v-show="activePanel" :width="panelWidth + 'px'" class="side-panel">
        <div class="panel-inner">
          <div v-show="activePanel === 'file'" class="panel-content">
            <div v-if="!rootPath" class="empty-text">暂无数据，请点击左下角导入文件夹</div>
            <FilePage v-else :root-path="rootPath" :root-name="rootName"/>
          </div>
          <div v-show="activePanel === 'todo'" class="panel-content">
            <TodoPage/>
          </div>
        </div>
        <div class="resizer" @mousedown="startResize"></div>
      </el-aside>

      <!-- 3. 主工作区 -->
      <el-main class="main-content">
        <div class="welcome-text">主工作区</div>
      </el-main>
    </el-container>

    <!-- 底部状态栏 -->
    <el-footer height="32px" class="status-bar">
      <div class="status-left">当前面板: {{ activePanel || '已收起' }}</div>
      <div class="status-center">v1.0.0</div>
      <div class="status-right">状态栏：就绪</div>
    </el-footer>
  </el-container>
</template>

<script setup>
import {ref, onBeforeUnmount, onMounted} from 'vue';
import {FolderOpened, List, Download, FolderAdd} from '@element-plus/icons-vue';
import FilePage from './views/FilePage.vue';
import TodoPage from './views/TodoPage.vue';
import {open} from '@tauri-apps/plugin-dialog';
import {basename} from '@tauri-apps/api/path';

const activePanel = ref('');
const rootPath = ref('');
const rootName = ref('');

// ========== 启动时加载历史记录 ==========
onMounted(async () => {
  try {
    const res = await fetch('http://127.0.0.1:8000/file/history/latest');
    const data = await res.json();
    if (data && data.path && data.name) {
      rootPath.value = data.path;
      rootName.value = data.name;
      activePanel.value = 'file'; // 有记录则直接展开侧边栏
    }
  } catch (err) {
    console.error('获取历史导入记录失败:', err);
  }
});

const togglePanel = (panelName) => {
  activePanel.value = activePanel.value === panelName ? '' : panelName;
};

// --- 宽度拖拽逻辑 ---
const panelWidth = ref(300);
let startX = 0, startWidth = 0;
const startResize = (e) => {
  startX = e.clientX;
  startWidth = panelWidth.value;
  document.addEventListener('mousemove', onMouseMove);
  document.addEventListener('mouseup', stopResize);
  document.body.style.userSelect = 'none';
  document.body.style.cursor = 'col-resize';
};
const onMouseMove = (e) => {
  let newWidth = startWidth + (e.clientX - startX);
  const MIN_WIDTH = 10, MAX_WIDTH = window.innerWidth - 60;
  if (newWidth < MIN_WIDTH) newWidth = MIN_WIDTH;
  if (newWidth > MAX_WIDTH) newWidth = MAX_WIDTH;
  panelWidth.value = newWidth;
};
const stopResize = () => {
  document.removeEventListener('mousemove', onMouseMove);
  document.removeEventListener('mouseup', stopResize);
  document.body.style.userSelect = '';
  document.body.style.cursor = '';
};
onBeforeUnmount(() => {
  document.removeEventListener('mousemove', onMouseMove);
  document.removeEventListener('mouseup', stopResize);
});

const handleExport = () => {
  console.log('导出功能待实现');
};

// ========== 导入文件及保存记录逻辑 ==========
const handleImport = async () => {
  try {
    const selected = await open({directory: true, multiple: false});
    if (!selected) return;

    rootPath.value = selected;
    rootName.value = await basename(selected);
    activePanel.value = 'file';

    // 导入成功后，通知后端保存记录
    await fetch('http://127.0.0.1:8000/file/history', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify({path: rootPath.value, name: rootName.value})
    });
  } catch (error) {
    console.error('导入失败:', error);
  }
};
</script>

<style>
html, body {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

#app {
  height: 100%;
}
</style>

<style scoped>
.layout-container {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.main-body {
  flex: 1;
  overflow: hidden;
}

.toolbar-aside {
  background-color: #2c3e50;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.1);
  z-index: 10;
  height: 100%;
}

.toolbar {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
  overflow: hidden;
}

.toolbar-top, .toolbar-bottom {
  width: 100%;
}

.toolbar-spacer {
  flex: 1;
}

.tool-btn {
  width: 100%;
  border-radius: 0 !important;
  margin: 0 !important;
  height: 40px;
  border: none;
}

.tool-btn.is-active {
  background-color: #1a252f;
  border-left: 3px solid #409eff;
}

.side-panel {
  background-color: #f7f8fa;
  border-right: 1px solid #dcdfe6;
  position: relative;
  display: flex;
  height: 100%;
}

.panel-inner {
  flex: 1;
  width: calc(100% - 4px);
  height: 100%;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.panel-content {
  height: 100%;
  width: 100%;
  overflow: auto;
}

.resizer {
  position: absolute;
  right: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  cursor: col-resize;
  z-index: 5;
}

.resizer:hover, .resizer:active {
  background-color: #409eff;
}

.empty-text {
  padding: 20px;
  color: #909399;
  font-size: 13px;
  text-align: center;
}

.main-content {
  background-color: #ffffff;
  padding: 24px;
  overflow: auto;
}

.welcome-text {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  color: #909399;
  font-size: 20px;
}

.status-bar {
  background-color: #e9eef3;
  border-top: 1px solid #dcdfe6;
  height: 32px;
  line-height: 32px;
  display: flex;
  padding: 0 16px;
  font-size: 13px;
  color: #606266;
  flex: 0 0 auto;
}

.status-left, .status-right {
  flex: 1;
}

.status-center {
  flex: 0 0 auto;
  text-align: center;
}

.status-right {
  text-align: right;
}
</style>