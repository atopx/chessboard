<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { NButton, NCard, NFlex, NForm, NFormItem, NInputNumber, NSelect } from 'naive-ui';
import { onMounted, ref } from 'vue';

const options = [
    {
        label: "本地分析",
        value: "Offline",
    },
    {
        label: "JJ象棋",
        value: "JJ象棋",
    },
    {
        label: "天天象棋",
        value: "天天象棋",
    },
]

interface EngineConfig {
    depth: number,
    time: number,
    threads: number,
    hash: number,
}

const appName = ref('本地分析')

const config = ref({
    depth: 24,
    time: 3.0,
    threads: 4,
    hash: 1024,
})

onMounted(async () => {
    await getEngineConfig()
})

async function copy_fen() {}

async function stopListen() {
    await invoke("stop_listen");
}

async function startListen() {
    await invoke("start_listen", { name: appName.value });
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

async function setEngineHash() {
    await invoke("set_engine_hash", { size: config.value.hash });
}

async function getEngineConfig() {
    let result: EngineConfig = await invoke("get_engine_config");
    config.value = result;
}

</script>

<template>
    <n-card class="toolbar" :bordered="false" content-style="padding: 0px;">
        <n-form inline :label-width="80" :model="config" style="font-family: xiaoli;">
            <n-form-item label="模式">
                <n-select size="small" v-model:value="appName" :options="options"
                    :consistent-menu-width="false" style="width: 110px;" />
            </n-form-item>
            <n-form-item label="深度">
                <n-input-number size="small" v-model:value="config.depth"
                    button-placement="both" :min="0" :max="200" style="width: 82px;" @update:value="setEngineDepth"/>
            </n-form-item>
            <n-form-item label="时间">
                <n-input-number size="small" v-model:value="config.time"
                    button-placement="both" :step="0.5" :precision="1" :min="0" :max="120" style="width: 84px;"  @update:value="setEngineTime"/>
            </n-form-item>

            <n-form-item label="线程数">
                <n-input-number size="small" v-model:value="config.threads"  @update:value="setEngineThreads"
                    button-placement="both" :min="0" :max="64" style="width: 70px;"  />
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