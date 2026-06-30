<!-- src/views/DataViewer.vue -->
<template>
  <div class="data-viewer-container">
    <div v-if="loading && !tableData.length" class="loading-text">数据加载中...</div>
    <div v-else-if="error" class="error-text">读取失败: {{ error }}</div>
    <div v-else class="table-wrapper">

      <!-- ================= 新增：顶部操作栏 ================= -->
      <div class="toolbar">
        <div class="toolbar-left">

          <!-- 选择列 下拉多选 -->
          <el-popover placement="bottom-start" width="220" trigger="click">
            <template #reference>
              <el-button title="选择需要展示的列">选择列</el-button>
            </template>
            <div class="column-selector">
              <el-checkbox
                  v-model="checkAll"
                  :indeterminate="isIndeterminate"
                  @change="handleCheckAllChange"
              >全选
              </el-checkbox>
              <el-divider style="margin: 8px 0;"/>
              <el-checkbox-group v-model="selectedColumns" @change="handleColumnChange">
                <div v-for="col in allColumns" :key="col" class="col-item">
                  <el-checkbox :label="col" :value="col">{{ col }}</el-checkbox>
                </div>
              </el-checkbox-group>
            </div>
          </el-popover>

          <!-- 分页器（从底部移到这里，紧挨着选择列按钮） -->
          <el-button-group class="custom-pager" style="margin-left: 12px;">
            <el-button :icon="DArrowLeft" :disabled="currentPage === 1" @click="goToPage(1)" title="首页"/>
            <el-button :icon="ArrowLeft" :disabled="currentPage === 1" @click="goToPage(currentPage - 1)"
                       title="上一页"/>

            <!-- 自适应宽度的页码输入框 -->
            <div class="page-display">
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
            <el-button :icon="DArrowRight" :disabled="currentPage === maxPage || maxPage === 0"
                       @click="goToPage(maxPage)"
                       title="尾页"/>
          </el-button-group>

        </div>
      </div>
      <!-- ================= 顶部操作栏结束 ================= -->

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
            :column-config="{ resizable: true }"
        >
          <!-- 动态渲染勾选的列 -->
          <vxe-column
              v-for="col in selectedColumns"
              :key="col"
              :field="col"
              :title="col"
              min-width="120"
          />
        </vxe-table>
      </div>

    </div>
  </div>
</template>

<script setup>
import {ref, watch, computed, nextTick} from 'vue';
import {ArrowLeft, ArrowRight, DArrowLeft, DArrowRight} from '@element-plus/icons-vue';

const emit = defineEmits(['update-stats']);

const props = defineProps({
  filePath: {type: String, required: true}
});

// 列选择相关状态
const allColumns = ref([]);
const selectedColumns = ref([]);
const checkAll = ref(true);
const isIndeterminate = ref(false);

const tableData = ref([]);
const loading = ref(false);
const error = ref('');

// 分页状态
const currentPage = ref(1);
const inputPage = ref(1);
const pageSize = ref(1000);
const total = ref(0);

const maxPage = computed(() => Math.ceil(total.value / pageSize.value) || 1);

// 处理列全选逻辑
const handleCheckAllChange = (val) => {
  selectedColumns.value = val ? [...allColumns.value] : [];
  isIndeterminate.value = false;
  fetchData();
};

// 处理单列勾选逻辑
const handleColumnChange = (value) => {
  const checkedCount = value.length;
  checkAll.value = checkedCount === allColumns.value.length;
  isIndeterminate.value = checkedCount > 0 && checkedCount < allColumns.value.length;
  fetchData();
};

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
        total: total.value,
        columns: selectedColumns.value
      })
    });
    const result = await res.json();

    if (result.error) {
      error.value = result.error;
    } else {
      // 首次加载初始化所有列
      if (allColumns.value.length === 0) {
        allColumns.value = result.columns;
        selectedColumns.value = result.columns;
      }

      tableData.value = result.data;
      total.value = result.total;

      // 让外层更新状态栏，注意需要用 setTimeout/nextTick 等待 DOM 渲染后统计展示行
      nextTick(() => {
        emit('update-stats', {
          loaded: tableData.value.length,
          total: total.value,
          time: result.cost_time || 0
        });
      });
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
  allColumns.value = [];
  selectedColumns.value = [];
  checkAll.value = true;
  isIndeterminate.value = false;

  // 发送 null 给父组件，让右下角立刻变回“状态: 就绪”
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

/* ======== 新增 Toolbar ======== */
.toolbar {
  flex: 0 0 auto;
  padding-bottom: 12px;
  display: flex;
  justify-content: flex-start;
  align-items: center;
}

.toolbar-left {
  display: flex;
  align-items: center;
}

.table-body {
  flex: 1;
  overflow: hidden;
}

/* ======== 下拉列选择框样式 ======== */
.column-selector {
  max-height: 300px;
  overflow-y: auto;
}

.col-item {
  margin-bottom: 4px;
}

/* ======== 分页样式 ======== */
.custom-pager {
  display: flex;
  align-items: center;
}

.page-display {
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 32px;
  padding: 0 8px;
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

.page-input {
  text-align: center;
  font-weight: 500;
  color: #409eff;
  border: none;
  background-color: transparent;
  outline: none;
  font-size: 14px;
}
</style>