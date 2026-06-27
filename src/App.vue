<template>
  <el-container class="layout-container">
    <el-container>
      <!-- 1. 左侧最边缘的工具栏 -->
      <el-aside width="40px" class="toolbar-aside">
        <div class="toolbar">

          <!-- 文件面板按钮 -->
          <el-button type="primary"
                     :icon="FolderOpened"
                     class="tool-btn"
                     :class="{ 'is-active': activePanel === 'file' }"
                     @click="togglePanel('file')"
          />

          <!-- 待办事项面板按钮 -->
          <el-button
              type="warning"
              :icon="List"
              class="tool-btn"
              :class="{ 'is-active': activePanel === 'todo' }"
              @click="togglePanel('todo')"
          />
          <div class="toolbar-spacer"></div>
          <el-button type="success" :icon="Download" class="tool-btn" @click="handleExport"/>
          <el-button type="success" :icon="FolderAdd" class="tool-btn" @click="handleImport"/>
        </div>
      </el-aside>

      <!-- 2. 动态侧边栏面板 -->
      <!-- 绑定动态宽度 panelWidth -->
      <el-aside
          v-show="activePanel"
          :width="panelWidth + 'px'"
          class="side-panel"
      >
        <!-- 面板内容区 -->
        <div class="panel-inner">
          <!-- 文件视图 -->
          <div v-show="activePanel === 'file'" class="panel-content">
            <div v-if="fileTree.length === 0" class="empty-text">
              暂无数据，请点击左下角导入文件夹
            </div>
            <FilePage v-else :tree="fileTree"/>
          </div>

          <!-- 待办视图 -->
          <div v-show="activePanel === 'todo'" class="panel-content">
            <TodoPage/>
          </div>
        </div>

        <!-- 拖拽调整宽度的手柄 -->
        <div class="resizer" @mousedown="startResize"></div>
      </el-aside>

      <!-- 3. 主工作区 -->
      <el-main class="main-content">
        <div class="welcome-text">主工作区 (可用于显示文件详情或报表)</div>
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
import {ref, onBeforeUnmount} from 'vue';
import {FolderOpened, List, Download, FolderAdd} from '@element-plus/icons-vue';
import FilePage from './views/FilePage.vue';
import TodoPage from './views/TodoPage.vue';
import {open} from '@tauri-apps/plugin-dialog';
import {readDir} from '@tauri-apps/plugin-fs';
import {join} from '@tauri-apps/api/path';

// 记录当前激活的面板：'file' | 'todo' | '' (空字符串代表收起)
const activePanel = ref('file');
const fileTree = ref([]);

// 切换面板的核心逻辑
const togglePanel = (panelName) => {
  if (activePanel.value === panelName) {
    // 如果点击的是当前已展开的面板，则收起
    activePanel.value = '';
  } else {
    // 否则切换到对应的面板
    activePanel.value = panelName;
  }
};

// --- 面板宽度拖拽逻辑 ---
const panelWidth = ref(300); // 初始宽度
let startX = 0;
let startWidth = 0;

const startResize = (e) => {
  startX = e.clientX;
  startWidth = panelWidth.value;
  document.addEventListener('mousemove', onMouseMove);
  document.addEventListener('mouseup', stopResize);
  document.body.style.userSelect = 'none';
  document.body.style.cursor = 'col-resize';
};

const onMouseMove = (e) => {
  const deltaX = e.clientX - startX;
  let newWidth = startWidth + deltaX;

  // 动态计算边界限制
  // 最小宽度：自身保留 50px
  const MIN_WIDTH = 150;
  // 最大宽度：窗口总宽度 - 左侧工具栏(40) - 右侧强制保留间距(100)
  const MAX_WIDTH = window.innerWidth - 40 - 100;

  // 限制 newWidth 在最小和最大值之间
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

// --- 原有导入/构建逻辑 ---
const handleExport = () => {
  console.log('导出功能待实现');
};

const handleImport = async () => {
  try {
    const selected = await open({directory: true, multiple: false});
    if (!selected) return;
    fileTree.value = await buildFileTree(selected);
    activePanel.value = 'file';
  } catch (error) {
    console.error('导入失败:', error);
  }
};

async function buildFileTree(dirPath) {
  const entries = await readDir(dirPath);
  const children = [];
  for (const entry of entries) {
    const isDir = entry.isDirectory;
    const fullPath = await join(dirPath, entry.name);
    const node = {
      name: entry.name, path: fullPath, expanded: false, children: isDir ? [] : null
    };
    if (isDir) node.children = await buildFileTree(fullPath);
    children.push(node);
  }
  children.sort((a, b) => {
    if (a.children && !b.children) return -1;
    if (!a.children && b.children) return 1;
    return a.name.localeCompare(b.name);
  });
  return children;
}
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
}

.toolbar-aside {
  background-color: #2c3e50;
  display: flex;
  flex-direction: column;
  align-items: center;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.1);
  z-index: 10;
}

.toolbar {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
}

.tool-btn {
  width: 100%;
  border-radius: 0 !important;
  margin: 0 !important;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
}

.tool-btn.is-active {
  background-color: #1a252f;
  border-left: 3px solid #409eff;
}

.toolbar-spacer {
  flex: 1;
}

.side-panel {
  background-color: #f7f8fa;
  border-right: 1px solid #dcdfe6;
  position: relative;
  display: flex;
}

/* 侧边栏真实内容区，减去手柄的宽度 */
.panel-inner {
  flex: 1;
  width: calc(100% - 4px);
  height: 100%;
  overflow: hidden;
}

.panel-content {
  height: 100%;
  overflow: auto;
}

/* --- 核心拖拽手柄样式 --- */
.resizer {
  position: absolute;
  right: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  cursor: col-resize;
  background-color: transparent;
  transition: background-color 0.2s;
  z-index: 5;
}

.resizer:hover, .resizer:active {
  background-color: #409eff; /* 鼠标悬停时亮起蓝边提示可拖拽 */
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
  display: flex;
  align-items: center;
  padding: 0 16px;
  font-size: 13px;
  color: #606266;
  border-top: 1px solid #dcdfe6;
  height: 32px;
  line-height: 32px;
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