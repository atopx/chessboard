<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { FormInst, NButton, NCard, NFlex, NForm, NFormItem, NInputNumber, NSelect } from 'naive-ui';
import { ref } from 'vue';

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

const formRef = ref<FormInst | null>(null)

const engineConfig = ref({
    appName: '本地分析',
    engineDepth: 24,
    engineTime: 3.0,
    enableYunku: 'true',
    engineThreads: 4,
})

async function stopListen() {
    await invoke("stop_listen");
}

async function startListen() {
    let result = await invoke("start_listen", { name: "JJ象棋" });
    console.log(result);
}

</script>

<template>
    <n-card class="toolbar" :bordered="false" content-style="padding: 0px;">
        <n-form ref="formRef" inline :label-width="80" :model="engineConfig" style="font-family: xiaoli;">
            <n-form-item label="模式" path="appName">
                <n-select size="small" v-model:value="engineConfig.appName" :options="options"
                    :consistent-menu-width="false" style="width: 110px;" />
            </n-form-item>
            <n-form-item label="深度" path="engineDepth">
                <n-input-number size="small" v-model:value="engineConfig.engineDepth" placeholder=""
                    button-placement="both" :min="0" :max="200" style="width: 82px;" />
            </n-form-item>
            <n-form-item label="时间" path="engineTime">
                <n-input-number size="small" v-model:value="engineConfig.engineTime" placeholder=""
                    button-placement="both" :step="0.5" :precision="1" :min="0" :max="120" style="width: 84px;" />
            </n-form-item>

            <n-form-item label="线程数" path="engineThreads">
                <n-input-number size="small" v-model:value="engineConfig.engineThreads" placeholder=""
                    button-placement="both" :min="0" :max="64" style="width: 70px;" />
            </n-form-item>

            <n-flex>
                <n-flex vertical>
                    <n-button size="small" tertiary type="info" @click="startListen">启动引擎</n-button>
                    <n-button size="small" tertiary type="info" @click="stopListen">停止引擎</n-button>
                </n-flex>

                <n-flex vertical>
                    <n-button size="small" tertiary type="info">图片识别</n-button>
                    <n-button size="small" tertiary type="info">复制局面</n-button>
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