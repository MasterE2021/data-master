<template>
  <el-container class="layout-container">
    <el-container>
      <!-- 1. 左侧最边缘的工具栏 -->
      <el-aside width="40px" class="toolbar-aside">
        <div class="toolbar">
          <!-- 点击文件按钮：切换文件面板的展开/收起 -->
          <el-button
              type="primary"
              :icon="FolderOpened"
              class="tool-btn"
              :class="{ 'is-active': showFilePanel }"
              @click="toggleFilePanel"
          />
          <el-button type="warning" :icon="List" class="tool-btn" @click="currentView = 'todo'"/>
          <div class="toolbar-spacer"></div>
          <el-button type="success" :icon="Download" class="tool-btn" @click="handleExport"/>
          <el-button type="success" :icon="FolderAdd" class="tool-btn" @click="handleImport"/>
        </div>
      </el-aside>

      <!-- 2. 文件树面板（有内容或被激活时才占用空间） -->
      <el-aside
          v-show="showFilePanel"
          width="250px"
          class="file-panel"
      >
        <div v-if="fileTree.length === 0" class="empty-text">
          暂无数据，请点击左下角导入文件夹
        </div>
        <FilePage v-else :tree="fileTree"/>
      </el-aside>

      <!-- 3. 主工作区 -->
      <el-main class="main-content">
        <TodoPage v-if="currentView === 'todo'"/>
        <div v-else class="welcome-text">欢迎使用 Data Master</div>
      </el-main>
    </el-container>

    <!-- 底部状态栏 -->
    <el-footer height="32px" class="status-bar">
      <div class="status-left"></div>
      <div class="status-center">v1.0.0</div>
      <div class="status-right">状态栏：就绪</div>
    </el-footer>
  </el-container>
</template>

<script setup>
import {ref} from 'vue';
import {FolderOpened, List, Download, FolderAdd} from '@element-plus/icons-vue';
import FilePage from './views/FilePage.vue';
import TodoPage from './views/TodoPage.vue';
import {open} from '@tauri-apps/plugin-dialog';
import {readDir} from '@tauri-apps/plugin-fs';
import {join} from '@tauri-apps/api/path';

const currentView = ref('todo');
const fileTree = ref([]);

// 控制文件面板的展开与收起
const showFilePanel = ref(false);

const toggleFilePanel = () => {
  showFilePanel.value = !showFilePanel.value;
};

// 导出按钮
const handleExport = () => {
  console.log('导出功能待实现');
};

// 导入按钮：选择文件夹并构建文件树
const handleImport = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择文件夹'
    });
    if (!selected) return;

    // 构建文件树
    const tree = await buildFileTree(selected);
    fileTree.value = tree;

    // 导入成功后自动展开文件树面板
    showFilePanel.value = true;
  } catch (error) {
    console.error('导入文件夹失败:', error);
  }
};

// 递归读取目录
async function buildFileTree(dirPath) {
  const entries = await readDir(dirPath);
  const children = [];

  for (const entry of entries) {
    const isDir = entry.isDirectory;
    const fullPath = await join(dirPath, entry.name);

    const node = {
      name: entry.name,
      path: fullPath,
      expanded: false,
      children: isDir ? [] : null
    };

    if (isDir) {
      node.children = await buildFileTree(fullPath);
    }
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

/* 文件按钮激活时的样式 */
.tool-btn.is-active {
  background-color: #1a252f;
  border-left: 3px solid #409eff;
}

.toolbar-spacer {
  flex: 1;
}

/* 文件树面板样式 */
.file-panel {
  background-color: #f7f8fa;
  border-right: 1px solid #dcdfe6;
  display: flex;
  flex-direction: column;
  overflow: auto;
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