<!-- src/views/VirtualScrollerTable.vue -->

<!-- Cadule Fable 5 提示词

# 需求: 开发一个vue表格组件VirtualScrollerTable.vue

## 前提

1. 暂时只实现前端相关逻辑, 后端使用假数据或者使用静态数据代替
2. 代码注释可以尽量详细一点
3. 目标是实现一个高性能虚拟滚动电子表格
4. 需要考虑超大行与列数据的加载与滚动

## 核心功能点

- 数据加载
    1. 高性能虚拟滚动。首次加载表格数据时, 从后端一次性加载100行数据, 这100行并不会同时渲染, 只会渲染当前画面能看得到的数据行与列
    2. 当用户通过滚轮滚动(或者直接拖拽滑轮)至底部时, 再次加载100行数据(隐式翻页), 如果继续向下滚动, 同理

- 数据呈现
    1. 生成行号。表格首页固定为数据行号, 行号为前端生成与后端无关, 主要用于给用户锚定数据行。
    2. 列宽初始化。首次加载数据时, 后端需要返回列宽。当列比较少时, 不要拉伸, 让表的右侧留白。
    3. 列宽可调整。鼠标悬浮至列的左右边缘时出现拖拽指引, 拖拽可调整列宽。
    4. 列宽调整后需要持久化。用户在调整列宽后, 下一次打开表格, 能直接呈现上一次调整的列宽。(需要后端记录相关信息)
    5. 列顺序可调整。可通过拖拽表头, 调整列顺序。用户在调整列顺序后, 下一次打开表格, 能直接呈现上一次调整的列顺序。(
       需要后端记录相关信息)
    6. 展示选择的列。可通过勾选, 选择展示的列。用户在选择展示的列后, 下一次打开表格, 能直接呈现上一次调整的列。(
       需要后端记录相关信息)
    7. 重置记录按钮。可通过点击此按钮, 重置列宽、列顺序、选择的列的信息, 将其恢复初始状态。(会将同步相关信息同步给后端)
    8. 长文本呈现。当文本过长cell无法展现完全的时候, 通过在cell末尾展示省略号, 如果用户调整列宽,
       会呈现更多内容而不是省略号。鼠标悬浮至cell超过1秒时, 通过浮窗展现cell全部内容(当然, 如果浮窗依旧无法展示完全,
       则使用省略号)
    9. 数据行背景色交替。数据行有3种背景色：淡色A, 淡色B, 无色。每隔几行换一次色，例如：1,2,4,5,7,8,10,11为无色; 3,9为淡色A,
       6,12为淡色B,依次类推。
    10. cell内容复制。在非编辑模式下, 双击cell会直接复制cell中的内容, 并气泡提醒"复制成功"。


- 数据编辑。
    1. 表格cell编辑。表格的cell可编辑(首列不可编辑)。通过点击编辑按钮进入编辑模式,
       进入编辑模式后单击cell可编辑cell内容。应用编辑才将修改后的数据提交至后端, 如果直接退出, 将不保存。
    2. 多个cell同时修改。在编辑模式下, 通过按住鼠标左键拖拽(只可同时修改同画面的数据, 即拖拽时不可滚动页面),
       会同时选中拖拽矩形范围内的cell, 可以同时对其进行修改。

## 代码模板

```vue

<script setup>

</script>

<template>

</template>

<style scoped>

</style>
```

-->


<script setup>
/**
 * ============================================================================
 * VirtualScrollerTable.vue —— 高性能虚拟滚动电子表格
 * ----------------------------------------------------------------------------
 * 功能清单:
 *  1. 行 + 列 双向虚拟滚动, 只渲染可视区域内的单元格
 *  2. 滚动触底隐式翻页, 每次向后端追加加载 100 行
 *  3. 首列为前端生成的行号(固定不随横向滚动)
 *  4. 列宽由后端下发; 列少时右侧留白不拉伸
 *  5. 拖拽表头左右边缘调整列宽, 调整结果持久化
 *  6. 拖拽表头调整列顺序(鼠标事件实现, 兼容 Tauri, 不用 HTML5 drag), 持久化
 *  7. 列显隐勾选面板, 持久化
 *  8. 一键重置列宽/列序/列显隐
 *  9. 长文本省略号 + 悬浮 1 秒气泡展示全文
 * 10. 数据行背景色: 每 3 行出现一次淡色, 淡色A/淡色B 交替 (3→A, 6→B, 9→A...)
 * 11. 非编辑模式下双击 cell 复制内容并提示"复制成功"
 * 12. 编辑模式: 单击 cell 编辑; 鼠标拖拽框选矩形区域批量修改;
 *     点击"应用"才提交后端, 直接退出则丢弃
 * ============================================================================
 */
import {ref, reactive, computed, onMounted, onBeforeUnmount, nextTick} from 'vue'

/* ============================================================================
 * 一、布局常量 (与 <style> 中的尺寸保持一致)
 * ========================================================================== */
const ROW_HEIGHT = 36          // 每行高度(px), 虚拟滚动定位的基础
const ROWNUM_WIDTH = 60        // 行号列宽度
const ROW_BUFFER = 5           // 行方向上下各多渲染几行, 减少快速滚动白屏
const COL_BUFFER = 2           // 列方向左右各多渲染几列
const PAGE_SIZE = 100          // 每次向后端请求的行数
const MIN_COL_WIDTH = 50       // 列宽拖拽的最小宽度
const LOAD_THRESHOLD = 300     // 距底部多少 px 时触发加载下一页
const CONFIG_KEY = 'vst_table_user_config' // 模拟后端持久化用的 localStorage key

/* ============================================================================
 * 二、模拟后端 (Mock Backend)
 * ----------------------------------------------------------------------------
 * 真实项目中把这些函数替换为:
 *   - Tauri:  import { invoke } from '@tauri-apps/api/core'; await invoke('xxx')
 *   - 或 http 请求转发给 Python 服务
 * 持久化配置暂存 localStorage, 保证"下次打开还原上次调整"的演示效果。
 * ========================================================================== */
const MOCK_TOTAL_ROWS = 10000  // 模拟后端总行数(演示超大数据量)
const MOCK_COL_COUNT = 30      // 模拟后端列数(演示列虚拟化)
const LONG_TEXT =
    '这是一段非常长的示例文本, 用来演示单元格内容溢出时展示省略号, ' +
    '以及鼠标悬浮 1 秒后通过浮窗展示完整内容的效果。' +
    '如果拖宽此列, 将看到更多内容而不是省略号。'

/** 模拟网络延迟 */
const delay = (ms) => new Promise((r) => setTimeout(r, ms))

/** 后端下发的默认列定义(含初始列宽) */
const DEFAULT_COLUMNS = Array.from({length: MOCK_COL_COUNT}, (_, i) => ({
  key: `col_${i + 1}`,
  title: `列${i + 1}`,
  width: 90 + ((i * 37) % 90), // 90 ~ 180 之间的确定性伪随机宽度
}))

const mockBackend = {
  /** 获取列定义(含初始列宽) */
  async fetchColumns() {
    await delay(120)
    return JSON.parse(JSON.stringify(DEFAULT_COLUMNS))
  },

  /** 分页获取行数据: offset 起始行, limit 行数 */
  async fetchRows(offset, limit) {
    await delay(200)
    const end = Math.min(offset + limit, MOCK_TOTAL_ROWS)
    const rows = []
    for (let r = offset; r < end; r++) {
      const row = {}
      for (let c = 0; c < MOCK_COL_COUNT; c++) {
        const key = `col_${c + 1}`
        // 每隔一些单元格塞入长文本, 用于演示省略号/浮窗
        row[key] =
            (r * 31 + c * 17) % 13 === 0
                ? `R${r + 1}C${c + 1} ${LONG_TEXT}`
                : `R${r + 1}C${c + 1}`
      }
      rows.push(row)
    }
    return {rows, hasMore: end < MOCK_TOTAL_ROWS}
  },

  /** 读取用户列配置 { order:[], widths:{}, hidden:[] } */
  async loadUserConfig() {
    await delay(50)
    try {
      return JSON.parse(localStorage.getItem(CONFIG_KEY)) || null
    } catch {
      return null
    }
  },

  /** 保存用户列配置 */
  async saveUserConfig(cfg) {
    await delay(50)
    localStorage.setItem(CONFIG_KEY, JSON.stringify(cfg))
  },

  /** 重置(删除)用户列配置 */
  async resetUserConfig() {
    await delay(50)
    localStorage.removeItem(CONFIG_KEY)
  },

  /** 提交单元格修改 [{rowIndex, key, value}] */
  async saveCellEdits(edits) {
    await delay(200)
    console.log('[mockBackend] 收到单元格修改:', edits)
    return true
  },
}

/* ============================================================================
 * 三、核心响应式状态
 * ========================================================================== */
// ---- 数据 ----
const allColumns = ref([])           // 后端下发的原始列定义
const rows = ref([])                 // 已加载的行数据(累加)
const hasMore = ref(true)            // 后端是否还有更多数据
const loadingRows = ref(false)       // 行加载中标记, 防止重复请求

// ---- 列配置(可持久化部分) ----
const columnOrder = ref([])          // 列顺序: key 数组
const columnWidths = reactive({})    // 列宽: { key: width }
const hiddenList = ref([])           // 被隐藏的列 key 数组

// ---- 滚动/视口 ----
const bodyRef = ref(null)            // 数据区滚动容器 DOM
const scrollTop = ref(0)
const scrollLeft = ref(0)
const viewportW = ref(800)           // 数据区可视宽度(由 ResizeObserver 更新)
const viewportH = ref(560)           // 数据区可视高度

// ---- 编辑相关 ----
const editMode = ref(false)          // 是否处于编辑模式
const columnSelect = ref(true)       // 是否展开列选择窗口
const pendingEdits = ref({})         // 未提交的修改: { "rowIndex:colKey": newValue }
const editingCell = ref(null)        // 正在编辑的单元格 { row, key }
const editingValue = ref('')         // 编辑框绑定值
let editInputEl = null               // 编辑输入框 DOM(函数 ref 收集, 用于聚焦)

// ---- 框选(批量编辑) ----
const selection = reactive({
  active: false,                     // 是否存在选区
  dragging: false,                   // 是否正在拖拽框选(此时禁止滚轮滚动)
  anchor: null,                      // 起点 { row, col } (col 为可见列序号)
  current: null,                     // 终点 { row, col }
})
const batchEditor = reactive({show: false, x: 0, y: 0, value: ''})
let batchInputEl = null

// ---- 列宽拖拽 ----
const resizing = reactive({key: null, startX: 0, startWidth: 0})

// ---- 表头拖拽排序 ----
const headerDrag = reactive({
  pendingKey: null,                  // 按下但尚未确认拖拽的列
  dragging: false,
  title: '',                         // 幽灵浮层显示的列名
  ghostX: 0, ghostY: 0,              // 幽灵浮层位置
  dropIndex: -1,                     // 落点列(可见列序号)
  dropAfter: false,                  // 插入到落点列之后还是之前
})
let headerStartX = 0, headerStartY = 0

// ---- 浮窗 / 气泡 ----
const tooltip = reactive({show: false, text: '', x: 0, y: 0})
let tooltipTimer = null
const toast = reactive({show: false, text: ''})
let toastTimer = null

// ---- 其它 UI ----
const showColumnPanel = ref(false)   // 列显隐设置面板

/* ============================================================================
 * 四、派生计算属性 (虚拟滚动的核心)
 * ========================================================================== */
/** key -> 原始列定义 的映射, 便于查询标题/默认宽 */
const columnMap = computed(() => {
  const m = {}
  for (const c of allColumns.value) m[c.key] = c
  return m
})

/**
 * 当前实际展示的列(应用了顺序/显隐/宽度), 并预计算每列的横向偏移 left。
 * left 是列虚拟化和单元格绝对定位的基础。
 */
const displayColumns = computed(() => {
  const hidden = new Set(hiddenList.value)
  const res = []
  let left = 0
  for (const key of columnOrder.value) {
    if (hidden.has(key)) continue
    const def = columnMap.value[key]
    if (!def) continue
    const width = columnWidths[key] ?? def.width
    res.push({key, title: def.title, width, left, orderIndex: res.length})
    left += width
  }
  return res
})

/** 内容总宽度: 列少时小于视口宽 → 右侧自然留白, 不做拉伸 */
const totalWidth = computed(() =>
    displayColumns.value.reduce((s, c) => s + c.width, 0)
)

/** 内容总高度 = 已加载行数 × 行高 (滚动条长度随加载增长, 即"隐式翻页") */
const totalHeight = computed(() => rows.value.length * ROW_HEIGHT)

/** 可视行范围 [rowStart, rowEnd), 前后各加 ROW_BUFFER 行缓冲 */
const rowStart = computed(() =>
    Math.max(0, Math.floor(scrollTop.value / ROW_HEIGHT) - ROW_BUFFER)
)
const rowEnd = computed(() =>
    Math.min(
        rows.value.length,
        Math.ceil((scrollTop.value + viewportH.value) / ROW_HEIGHT) + ROW_BUFFER
    )
)
/** 需要渲染的行下标数组 */
const visibleRowIndexes = computed(() => {
  const arr = []
  for (let i = rowStart.value; i < rowEnd.value; i++) arr.push(i)
  return arr
})

/** 二分查找: 横坐标 x 落在哪一列上 (列宽不等, 不能用除法) */
function findColIndexByX(x) {
  const cols = displayColumns.value
  if (!cols.length) return 0
  let lo = 0, hi = cols.length - 1, ans = cols.length - 1
  while (lo <= hi) {
    const mid = (lo + hi) >> 1
    if (cols[mid].left + cols[mid].width > x) {
      ans = mid;
      hi = mid - 1
    } else lo = mid + 1
  }
  return ans
}

/** 可视列范围 [start, end), 左右各加 COL_BUFFER 列缓冲 */
const colRange = computed(() => {
  const cols = displayColumns.value
  if (!cols.length) return [0, 0]
  let start = findColIndexByX(scrollLeft.value)
  let end = start
  const right = scrollLeft.value + viewportW.value
  while (end < cols.length && cols[end].left < right) end++
  return [Math.max(0, start - COL_BUFFER), Math.min(cols.length, end + COL_BUFFER)]
})
/** 需要渲染的列(带 left/width, 模板直接用) */
const visibleCols = computed(() =>
    displayColumns.value.slice(colRange.value[0], colRange.value[1])
)

/** 归一化后的选区矩形 { r1,r2,c1,c2 } */
const selRect = computed(() => {
  if (!selection.active || !selection.anchor || !selection.current) return null
  const a = selection.anchor, b = selection.current
  return {
    r1: Math.min(a.row, b.row), r2: Math.max(a.row, b.row),
    c1: Math.min(a.col, b.col), c2: Math.max(a.col, b.col),
  }
})
/** 选中的单元格数量(批量编辑框上显示) */
const selectedCount = computed(() => {
  const s = selRect.value
  return s ? (s.r2 - s.r1 + 1) * (s.c2 - s.c1 + 1) : 0
})

/** 未提交修改数 */
const editedCount = computed(() => Object.keys(pendingEdits.value).length)

/* ============================================================================
 * 五、通用小工具
 * ========================================================================== */
/** 气泡提示 */
function showToast(text) {
  toast.text = text
  toast.show = true
  clearTimeout(toastTimer)
  toastTimer = setTimeout(() => (toast.show = false), 1500)
}

/**
 * 行背景色规则(rowNo 为 1 开始的行号):
 *   不是 3 的倍数 → 无色;  3,9,15... (mod 6 == 3) → 淡色A;  6,12,18... → 淡色B
 */
function rowColorClass(rowNo) {
  if (rowNo % 3 !== 0) return ''
  return rowNo % 6 === 3 ? 'row-a' : 'row-b'
}

/** 读取单元格展示值: 优先取未提交的修改, 否则取原始数据 */
function getCellValue(rowIndex, key) {
  const k = `${rowIndex}:${key}`
  if (k in pendingEdits.value) return pendingEdits.value[k]
  const row = rows.value[rowIndex]
  return row ? String(row[key] ?? '') : ''
}

/** 该单元格是否有未提交的修改(用于高亮标记) */
function isEdited(rowIndex, key) {
  return `${rowIndex}:${key}` in pendingEdits.value
}

/** 该单元格是否在框选选区内 */
function isSelected(rowIndex, colOrderIndex) {
  const s = selRect.value
  return !!s && rowIndex >= s.r1 && rowIndex <= s.r2 &&
      colOrderIndex >= s.c1 && colOrderIndex <= s.c2
}

/** 该单元格是否正处于编辑输入状态 */
function isEditingCell(rowIndex, key) {
  return editingCell.value &&
      editingCell.value.row === rowIndex && editingCell.value.key === key
}

/** 把当前列配置持久化到后端 */
function persistConfig() {
  mockBackend.saveUserConfig({
    order: columnOrder.value,
    widths: {...columnWidths},
    hidden: hiddenList.value,
  })
}

/* ============================================================================
 * 六、数据加载 (首屏 + 触底隐式翻页)
 * ========================================================================== */
async function loadMore() {
  if (loadingRows.value || !hasMore.value) return
  loadingRows.value = true
  try {
    const {rows: newRows, hasMore: more} =
        await mockBackend.fetchRows(rows.value.length, PAGE_SIZE)
    rows.value.push(...newRows) // 追加而不是替换 → 滚动条自然变长
    hasMore.value = more
  } finally {
    loadingRows.value = false
  }
}

/** 滚动事件: 更新虚拟滚动偏移 + 检测触底 */
function onScroll(e) {
  const el = e.target
  scrollTop.value = el.scrollTop
  scrollLeft.value = el.scrollLeft
  // 距底部小于阈值时加载下一页(拖拽滑轮到底同样触发)
  if (el.scrollTop + el.clientHeight >= el.scrollHeight - LOAD_THRESHOLD) {
    loadMore()
  }
}

/** 框选/拖表头/调列宽期间禁止滚轮滚动页面 */
function onWheel(e) {
  if (selection.dragging || headerDrag.dragging || resizing.key) e.preventDefault()
}

/* ============================================================================
 * 七、列宽拖拽调整
 * ========================================================================== */
function onResizeStart(key, e) {
  resizing.key = key
  resizing.startX = e.clientX
  resizing.startWidth = columnWidths[key] ?? columnMap.value[key]?.width ?? 100
  document.addEventListener('mousemove', onResizeMove)
  document.addEventListener('mouseup', onResizeEnd)
  document.body.style.cursor = 'col-resize'
  document.body.style.userSelect = 'none'
}

function onResizeMove(e) {
  if (!resizing.key) return
  columnWidths[resizing.key] = Math.max(
      MIN_COL_WIDTH,
      resizing.startWidth + (e.clientX - resizing.startX)
  )
}

function onResizeEnd() {
  document.removeEventListener('mousemove', onResizeMove)
  document.removeEventListener('mouseup', onResizeEnd)
  document.body.style.cursor = ''
  document.body.style.userSelect = ''
  if (resizing.key) persistConfig() // 拖完立即持久化
  resizing.key = null
}

/* ============================================================================
 * 八、表头拖拽排序
 * ----------------------------------------------------------------------------
 * 注意: 使用 mousedown/mousemove/mouseup 实现, 而不是 HTML5 drag API,
 *       从而规避 Tauri dragDropEnabled 拦截拖拽事件的问题。
 * ========================================================================== */
function onHeaderMouseDown(col, e) {
  if (e.button !== 0) return           // 只响应左键
  headerDrag.pendingKey = col.key
  headerDrag.title = col.title
  headerStartX = e.clientX
  headerStartY = e.clientY
  document.addEventListener('mousemove', onHeaderDragMove)
  document.addEventListener('mouseup', onHeaderDragUp)
}

function onHeaderDragMove(e) {
  // 移动超过 5px 才认定为拖拽, 避免普通点击误触
  if (!headerDrag.dragging) {
    if (Math.abs(e.clientX - headerStartX) < 5 &&
        Math.abs(e.clientY - headerStartY) < 5) return
    headerDrag.dragging = true
    document.body.style.userSelect = 'none'
  }
  headerDrag.ghostX = e.clientX + 12
  headerDrag.ghostY = e.clientY + 12
  // 把鼠标横坐标换算成"内容坐标系"里的 x, 再找到落点列
  const rect = bodyRef.value.getBoundingClientRect()
  const x = e.clientX - rect.left + scrollLeft.value
  const idx = findColIndexByX(Math.max(0, Math.min(x, totalWidth.value - 1)))
  const target = displayColumns.value[idx]
  headerDrag.dropIndex = idx
  // 越过目标列中线 → 插到它后面, 否则插到前面
  headerDrag.dropAfter = target ? x > target.left + target.width / 2 : false
}

function onHeaderDragUp() {
  document.removeEventListener('mousemove', onHeaderDragMove)
  document.removeEventListener('mouseup', onHeaderDragUp)
  document.body.style.userSelect = ''
  if (headerDrag.dragging && headerDrag.dropIndex >= 0) {
    const fromKey = headerDrag.pendingKey
    const target = displayColumns.value[headerDrag.dropIndex]
    if (target && target.key !== fromKey) {
      // 在底层 columnOrder(包含隐藏列)上重排, 保持隐藏列相对位置不变
      const order = [...columnOrder.value]
      order.splice(order.indexOf(fromKey), 1)
      const insertAt = order.indexOf(target.key) + (headerDrag.dropAfter ? 1 : 0)
      order.splice(insertAt, 0, fromKey)
      columnOrder.value = order
      persistConfig()
    }
  }
  headerDrag.pendingKey = null
  headerDrag.dragging = false
  headerDrag.dropIndex = -1
}

/* ============================================================================
 * 九、列显隐 + 重置
 * ========================================================================== */
function toggleColumn(key) {
  const list = [...hiddenList.value]
  const i = list.indexOf(key)
  if (i >= 0) list.splice(i, 1)
  else {
    if (displayColumns.value.length <= 1) {         // 至少保留一列
      showToast('至少需要保留一列')
      return
    }
    list.push(key)
  }
  hiddenList.value = list
  persistConfig()
}

/** 重置列宽/列序/显隐为后端初始状态, 并同步后端 */
async function resetConfig() {
  await mockBackend.resetUserConfig()
  columnOrder.value = allColumns.value.map((c) => c.key)
  for (const c of allColumns.value) columnWidths[c.key] = c.width
  hiddenList.value = []
  showToast('列配置已重置')
}

/* ============================================================================
 * 十、长文本浮窗 (悬浮 1 秒且文本确实溢出才展示)
 * ========================================================================== */
function onCellEnter(e, rowIndex, col) {
  const cellEl = e.currentTarget
  clearTimeout(tooltipTimer)
  tooltipTimer = setTimeout(() => {
    const textEl = cellEl.querySelector('.cell-text')
    // 只有内容真的被截断(scrollWidth > clientWidth)才弹浮窗
    if (textEl && textEl.scrollWidth > textEl.clientWidth) {
      const rect = cellEl.getBoundingClientRect()
      tooltip.text = getCellValue(rowIndex, col.key)
      tooltip.x = Math.max(8, Math.min(rect.left, window.innerWidth - 428))
      // 下方放不下就放上方
      tooltip.y = rect.bottom + 260 < window.innerHeight
          ? rect.bottom + 6
          : Math.max(8, rect.top - 260)
      tooltip.show = true
    }
  }, 1000)
}

function onCellLeave() {
  clearTimeout(tooltipTimer)
  tooltip.show = false
}

/* ============================================================================
 * 十一、双击复制 (仅非编辑模式)
 * ========================================================================== */
async function onCellDblClick(rowIndex, col) {
  if (editMode.value) return
  const text = getCellValue(rowIndex, col.key)
  try {
    await navigator.clipboard.writeText(text)
  } catch {
    // Clipboard API 不可用时的降级方案
    const ta = document.createElement('textarea')
    ta.value = text
    ta.style.cssText = 'position:fixed;opacity:0'
    document.body.appendChild(ta)
    ta.select()
    document.execCommand('copy')
    ta.remove()
  }
  showToast('复制成功')
}

/* ============================================================================
 * 十二、编辑模式: 单格编辑 / 框选批量编辑 / 应用 / 退出
 * ----------------------------------------------------------------------------
 * 行号列本身不参与编辑(即"首列不可编辑")。
 * 所有修改先落入 pendingEdits, 点击"应用"才提交后端并合并进本地数据;
 * 直接"退出编辑"则整体丢弃。
 * ========================================================================== */
function enterEditMode() {
  editMode.value = true
}

function exitEditMode() {
  const n = editedCount.value
  editMode.value = false
  editingCell.value = null
  selection.active = false
  batchEditor.show = false
  pendingEdits.value = {}                    // 未应用的修改全部丢弃
  if (n) showToast(`已放弃 ${n} 处未保存的修改`)
}

async function applyEdits() {
  if (editedCount.value === 0) {
    editMode.value = false;
    return
  }
  // 组装提交载荷
  const payload = Object.entries(pendingEdits.value).map(([k, value]) => {
    const idx = k.indexOf(':')
    return {rowIndex: Number(k.slice(0, idx)), key: k.slice(idx + 1), value}
  })
  await mockBackend.saveCellEdits(payload)
  // 提交成功后合并进本地行数据
  for (const it of payload) {
    if (rows.value[it.rowIndex]) rows.value[it.rowIndex][it.key] = it.value
  }
  pendingEdits.value = {}
  editMode.value = false
  editingCell.value = null
  selection.active = false
  batchEditor.show = false
  showToast(`已保存 ${payload.length} 处修改`)
}

// ---- 单格编辑 ----
function openEditor(rowIndex, key) {
  editingCell.value = {row: rowIndex, key}
  editingValue.value = getCellValue(rowIndex, key)
  nextTick(() => editInputEl && editInputEl.focus())
}

function setEditInputRef(el) {
  editInputEl = el
} // 函数 ref(v-for 内更可靠)

function commitEdit() {
  if (!editingCell.value) return
  const {row, key} = editingCell.value
  const original = String(rows.value[row]?.[key] ?? '')
  const k = `${row}:${key}`
  if (editingValue.value !== original) pendingEdits.value[k] = editingValue.value
  else delete pendingEdits.value[k]          // 改回原值 → 视为未修改
  editingCell.value = null
}

function cancelEdit() {
  editingCell.value = null
}

// ---- 框选批量编辑 ----
let cellDrag = null // { rowIndex, orderIndex, startX, startY, moved }

function onCellMouseDown(e, rowIndex, col) {
  if (!editMode.value || e.button !== 0) return
  if (e.target.tagName === 'INPUT') return   // 点在编辑框内, 不处理
  e.preventDefault()                          // 阻止拖拽时选中文本
  batchEditor.show = false
  selection.active = false
  cellDrag = {
    rowIndex, orderIndex: col.orderIndex,
    startX: e.clientX, startY: e.clientY, moved: false,
  }
  document.addEventListener('mousemove', onCellDragMove)
  document.addEventListener('mouseup', onCellDragUp)
}

function onCellDragMove(e) {
  if (!cellDrag) return
  if (!cellDrag.moved) {
    if (Math.abs(e.clientX - cellDrag.startX) < 4 &&
        Math.abs(e.clientY - cellDrag.startY) < 4) return
    // 确认为拖拽 → 开启框选
    cellDrag.moved = true
    selection.active = true
    selection.dragging = true               // dragging 期间 onWheel 会禁止滚动
    selection.anchor = {row: cellDrag.rowIndex, col: cellDrag.orderIndex}
    editingCell.value = null
  }
  // 把鼠标位置换算成 行/列 下标, 并夹紧到"当前画面可见范围"内(框选不允许滚屏)
  const rect = bodyRef.value.getBoundingClientRect()
  const x = e.clientX - rect.left + scrollLeft.value
  const y = e.clientY - rect.top + scrollTop.value
  const firstRow = Math.floor(scrollTop.value / ROW_HEIGHT)
  const lastRow = Math.min(
      rows.value.length - 1,
      Math.floor((scrollTop.value + viewportH.value - 1) / ROW_HEIGHT)
  )
  const row = Math.max(firstRow, Math.min(lastRow, Math.floor(y / ROW_HEIGHT)))
  const colIdx = findColIndexByX(Math.max(0, Math.min(x, totalWidth.value - 1)))
  selection.current = {row, col: colIdx}
}

function onCellDragUp(e) {
  document.removeEventListener('mousemove', onCellDragMove)
  document.removeEventListener('mouseup', onCellDragUp)
  if (!cellDrag) return
  if (!cellDrag.moved) {
    // 没有拖动 → 普通单击 → 打开单格编辑
    openEditor(cellDrag.rowIndex, displayColumns.value[cellDrag.orderIndex].key)
  } else {
    // 完成框选 → 在鼠标附近弹出批量编辑输入框
    selection.dragging = false
    batchEditor.value = ''
    batchEditor.x = Math.min(e.clientX + 8, window.innerWidth - 260)
    batchEditor.y = Math.min(e.clientY + 8, window.innerHeight - 90)
    batchEditor.show = true
    nextTick(() => batchInputEl && batchInputEl.focus())
  }
  cellDrag = null
}

function setBatchInputRef(el) {
  batchInputEl = el
}

/** 把批量输入的值写入选区内所有单元格(仅进入 pendingEdits, 不提交) */
function applyBatchEdit() {
  const s = selRect.value
  if (!s) return
  for (let r = s.r1; r <= s.r2; r++) {
    for (let c = s.c1; c <= s.c2; c++) {
      const key = displayColumns.value[c].key
      pendingEdits.value[`${r}:${key}`] = batchEditor.value
    }
  }
  batchEditor.show = false
  selection.active = false
  showToast(`已修改 ${selectedCount.value || (s.r2 - s.r1 + 1) * (s.c2 - s.c1 + 1)} 个单元格(待应用)`)
}

function cancelBatchEdit() {
  batchEditor.show = false
  selection.active = false
}

/* ============================================================================
 * 十三、生命周期: 初始化(列定义 → 用户配置合并 → 首批数据) / 清理
 * ========================================================================== */
let resizeObserver = null

onMounted(async () => {
  // 1) 拉取列定义, 建立默认配置
  const cols = await mockBackend.fetchColumns()
  allColumns.value = cols
  columnOrder.value = cols.map((c) => c.key)
  for (const c of cols) columnWidths[c.key] = c.width

  // 2) 加载并合并用户已持久化的列配置(容错: 过滤失效列、补齐新增列)
  const cfg = await mockBackend.loadUserConfig()
  if (cfg) {
    const defaultKeys = cols.map((c) => c.key)
    if (Array.isArray(cfg.order)) {
      const valid = cfg.order.filter((k) => defaultKeys.includes(k))
      const missing = defaultKeys.filter((k) => !valid.includes(k))
      columnOrder.value = [...valid, ...missing]
    }
    if (cfg.widths) {
      for (const k of Object.keys(cfg.widths)) {
        if (k in columnWidths) columnWidths[k] = cfg.widths[k]
      }
    }
    if (Array.isArray(cfg.hidden)) {
      hiddenList.value = cfg.hidden.filter((k) => defaultKeys.includes(k))
    }
  }

  // 3) 首批 100 行
  await loadMore()

  // 4) 监听数据区尺寸变化, 保证可视范围计算准确
  resizeObserver = new ResizeObserver((entries) => {
    for (const en of entries) {
      viewportW.value = en.contentRect.width
      viewportH.value = en.contentRect.height
    }
  })
  if (bodyRef.value) resizeObserver.observe(bodyRef.value)
})

onBeforeUnmount(() => {
  resizeObserver?.disconnect()
  clearTimeout(tooltipTimer)
  clearTimeout(toastTimer)
  // 兜底移除可能残留的全局监听
  document.removeEventListener('mousemove', onResizeMove)
  document.removeEventListener('mouseup', onResizeEnd)
  document.removeEventListener('mousemove', onHeaderDragMove)
  document.removeEventListener('mouseup', onHeaderDragUp)
  document.removeEventListener('mousemove', onCellDragMove)
  document.removeEventListener('mouseup', onCellDragUp)
})
</script>

<template>
  <div class="vst-root">

    <!-- 左侧空间-->
    <div class="left-space">
      <!-- 列显隐勾选面板 -->
      <div v-if="showColumnPanel" class="column-panel">
        <label v-for="key in columnOrder" :key="key" class="col-panel-item">
          <input
              type="checkbox"
              :checked="!hiddenList.includes(key)"
              @change="toggleColumn(key)"
          />
          {{ columnMap[key]?.title }}
        </label>
      </div>
    </div>

    <div class="mian-space">
      <!-- ==================== 工具栏 ==================== -->
      <div class="vst-toolbar">

        <div class="col-panel-wrap">
          <button class="btn" @click="showColumnPanel = !showColumnPanel">
            << 列选择
          </button>
        </div>

        <button class="btn" @click="resetConfig">重置列配置</button>

        <span class="spacer"></span>

        <template v-if="!editMode">
          <button class="btn primary" @click="enterEditMode">编辑</button>
        </template>
        <template v-else>
          <button class="btn success" @click="applyEdits">
            应用 ({{ editedCount }})
          </button>
          <button class="btn" @click="exitEditMode">退出编辑</button>
          <span class="hint">单击 cell 编辑, 按住左键拖拽可框选批量修改</span>
        </template>

        <span class="status">
        已加载 {{ rows.length }} 行{{ hasMore ? '' : ' (已全部加载)' }}
      </span>
      </div>

      <!-- ==================== 表格主体 (grid 四象限布局) ====================
           ┌────────┬──────────────┐
           │ 角落#  │  表头(横向同步) │
           ├────────┼──────────────┤
           │ 行号列  │  数据区(滚动源) │
           │(纵向同步)│              │
           └────────┴──────────────┘
           表头/行号列用 overflow:hidden + transform 与数据区滚动位置同步,
           从而实现"表头吸顶 + 行号列固定"。
      -->
      <div class="vst-wrapper">
        <!-- 左上角: 行号列表头 -->
        <div class="vst-corner">#</div>

        <!-- 表头视口: 只随横向滚动 -->
        <div class="vst-header-viewport">
          <div
              class="vst-header-inner"
              :style="{
            width: totalWidth + 'px',
            transform: `translateX(${-scrollLeft}px)`,
          }"
          >
            <!-- 仅渲染可视范围内的表头单元格 -->
            <div
                v-for="col in visibleCols"
                :key="col.key"
                class="vst-header-cell"
                :class="{
              'drag-source': headerDrag.dragging && headerDrag.pendingKey === col.key,
              'drop-before':
                headerDrag.dragging && headerDrag.dropIndex === col.orderIndex && !headerDrag.dropAfter,
              'drop-after':
                headerDrag.dragging && headerDrag.dropIndex === col.orderIndex && headerDrag.dropAfter,
            }"
                :style="{ left: col.left + 'px', width: col.width + 'px' }"
                @mousedown="onHeaderMouseDown(col, $event)"
            >
              <span class="header-title">{{ col.title }}</span>
              <!-- 左边缘拖拽柄: 调整前一列宽度 -->
              <div
                  v-if="col.orderIndex > 0"
                  class="resize-handle left"
                  @mousedown.stop.prevent="onResizeStart(displayColumns[col.orderIndex - 1].key, $event)"
              ></div>
              <!-- 右边缘拖拽柄: 调整本列宽度 -->
              <div
                  class="resize-handle right"
                  @mousedown.stop.prevent="onResizeStart(col.key, $event)"
              ></div>
            </div>
          </div>
        </div>

        <!-- 行号列视口: 只随纵向滚动。行号为前端生成, 用于锚定数据行, 不可编辑 -->
        <div class="vst-rownum-viewport">
          <div
              class="vst-rownum-inner"
              :style="{
            height: totalHeight + 'px',
            transform: `translateY(${-scrollTop}px)`,
          }"
          >
            <div
                v-for="ri in visibleRowIndexes"
                :key="ri"
                class="vst-rownum-cell"
                :class="rowColorClass(ri + 1)"
                :style="{ top: ri * ROW_HEIGHT + 'px', height: ROW_HEIGHT + 'px' }"
            >
              {{ ri + 1 }}
            </div>
          </div>
        </div>

        <!-- 数据区: 唯一的滚动源 -->
        <div
            ref="bodyRef"
            class="vst-body-viewport"
            @scroll="onScroll"
            @wheel="onWheel"
        >
          <!-- 占位元素: 撑出与全部已加载内容等大的滚动区域(总宽×总高)。
               列少时 totalWidth < 视口宽 → 右侧留白, 不拉伸列 -->
          <div
              class="vst-phantom"
              :style="{ width: totalWidth + 'px', height: totalHeight + 'px' }"
          ></div>

          <!-- 仅渲染可视行; 每行内部仅渲染可视列 → 双向虚拟化 -->
          <div
              v-for="ri in visibleRowIndexes"
              :key="ri"
              class="vst-row"
              :class="rowColorClass(ri + 1)"
              :style="{
            top: ri * ROW_HEIGHT + 'px',
            height: ROW_HEIGHT + 'px',
            width: totalWidth + 'px',
          }"
          >
            <div
                v-for="col in visibleCols"
                :key="col.key"
                class="vst-cell"
                :class="{
              selected: isSelected(ri, col.orderIndex),
              edited: isEdited(ri, col.key),
              editing: isEditingCell(ri, col.key),
              editable: editMode,
            }"
                :style="{ left: col.left + 'px', width: col.width + 'px' }"
                @mousedown="onCellMouseDown($event, ri, col)"
                @dblclick="onCellDblClick(ri, col)"
                @mouseenter="onCellEnter($event, ri, col)"
                @mouseleave="onCellLeave"
            >
              <!-- 编辑态: 内嵌输入框 -->
              <input
                  v-if="isEditingCell(ri, col.key)"
                  :ref="setEditInputRef"
                  v-model="editingValue"
                  class="cell-input"
                  @keydown.enter="commitEdit"
                  @keydown.esc="cancelEdit"
                  @blur="commitEdit"
              />
              <!-- 展示态: 超出宽度自动省略号 -->
              <span v-else class="cell-text">{{ getCellValue(ri, col.key) }}</span>
            </div>
          </div>
        </div>

        <!-- 触底加载提示 -->
        <div v-if="loadingRows" class="vst-loading">加载中…</div>
      </div>

    </div>

    <!-- ==================== 全局浮层 ==================== -->
    <!-- 表头拖拽跟随鼠标的幽灵浮层 -->
    <div
        v-if="headerDrag.dragging"
        class="drag-ghost"
        :style="{ left: headerDrag.ghostX + 'px', top: headerDrag.ghostY + 'px' }"
    >
      {{ headerDrag.title }}
    </div>

    <!-- 长文本浮窗(自身也限制最大尺寸, 超出仍用省略号) -->
    <div
        v-if="tooltip.show"
        class="cell-tooltip"
        :style="{ left: tooltip.x + 'px', top: tooltip.y + 'px' }"
    >
      {{ tooltip.text }}
    </div>

    <!-- 批量编辑输入浮窗 -->
    <div
        v-if="batchEditor.show"
        class="batch-editor"
        :style="{ left: batchEditor.x + 'px', top: batchEditor.y + 'px' }"
    >
      <div class="batch-title">批量修改 {{ selectedCount }} 个单元格</div>
      <input
          :ref="setBatchInputRef"
          v-model="batchEditor.value"
          placeholder="输入后按 Enter 应用到选区"
          @keydown.enter="applyBatchEdit"
          @keydown.esc="cancelBatchEdit"
      />
    </div>

    <!-- 气泡提示(复制成功等) -->
    <transition name="fade">
      <div v-if="toast.show" class="toast">{{ toast.text }}</div>
    </transition>
  </div>
</template>

<style scoped>
/* ======================= 整体 ======================= */
.vst-root {
  display: flex;
  font-size: 13px;
  color: #333;
  font-family: 'Segoe UI', 'Microsoft YaHei', sans-serif;
}

.mian-space {
  display: flex;
  flex-direction: column;
}

/* ======================= 工具栏 ======================= */
.vst-toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 0;
}

.btn {
  padding: 5px 14px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  background: #fff;
  cursor: pointer;
  font-size: 13px;
}

.btn:hover {
  background: #f2f6fc;
}

.btn.primary {
  background: #409eff;
  border-color: #409eff;
  color: #fff;
}

.btn.success {
  background: #67c23a;
  border-color: #67c23a;
  color: #fff;
}

.spacer {
  flex: 1;
}

.hint {
  color: #999;
  font-size: 12px;
}

.status {
  color: #888;
  font-size: 12px;
}

/* 列设置下拉面板 */
.col-panel-wrap {
  position: relative;
}

.column-panel {
  top: 32px;
  right: 0;
  z-index: 100;
  width: 160px;
  max-height: 320px;
  overflow-y: auto;
  background: #fff;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12);
  padding: 6px 0;
}

.col-panel-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 12px;
  cursor: pointer;
}

.col-panel-item:hover {
  background: #f5f7fa;
}

/* ======================= 四象限网格布局 ======================= */
.vst-wrapper {
  position: relative;
  display: grid;
  grid-template-columns: 60px 1fr; /* 60px = ROWNUM_WIDTH */
  grid-template-rows: 40px 1fr; /* 40px = 表头高度 */
  height: 1000px;
  border: 1px solid #dcdfe6;
  background: #fff;
  overflow: hidden;
}

/* 左上角 */
.vst-corner {
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f5f7fa;
  border-right: 1px solid #dcdfe6;
  border-bottom: 1px solid #dcdfe6;
  font-weight: 600;
  z-index: 3;
}

/* ======================= 表头 ======================= */
.vst-header-viewport {
  overflow: hidden; /* 不产生自己的滚动条, 由 transform 同步 */
  border-bottom: 1px solid #dcdfe6;
  background: #f5f7fa;
  z-index: 2;
}

.vst-header-inner {
  position: relative;
  height: 100%;
}

.vst-header-cell {
  position: absolute;
  top: 0;
  height: 100%;
  display: flex;
  align-items: center;
  padding: 0 8px;
  box-sizing: border-box;
  border-right: 1px solid #e4e7ed;
  background: #f5f7fa;
  cursor: grab;
  user-select: none;
}

.vst-header-cell:hover {
  background: #e8ecf1;
}

.header-title {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-weight: 600;
}

/* 拖拽排序的视觉反馈 */
.vst-header-cell.drag-source {
  opacity: 0.4;
  background: #d0e2ff;
}

.vst-header-cell.drop-before {
  box-shadow: inset 3px 0 0 #409eff;
}

.vst-header-cell.drop-after {
  box-shadow: inset -3px 0 0 #409eff;
}

/* 列宽拖拽柄: 悬浮在表头左右边缘时出现 col-resize 指引 */
.resize-handle {
  position: absolute;
  top: 0;
  width: 7px;
  height: 100%;
  cursor: col-resize;
  z-index: 5;
}

.resize-handle.right {
  right: -3px;
}

.resize-handle.left {
  left: -3px;
}

.resize-handle:hover {
  background: rgba(64, 158, 255, 0.35);
}

/* ======================= 行号列 ======================= */
.vst-rownum-viewport {
  overflow: hidden;
  border-right: 1px solid #dcdfe6;
  background: #fafafa;
  z-index: 2;
}

.vst-rownum-inner {
  position: relative;
}

.vst-rownum-cell {
  position: absolute;
  left: 0;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  box-sizing: border-box;
  border-bottom: 1px solid #f0f0f0;
  color: #888;
  background: #fafafa;
  user-select: none;
}

/* ======================= 数据区 ======================= */
.vst-body-viewport {
  position: relative;
  overflow: auto; /* 唯一滚动源 */
}

.vst-phantom {
  pointer-events: none;
}

/* 只负责撑开滚动区域 */

.vst-row {
  position: absolute;
  left: 0;
  box-sizing: border-box;
  border-bottom: 1px solid #f0f0f0;
}

/* 行背景色: 无色 / 淡色A(蓝) / 淡色B(绿), 规则见 rowColorClass() */
.row-a {
  background: #e8f4ff;
}

.row-b {
  background: #e9f9e7;
}

.vst-cell {
  position: absolute;
  top: 0;
  height: 100%;
  display: flex;
  align-items: center;
  padding: 0 8px;
  box-sizing: border-box;
  border-right: 1px solid #f0f0f0;
  user-select: none;
}

.vst-cell.editable {
  cursor: cell;
}

/* 长文本 → 省略号; 拖宽列后自然显示更多内容 */
.cell-text {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  width: 100%;
}

/* 有未提交修改的单元格: 淡橙背景 + 右上角橙色小三角标记 */
.vst-cell.edited {
  background: #fff3e0;
}

.vst-cell.edited::after {
  content: '';
  position: absolute;
  top: 0;
  right: 0;
  border-top: 6px solid #ff9800;
  border-left: 6px solid transparent;
}

/* 框选选中的单元格 */
.vst-cell.selected {
  background: rgba(64, 158, 255, 0.18);
  box-shadow: inset 0 0 0 1px #409eff;
}

/* 正在编辑的单元格 */
.vst-cell.editing {
  padding: 0 2px;
  box-shadow: inset 0 0 0 2px #409eff;
}

.cell-input {
  width: 100%;
  height: 26px;
  border: none;
  outline: none;
  padding: 0 6px;
  font-size: 13px;
  box-sizing: border-box;
}

/* 触底加载提示 */
.vst-loading {
  position: absolute;
  bottom: 8px;
  left: 50%;
  transform: translateX(-50%);
  padding: 4px 16px;
  background: rgba(0, 0, 0, 0.65);
  color: #fff;
  border-radius: 12px;
  font-size: 12px;
  pointer-events: none;
  z-index: 10;
}

/* ======================= 全局浮层 ======================= */
/* 表头拖拽幽灵 */
.drag-ghost {
  position: fixed;
  z-index: 9999;
  padding: 5px 14px;
  background: #409eff;
  color: #fff;
  border-radius: 4px;
  font-size: 13px;
  pointer-events: none;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.25);
}

/* 长文本浮窗: 自身也限制尺寸, 超出部分多行省略 */
.cell-tooltip {
  position: fixed;
  z-index: 9998;
  max-width: 420px;
  max-height: 240px;
  padding: 10px 12px;
  background: #303133;
  color: #fff;
  border-radius: 6px;
  font-size: 12px;
  line-height: 1.6;
  word-break: break-all;
  overflow: hidden;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 10; /* 最多 10 行, 超出显示省略号 */
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
  pointer-events: none;
}

/* 批量编辑浮窗 */
.batch-editor {
  position: fixed;
  z-index: 9999;
  width: 240px;
  background: #fff;
  border: 1px solid #dcdfe6;
  border-radius: 6px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.18);
  padding: 10px;
}

.batch-title {
  font-size: 12px;
  color: #666;
  margin-bottom: 6px;
}

.batch-editor input {
  width: 100%;
  height: 28px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  padding: 0 8px;
  box-sizing: border-box;
  outline: none;
}

.batch-editor input:focus {
  border-color: #409eff;
}

/* 气泡提示 */
.toast {
  position: fixed;
  top: 60px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 10000;
  padding: 8px 24px;
  background: rgba(0, 0, 0, 0.75);
  color: #fff;
  border-radius: 4px;
  font-size: 13px;
  pointer-events: none;
}

.fade-enter-active, .fade-leave-active {
  transition: opacity 0.3s;
}

.fade-enter-from, .fade-leave-to {
  opacity: 0;
}
</style>