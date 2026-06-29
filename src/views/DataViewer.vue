<!-- src/views/DataViewer.vue -->
<template>
  <div class="data-viewer-container">
    <div v-if="loading && !tableData.length" class="loading-text">数据加载中...</div>
    <div v-else-if="error" class="error-text">读取失败: {{ error }}</div>
    <div v-else class="table-wrapper">

      <div class="table-header">
        <h3>数据预览: {{ fileName }}</h3>
      </div>

      <!-- vxe-table 虚拟滚动表格区域 -->
      <div class="table-body">
        <vxe-table
            border
            stripe
            show-overflow
            height="auto"
            auto-resize
            :data="tableData"
            :loading="loading"
            :scroll-y="{ enabled: true, gt: 100 }"
        >
          <!-- 动态渲染列 -->
          <vxe-column
              v-for="col in columns"
              :key="col"
              :field="col"
              :title="col"
              min-width="120"
          />
        </vxe-table>
      </div>

      <!-- 自定义精简分页区域 -->
      <div class="pagination-wrapper">
        <el-button-group class="custom-pager">
          <el-button :icon="DArrowLeft" :disabled="currentPage === 1" @click="goToPage(1)" title="首页"/>
          <el-button :icon="ArrowLeft" :disabled="currentPage === 1" @click="goToPage(currentPage - 1)" title="上一页"/>

          <div class="page-display">
            <!-- 只有一个输入框，宽度动态绑定：最大页数的字符长度 * 1ch -->
            <input
                v-model="inputPage"
                class="page-input"
                type="text"
                :style="{ width: Math.max(1.5, String(maxPage).length) + 'ch' }"
                @keyup.enter="handleJump"
                @blur="handleJump"
                title="输入页码后回车跳转"
            />
          </div>

          <el-button :icon="ArrowRight" :disabled="currentPage === maxPage || maxPage === 0"
                     @click="goToPage(currentPage + 1)" title="下一页"/>
          <el-button :icon="DArrowRight" :disabled="currentPage === maxPage || maxPage === 0" @click="goToPage(maxPage)"
                     title="尾页"/>
        </el-button-group>
      </div>

    </div>
  </div>
</template>

<script setup>
import {ref, watch, computed} from 'vue';
import {ArrowLeft, ArrowRight, DArrowLeft, DArrowRight} from '@element-plus/icons-vue';

const emit = defineEmits(['update-stats']);

const props = defineProps({
  filePath: {type: String, required: true}
});

const columns = ref([]);
const tableData = ref([]);
const loading = ref(false);
const error = ref('');

// 分页状态
const currentPage = ref(1);
const inputPage = ref(1);
const pageSize = ref(1000);
const total = ref(0);

const fileName = computed(() => {
  return props.filePath.split(/[/\\]/).pop();
});

// 计算最大页数
const maxPage = computed(() => {
  return Math.ceil(total.value / pageSize.value) || 1;
});

// 点击按钮跳转
const goToPage = (page) => {
  if (page >= 1 && page <= maxPage.value && page !== currentPage.value) {
    currentPage.value = page;
    inputPage.value = page;
    fetchData();
  }
};

// 输入框回车/失焦跳转
const handleJump = () => {
  let target = parseInt(inputPage.value, 10);
  if (isNaN(target)) {
    inputPage.value = currentPage.value;
    return;
  }

  if (target < 1) target = 1;
  if (target > maxPage.value) target = maxPage.value;

  inputPage.value = target;
  if (target !== currentPage.value) {
    currentPage.value = target;
    fetchData();
  }
};

// 获取数据
const fetchData = async () => {
  loading.value = true;
  error.value = '';

  try {
    const res = await fetch('http://127.0.0.1:8000/data/preview', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify({
        path: props.filePath,
        page: currentPage.value,
        page_size: pageSize.value,
        total: total.value
      })
    });
    const result = await res.json();

    emit('update-stats', {
      loaded: tableData.value.length, // 当前实际渲染的行数
      total: total.value,             // 总行数
      time: result.cost_time || 0   // 后端返回的查询耗时
    });

    if (result.error) {
      error.value = result.error;
    } else {
      columns.value = result.columns;
      tableData.value = result.data;
      total.value = result.total;
    }
  } catch (err) {
    error.value = '网络请求异常: ' + err.message;
  } finally {
    loading.value = false;
  }
};

// 监听文件路径变化
watch(() => props.filePath, (newPath) => {
  if (!newPath) return;
  currentPage.value = 1;
  inputPage.value = 1;
  total.value = 0;
  tableData.value = [];
  fetchData();
  emit('update-stats', null);
  fetchData();
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

.total-text {
  font-size: 13px;
  color: #606266;
}

.table-body {
  flex: 1;
  overflow: hidden;
}

.pagination-wrapper {
  flex: 0 0 auto;
  padding-top: 12px;
  display: flex;
  justify-content: flex-end;
  align-items: center;
}

.custom-pager {
  display: flex;
  align-items: center;
}

/* ======== 单一输入框容器 ======== */
.page-display {
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 32px; /* 与 Element Plus 默认图标按钮宽度完全一致 */
  padding: 0 8px; /* 两侧留出呼吸空间 */
  height: 32px;
  box-sizing: border-box;
  background-color: #ffffff;
  border: 1px solid #dcdfe6;
  margin-left: -1px;
  margin-right: -1px;
  z-index: 1;
  transition: border-color 0.2s;
}

.page-display:hover, .page-display:focus-within {
  border-color: #c6e2ff;
  z-index: 2;
}

/* 输入框本身 */
.page-input {
  /* 只有输入框，居中对齐最美观 */
  text-align: center;
  font-weight: 500;
  color: #409eff;
  border: none;
  background-color: transparent;
  outline: none;
  font-size: 14px;
}
</style>