<template>
  <div class="home-app">
    <div class="nav-header">
      <div class="logo"><span class="brand-dot"></span> 书签管理系统</div>
      <div class="header-actions">
        <!-- 数据管理下拉菜单 -->
        <el-dropdown trigger="click" @command="handleCommand">
          <button class="white-nav-btn small-btn mr-10" style="height: auto">
            数据管理
            <svg
              class="svg-icon"
              style="width: 12px; height: 12px; margin-left: 5px"
              viewBox="0 0 24 24"
            >
              <polyline points="6 9 12 15 18 9"></polyline>
            </svg>
          </button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="import-json">
                <svg
                  class="svg-icon"
                  viewBox="0 0 24 24"
                  style="margin-right: 5px; width: 14px; height: 14px"
                >
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                  <polyline points="7 10 12 15 17 10"></polyline>
                  <line x1="12" y1="15" x2="12" y2="3"></line>
                </svg>
                导入 JSON
              </el-dropdown-item>
              <el-dropdown-item command="import-html">
                <svg
                  class="svg-icon"
                  viewBox="0 0 24 24"
                  style="margin-right: 5px; width: 14px; height: 14px"
                >
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                  <polyline points="7 10 12 15 17 10"></polyline>
                  <line x1="12" y1="15" x2="12" y2="3"></line>
                </svg>
                导入 HTML
              </el-dropdown-item>
              <el-dropdown-item divided command="export-json">
                <svg
                  class="svg-icon"
                  viewBox="0 0 24 24"
                  style="margin-right: 5px; width: 14px; height: 14px"
                >
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                  <polyline points="17 8 12 3 7 8"></polyline>
                  <line x1="12" y1="3" x2="12" y2="15"></line>
                </svg>
                导出 JSON
              </el-dropdown-item>
              <el-dropdown-item command="export-html">
                <svg
                  class="svg-icon"
                  viewBox="0 0 24 24"
                  style="margin-right: 5px; width: 14px; height: 14px"
                >
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                  <polyline points="17 8 12 3 7 8"></polyline>
                  <line x1="12" y1="3" x2="12" y2="15"></line>
                </svg>
                导出 HTML
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>

        <button class="white-nav-btn small-btn" @click="logout">
          退出登录
        </button>
      </div>
    </div>

    <div class="layout">
      <!-- 侧边栏 -->
      <aside class="sidebar">
        <div class="sidebar-header">分类</div>

        <div class="sidebar-all-books">
          <div
            class="cate-item"
            :class="{ active: selectedCategoryId === null }"
            @click="selectedCategoryId = null"
          >
            <div class="cate-content-wrapper">
              <span class="cate-name-text">
                <svg class="svg-icon menu-icon" viewBox="0 0 24 24">
                  <rect x="3" y="3" width="7" height="7"></rect>
                  <rect x="14" y="3" width="7" height="7"></rect>
                  <rect x="14" y="14" width="7" height="7"></rect>
                  <rect x="3" y="14" width="7" height="7"></rect>
                </svg>
                所有书签
              </span>
            </div>
          </div>
        </div>

        <div class="cate-tree-container">
          <!-- 使用 Element Plus 树形组件实现任意嵌套与拖拽排序 -->
          <el-tree
            :data="cateTree"
            node-key="id"
            draggable
            :allow-drop="() => true"
            @node-drop="handleNodeDrop"
            @node-drag-start="handleNodeDragStart"
            @node-drag-end="handleNodeDragEnd"
            default-expand-all
            :expand-on-click-node="false"
            class="custom-tree"
          >
            <template #default="{ node, data }">
              <!-- 复用我们定制的像素吃豆人交互节点 -->
              <div
                class="cate-item"
                :class="{
                  active: selectedCategoryId === data.id,
                  'drag-over': dragHoverCategoryId === data.id,
                }"
                @click="selectedCategoryId = data.id"
                @dragover.prevent="onDragOver(data.id)"
                @dragleave="onDragLeave"
                @drop="onDrop($event, data.id)"
              >
                <div class="cate-content-wrapper">
                  <!-- 原本文本 -->
                  <span
                    class="cate-name-text"
                    :class="{
                      'hide-text':
                        dragHoverCategoryId === data.id ||
                        eatingCategoryId === data.id,
                    }"
                  >
                    <svg class="svg-icon menu-icon" viewBox="0 0 24 24">
                      <path
                        d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
                      ></path>
                    </svg>
                    {{ data.name }}
                  </span>

                  <!-- 像素吃豆人动画组件 -->
                  <div
                    class="pacman-container"
                    :class="{
                      'show-pacman':
                        dragHoverCategoryId === data.id ||
                        eatingCategoryId === data.id,
                      eating: eatingCategoryId === data.id,
                    }"
                  >
                    <svg
                      class="pixel-pacman"
                      viewBox="0 0 13 13"
                      shape-rendering="crispEdges"
                    >
                      <rect x="4" y="0" width="5" height="1" fill="#DDA142" />
                      <rect x="2" y="1" width="9" height="1" fill="#DDA142" />
                      <rect x="1" y="2" width="11" height="1" fill="#DDA142" />
                      <rect x="0" y="3" width="13" height="1" fill="#DDA142" />
                      <rect x="0" y="9" width="13" height="1" fill="#DDA142" />
                      <rect x="1" y="10" width="11" height="1" fill="#DDA142" />
                      <rect x="2" y="11" width="9" height="1" fill="#DDA142" />
                      <rect x="4" y="12" width="5" height="1" fill="#DDA142" />
                      <rect x="0" y="4" width="11" height="1" fill="#DDA142" />
                      <rect
                        x="11"
                        y="4"
                        width="2"
                        height="1"
                        class="pac-jaw-top"
                        fill="#DDA142"
                      />
                      <rect x="0" y="5" width="8" height="1" fill="#DDA142" />
                      <rect
                        x="8"
                        y="5"
                        width="5"
                        height="1"
                        class="pac-jaw-top"
                        fill="#DDA142"
                      />
                      <rect x="0" y="6" width="6" height="1" fill="#DDA142" />
                      <rect
                        x="6"
                        y="6"
                        width="7"
                        height="1"
                        class="pac-jaw-mid"
                        fill="#DDA142"
                      />
                      <rect x="0" y="7" width="8" height="1" fill="#DDA142" />
                      <rect
                        x="8"
                        y="7"
                        width="5"
                        height="1"
                        class="pac-jaw-bottom"
                        fill="#DDA142"
                      />
                      <rect x="0" y="8" width="11" height="1" fill="#DDA142" />
                      <rect
                        x="11"
                        y="8"
                        width="2"
                        height="1"
                        class="pac-jaw-bottom"
                        fill="#DDA142"
                      />
                      <rect x="7" y="2" width="2" height="2" fill="#000" />
                    </svg>
                  </div>
                </div>

                <!-- 悬浮操作图标区 -->
                <div class="cate-actions">
                  <span
                    class="action-icon"
                    @click.stop="openAddSubCate(data.id)"
                    title="添加子分类"
                  >
                    <svg class="svg-icon" viewBox="0 0 24 24">
                      <line x1="12" y1="5" x2="12" y2="19"></line>
                      <line x1="5" y1="12" x2="19" y2="12"></line>
                    </svg>
                  </span>
                  <span
                    class="action-icon"
                    @click.stop="openEditCate(data)"
                    title="编辑名称"
                  >
                    <svg class="svg-icon" viewBox="0 0 24 24">
                      <path d="M12 20h9"></path>
                      <path
                        d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"
                      ></path>
                    </svg>
                  </span>
                </div>
              </div>
            </template>
          </el-tree>
        </div>

        <div class="sidebar-footer">
          <input
            v-model="cateName"
            placeholder="新建根分类名称..."
            class="modern-input"
            @keyup.enter="addCate(null)"
          />
          <button class="primary-btn small-btn" @click="addCate(null)">
            添加分类
          </button>
        </div>
      </aside>

      <!-- 主要内容区 -->
      <main class="content">
        <div class="content-header">
          <h2 class="category-title">
            <span v-if="selectedCategoryId !== null" class="brand-dot"></span>
            {{ currentCategoryName }}
          </h2>
          <div class="book-add-bar" v-if="selectedCategoryId !== null">
            <input
              v-model="bookTitle"
              placeholder="书签名称"
              class="modern-input"
            />
            <input
              v-model="bookUrl"
              placeholder="网址 (如 https://...)"
              class="modern-input url-input"
            />
            <button class="primary-btn" @click="addBook">添加书签</button>
          </div>
          <div class="book-add-bar disabled-bar" v-else>
            <span
              ><span class="brand-dot small"></span>
              请先在左侧选择一个特定分类，然后再添加书签。</span
            >
          </div>
        </div>

        <div class="book-grid">
          <div
            v-for="book in filteredBookList"
            :key="book.id"
            class="book-card"
            draggable="true"
            @dragstart="onDragStart($event, book)"
            @dragend="onDragEnd"
          >
            <div class="drag-handle">⋮⋮</div>
            <div class="book-info">
              <a :href="book.url" target="_blank" class="book-title">{{
                book.title
              }}</a>
              <span class="book-url-text">{{ book.url }}</span>
            </div>
            <div class="book-actions">
              <span
                class="action-icon"
                @click="openEditBook(book)"
                title="编辑"
              >
                <svg class="svg-icon" viewBox="0 0 24 24">
                  <path d="M12 20h9"></path>
                  <path
                    d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"
                  ></path>
                </svg>
              </span>
            </div>
          </div>
        </div>

        <div v-if="filteredBookList.length === 0" class="empty-state">
          <svg class="svg-icon empty-icon" viewBox="0 0 24 24">
            <path
              d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
            ></path>
          </svg>
          该分类下暂无书签
        </div>
      </main>
    </div>

    <!-- 弹窗 -->
    <el-dialog v-model="showAddSubCate" title="新建子分类" width="400px">
      <input
        v-model="newSubCateName"
        class="modern-input full-width"
        placeholder="请输入子分类名称"
        @keyup.enter="saveAddSubCate"
      />
      <template #footer>
        <button
          class="outline-btn small-btn mr-10"
          @click="showAddSubCate = false"
        >
          取消
        </button>
        <button class="primary-btn small-btn" @click="saveAddSubCate">
          保存
        </button>
      </template>
    </el-dialog>

    <el-dialog v-model="showEditCate" title="编辑分类" width="400px">
      <input v-model="editCate.name" class="modern-input full-width" />
      <template #footer>
        <button
          class="outline-btn small-btn mr-10"
          @click="showEditCate = false"
        >
          取消
        </button>
        <button class="primary-btn small-btn" @click="saveEditCate">
          保存
        </button>
      </template>
    </el-dialog>

    <el-dialog v-model="showEditBook" title="编辑书签" width="400px">
      <input
        v-model="editBook.title"
        class="modern-input full-width mb-10"
        placeholder="标题"
      />
      <input
        v-model="editBook.url"
        class="modern-input full-width"
        placeholder="URL"
      />
      <template #footer>
        <button
          class="outline-btn small-btn mr-10"
          @click="showEditBook = false"
        >
          取消
        </button>
        <button class="primary-btn small-btn" @click="saveEditBook">
          保存
        </button>
      </template>
    </el-dialog>

    <!-- 右下角吃豆人垃圾桶 (现在兼容删除分类和删除书签) -->
    <div
      class="trash-container"
      :class="{
        'dragging-active': isDraggingAny,
        'drag-over': dragHoverTrash,
        eating: eatingTrash,
      }"
      @dragover.prevent="dragHoverTrash = true"
      @dragleave="dragHoverTrash = false"
      @drop="onTrashDrop"
    >
      <div class="trash-wrapper">
        <!-- 像素垃圾桶 -->
        <svg
          class="pixel-trash"
          :class="{ hide: dragHoverTrash || eatingTrash }"
          viewBox="0 0 13 13"
          shape-rendering="crispEdges"
        >
          <rect x="4" y="2" width="5" height="1" fill="#bbbbbb" />
          <rect x="2" y="3" width="9" height="1" fill="#bbbbbb" />
          <rect x="3" y="4" width="1" height="7" fill="#bbbbbb" />
          <rect x="9" y="4" width="1" height="7" fill="#bbbbbb" />
          <rect x="3" y="11" width="7" height="1" fill="#bbbbbb" />
          <rect x="5" y="5" width="1" height="5" fill="#bbbbbb" />
          <rect x="7" y="5" width="1" height="5" fill="#bbbbbb" />
        </svg>
        <!-- 镜像吃豆人 -->
        <svg
          class="pixel-pacman flipped"
          :class="{ show: dragHoverTrash || eatingTrash }"
          viewBox="0 0 13 13"
          shape-rendering="crispEdges"
        >
          <rect x="4" y="0" width="5" height="1" fill="#DDA142" />
          <rect x="2" y="1" width="9" height="1" fill="#DDA142" />
          <rect x="1" y="2" width="11" height="1" fill="#DDA142" />
          <rect x="0" y="3" width="13" height="1" fill="#DDA142" />
          <rect x="0" y="9" width="13" height="1" fill="#DDA142" />
          <rect x="1" y="10" width="11" height="1" fill="#DDA142" />
          <rect x="2" y="11" width="9" height="1" fill="#DDA142" />
          <rect x="4" y="12" width="5" height="1" fill="#DDA142" />
          <rect x="0" y="4" width="11" height="1" fill="#DDA142" />
          <rect
            x="11"
            y="4"
            width="2"
            height="1"
            class="pac-jaw-top"
            fill="#DDA142"
          />
          <rect x="0" y="5" width="8" height="1" fill="#DDA142" />
          <rect
            x="8"
            y="5"
            width="5"
            height="1"
            class="pac-jaw-top"
            fill="#DDA142"
          />
          <rect x="0" y="6" width="6" height="1" fill="#DDA142" />
          <rect
            x="6"
            y="6"
            width="7"
            height="1"
            class="pac-jaw-mid"
            fill="#DDA142"
          />
          <rect x="0" y="7" width="8" height="1" fill="#DDA142" />
          <rect
            x="8"
            y="7"
            width="5"
            height="1"
            class="pac-jaw-bottom"
            fill="#DDA142"
          />
          <rect x="0" y="8" width="11" height="1" fill="#DDA142" />
          <rect
            x="11"
            y="8"
            width="2"
            height="1"
            class="pac-jaw-bottom"
            fill="#DDA142"
          />
          <rect x="7" y="2" width="2" height="2" fill="#000" />
        </svg>
      </div>
    </div>
    <!-- 隐藏的文件上传 input (现在移动到外面，因为触发是独立的) -->
    <input
      type="file"
      ref="fileInput"
      style="display: none"
      @change="handleFileUpload"
    />

    <!-- 全屏动画遮罩 -->
    <div class="fullscreen-overlay" v-if="isImportingAnim || isExportingAnim">
      <div class="anim-theater">
        <!-- 像素吃豆人巨怪 -->
        <svg
          class="pixel-pacman giant-pacman"
          :class="{
            spitting: isExportingAnim,
            'eating-frenzy': isImportingAnim,
          }"
          viewBox="0 0 13 13"
          shape-rendering="crispEdges"
        >
          <rect x="4" y="0" width="5" height="1" fill="#DDA142" />
          <rect x="2" y="1" width="9" height="1" fill="#DDA142" />
          <rect x="1" y="2" width="11" height="1" fill="#DDA142" />
          <rect x="0" y="3" width="13" height="1" fill="#DDA142" />
          <rect x="0" y="9" width="13" height="1" fill="#DDA142" />
          <rect x="1" y="10" width="11" height="1" fill="#DDA142" />
          <rect x="2" y="11" width="9" height="1" fill="#DDA142" />
          <rect x="4" y="12" width="5" height="1" fill="#DDA142" />
          <rect x="0" y="4" width="11" height="1" fill="#DDA142" />
          <rect
            x="11"
            y="4"
            width="2"
            height="1"
            class="pac-jaw-top"
            fill="#DDA142"
          />
          <rect x="0" y="5" width="8" height="1" fill="#DDA142" />
          <rect
            x="8"
            y="5"
            width="5"
            height="1"
            class="pac-jaw-top"
            fill="#DDA142"
          />
          <rect x="0" y="6" width="6" height="1" fill="#DDA142" />
          <rect
            x="6"
            y="6"
            width="7"
            height="1"
            class="pac-jaw-mid"
            fill="#DDA142"
          />
          <rect x="0" y="7" width="8" height="1" fill="#DDA142" />
          <rect
            x="8"
            y="7"
            width="5"
            height="1"
            class="pac-jaw-bottom"
            fill="#DDA142"
          />
          <rect x="0" y="8" width="11" height="1" fill="#DDA142" />
          <rect
            x="11"
            y="8"
            width="2"
            height="1"
            class="pac-jaw-bottom"
            fill="#DDA142"
          />
          <rect x="7" y="2" width="2" height="2" fill="#000" />
        </svg>

        <!-- 导入吃粒子 -->
        <div class="particles import-particles" v-if="isImportingAnim">
          <div
            class="particle"
            v-for="i in 10"
            :key="'in' + i"
            :style="{ animationDelay: i * 0.1 + 's' }"
          ></div>
        </div>

        <!-- 导出吐粒子 -->
        <div class="particles export-particles" v-if="isExportingAnim">
          <div
            class="particle"
            v-for="i in 15"
            :key="'out' + i"
            :style="{
              '--tx': Math.random() * 300 - 150 + 'px',
              '--ty': Math.random() * 200 - 100 + 'px',
              animationDelay: i * 0.05 + 's',
            }"
          ></div>
        </div>
      </div>
      <div class="anim-text">
        {{ isImportingAnim ? "疯狂吞食数据中..." : "疯狂喷射数据中..." }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { useRouter } from "vue-router";
import { ElMessage, ElMessageBox } from "element-plus";
import request from "../utils/request.js";

const router = useRouter();

// User Info
const currentUser = JSON.parse(localStorage.getItem("user") || "{}");
const userId = currentUser.id;

// State
const cateList = ref([]);
const cateTree = ref([]);
const bookList = ref([]);
const selectedCategoryId = ref(null);

// Drag & Drop State
const isDraggingBookmark = ref(false);
const isDraggingCategory = ref(false);
const draggingCateId = ref(null);
const isDraggingAny = computed(
  () => isDraggingBookmark.value || isDraggingCategory.value,
);

const dragHoverCategoryId = ref(null);
const eatingCategoryId = ref(null);

const dragHoverTrash = ref(false);
const eatingTrash = ref(false);

const cateName = ref("");
const bookTitle = ref("");
const bookUrl = ref("");
const showEditCate = ref(false);
const editCate = ref({});
const showEditBook = ref(false);
const editBook = ref({});

// --- 数据导入导出逻辑 ---
const fileInput = ref(null);
const importFormat = ref("json"); // 记录当前选择的导入格式

const isImportingAnim = ref(false);
const isExportingAnim = ref(false);

const handleCommand = (command) => {
  if (command === "export-json") {
    exportBookmarks("json");
  } else if (command === "export-html") {
    exportBookmarks("html");
  } else if (command === "import-json") {
    triggerImport("json");
  } else if (command === "import-html") {
    triggerImport("html");
  }
};

const exportBookmarks = (format) => {
  isExportingAnim.value = true;
  // 动画时长 1.5s，等喷发完毕再下载
  setTimeout(() => {
    isExportingAnim.value = false;
    const url = `${request.defaults.baseURL}/bookmark/export?userId=${userId}&format=${format}`;
    window.open(url, "_blank");
  }, 1500);
};

const triggerImport = (format) => {
  importFormat.value = format;
  if (fileInput.value) {
    fileInput.value.accept = format === "json" ? ".json" : ".html";
    fileInput.value.value = ""; // 重置，允许重复上传同一个文件
    fileInput.value.click();
  }
};

const handleFileUpload = async (event) => {
  const file = event.target.files[0];
  if (!file) return;

  const formData = new FormData();
  formData.append("file", file);
  formData.append("userId", userId);
  formData.append("format", importFormat.value);

  // 触发疯狂进食动画
  isImportingAnim.value = true;

  try {
    const res = await request.post("/bookmark/import", formData, {
      timeout: 60000,
    });

    // 给动画留点播放时间 1.5s
    setTimeout(() => {
      isImportingAnim.value = false;
      if (res.data && res.data.code === 200) {
        ElMessage.success(`导入成功，共吃进 ${res.data.data} 个书签`);
        loadData();
      } else {
        ElMessage.error(res.data.msg || "导入失败");
      }
      if (fileInput.value) fileInput.value.value = "";
    }, 1500);
  } catch (error) {
    setTimeout(() => {
      isImportingAnim.value = false;
      ElMessage.error("网络异常，导入失败");
      if (fileInput.value) fileInput.value.value = "";
    }, 1500);
  }
};

// Computed
const currentCategoryName = computed(() => {
  if (selectedCategoryId.value === null) return "所有书签";
  const cate = cateList.value.find((c) => c.id === selectedCategoryId.value);
  return cate ? cate.name : "未知分类";
});

const filteredBookList = computed(() => {
  if (selectedCategoryId.value === null) {
    return bookList.value;
  }
  return bookList.value.filter(
    (b) => b.categoryId === selectedCategoryId.value,
  );
});

// Category Tree Builder
const buildTree = (list) => {
  const map = {};
  const tree = [];
  list.forEach((node) => {
    map[node.id] = { ...node, children: [] };
  });
  list.forEach((node) => {
    if (node.parentId && map[node.parentId]) {
      map[node.parentId].children.push(map[node.id]);
    } else {
      tree.push(map[node.id]);
    }
  });
  // Sort
  const sortByOrder = (a, b) => (a.sortOrder || 0) - (b.sortOrder || 0);
  tree.sort(sortByOrder);
  Object.values(map).forEach((n) => n.children.sort(sortByOrder));
  return tree;
};

// Tree Drag & Drop Handlers (for nested sorting)
const handleNodeDragStart = (node, ev) => {
  isDraggingCategory.value = true;
  draggingCateId.value = node.data.id;
};

const handleNodeDragEnd = (node, dropNode, dropType, ev) => {
  isDraggingCategory.value = false;
  draggingCateId.value = null;
  dragHoverTrash.value = false;
};

const handleNodeDrop = async (draggingNode, dropNode, dropType, ev) => {
  // Compute flat updates array based on new cateTree structure
  const flatUpdates = [];
  let globalOrder = 0;

  const flatten = (nodes, parentId = null) => {
    nodes.forEach((node) => {
      flatUpdates.push({
        id: node.id,
        name: node.name,
        userId: node.userId,
        parentId: parentId,
        sortOrder: globalOrder++,
      });
      if (node.children && node.children.length > 0) {
        flatten(node.children, node.id);
      }
    });
  };

  flatten(cateTree.value);

  try {
    await request.post("/category/batchUpdate", flatUpdates);
    // Silent success
  } catch (e) {
    ElMessage.error("排序保存失败，正在恢复");
    loadData();
  }
};

// Bookmark Drag & Drop Handlers
const onDragStart = (event, book) => {
  event.dataTransfer.setData("text/plain", book.id);
  event.dataTransfer.effectAllowed = "move";
  event.target.style.opacity = "0.5";
  isDraggingBookmark.value = true;
};

const onDragEnd = (event) => {
  event.target.style.opacity = "1";
  dragHoverCategoryId.value = null;
  dragHoverTrash.value = false;
  isDraggingBookmark.value = false;
};

const onDragOver = (cateId) => {
  if (isDraggingBookmark.value) {
    dragHoverCategoryId.value = cateId;
  }
};

const onDragLeave = () => {
  dragHoverCategoryId.value = null;
};

const onDrop = async (event, targetCateId) => {
  dragHoverCategoryId.value = null;

  // 必须是拖拽的书签
  if (!isDraggingBookmark.value) return;

  const bookIdStr = event.dataTransfer.getData("text/plain");
  if (!bookIdStr) return;

  const bookId = parseInt(bookIdStr, 10);
  const book = bookList.value.find((b) => b.id === bookId);

  if (book && book.categoryId !== targetCateId) {
    eatingCategoryId.value = targetCateId;
    setTimeout(async () => {
      eatingCategoryId.value = null;
      book.categoryId = targetCateId;
      try {
        await request.post("/bookmark/add", book);
        ElMessage.success("书签已移入分类");
        loadData();
      } catch (e) {
        ElMessage.error("移动失败");
        loadData();
      }
    }, 600);
  }
};

// Trash Bin Handler
const onTrashDrop = async (event) => {
  dragHoverTrash.value = false;

  // 1. 处理分类的级联删除
  if (isDraggingCategory.value && draggingCateId.value) {
    const cateId = draggingCateId.value;
    eatingTrash.value = true;
    setTimeout(async () => {
      eatingTrash.value = false;
      try {
        await request.post("/category/delete", { id: cateId });
        ElMessage.success("分类已被斩草除根");
        if (selectedCategoryId.value === cateId) {
          selectedCategoryId.value = null;
        }
        loadData();
      } catch (e) {
        ElMessage.error("删除失败");
        loadData();
      }
    }, 600);
    return;
  }

  // 2. 处理书签的删除
  if (isDraggingBookmark.value) {
    const bookIdStr = event.dataTransfer.getData("text/plain");
    if (!bookIdStr) return;
    const bookId = parseInt(bookIdStr, 10);

    eatingTrash.value = true;
    setTimeout(async () => {
      eatingTrash.value = false;
      bookList.value = bookList.value.filter((b) => b.id !== bookId);
      try {
        await request.post("/bookmark/delete", { id: bookId });
        ElMessage.success("书签已粉碎");
        loadData();
      } catch (e) {
        ElMessage.error("删除失败");
        loadData();
      }
    }, 600);
  }
};

// API Logic
const loadData = async () => {
  const c = await request.get(`/category/list?userId=${userId}`);
  const b = await request.get(`/bookmark/list?userId=${userId}`);
  cateList.value = c.data.data;
  cateTree.value = buildTree(cateList.value);
  bookList.value = b.data.data;
};

// addCate 现在仅处理根分类的添加
const addCate = async (parentId = null) => {
  if (parentId === null) {
    const name = cateName.value.trim();
    if (!name) return;
    await request.post("/category/add", {
      name,
      userId,
      parentId: null,
      sortOrder: 0,
    });
    cateName.value = "";
    loadData();
  }
};

// --- 子分类添加逻辑 ---
const showAddSubCate = ref(false);
const newSubCateName = ref("");
const newSubCateParentId = ref(null);

const openAddSubCate = (parentId) => {
  newSubCateParentId.value = parentId;
  newSubCateName.value = "";
  showAddSubCate.value = true;
};

const saveAddSubCate = async () => {
  const name = newSubCateName.value.trim();
  if (!name) {
    ElMessage.warning("名称不能为空");
    return;
  }
  await request.post("/category/add", {
    name,
    userId,
    parentId: newSubCateParentId.value,
    sortOrder: 0,
  });
  showAddSubCate.value = false;
  loadData();
};

const openEditCate = (cate) => {
  editCate.value = { ...cate };
  showEditCate.value = true;
};

const saveEditCate = async () => {
  await request.post("/category/add", editCate.value);
  showEditCate.value = false;
  loadData();
};

const addBook = async () => {
  if (!bookTitle.value.trim() || !bookUrl.value.trim()) {
    ElMessage.warning("请填写书签名称和网址");
    return;
  }
  await request.post("/bookmark/add", {
    title: bookTitle.value,
    url: bookUrl.value,
    categoryId: selectedCategoryId.value,
    userId,
  });
  bookTitle.value = "";
  bookUrl.value = "";
  loadData();
};

const openEditBook = (book) => {
  editBook.value = { ...book };
  showEditBook.value = true;
};

const saveEditBook = async () => {
  await request.post("/bookmark/add", editBook.value);
  showEditBook.value = false;
  loadData();
};

const logout = () => {
  localStorage.removeItem("user");
  ElMessage.success("已退出登录");
  router.push({ name: "Login" });
};

onMounted(() => {
  loadData();
});
</script>

<style scoped>
.home-app {
  font-family:
    "Inter",
    -apple-system,
    BlinkMacSystemFont,
    "Segoe UI",
    Roboto,
    sans-serif;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: #f5f5f5;
  color: #222222;
}

/* --- SVG 与品牌元素 --- */
.brand-dot {
  display: inline-block;
  width: 12px;
  height: 12px;
  background-color: #dda142;
  border-radius: 50%;
  margin-right: 8px;
}
.brand-dot.small {
  width: 8px;
  height: 8px;
}
.svg-icon {
  width: 16px;
  height: 16px;
  stroke: currentColor;
  stroke-width: 2;
  stroke-linecap: round;
  stroke-linejoin: round;
  fill: none;
}
.menu-icon {
  margin-right: 8px;
  vertical-align: -3px;
  opacity: 0.7;
}
.active .menu-icon {
  opacity: 1;
  stroke: #dda142;
}

/* 顶部导航 */
.nav-header {
  height: 60px;
  background-color: #333333;
  color: white;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 30px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  z-index: 10;
}
.header-actions {
  display: flex;
  align-items: center;
}
.logo {
  font-size: 20px;
  font-weight: 500;
  letter-spacing: 1px;
  display: flex;
  align-items: center;
}

/* 核心布局 */
.layout {
  flex: 1;
  display: flex;
  overflow: hidden;
}

/* 侧边栏 */
.sidebar {
  width: 320px; /* 稍微加宽以容纳树形缩进 */
  background-color: white;
  border-right: 1px solid rgba(0, 0, 0, 0.06);
  display: flex;
  flex-direction: column;
  z-index: 5;
}
.sidebar-header {
  padding: 20px;
  font-size: 14px;
  color: #999999;
  text-transform: uppercase;
  letter-spacing: 1px;
  font-weight: 600;
}
.sidebar-all-books {
  padding: 0 10px;
}
.cate-tree-container {
  flex: 1;
  overflow-y: auto;
  padding: 10px;
}

/* 覆盖 el-tree 默认样式使其隐形并支持全宽动画 */
:deep(.custom-tree) {
  background: transparent;
}
:deep(.el-tree-node__content) {
  height: 48px;
  background-color: transparent !important;
  padding-right: 0;
  margin-bottom: 5px;
  border-radius: 8px;
}
:deep(.el-tree-node__content:hover) {
  background-color: transparent !important;
}
:deep(.el-tree-node__expand-icon) {
  color: #aaaaaa;
  font-size: 16px;
  padding: 6px;
}
:deep(.el-tree-node__expand-icon.is-leaf) {
  color: transparent;
}

.cate-item {
  flex: 1;
  height: 48px;
  padding: 0 15px 0 5px; /* 右侧留白，左侧紧贴展开箭头 */
  border-radius: 8px;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: all 0.2s ease;
  border: 1px solid transparent;
}
.cate-item:hover {
  background-color: #f9f9f9;
}
.cate-item.active {
  background-color: #f0f0f0;
  color: #111111;
  font-weight: 500;
}
.cate-item.drag-over {
  background-color: #eaeaea;
  border: 1px dashed #333333;
}

/* --- 吃豆人动画容器与文本 --- */
.cate-content-wrapper {
  flex: 1;
  display: flex;
  align-items: center;
  position: relative;
  overflow: hidden;
  height: 100%;
}
.cate-name-text {
  flex: 1;
  display: flex;
  align-items: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  transform-origin: left center;
}
.cate-name-text.hide-text {
  opacity: 0;
  transform: scale(0.5) translateX(-20px);
}

.pacman-container {
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%) translateX(-20px) scale(0.5);
  opacity: 0;
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  pointer-events: none;
}
.pacman-container.show-pacman {
  opacity: 1;
  transform: translateY(-50%) translateX(0) scale(1.3);
}
.pacman-container.eating {
  opacity: 1;
  animation: pounce 0.6s ease-in-out forwards;
}

.pixel-pacman {
  width: 18px;
  height: 18px;
  display: block;
}
.pac-jaw-top,
.pac-jaw-bottom,
.pac-jaw-mid {
  opacity: 0;
}
.eating .pixel-pacman .pac-jaw-top,
.eating .pixel-pacman .pac-jaw-bottom {
  animation: chomp-jaw 0.15s infinite alternate;
}
.eating .pixel-pacman .pac-jaw-mid {
  animation: chomp-mid 0.15s infinite alternate;
}

@keyframes chomp-jaw {
  0%,
  30% {
    opacity: 0;
  }
  70%,
  100% {
    opacity: 1;
  }
}
@keyframes chomp-mid {
  0%,
  30% {
    opacity: 0;
    fill: #dda142;
  }
  70%,
  100% {
    opacity: 1;
    fill: #222;
  }
}
@keyframes pounce {
  0% {
    transform: translateY(-50%) translateX(0) scale(1.3);
  }
  25% {
    transform: translateY(-50%) translateX(-5px) scale(1.3);
  }
  50% {
    transform: translateY(-50%) translateX(10px) scale(1.6);
  }
  75% {
    transform: translateY(-50%) translateX(10px) scale(1.6);
    opacity: 1;
  }
  100% {
    transform: translateY(-50%) translateX(10px) scale(0.5);
    opacity: 0;
  }
}

.cate-actions {
  display: flex;
  gap: 8px;
  opacity: 0;
  visibility: hidden;
  transition: opacity 0.2s;
}
.cate-item:hover .cate-actions {
  opacity: 1;
  visibility: visible;
}
.action-icon {
  opacity: 0.5;
  cursor: pointer;
  transition:
    opacity 0.2s,
    color 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}
.action-icon:hover {
  opacity: 1;
  color: #dda142;
}

.sidebar-footer {
  padding: 20px;
  border-top: 1px solid rgba(0, 0, 0, 0.06);
  display: flex;
  flex-direction: column;
  gap: 10px;
}

/* 主内容区 */
.content {
  flex: 1;
  padding: 40px;
  overflow-y: auto;
  background-color: #fbfbfb;
}
.content-header {
  margin-bottom: 30px;
}
.category-title {
  font-size: 28px;
  font-weight: 500;
  margin-bottom: 20px;
  color: #222222;
  display: flex;
  align-items: center;
}

.book-add-bar {
  display: flex;
  gap: 15px;
  background: white;
  padding: 20px;
  border-radius: 12px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.03);
  border: 1px solid rgba(0, 0, 0, 0.04);
}
.disabled-bar {
  background: transparent;
  box-shadow: none;
  border: 1px dashed #cccccc;
  color: #999999;
  justify-content: center;
  padding: 15px;
  display: flex;
  align-items: center;
}
.url-input {
  flex: 1;
}

.book-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}
.book-card {
  background: white;
  border-radius: 12px;
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 15px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.04);
  border: 1px solid rgba(0, 0, 0, 0.05);
  transition:
    transform 0.2s,
    box-shadow 0.2s;
  cursor: grab;
}
.book-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.08);
}
.book-card:active {
  cursor: grabbing;
}
.drag-handle {
  color: #cccccc;
  font-size: 18px;
  cursor: grab;
  user-select: none;
  line-height: 1;
}
.book-info {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  gap: 5px;
}
.book-title {
  font-weight: 500;
  color: #333333;
  text-decoration: none;
  font-size: 16px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.book-title:hover {
  text-decoration: underline;
}
.book-url-text {
  font-size: 12px;
  color: #999999;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 15px;
  padding: 80px;
  color: #bbbbbb;
  font-size: 16px;
}
.empty-icon {
  width: 48px;
  height: 48px;
  stroke: #dddddd;
  stroke-width: 1.5;
}

/* --- 右下角吃豆人垃圾桶 --- */
.trash-container {
  position: fixed;
  right: 30px;
  bottom: 30px;
  width: 120px;
  height: 120px;
  background-color: white;
  border-radius: 50%;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  opacity: 0.5;
  z-index: 100;
  border: 2px solid transparent;
}
.trash-container.dragging-active {
  transform: scale(1.1);
  opacity: 1;
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
  border: 2px dashed #dddddd;
}
.trash-container.drag-over {
  transform: scale(1.3);
  background-color: #fcfcfc;
  border-color: #dda142;
  box-shadow: 0 8px 25px rgba(221, 161, 66, 0.3);
}
.trash-container.eating {
  transform: scale(1.3);
  background-color: #fcfcfc;
  border-color: #dda142;
  animation: pounce-left 0.6s ease-in-out forwards;
}

.trash-wrapper {
  position: relative;
  width: 60px;
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.pixel-trash {
  width: 56px;
  height: 56px;
  position: absolute;
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  transform-origin: center bottom;
}
.pixel-trash.hide {
  opacity: 0;
  transform: scale(0.2);
}

.trash-wrapper .pixel-pacman {
  width: 56px;
  height: 56px;
  position: absolute;
  opacity: 0;
  transform: scale(0.2) scaleX(-1);
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}
.trash-wrapper .pixel-pacman.show {
  opacity: 1;
  transform: scale(1.3) scaleX(-1);
}

@keyframes pounce-left {
  0% {
    transform: scale(1.3) translateX(0);
  }
  25% {
    transform: scale(1.3) translateX(4px);
  }
  50% {
    transform: scale(1.5) translateX(-8px);
  }
  75% {
    transform: scale(1.5) translateX(-8px);
    opacity: 1;
  }
  100% {
    transform: scale(1.1) translateX(0);
    opacity: 1;
  }
}

.modern-input {
  padding: 12px 20px;
  background-color: #eaeaea;
  border: 1px solid transparent;
  border-radius: 30px;
  font-size: 14px;
  color: #222222;
  outline: none;
  transition: all 0.3s;
}
.modern-input:focus {
  background-color: #ffffff;
  border: 1px solid #c0c0c0;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
}
.full-width {
  width: 100%;
  box-sizing: border-box;
}
.primary-btn {
  background-color: #333333;
  color: #ffffff;
  border: none;
  padding: 12px 25px;
  border-radius: 30px;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.2s;
}
.primary-btn:hover {
  background-color: #111111;
  transform: translateY(-1px);
}
.outline-btn {
  background-color: transparent;
  border: 1px solid #ffffff;
  color: #ffffff;
  padding: 10px 20px;
  border-radius: 30px;
  cursor: pointer;
  transition: all 0.2s;
}
.outline-btn:hover {
  background-color: rgba(255, 255, 255, 0.1);
}
.small-btn {
  padding: 10px 15px;
  font-size: 13px;
}
.outline-btn.small-btn {
  border-color: #333;
  color: #333;
}
.outline-btn.small-btn:hover {
  background-color: #f5f5f5;
}
.white-nav-btn {
  background-color: #ffffff;
  color: #333333;
  border: none;
  padding: 10px 20px;
  border-radius: 30px;
  cursor: pointer;
  transition: all 0.2s;
  font-weight: 500;
  display: flex;
  align-items: center;
  justify-content: center;
}
.white-nav-btn:hover {
  background-color: #f0f0f0;
  transform: translateY(-1px);
}
.mb-10 {
  margin-bottom: 10px;
}
.mr-10 {
  margin-right: 10px;
}

/* 全屏动画遮罩 */
.fullscreen-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.85);
  z-index: 9999;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
.anim-theater {
  position: relative;
  width: 300px;
  height: 200px;
  display: flex;
  align-items: center;
  justify-content: center;
}
.giant-pacman {
  width: 120px;
  height: 120px;
  transform-origin: center;
}

/* 进食动画 */
.giant-pacman.eating-frenzy .pac-jaw-top,
.giant-pacman.eating-frenzy .pac-jaw-bottom {
  animation: frenzy-chomp-jaw 0.1s infinite alternate;
}
.giant-pacman.eating-frenzy .pac-jaw-mid {
  animation: frenzy-chomp-mid 0.1s infinite alternate;
}
@keyframes frenzy-chomp-jaw {
  0% {
    opacity: 0;
  }
  100% {
    opacity: 1;
  }
}
@keyframes frenzy-chomp-mid {
  0% {
    opacity: 0;
    fill: #dda142;
  }
  100% {
    opacity: 1;
    fill: #222;
  }
}

/* 喷射动画 (面向左) */
.giant-pacman.spitting {
  transform: scaleX(-1);
  animation: spit-recoil 0.5s ease-in-out infinite;
}
.giant-pacman.spitting .pac-jaw-top,
.giant-pacman.spitting .pac-jaw-bottom {
  opacity: 1; /* 嘴巴大张 */
}
.giant-pacman.spitting .pac-jaw-mid {
  opacity: 1;
  fill: #222;
}
@keyframes spit-recoil {
  0%,
  100% {
    transform: scaleX(-1) translateX(0);
  }
  50% {
    transform: scaleX(-1) translateX(-10px);
  }
}

/* 粒子系统 */
.particles {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
}
.particle {
  position: absolute;
  width: 12px;
  height: 12px;
  background-color: white;
  border-radius: 2px;
  opacity: 0;
}

/* 进食粒子 - 从右飞向中心嘴部 */
.import-particles .particle {
  top: 50%;
  right: -50vw;
  transform: translateY(-50%);
  animation: fly-in 1s linear infinite;
  background-color: #dda142;
}
@keyframes fly-in {
  0% {
    right: -50vw;
    opacity: 1;
    transform: translateY(-50%) scale(1);
  }
  90% {
    right: 80px;
    opacity: 1;
    transform: translateY(-50%) scale(0.5);
  }
  100% {
    right: 120px;
    opacity: 0;
    transform: translateY(-50%) scale(0);
  }
}

/* 喷射粒子 - 从中心嘴部向外发散抛物线 */
.export-particles .particle {
  top: 50%;
  left: 50%; /* 吃豆人面向左，所以嘴巴在左侧一点 */
  transform: translate(-50%, -50%);
  animation: spray-out 0.8s cubic-bezier(0.25, 1, 0.5, 1) infinite;
  background-color: #dda142;
}
@keyframes spray-out {
  0% {
    transform: translate(-20px, 0) scale(0);
    opacity: 1;
  }
  80% {
    opacity: 1;
  }
  100% {
    transform: translate(calc(-50px + var(--tx)), var(--ty)) scale(1.5);
    opacity: 0;
  }
}

.anim-text {
  color: #dda142;
  font-size: 24px;
  font-weight: 600;
  margin-top: 20px;
  letter-spacing: 2px;
  animation: pulse-text 0.5s infinite alternate;
}
@keyframes pulse-text {
  from {
    opacity: 0.6;
    transform: scale(1);
  }
  to {
    opacity: 1;
    transform: scale(1.05);
  }
}
</style>
