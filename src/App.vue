<template>
  <el-container class="layout-container">
    <el-container>
      <!-- 左侧工具栏 -->
      <el-aside width="40px" class="toolbar-aside">
        <div class="toolbar">
          <el-button type="primary" :icon="FolderOpened" class="tool-btn" @click="switchTo('file')"/>
          <el-button type="warning" :icon="List" class="tool-btn" @click="switchTo('todo')"/>
          <div class="toolbar-spacer"></div>

          <!-- 导出按钮（仅示意，可绑定功能） -->
          <el-button type="success" :icon="Download" class="tool-btn" @click="handleExport"/>

          <!-- 新增导入按钮 -->
          <el-button type="success" :icon="FolderAdd" class="tool-btn" @click="handleImport"/>
        </div>
      </el-aside>

      <!-- 主工作区 -->
      <el-main class="main-content">
        <!-- 文件视图需要传入文件树 -->
        <FilePage v-if="currentViewKey === 'file'" :tree="fileTree"/>
        <TodoPage v-else-if="currentViewKey === 'todo'"/>
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
// 【新增】引入路径拼接方法
import {join} from '@tauri-apps/api/path';

const views = {
  file: 'file',
  todo: 'todo'
};

const currentViewKey = ref(views.file);
const fileTree = ref([]);

const switchTo = (key) => {
  currentViewKey.value = key;
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

    // 开始构建文件树
    const tree = await buildFileTree(selected);
    fileTree.value = tree;
  } catch (error) {
    console.error('导入文件夹失败:', error);
  }
};

// 递归读取目录，构建树形结构
async function buildFileTree(dirPath) {
  const entries = await readDir(dirPath);
  const children = [];

  for (const entry of entries) {
    const isDir = entry.isDirectory;
    // 【修改】手动拼接完整路径 (兼容 Windows 和 Mac 的路径分隔符)
    const fullPath = await join(dirPath, entry.name);

    const node = {
      name: entry.name,
      path: fullPath,
      expanded: false,
      children: isDir ? [] : null
    };

    if (isDir) {
      // 【修改】使用拼接好的 fullPath 进行递归
      node.children = await buildFileTree(fullPath);
    }
    children.push(node);
  }

  // 按文件夹在前、文件在后排序
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
  padding: 0;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.1);
}

.toolbar {
  display: flex;
  flex-direction: column;
  align-items: center;
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
}

.toolbar-spacer {
  flex: 1;
}

.main-content {
  background-color: #ffffff;
  padding: 24px;
  overflow: auto;
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

.status-left,
.status-right {
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