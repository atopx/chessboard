<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { NButton, NCard, NFlex, NForm, NFormItem, NInputNumber, NSelect } from "naive-ui";
import { onMounted, ref, h } from "vue";
import { useDialog } from "naive-ui";

const options = [
    {
        label: "人机对弈",
        value: "Offline",
    },
    {
        label: "连线对战",
        value: "LinkPlay",
    },
    {
        label: "连线分析",
        value: "LinkAnaly",
    },
];

interface EngineConfig {
    depth: number;
    time: number;
    threads: number;
    hash: number;
}

const mode = ref(options[0].value);

const config = ref({
    depth: 24,
    time: 3.0,
    threads: 4,
    hash: 1024,
});

onMounted(async () => {
    await getEngineConfig();
});

async function copy_fen() {}

async function stopListen() {
    await invoke("stop_listen");
}

interface Window {
    id: number;
    title: string;
    app_name: string;
    width: number;
    height: number;
}

const dialog = useDialog();

async function startListen() {
    try {
        // 获取窗口列表
        const windows: Window[] = await invoke("list_windows");

        if (windows.length === 0) {
            dialog.warning({
                title: "警告",
                content: "没有找到可用的窗口",
            });
            return;
        }

        // 显示窗口选择对话框
        dialog.info({
            title: "选择窗口",
            content: () =>
                h(NSelect, {
                    value: selectedWindowId.value,
                    "onUpdate:value": (val: number) => (selectedWindowId.value = val),
                    options: windows.map((w) => ({ label: w.title, value: w.id })),
                    placeholder: "请选择要监听的窗口",
                }),
            positiveText: "确定",
            negativeText: "取消",
            onPositiveClick: async () => {
                if (!selectedWindowId.value) return;

                const window = windows.find((w) => w.id === selectedWindowId.value);
                if (window) {
                    try {
                        await invoke("start_listen", { target: window });
                    } catch (error) {
                        dialog.error({
                            title: "错误",
                            content: "启动监听失败:" + String(error),
                        });
                    }
                }
            },
        });

        const selectedWindowId = ref<number | null>(null);
    } catch (error) {
        console.error("启动监听失败:", error);
        dialog.error({
            title: "错误",
            content: "启动监听失败: " + String(error),
        });
    }
}

async function setEngineDepth() {
    await invoke("set_engine_depth", { depth: config.value.depth });
}

async function setEngineTime() {
    await invoke("set_engine_time", { time: config.value.time });
}
async function setEngineThreads() {
    await invoke("set_engine_threads", { num: config.value.threads });
}

// async function setEngineHash() {
//     await invoke("set_engine_hash", { size: config.value.hash });
// }

async function getEngineConfig() {
    let result: EngineConfig = await invoke("get_engine_config");
    config.value = result;
}
</script>

<template>
    <n-card class="toolbar" :bordered="false" content-style="padding: 0px;">
        <n-form inline :label-width="80" :model="config">
            <n-form-item label="模式">
                <n-select
                    size="small"
                    v-model:value="mode"
                    :options="options"
                    :consistent-menu-width="false"
                    style="width: 110px"
                />
            </n-form-item>
            <n-form-item label="深度">
                <n-input-number
                    size="small"
                    v-model:value="config.depth"
                    button-placement="both"
                    :min="0"
                    :max="200"
                    style="width: 82px"
                    @update:value="setEngineDepth"
                />
            </n-form-item>
            <n-form-item label="时间">
                <n-input-number
                    size="small"
                    v-model:value="config.time"
                    button-placement="both"
                    :step="0.5"
                    :precision="1"
                    :min="0"
                    :max="120"
                    style="width: 84px"
                    @update:value="setEngineTime"
                />
            </n-form-item>

            <n-form-item label="线程数">
                <n-input-number
                    size="small"
                    v-model:value="config.threads"
                    @update:value="setEngineThreads"
                    button-placement="both"
                    :min="0"
                    :max="64"
                    style="width: 70px"
                />
            </n-form-item>

            <n-flex>
                <n-flex vertical>
                    <n-button size="small" tertiary type="info" @click="startListen">启动引擎</n-button>
                    <n-button size="small" tertiary type="info" @click="stopListen">停止引擎</n-button>
                </n-flex>

                <n-flex vertical>
                    <n-button size="small" tertiary type="info">图片识别</n-button>
                    <n-button size="small" tertiary type="info" @click="copy_fen">复制局面</n-button>
                </n-flex>
            </n-flex>
        </n-form>
    </n-card>
</template>

<style scoped>
.toolbar {
    width: 650px;
    height: 100px;
    left: 10px;
    top: 4px;
}

button {
    width: 100px;
}
</style>
