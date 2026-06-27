<template>
  <div class="file-layout">
    <!-- 左侧文件树 -->
    <aside class="file-sidebar">
      <div v-if="tree.length === 0" class="empty-hint">
        暂无文件，请点击左侧“导入”按钮选择文件夹
      </div>
      <FileTree
          v-else
          :tree="tree"
          @select="onSelectFile"
      />
    </aside>

    <!-- 右侧主区域 -->
    <main class="file-content">
      <div v-if="selectedFile" class="file-preview">
        <h4>📄 {{ selectedFile.name }}</h4>
        <p>路径：{{ selectedFile.path }}</p>
        <!-- 未来可在此展示文件内容 -->
      </div>
      <div v-else class="placeholder">
        <p>选择左侧文件以查看详情</p>
      </div>
    </main>
  </div>
</template>

<script setup>
import {ref} from 'vue';
import FileTree from '../components/FileTree.vue';

const props = defineProps({
  tree: {
    type: Array,
    default: () => []
  }
});

const selectedFile = ref(null);

const onSelectFile = (file) => {
  selectedFile.value = file;
};
</script>

<style scoped>
.file-layout {
  display: flex;
  height: 100%;
  gap: 1px;
  background-color: #e2e8f0;
}

.file-sidebar {
  width: 220px;
  background: white;
  padding: 8px;
  overflow: auto;
  flex-shrink: 0;
}

.empty-hint {
  color: #94a3b8;
  font-size: 13px;
  text-align: center;
  margin-top: 20px;
}

.file-content {
  flex: 1;
  background: white;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.placeholder {
  color: #94a3b8;
}
</style>