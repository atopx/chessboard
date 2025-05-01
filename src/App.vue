<script setup lang="ts">
import { NDivider, NLayoutFooter, NDialogProvider } from "naive-ui";
import Analyse from "./components/Analyse.vue";
import Chessboard from "./components/Chessboard.vue";
import Toolbar from "./components/Toolbar.vue";
</script>

<template>
    <n-dialog-provider>
        <!-- 最上面 -->
        <Toolbar />

        <n-divider class="spliter-toolbar" />

        <!-- 下方左侧 -->
        <Chessboard />

        <n-divider vertical class="spliter-middle" />

        <!-- 下方右侧 -->
        <Analyse />

        <n-layout-footer class="footer" position="absolute">
            <a href="https://github.com/atopx/chessboard.git">atopx: 中国象棋学习工具, 开源免费</a>
        </n-layout-footer>
    </n-dialog-provider>
</template>

<style scoped>
.footer {
    left: 13px;
    bottom: 5px;
    font-size: x-small;
    font-family: xiaoli;
}

.spliter-toolbar {
    position: absolute;
    width: 630px;
    top: 60px;
    left: 10px;
    z-index: 1;
}

.spliter-middle {
    position: absolute;
    left: 398px;
    top: 100px;
    height: 425px;
    z-index: 1;
}
</style>
