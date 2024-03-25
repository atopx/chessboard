<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { LogInst, NCard, NDivider, NFlex, NLog, NText } from 'naive-ui';
import { nextTick, onMounted, ref, watchEffect } from 'vue';

interface Analyse {
    depth: number,   // 深度
    score: number,   // 得分
    time: number,    // 时间
    pvs: string[],   // 思考(iccs)
    moves: string[], // 思考(chinese)
    state: string,   // 状态
    source: string,  // 来源
}


const logs = ref([""])
const logRef = ref(logs)
const best = ref({
    move: "---",
    depth: 0,
    score: 0,
})

listen('analyse', async (event) => {
    let data = event.payload as Analyse;
    let mvs = data.moves.join(" ");
    logs.value.push(`>>> ${mvs}`)

    best.value.move = data.moves[0];
    best.value.depth = data.depth;
    best.value.score = data.score;

    // 移除旧的b-select
    document.querySelectorAll(".b-select").forEach(element => {
        element.classList.remove("b-select")
    });

    // 设置新的b-select
    let pv = data.pvs[0];
    let from = pv.substring(0, 2);
    let to = pv.substring(2, 4)
    document.getElementById(from)?.classList.add("b-select");
    document.getElementById(to)?.classList.add("b-select");
})


const logInstRef = ref<LogInst | null>(null)


onMounted(() => {
    watchEffect(() => {
        if (logRef.value) {
            nextTick(() => {
                if (logs.value.length > 50) {
                    // 只保留50条, 从左边删除1条
                    logs.value.shift();
                }
                logInstRef.value?.scrollTo({ position: 'bottom', silent: true })
            })
        }
    })
})

const fenType = ref("error")

</script>

<template>
    <n-card title="局面分析" :bordered="false" class="textlog" content-style="color: blue">
        <n-flex justify="space-between" align="end">
            <n-text type="info" class="analyse-title" strong>
                {{ best.move }}
            </n-text>

            <n-text :type="fenType">
                {{ best.score }}
            </n-text>
            <n-text type="warning">
                {{ best.depth }}
            </n-text>
        </n-flex>
        <n-divider />
        <n-log :rows=18 ref="logInst" :line-height="1.5" :lines="logs" :font-size="10" />
    </n-card>
</template>

<style scoped>
.analyse-title {
    font-size: x-large;
    font-family: xiaoli;
}

.textlog {
    width: 260px;
    height: 440px;
    left: 400px;
    top: 0px;
    font-family: qiti;
}
</style>