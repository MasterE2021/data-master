<template>
  <div class="file-tree">
    <div
        v-for="item in tree"
        :key="item.path"
        class="tree-item"
        :style="{ paddingLeft: depth * 16 + 'px' }"
    >
      <!-- 文件夹：可点击展开/折叠 -->
      <div
          v-if="item.children"
          class="tree-folder"
          @click="toggle(item)"
      >
        <span class="arrow">{{ item.expanded ? '▼' : '▶' }}</span>
        <span>📁 {{ item.name }}</span>
      </div>
      <!-- 文件：可点击选中 -->
      <div
          v-else
          class="tree-file"
          @click="$emit('select', item)"
      >
        <span style="margin-left: 16px">📄 {{ item.name }}</span>
      </div>

      <!-- 递归子文件夹 -->
      <FileTree
          v-if="item.children && item.expanded"
          :tree="item.children"
          :depth="depth + 1"
          @select="$emit('select', $event)"
      />
    </div>
  </div>
</template>

<script setup>
defineProps({
  tree: Array,
  depth: {type: Number, default: 0}
});

defineEmits(['select']);

const toggle = (item) => {
  item.expanded = !item.expanded;
};
</script>

<style scoped>
.tree-item {
  cursor: pointer;
  user-select: none;
  white-space: nowrap;
  padding: 2px 0;
  font-size: 13px;
}

.tree-folder:hover,
.tree-file:hover {
  background-color: #e2e8f0;
}
</style>