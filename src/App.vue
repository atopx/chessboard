<!-- The exported code uses Tailwind CSS. Install Tailwind CSS in your dev environment to ensure all styles work. -->
<template>
    <div class="min-h-screen bg-gray-100 flex flex-col items-center">
        <div class="container mx-auto bg-white shadow-lg rounded-lg overflow-hidden w-full max-w-7xl">
            <div
                v-if="showEngineConfig"
                class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
            >
                <div class="bg-white rounded-lg w-[600px] shadow-xl">
                    <div class="border-b p-4 flex justify-between items-center">
                        <h2 class="text-xl font-semibold">引擎配置</h2>
                        <button @click="showEngineConfig = false" class="text-gray-500 hover:text-gray-700">
                            <i class="fas fa-times"></i>
                        </button>
                    </div>
                    <div class="p-6 space-y-6">
                        <!-- Basic Settings -->
                        <div class="space-y-4">
                            <div class="space-y-2">
                                <div class="flex justify-between">
                                    <label class="font-medium"
                                        >棋力等级<span class="text-sm text-gray-500"
                                            >(范围0~20, 20为不限制棋力)</span
                                        ></label
                                    >
                                    <div class="text-right text-sm text-blue-600">
                                        {{ engineConfig.strength }}
                                    </div>
                                </div>
                                <input
                                    type="range"
                                    v-model="engineConfig.strength"
                                    min="0"
                                    max="20"
                                    class="w-full"
                                    @input="updateEngineConfig"
                                />
                            </div>
                            <div class="flex justify-between">
                                <label class="font-medium">线程数</label>
                                <input
                                    type="number"
                                    v-model="engineConfig.threads"
                                    min="1"
                                    max="1024"
                                    class="w-[40%] p-2 border rounded"
                                    @input="updateEngineConfig"
                                />
                            </div>
                            <div class="flex justify-between">
                                <label class="font-medium">置换表大小(m)</label>
                                <input
                                    type="number"
                                    v-model="engineConfig.hashSize"
                                    min="1"
                                    max="33554432"
                                    class="w-[40%] p-2 border rounded"
                                    @input="updateEngineConfig"
                                />
                            </div>
                            <div class="space-y-2">
                                <div class="flex justify-between">
                                    <label class="font-medium">显示着法数<span class="text-sm text-gray-500">请谨慎设置，数量越多棋力越弱</span></label>
                                    <div class="text-right text-sm text-blue-600">
                                    {{ engineConfig.displayMoves }}
                                </div>
                                </div>
                                <input
                                    type="range"
                                    v-model="engineConfig.displayMoves"
                                    min="1"
                                    max="128"
                                    class="w-full"
                                    @input="updateEngineConfig"
                                />
                            </div>
                            <div class="flex justify-between">
                                <label class="font-medium">步时(秒)</label>
                                <input
                                    type="number"
                                    v-model="engineConfig.moveTime"
                                    min="0.1"
                                    max="600"
                                    step="0.1"
                                    class="w-[40%] p-2 border rounded"
                                    @input="updateEngineConfig"
                                />
                            </div>
                            <div class="flex justify-between">
                                <label class="font-medium">最大深度</label>
                                <input
                                    type="number"
                                    v-model="engineConfig.maxDepth"
                                    min="1"
                                    max="100"
                                    class="w-[40%] p-2 border rounded"
                                    @input="updateEngineConfig"
                                />
                            </div>
                        </div>
                        <!-- Book Settings -->
                        <div class="space-y-4 pt-4 border-t">
                            <div class="space-y-2">
                                <label class="font-medium">设置本地开局库</label>
                                <div class="flex space-x-2">
                                    <input
                                        type="text"
                                        v-model="engineConfig.localBookPath"
                                        readonly
                                        class="w-full p-2 border rounded bg-gray-50"
                                        placeholder="未选择文件"
                                    />
                                    <button
                                        @click="selectLocalBook"
                                        class="px-4 py-2 rounded-button bg-blue-500 text-white hover:bg-blue-600 whitespace-nowrap"
                                    >
                                        选择文件
                                    </button>
                                </div>
                            </div>
                            <div class="space-y-4 border p-4 border-gray-300">
                                <div class="flex items-center justify-between">
                                    <label class="font-medium">启用云库</label>
                                    <button
                                        @click="engineConfig.useCloudBook = !engineConfig.useCloudBook"
                                        class="relative w-12 h-6 rounded-full transition-colors duration-200 ease-in-out"
                                        :class="engineConfig.useCloudBook ? 'bg-blue-500' : 'bg-gray-300'"
                                    >
                                        <span
                                            class="block w-4 h-4 bg-white rounded-full transform transition-transform duration-200 ease-in-out"
                                            :class="
                                                engineConfig.useCloudBook ? 'translate-x-7' : 'translate-x-1'
                                            "
                                        ></span>
                                    </button>
                                </div>
                                <div
                                    class="pl-4 space-y-4"
                                    :class="{ 'opacity-50': !engineConfig.useCloudBook }"
                                >
                                    <div class="flex justify-between">
                                        <label class="font-medium">脱库步数</label>
                                        <input
                                            type="number"
                                            v-model="engineConfig.bookExitMoves"
                                            min="1"
                                            max="100"
                                            class="w-[40%] p-2 border rounded"
                                            :disabled="!engineConfig.useCloudBook"
                                        />
                                    </div>
                                    <div class="flex items-center justify-between">
                                        <label class="font-medium">启用残局库</label>
                                        <button
                                            @click="
                                                engineConfig.useEndgameBook = !engineConfig.useEndgameBook
                                            "
                                            class="relative w-12 h-6 rounded-full transition-colors duration-200 ease-in-out"
                                            :class="[
                                                engineConfig.useEndgameBook && engineConfig.useCloudBook
                                                    ? 'bg-blue-500'
                                                    : 'bg-gray-300',
                                            ]"
                                            :disabled="!engineConfig.useCloudBook"
                                        >
                                            <span
                                                class="block w-4 h-4 bg-white rounded-full transform transition-transform duration-200 ease-in-out"
                                                :class="
                                                    engineConfig.useEndgameBook && engineConfig.useCloudBook
                                                        ? 'translate-x-7'
                                                        : 'translate-x-1'
                                                "
                                            ></span>
                                        </button>
                                    </div>
                                    <div class="flex items-center justify-between">
                                        <div>
                                            <label class="font-medium">优先云库</label>
                                            <div class="text-sm text-gray-500">
                                                启用后优先查找云库，否则默认优先本地库
                                            </div>
                                        </div>
                                        <button
                                            @click="
                                                engineConfig.priorityCloudBook =
                                                    !engineConfig.priorityCloudBook
                                            "
                                            class="relative w-12 h-6 rounded-full transition-colors duration-200 ease-in-out"
                                            :class="[
                                                engineConfig.priorityCloudBook && engineConfig.useCloudBook
                                                    ? 'bg-blue-500'
                                                    : 'bg-gray-300',
                                            ]"
                                            :disabled="!engineConfig.useCloudBook"
                                        >
                                            <span
                                                class="block w-4 h-4 bg-white rounded-full transform transition-transform duration-200 ease-in-out"
                                                :class="
                                                    engineConfig.priorityCloudBook &&
                                                    engineConfig.useCloudBook
                                                        ? 'translate-x-7'
                                                        : 'translate-x-1'
                                                "
                                            ></span>
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="border-t p-4 flex justify-end space-x-4">
                        <button
                            @click="showEngineConfig = false"
                            class="px-4 py-2 rounded-button border border-gray-300 hover:bg-gray-100"
                        >
                            取消
                        </button>
                        <button
                            @click="saveEngineConfig"
                            class="px-4 py-2 rounded-button bg-blue-500 text-white hover:bg-blue-600"
                        >
                            保存设置
                        </button>
                    </div>
                </div>
            </div>
            <!-- Settings Dialog -->
            <div
                v-if="showSettings"
                class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
            >
                <div class="bg-white rounded-lg w-[500px] shadow-xl">
                    <div class="border-b p-4 flex justify-between items-center">
                        <h2 class="text-xl font-semibold">设置</h2>
                        <button @click="showSettings = false" class="text-gray-500 hover:text-gray-700">
                            <i class="fas fa-times"></i>
                        </button>
                    </div>
                    <div class="p-6">
                        <div class="space-y-6">
                            <!-- Piece Theme -->
                            <div class="space-y-2">
                                <label class="font-medium">棋子主题</label>
                                <div class="relative">
                                    <button
                                        @click="togglePieceThemeDropdown"
                                        class="w-full p-2 rounded-button border border-gray-300 flex justify-between items-center"
                                    >
                                        <span>{{ pieceThemes[selectedPieceTheme] }}</span>
                                        <i class="fas fa-chevron-down"></i>
                                    </button>
                                    <div
                                        v-if="showPieceThemeDropdown"
                                        class="absolute z-50 w-full mt-1 bg-white border border-gray-200 rounded-lg shadow-lg"
                                    >
                                        <div
                                            v-for="(theme, id) in pieceThemes"
                                            :key="id"
                                            @click="selectPieceTheme(id)"
                                            class="p-2 hover:bg-gray-50 cursor-pointer"
                                            :class="{ 'bg-blue-50': selectedPieceTheme === id }"
                                        >
                                            {{ theme }}
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <!-- Board Theme -->
                            <div class="space-y-2">
                                <label class="font-medium">棋盘主题</label>
                                <div class="relative">
                                    <button
                                        @click="toggleBoardThemeDropdown"
                                        class="w-full p-2 rounded-button border border-gray-300 flex justify-between items-center"
                                    >
                                        <span>{{ boardThemes[selectedBoardTheme] }}</span>
                                        <i class="fas fa-chevron-down"></i>
                                    </button>
                                    <div
                                        v-if="showBoardThemeDropdown"
                                        class="absolute z-50 w-full mt-1 bg-white border border-gray-200 rounded-lg shadow-lg"
                                    >
                                        <div
                                            v-for="(theme, id) in boardThemes"
                                            :key="id"
                                            @click="selectBoardTheme(id)"
                                            class="p-2 hover:bg-gray-50 cursor-pointer"
                                            :class="{ 'bg-blue-50': selectedBoardTheme === id }"
                                        >
                                            {{ theme }}
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <!-- Toggle Settings -->
                            <div class="space-y-4">
                                <!-- Background Music -->
                                <div class="flex items-center justify-between">
                                    <span class="font-medium">启用背景音乐</span>
                                    <button
                                        @click="bgMusicEnabled = !bgMusicEnabled"
                                        class="relative w-12 h-6 rounded-full transition-colors duration-200 ease-in-out"
                                        :class="bgMusicEnabled ? 'bg-blue-500' : 'bg-gray-300'"
                                    >
                                        <span
                                            class="block w-4 h-4 bg-white rounded-full transform transition-transform duration-200 ease-in-out"
                                            :class="bgMusicEnabled ? 'translate-x-7' : 'translate-x-1'"
                                        ></span>
                                    </button>
                                </div>
                                <!-- Sound Effects -->
                                <div class="flex items-center justify-between">
                                    <span class="font-medium">启用音效</span>
                                    <button
                                        @click="soundEnabled = !soundEnabled"
                                        class="relative w-12 h-6 rounded-full transition-colors duration-200 ease-in-out"
                                        :class="soundEnabled ? 'bg-blue-500' : 'bg-gray-300'"
                                    >
                                        <span
                                            class="block w-4 h-4 bg-white rounded-full transform transition-transform duration-200 ease-in-out"
                                            :class="soundEnabled ? 'translate-x-7' : 'translate-x-1'"
                                        ></span>
                                    </button>
                                </div>
                                <!-- Animation -->
                                <div class="flex items-center justify-between">
                                    <span class="font-medium">启用动画</span>
                                    <button
                                        @click="animationEnabled = !animationEnabled"
                                        class="relative w-12 h-6 rounded-full transition-colors duration-200 ease-in-out"
                                        :class="animationEnabled ? 'bg-blue-500' : 'bg-gray-300'"
                                    >
                                        <span
                                            class="block w-4 h-4 bg-white rounded-full transform transition-transform duration-200 ease-in-out"
                                            :class="animationEnabled ? 'translate-x-7' : 'translate-x-1'"
                                        ></span>
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="border-t p-4 flex justify-end space-x-4">
                        <button
                            @click="showSettings = false"
                            class="px-4 py-2 rounded-button border border-gray-300 hover:bg-gray-100"
                        >
                            取消
                        </button>
                        <button
                            @click="saveSettings"
                            class="px-4 py-2 rounded-button bg-blue-500 text-white hover:bg-blue-600"
                        >
                            保存设置
                        </button>
                    </div>
                </div>
            </div>
            <!-- 顶部工具栏 -->
            <div class="bg-gray-200 p-1 flex justify-between items-center border-b border-gray-300">
                <div class="flex space-x-2">
                    <button
                        @click="copyFEN"
                        class="p-2 rounded-button hover:bg-gray-300 cursor-pointer whitespace-nowrap"
                        title="复制局面"
                    >
                        <i class="fas fa-copy"></i>
                    </button>
                    <button
                        @click="showImportDialog = true"
                        class="p-2 rounded-button hover:bg-gray-300 cursor-pointer whitespace-nowrap"
                        title="导入局面"
                    >
                        <i class="fas fa-file-import"></i>
                    </button>
                    <button
                        @click="toggleRedEngine"
                        :class="[
                            'p-2 rounded-button hover:bg-gray-300 cursor-pointer whitespace-nowrap',
                            { 'bg-red-200': redEngineEnabled },
                        ]"
                        title="红方引擎"
                    >
                        <i class="fas fa-robot text-red-600"></i>
                    </button>
                    <button
                        @click="toggleBlackEngine"
                        :class="[
                            'p-2 rounded-button hover:bg-gray-300 cursor-pointer whitespace-nowrap',
                            { 'bg-gray-700': blackEngineEnabled },
                        ]"
                        title="黑方引擎"
                    >
                        <i class="fas fa-robot text-gray-900"></i>
                    </button>
                    <button
                        @click="toggleSearch"
                        :class="[
                            'p-2 rounded-button hover:bg-gray-300 cursor-pointer whitespace-nowrap',
                            { 'bg-blue-200': searchEnabled },
                        ]"
                        title="搜索分析"
                    >
                        <i class="fas fa-search"></i>
                    </button>
                    <button
                        @click="flipBoard"
                        class="p-2 rounded-button hover:bg-gray-300 cursor-pointer whitespace-nowrap"
                        title="翻转棋盘"
                    >
                        <i class="fas fa-sync-alt"></i>
                    </button>
                    <button
                        @click="startDrawing"
                        :class="[
                            'p-2 rounded-button hover:bg-gray-300 cursor-pointer whitespace-nowrap',
                            { 'bg-yellow-200': isDrawing },
                        ]"
                        title="连线"
                    >
                        <i class="fas fa-link"></i>
                    </button>
                </div>
                <div class="flex space-x-2">
                    <div class="relative">
                        <button
                            @click="toggleShareMenu"
                            class="p-2 rounded-button hover:bg-gray-300 cursor-pointer whitespace-nowrap"
                            title="分享"
                        >
                            <i class="fas fa-share-alt"></i>
                        </button>
                        <div
                            v-if="showShareMenu"
                            class="absolute right-0 mt-2 w-56 bg-white rounded-lg shadow-xl z-50 border border-gray-200"
                        >
                            <div class="py-1">
                                <button
                                    @click="copyBoardImage"
                                    class="w-full px-4 py-3 text-left hover:bg-gray-50 flex items-center space-x-2 border-b border-gray-100"
                                >
                                    <i class="fas fa-copy text-blue-500"></i>
                                    <span>生成局面图片(复制)</span>
                                </button>
                                <button
                                    @click="downloadBoardImage"
                                    class="w-full px-4 py-3 text-left hover:bg-gray-50 flex items-center space-x-2 border-b border-gray-100"
                                >
                                    <i class="fas fa-download text-green-500"></i>
                                    <span>下载当前局面图片</span>
                                </button>
                                <button
                                    @click="downloadMoveText"
                                    class="w-full px-4 py-3 text-left hover:bg-gray-50 flex items-center space-x-2"
                                >
                                    <i class="fas fa-file-alt text-purple-500"></i>
                                    <span>下载文字棋谱</span>
                                </button>
                            </div>
                        </div>
                    </div>
                    <button
                        @click="toggleEditMode"
                        :class="[
                            'p-2 rounded-button hover:bg-gray-300 cursor-pointer whitespace-nowrap',
                            { 'bg-blue-200': editModeEnabled },
                        ]"
                        title="编辑局面"
                    >
                        <i class="fas fa-edit"></i>
                    </button>
                    <button
                        @click="showEngineConfig = true"
                        class="p-2 rounded-button hover:bg-gray-300 cursor-pointer whitespace-nowrap"
                        title="引擎配置"
                    >
                        <i class="fas fa-microchip"></i>
                    </button>
                    <button
                        @click="showSettings = true"
                        class="p-2 rounded-button hover:bg-gray-300 cursor-pointer whitespace-nowrap"
                        title="设置"
                    >
                        <i class="fas fa-cog"></i>
                    </button>
                </div>
                <!-- 导入局面对话框 -->
                <div
                    v-if="showImportDialog"
                    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
                >
                    <div class="bg-white rounded-lg w-[500px] shadow-xl">
                        <div class="border-b p-4 flex justify-between items-center">
                            <h2 class="text-xl font-semibold">导入局面</h2>
                            <button
                                @click="showImportDialog = false"
                                class="text-gray-500 hover:text-gray-700"
                            >
                                <i class="fas fa-times"></i>
                            </button>
                        </div>
                        <div class="p-6">
                            <textarea
                                v-model="fenString"
                                class="w-full h-24 p-2 border rounded"
                                placeholder="请输入FEN字符串"
                            ></textarea>
                            <div class="mt-4 flex justify-end space-x-4">
                                <button
                                    @click="showImportDialog = false"
                                    class="px-4 py-2 rounded-button border border-gray-300 hover:bg-gray-100"
                                >
                                    取消
                                </button>
                                <button
                                    @click="importFEN"
                                    class="px-4 py-2 rounded-button bg-blue-500 text-white hover:bg-blue-600"
                                >
                                    确定
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
                <!-- 编辑模式棋子选择面板 -->
                <div
                    v-if="editModeEnabled"
                    class="fixed right-4 top-20 bg-white rounded-lg shadow-xl p-4 w-48"
                >
                    <div class="mb-4">
                        <h3 class="font-bold mb-2">红方棋子</h3>
                        <div class="grid grid-cols-4 gap-2">
                            <img
                                v-for="piece in redPieces"
                                :key="piece"
                                :src="piece"
                                class="w-8 h-8 cursor-pointer hover:scale-110 transition-transform"
                                @click="selectPiece(piece)"
                            />
                        </div>
                    </div>
                    <div>
                        <h3 class="font-bold mb-2">黑方棋子</h3>
                        <div class="grid grid-cols-4 gap-2">
                            <img
                                v-for="piece in blackPieces"
                                :key="piece"
                                :src="piece"
                                class="w-8 h-8 cursor-pointer hover:scale-110 transition-transform"
                                @click="selectPiece(piece)"
                            />
                        </div>
                    </div>
                </div>
            </div>
            <!-- 主内容区 -->
            <div class="flex flex-row">
                <!-- 棋盘区域 -->
                <div class="w-1/2 p-2 relative">
                    <div class="chess-board relative w-[624px] h-[694px]">
                        <img :src="chessboardImage" alt="象棋棋盘" class="w-full h-full" />
                        <!-- 棋子布局 - 这里使用绝对定位来放置棋子 -->
                        <div class="absolute inset-0 grid grid-cols-9 grid-rows-10">
                            <!-- 棋子会根据实际游戏状态动态生成 -->
                            <div
                                v-for="(piece, index) in chessPieces"
                                :key="index"
                                :style="getPieceStyle(piece)"
                                class="piece-container flex items-center justify-center"
                                @click="handlePieceClick(index)"
                            >
                                <img
                                    :src="piece.image"
                                    alt="chess piece"
                                    class="chess-piece w-[60px] h-[60px]"
                                    :class="{ selected: selectedPiece === index }"
                                />
                            </div>
                        </div>
                        <!-- 导航控制 -->
                        <div class="flex justify-between mt-4">
                            <button
                                class="p-2 rounded-button bg-gray-200 hover:bg-gray-300 cursor-pointer whitespace-nowrap"
                            >
                                <i class="fas fa-angle-double-left"></i>
                            </button>
                            <button
                                class="p-2 rounded-button bg-gray-200 hover:bg-gray-300 cursor-pointer whitespace-nowrap"
                            >
                                <i class="fas fa-angle-left"></i>
                            </button>
                            <button
                                class="p-2 rounded-button bg-gray-200 hover:bg-gray-300 cursor-pointer whitespace-nowrap"
                            >
                                <i class="fas fa-angle-right"></i>
                            </button>
                            <button
                                class="p-2 rounded-button bg-gray-200 hover:bg-gray-300 cursor-pointer whitespace-nowrap"
                            >
                                <i class="fas fa-angle-double-right"></i>
                            </button>
                        </div>
                    </div>
                </div>
                <!-- 行棋记录区 -->
                <div class="w-1/4 border-l border-r border-gray-300 flex flex-col" style="height: 700px">
                    <div class="p-4 border-b">
                        <div class="font-bold text-blue-600 cursor-pointer whitespace-nowrap">
                            引擎
                            <span class="font-normal text-green-600"> {{ engine.name }}</span>
                        </div>
                        
                    </div>
                    <div class="move-records flex-1 overflow-y-auto p-2">
                        <div
                            v-for="(record, index) in moveRecords"
                            :key="index"
                            :class="['move-record p-2 mb-2 rounded', record.highlight ? 'bg-blue-100' : '']"
                        >
                            <div class="text-red-600">{{ record.title }}</div>
                            <div
                                class="text-sm text-gray-700"
                                v-for="(line, lineIndex) in record.details"
                                :key="lineIndex"
                            >
                                {{ line }}
                            </div>
                        </div>
                    </div>
                </div>
                <!-- 引擎分析区 -->
                <div class="w-1/4 flex flex-col" style="height: 700px">
                    <div class="sticky top-0 bg-white z-10">
                        <div class="p-2">
                            <div class="flex justify-between items-center mb-2">
                                <div class="font-bold text-blue-600">对局趋势</div>
                                <div class="text-sm text-gray-500">{{ currentDate }}</div>
                            </div>
                            <div
                                ref="chartContainer"
                                class="w-full h-32 bg-white border border-gray-200 rounded-lg"
                            ></div>
                        </div>
                    </div>
                    <!-- 棋谱记录 -->
                    <div class="flex-1 overflow-y-auto p-4">
                        <div class="font-bold text-blue-600 mb-2">棋谱记录</div>
                        <div class="space-y-2">
                            <div
                                v-for="(option, index) in engineOptions"
                                :key="index"
                                class="p-2 border border-green-700 rounded-lg flex justify-between items-center hover:bg-gray-200"
                            >
                                <div class="flex items-center">
                                    <span class="text-sm font-medium text-gray-500"
                                        >{{ index + 1 }}. {{ option.move }}</span
                                    >
                                </div>
                                <div class="text-sm text-gray-600">{{ option.score }}</div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, reactive } from "vue";
import * as echarts from "echarts";

// Settings state
const showSettings = ref(false);
const showEngineConfig = ref(false);
const showEngineSettings = ref(false);
// Engine Settings
const engineSettings = reactive({
    strength: 10,
    threads: 1,
    hashSize: 1024,
    displayMoves: 10,
});
// Engine Configuration
const engineConfig = reactive({
    strength: 10,
    threads: 1,
    hashSize: 1024,
    displayMoves: 10,
    moveTime: 1.0,
    maxDepth: 20,
    localBookPath: "",
    useCloudBook: true,
    bookExitMoves: 20,
    useEndgameBook: true,
    priorityCloudBook: false,
});
const updateEngineConfig = () => {
    // Validate input values
    engineConfig.strength = Math.min(Math.max(engineConfig.strength, 0), 20);
    engineConfig.threads = Math.min(Math.max(engineConfig.threads, 1), 1024);
    engineConfig.hashSize = Math.min(Math.max(engineConfig.hashSize, 1), 33554432);
    engineConfig.displayMoves = Math.min(Math.max(engineConfig.displayMoves, 1), 128);
    engineConfig.moveTime = Math.min(Math.max(engineConfig.moveTime, 0.1), 600);
    engineConfig.maxDepth = Math.min(Math.max(engineConfig.maxDepth, 1), 100);
    engineConfig.bookExitMoves = Math.min(Math.max(engineConfig.bookExitMoves, 1), 100);
};
const selectLocalBook = () => {
    // Create a file input element
    const input = document.createElement("input");
    input.type = "file";
    input.accept = ".bin,.txt,.dat"; // Add appropriate file extensions
    // Handle file selection
    input.onchange = (e) => {
        const file = (e.target as HTMLInputElement).files?.[0];
        if (file) {
            engineConfig.localBookPath = file.name;
        }
    };
    // Trigger file selection dialog
    input.click();
};
const saveEngineConfig = () => {
    updateEngineConfig();
    // Here you would typically save the engine configuration
    showEngineConfig.value = false;
};
const showPieceThemeDropdown = ref(false);
const showBoardThemeDropdown = ref(false);
// Theme settings
const pieceThemes = {
    classic: "经典",
    modern: "现代",
    simple: "简约",
    traditional: "传统",
};
const boardThemes = {
    classic: "经典棋盘",
    modern: "现代棋盘",
    wood: "实木棋盘",
    marble: "大理石棋盘",
};
const selectedPieceTheme = ref("classic");
const selectedBoardTheme = ref("classic");
// Toggle settings
const bgMusicEnabled = ref(false);
const soundEnabled = ref(false);
const animationEnabled = ref(true);
// Theme dropdown functions
const togglePieceThemeDropdown = () => {
    showPieceThemeDropdown.value = !showPieceThemeDropdown.value;
    showBoardThemeDropdown.value = false;
};
const toggleBoardThemeDropdown = () => {
    showBoardThemeDropdown.value = !showBoardThemeDropdown.value;
    showPieceThemeDropdown.value = false;
};
const selectPieceTheme = (themeId: string) => {
    selectedPieceTheme.value = themeId;
    showPieceThemeDropdown.value = false;
};
const selectBoardTheme = (themeId: string) => {
    selectedBoardTheme.value = themeId;
    showBoardThemeDropdown.value = false;
};
// Click outside to close dropdowns
const closeDropdowns = (event: MouseEvent) => {
    const target = event.target as HTMLElement;
    if (!target.closest(".theme-dropdown")) {
        showPieceThemeDropdown.value = false;
        showBoardThemeDropdown.value = false;
    }
};
onMounted(() => {
    document.addEventListener("click", closeDropdowns);
});
// Settings functions
const saveSettings = () => {
    // Here you would typically save the settings to local storage
    // or send them to a backend server
    showSettings.value = false;
};
const saveEngineSettings = () => {
    // Validate and save engine settings
    if (engineSettings.threads < 1 || engineSettings.threads > 1024) {
        engineSettings.threads = Math.min(Math.max(engineSettings.threads, 1), 1024);
    }
    if (engineSettings.hashSize < 1 || engineSettings.hashSize > 33554432) {
        engineSettings.hashSize = Math.min(Math.max(engineSettings.hashSize, 1), 33554432);
    }
    // Here you would typically save the engine settings to local storage
    // or send them to a backend server
    showEngineSettings.value = false;
};
// 棋盘背景图片
const chessboardImage = "./assets/board/board_lite.webp";
// 当前日期
const currentDate = "2025-04-29";
// 棋子数据
const chessPieces = reactive([
    // 黑方棋子
    { image: "./assets/piece/bb.webp", x: 2, y: 0 },
    { image: "./assets/piece/ba.webp", x: 3, y: 0 },
    { image: "./assets/piece/bk.webp", x: 4, y: 0 },
    { image: "./assets/piece/ba.webp", x: 5, y: 0 },
    { image: "./assets/piece/bb.webp", x: 6, y: 0 },
    { image: "./assets/piece/bn.webp", x: 1, y: 0 },
    { image: "./assets/piece/br.webp", x: 0, y: 0 },
    { image: "./assets/piece/bc.webp", x: 1, y: 2 },
    { image: "./assets/piece/bn.webp", x: 7, y: 0 },
    { image: "./assets/piece/br.webp", x: 8, y: 0 },
    { image: "./assets/piece/bc.webp", x: 7, y: 2 },
    { image: "./assets/piece/bp.webp", x: 0, y: 3 },
    { image: "./assets/piece/bp.webp", x: 2, y: 3 },
    { image: "./assets/piece/bp.webp", x: 4, y: 3 },
    { image: "./assets/piece/bp.webp", x: 6, y: 3 },
    { image: "./assets/piece/bp.webp", x: 8, y: 3 },
    // 红方棋子
    { image: "./assets/piece/wb.webp", x: 2, y: 9 },
    { image: "./assets/piece/wa.webp", x: 3, y: 9 },
    { image: "./assets/piece/wk.webp", x: 4, y: 9 },
    { image: "./assets/piece/wa.webp", x: 5, y: 9 },
    { image: "./assets/piece/wb.webp", x: 6, y: 9 },
    { image: "./assets/piece/wn.webp", x: 1, y: 9 },
    { image: "./assets/piece/wr.webp", x: 0, y: 9 },
    { image: "./assets/piece/wc.webp", x: 1, y: 7 },
    { image: "./assets/piece/wn.webp", x: 7, y: 9 },
    { image: "./assets/piece/wr.webp", x: 8, y: 9 },
    { image: "./assets/piece/wc.webp", x: 7, y: 7 },
    { image: "./assets/piece/wp.webp", x: 0, y: 6 },
    { image: "./assets/piece/wp.webp", x: 2, y: 6 },
    { image: "./assets/piece/wp.webp", x: 4, y: 6 },
    { image: "./assets/piece/wp.webp", x: 6, y: 6 },
    { image: "./assets/piece/wp.webp", x: 8, y: 6 },
]);
// 计算棋子位置样式
const getPieceStyle = (piece) => {
    return {
        gridColumn: `${piece.x + 1} / ${piece.x + 2}`,
        gridRow: `${piece.y + 1} / ${piece.y + 2}`,
    };
};
// 行棋记录
const moveRecords = reactive([
    {
        title: "深度: 16 红方: 0.1秒时 NPS: 383K",
        details: [
            "炮二平五/炮8平5 马二进三/马2进3 车九进一/车9进1",
            "炮八平五/炮2平5 马八进七/马8进7 车一进一/车1进1",
            "炮八九平/炮89平 前炮进三/前炮进3",
        ],
        highlight: true,
    },
    {
        title: "深度: 14 红方: 0.1秒时 NPS: 366K",
        details: [
            "炮二平五/炮8平5 马二进三/马2进3 车九进一/车9进1",
            "炮八平五/炮2平5 马八进七/马8进7 车一进一/车1进1",
            "炮八九平/炮89平 前炮进三/前炮进3",
        ],
        highlight: false,
    },
    {
        title: "深度: 13 红方: 0.1秒时 NPS: 412K",
        details: [
            "炮二平五/炮8平5 马二进三/马2进3 车九进一/车9进1",
            "车九进七/车9进7 炮二进四/炮2进4 车一平二/车1平2",
            "马八进七/马8进7 车一进四/车1进4",
        ],
        highlight: false,
    },
    {
        title: "深度: 12 红方: 0.1秒时 NPS: 87K",
        details: [
            "炮二平五/炮8平5 马二进三/马2进3 车九进一/车9进1",
            "车九平七/车9平7 炮二平四/炮2平4 炮八进三/炮8进3",
            "车一平二/车1平2 车一进五/车1进5 马八进七/马8进7",
        ],
        highlight: false,
    },
    {
        title: "深度: 11 红方: 0.1秒时 NPS: 61K",
        details: [
            "炮二平五/炮8平5 马二进三/马2进3 车九进一/车9进1",
            "车九平七/车9平7 马八进七/马8进7 车一平二/车1平2",
            "车一平三/车1平3",
        ],
        highlight: false,
    },
    {
        title: "深度: 10 红方: 0.1秒时 NPS: 66K",
        details: [
            "炮二平五/炮8平5 马二进三/马2进3 车九进一/车9进1",
            "车九平七/车9平7 马八进七/马8进7 车一平二/车1平2",
            "车一平三/车1平3",
        ],
        highlight: false,
    },
]);
// 引擎推荐走法
const engineOptions = reactive([
    { move: "一平二", score: "±0.93" },
    { move: "二进三", score: "±0.51" },
    { move: "三进一", score: "±0.38" },
    { move: "四平五", score: "±0.29" },
    { move: "五进七", score: "±1.72" },
    { move: "六进五", score: "±0.82" },
    { move: "七平一", score: "±0.93" },
    { move: "八进九", score: "±0.74" },
    { move: "九进六", score: "±0.29" },
    { move: "十进八", score: "±0.83" },
    { move: "±4.93", score: "±4.93" },
]);
// 工具栏状态
const engine = ref({
    name: "Pikafish"
})
const redEngineEnabled = ref(false);
const blackEngineEnabled = ref(false);
const searchEnabled = ref(false);
const isDrawing = ref(false);
const showShareMenu = ref(false);
const editModeEnabled = ref(false);
const showImportDialog = ref(false);
const fenString = ref("");
const boardFlipped = ref(false);
// 趋势图数据和计算函数
const trendData = reactive([0, 15, -8, 12, -10, 5, -6, -3, 2, 8, -5, 10, -12, 7, -4]);
const getTrendPath = (data: number[]) => {
    const height = 200;
    const width = 400;
    const maxValue = Math.max(...data.map(Math.abs));
    const scale = height / (2 * maxValue);
    const step = width / (data.length - 1);
    return data.reduce((path, value, index) => {
        const x = index * step;
        const y = height / 2 - value * scale;
        return path + (index === 0 ? `M ${x},${y}` : ` L ${x},${y}`);
    }, "");
};
// 编辑模式棋子
const redPieces = [
    "./assets/piece/wr.webp",
    "./assets/piece/wn.webp",
    "./assets/piece/wb.webp",
    "./assets/piece/wa.webp",
    "./assets/piece/wk.webp",
    "./assets/piece/wc.webp",
    "./assets/piece/wp.webp",
];
const blackPieces = [
    "./assets/piece/br.webp",
    "./assets/piece/bn.webp",
    "./assets/piece/bb.webp",
    "./assets/piece/ba.webp",
    "./assets/piece/bk.webp",
    "./assets/piece/bc.webp",
    "./assets/piece/bp.webp",
];
// 功能函数
const copyFEN = async () => {
    const fen = "2b1kab2/4a4/n1c3nc1/p3p1p1p/5r3/2B3P2/P3P3P/2C3NC1/3R5/1N1AKAB2 w";
    try {
        await navigator.clipboard.writeText(fen);
        // 添加复制成功的视觉反馈
        const copyButton = document.querySelector('[title="复制局面"]');
        if (copyButton) {
            const originalClass = copyButton.className;
            copyButton.className += " bg-green-200";
            setTimeout(() => {
                copyButton.className = originalClass;
            }, 1000);
        }
    } catch (err) {
        console.error("Failed to copy FEN:", err);
    }
};
const toggleRedEngine = () => {
    redEngineEnabled.value = !redEngineEnabled.value;
};
const toggleBlackEngine = () => {
    blackEngineEnabled.value = !blackEngineEnabled.value;
};
const toggleSearch = () => {
    searchEnabled.value = !searchEnabled.value;
};
const flipBoard = () => {
    boardFlipped.value = !boardFlipped.value;
    // 在这里添加翻转棋盘的具体逻辑
};
const startDrawing = () => {
    isDrawing.value = !isDrawing.value;
    if (isDrawing.value) {
        document.body.style.cursor = "crosshair";
    } else {
        document.body.style.cursor = "default";
    }
};
const toggleShareMenu = () => {
    showShareMenu.value = !showShareMenu.value;
};
const copyBoardImage = async () => {
    try {
        const canvas = await generateBoardCanvas();
        const blob = await new Promise((resolve) => canvas.toBlob(resolve));
        const data = new ClipboardItem({ "image/png": blob as Blob });
        await navigator.clipboard.write([data]);
        // Show success feedback
        const shareButton = document.querySelector('[title="分享"]');
        if (shareButton) {
            const originalClass = shareButton.className;
            shareButton.className += " bg-green-200";
            setTimeout(() => {
                shareButton.className = originalClass;
            }, 1000);
        }
    } catch (err) {
        console.error("Failed to copy board image:", err);
    }
    showShareMenu.value = false;
};
const downloadBoardImage = async () => {
    try {
        const canvas = await generateBoardCanvas();
        const link = document.createElement("a");
        link.download = `chess_position_${new Date().getTime()}.png`;
        link.href = canvas.toDataURL("image/png");
        link.click();
    } catch (err) {
        console.error("Failed to download board image:", err);
    }
    showShareMenu.value = false;
};
const downloadMoveText = () => {
    const moveText = moveRecords
        .map((record) => {
            return `${record.title}\n${record.details.join("\n")}`;
        })
        .join("\n\n");
    const blob = new Blob([moveText], { type: "text/plain" });
    const link = document.createElement("a");
    link.download = `chess_moves_${new Date().getTime()}.txt`;
    link.href = URL.createObjectURL(blob);
    link.click();
    URL.revokeObjectURL(link.href);
    showShareMenu.value = false;
};
const generateBoardCanvas = async (): Promise<HTMLCanvasElement> => {
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d");
    if (!ctx) throw new Error("Failed to get canvas context");
    // Set canvas size to match board size
    canvas.width = 624;
    canvas.height = 694;
    // Draw board
    const boardImg = new Image();
    boardImg.src = chessboardImage;
    await new Promise((resolve) => (boardImg.onload = resolve));
    ctx.drawImage(boardImg, 0, 0, canvas.width, canvas.height);
    // Draw pieces
    for (const piece of chessPieces) {
        const pieceImg = new Image();
        pieceImg.src = piece.image;
        await new Promise((resolve) => (pieceImg.onload = resolve));
        const x = piece.x * (canvas.width / 9);
        const y = piece.y * (canvas.height / 10);
        ctx.drawImage(pieceImg, x, y, 60, 60);
    }
    return canvas;
};
const toggleEditMode = () => {
    editModeEnabled.value = !editModeEnabled.value;
};
const importFEN = async () => {
    try {
        // 如果剪贴板有内容且是FEN格式，自动填充
        const clipboardText = await navigator.clipboard.readText();
        if (clipboardText.match(/^[\w\d/\s-]+\s[bw]\s[-\w]+$/)) {
            fenString.value = clipboardText;
        }
        // 这里添加FEN字符串验证和导入逻辑
        if (fenString.value.trim()) {
            // TODO: 实现FEN导入逻辑
            showImportDialog.value = false;
            fenString.value = "";
        }
    } catch (err) {
        console.error("Failed to import FEN:", err);
    }
};
const selectedPiece = ref<number | null>(null);

const selectPiece = (piece: string) => {
    // Handle piece selection in edit mode
};

const handlePieceClick = (index: number) => {
    if (selectedPiece.value === null) {
        selectedPiece.value = index;
    } else {
        // Deselect if clicking the same piece
        if (selectedPiece.value === index) {
            selectedPiece.value = null;
            return;
        }

        // Add moving animation class
        const pieces = document.querySelectorAll(".chess-piece");
        pieces[index].classList.add("moving");

        // Remove animation class after animation completes
        setTimeout(() => {
            pieces[index].classList.remove("moving");
        }, 300);

        // Reset selection
        selectedPiece.value = null;
    }
};

const chartContainer = ref(null);
let chart: any = null;

// 点击外部关闭分享菜单
onMounted(() => {
    document.addEventListener("click", (event) => {
        // 初始化图表
        if (chartContainer.value) {
            chart = echarts.init(chartContainer.value);
            const option = {
                animation: false,
                grid: {
                    top: 20,
                    right: 20,
                    bottom: 20,
                    left: 50,
                },
                xAxis: {
                    type: "category",
                    data: ["0", "10", "20", "30", "40", "50", "60"],
                    axisLine: {
                        lineStyle: {
                            color: "#ccc",
                        },
                    },
                    axisLabel: {
                        color: "#666",
                    },
                },
                yAxis: {
                    type: "value",
                    min: -200,
                    max: 200,
                    interval: 100,
                    axisLine: {
                        lineStyle: {
                            color: "#ccc",
                        },
                    },
                    axisLabel: {
                        color: "#666",
                        formatter: "{value}",
                    },
                },
                series: [
                    {
                        data: [0, 20, 10, 30, 15, 5, -10, -5, 0, 10],
                        type: "line",
                        smooth: true,
                        symbol: "none",
                        lineStyle: {
                            color: "green",
                        },
                        areaStyle: {
                            color: {
                                type: "linear",
                                x: 0,
                                y: 0,
                                x2: 0,
                                y2: 1,
                                colorStops: [
                                    {
                                        offset: 0,
                                        color: "rgba(59, 130, 246, 0.2)",
                                    },
                                    {
                                        offset: 1,
                                        color: "rgba(59, 130, 246, 0.05)",
                                    },
                                ],
                            },
                        },
                    },
                ],
            };
            chart.setOption(option);
            window.addEventListener("resize", () => {
                chart && chart.resize();
            });
        }

        const target = event.target as HTMLElement;
        if (!target.closest(".share-menu") && !target.closest("[data-share-button]")) {
            showShareMenu.value = false;
        }
    });
});
</script>

<style scoped>
.chess-board {
    position: relative;
    margin: 0 auto;
}
.chess-piece {
    transition: all 0.3s ease;
    user-select: none;
    position: relative;
}
.chess-piece:hover {
    transform: scale(1.05);
    cursor: pointer;
}
.chess-piece.selected {
    transform: scale(1.15);
    box-shadow: 0 0 15px rgba(52, 34, 241, 0.912);
    border-radius: 50%;
}
.chess-piece.moving {
    animation: movePiece 0.3s ease;
}
@keyframes movePiece {
    0% {
        transform: scale(1);
    }
    50% {
        transform: scale(1.1);
    }
    100% {
        transform: scale(1);
    }
}
.piece-container {
    position: relative;
    z-index: 10;
}
.move-record {
    border: 1px solid #e2e8f0;
    transition: all 0.2s;
}
.move-record:hover {
    background-color: #f0f4f8;
    cursor: pointer;
}
.trend-line {
    width: 4px;
    transition: all 0.3s ease;
}
.trend-line-up {
    transform: translateY(-50%);
}
.trend-line-down {
    transform: translateY(50%);
}
</style>
