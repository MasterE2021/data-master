<template>
  <div class="file-page-wrapper">
    <!-- 1. 顶部：展示根文件名称与路径 -->
    <div class="project-header">
      <div class="project-name">
        <span class="icon">📂</span> {{ rootName }}
      </div>
      <!-- 使用 title 属性，鼠标悬浮时可以查看完整路径 -->
      <div class="project-path" :title="rootPath">
        {{ rootPath }}
      </div>
    </div>

    <!-- 2. 底部：懒加载文件树 -->
    <div class="file-tree-container">
      <el-tree
          :props="defaultProps"
          :load="loadNode"
          lazy
          node-key="path"
          empty-text="加载中..."
          highlight-current
          class="custom-tree"
      >
        <template #default="{ node, data }">
          <span class="custom-tree-node">
            <span class="icon">{{ data.isDir ? '📁' : '📄' }}</span>
            <span class="label" :title="data.name">{{ data.name }}</span>
          </span>
        </template>
      </el-tree>
    </div>
  </div>
</template>

<script setup>
import {defineProps} from 'vue';
import {readDir} from '@tauri-apps/plugin-fs';
import {join} from '@tauri-apps/api/path';

const props = defineProps({
  rootPath: {type: String, required: true},
  rootName: {type: String, required: true}
});

const defaultProps = {
  label: 'name',
  children: 'children',
  isLeaf: 'isLeaf'
};

const loadNode = async (node, resolve) => {
  try {
    // 关键改变：当是第一层级 (level === 0) 时，直接读取传进来的根目录内容
    // 这样树的最外层直接就是子文件和子文件夹
    const currentPath = node.level === 0 ? props.rootPath : node.data.path;

    const entries = await readDir(currentPath);
    const children = [];

    for (const entry of entries) {
      const fullPath = await join(currentPath, entry.name);
      children.push({
        name: entry.name,
        path: fullPath,
        isDir: entry.isDirectory,
        isLeaf: !entry.isDirectory
      });
    }

    children.sort((a, b) => {
      if (a.isDir && !b.isDir) return -1;
      if (!a.isDir && b.isDir) return 1;
      return a.name.localeCompare(b.name);
    });

    resolve(children);
  } catch (error) {
    console.error('读取文件夹内容失败:', error);
    resolve([]);
  }
};
</script>

<style scoped>
/* 整个文件页面的布局：Flex 列布局 */
.file-page-wrapper {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
  overflow: hidden;
}

/* --- 顶部的项目信息栏样式 --- */
.project-header {
  padding: 12px 15px;
  background-color: #ecf5ff;
  border-bottom: 1px solid #dcdfe6;
  flex: 0 0 auto; /* 固定高度不被压缩 */
}

.project-name {
  font-size: 14px;
  font-weight: bold;
  color: #303133;
  margin-bottom: 4px;
  display: flex;
  align-items: center;
}

.project-path {
  font-size: 11px;
  color: #909399;
  /* 路径太长时自动省略号显示 */
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  cursor: help; /* 提示用户可以悬浮查看全称 */
}

/* --- 下方的树形结构样式 --- */
.file-tree-container {
  flex: 1; /* 占据剩下的所有空间 */
  overflow: auto; /* 允许横向和纵向滚动 */
}

.custom-tree {
  min-width: max-content;
  padding: 10px;
  background-color: transparent;
}

.custom-tree-node {
  display: flex;
  align-items: center;
  font-size: 13px;
}

.icon {
  margin-right: 6px;
  font-size: 14px;
}

.label {
  white-space: nowrap;
}
</style>