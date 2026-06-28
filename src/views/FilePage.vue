<template>
  <div class="file-page-wrapper">
    <div class="file-tree-container">
      <!-- 增加 :key 强制在新增文件夹时重新渲染树级 -->
      <el-tree
          :key="treeKey"
          :props="defaultProps"
          :load="loadNode"
          lazy
          node-key="path"
          empty-text="加载中..."
          highlight-current
          class="custom-tree"
      >
        <template #default="{ node, data }">
          <!-- 动态绑定 class，用于区分根节点样式 -->
          <span class="custom-tree-node" :class="{ 'is-root-node': data.isRoot }">
            <span class="icon">{{ data.isRoot ? '📦' : (data.isDir ? '📁' : '📄') }}</span>
            <span class="label" :title="data.name">{{ data.name }}</span>

            <!-- 如果当前节点是根文件夹，在后方展示浅色的全路径 -->
            <span v-if="data.isRoot" class="root-path-hint" :title="data.path">
              - {{ data.path }}
            </span>
          </span>
        </template>
      </el-tree>
    </div>
  </div>
</template>

<script setup>
import {defineProps, ref, watch} from 'vue';
import {readDir} from '@tauri-apps/plugin-fs';
import {join} from '@tauri-apps/api/path';

const props = defineProps({
  workspaceFolders: {type: Array, required: true}
});

const defaultProps = {
  label: 'name',
  children: 'children',
  isLeaf: 'isLeaf'
};

const treeKey = ref(0);

// 当外部新增文件夹时，更新 key 以强制 el-tree 重新加载顶级节点
watch(() => props.workspaceFolders, () => {
  treeKey.value++;
}, {deep: true});

const loadNode = async (node, resolve) => {
  try {
    // 1. 如果是第 0 层，直接渲染所有根文件夹（使其可折叠）
    if (node.level === 0) {
      const roots = props.workspaceFolders.map(folder => ({
        name: folder.name,
        path: folder.path,
        isDir: true,
        isLeaf: false,
        isRoot: true // 标记为根节点
      }));
      return resolve(roots);
    }

    // 2. 如果是子层级，读取该目录下的内容
    const currentPath = node.data.path;
    const entries = await readDir(currentPath);
    const children = [];

    for (const entry of entries) {
      const fullPath = await join(currentPath, entry.name);
      children.push({
        name: entry.name,
        path: fullPath,
        isDir: entry.isDirectory,
        isLeaf: !entry.isDirectory,
        isRoot: false
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
.file-page-wrapper {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
  overflow: hidden;
}

.file-tree-container {
  flex: 1;
  overflow: auto;
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

/* 默认 label 样式 */
.custom-tree-node .label {
  white-space: nowrap;
}

/* 根节点专属样式（通过动态 class 控制） */
.custom-tree-node.is-root-node .label {
  font-weight: bold;
  color: #303133;
}

/* 根路径提示文字样式 */
.root-path-hint {
  margin-left: 8px;
  font-size: 12px;
  color: #909399;
  font-weight: normal;
  white-space: nowrap;
}
</style>