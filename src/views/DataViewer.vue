<!-- src/views/DataViewer.vue -->
<template>
  <div class="data-viewer-container">
    <div v-if="loading" class="loading-text">数据加载中...</div>
    <div v-else-if="error" class="error-text">读取失败: {{ error }}</div>
    <div v-else class="table-wrapper">
      <div class="table-header">
        <h3>数据预览: {{ fileName }}</h3>
        <span class="row-count">预览前 100 行</span>
      </div>
      <el-table
          :data="tableData"
          border
          stripe
          height="100%"
          style="width: 100%"
      >
        <el-table-column
            v-for="col in columns"
            :key="col"
            :prop="col"
            :label="col"
            min-width="120"
            show-overflow-tooltip
        />
      </el-table>
    </div>
  </div>
</template>

<script setup>
import {ref, watch, computed} from 'vue';

const props = defineProps({
  filePath: {type: String, required: true}
});

const columns = ref([]);
const tableData = ref([]);
const loading = ref(false);
const error = ref('');

const fileName = computed(() => {
  return props.filePath.split(/[/\\]/).pop();
});

// 监听文件路径变化，自动请求新数据
watch(() => props.filePath, async (newPath) => {
  if (!newPath) return;

  loading.value = true;
  error.value = '';

  try {
    const res = await fetch('http://127.0.0.1:8000/data/preview', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify({path: newPath})
    });
    const result = await res.json();

    if (result.error) {
      error.value = result.error;
    } else {
      columns.value = result.columns;
      tableData.value = result.data;
    }
  } catch (err) {
    error.value = '网络请求异常: ' + err.message;
  } finally {
    loading.value = false;
  }
}, {immediate: true});
</script>

<style scoped>
.data-viewer-container {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.loading-text, .error-text {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  color: #909399;
}

.error-text {
  color: #f56c6c;
}

.table-wrapper {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.table-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
  flex: 0 0 auto;
}

.table-header h3 {
  margin: 0;
  font-size: 16px;
  color: #303133;
}

.row-count {
  font-size: 12px;
  color: #909399;
}
</style>