<template>
  <el-container class="layout-container">
    <el-container>
      <!-- 左侧工具栏 -->
      <el-aside width="40px" class="toolbar-aside">
        <div class="toolbar">
          <el-button type="primary" :icon="FolderOpened" class="tool-btn" @click="switchTo('file')"/>
          <el-button type="warning" :icon="List" class="tool-btn" @click="switchTo('todo')"/>
          <div class="toolbar-spacer"></div>
          <el-button type="success" :icon="Upload" class="tool-btn" @click="handleImport"/>
        </div>
      </el-aside>

      <!-- 主工作区：动态加载视图 -->
      <el-main class="main-content">
        <component :is="currentView"/>
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
import {FolderOpened, List, Upload} from '@element-plus/icons-vue';
import FilePage from './views/FilePage.vue';
import TodoPage from './views/TodoPage.vue';

const views = {
  file: FilePage,
  todo: TodoPage
};

const currentView = ref(views.file); // 默认显示文件页

const switchTo = (key) => {
  currentView.value = views[key];
};

const handleImport = () => {
  // 导入相关操作
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